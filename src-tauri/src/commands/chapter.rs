use std::{
    collections::VecDeque,
    path::PathBuf,
    sync::Arc,
};

use manrex::{
    model::{
        chapter::{Chapter, ChapterFilter},
        Paginated,
    },
    ChapterId, Client, MangaId,
};
use tauri::{ipc::Channel, State};
use tokio::sync::Mutex;

use crate::PNAME;

#[tauri::command]
pub async fn get_chapters(
    client: State<'_, Mutex<Option<Client>>>,
    filters: Option<ChapterFilter>,
) -> Result<Paginated<Vec<Chapter>>, tauri::Error> {
    let mut client = client.lock().await;
    match client.as_mut() {
        None => Err(tauri::Error::Anyhow(anyhow::anyhow!(
            "client is not available"
        ))),
        Some(client) => Ok(client
            .list_chapters(filters)
            .await
            .map_err(anyhow::Error::new)?),
    }
}

#[tauri::command]
pub async fn get_chapter(
    client: State<'_, Mutex<Option<Client>>>,
    id: ChapterId,
) -> Result<Chapter, tauri::Error> {
    let mut client = client.lock().await;
    match client.as_mut() {
        None => Err(tauri::Error::Anyhow(anyhow::anyhow!(
            "client is not available"
        ))),
        Some(client) => Ok(client.get_chapter(id).await.map_err(anyhow::Error::new)?),
    }
}

#[tauri::command]
pub async fn get_pages(
    client: State<'_, Mutex<Option<Client>>>,
    manga: MangaId,
    chapter: ChapterId,
    on_page: Channel<(usize, PathBuf)>,
) -> Result<(), tauri::Error> {
    let base = std::env::temp_dir().join(PNAME).join(manga.as_ref());
    let mut client = client.lock().await;

    match client.as_mut() {
        None => Err(tauri::Error::Anyhow(anyhow::anyhow!(
            "client is not available"
        ))),
        Some(client) => {
            let path = base.join(chapter.as_ref());

            if path.exists() {
                let mut paths = std::fs::read_dir(&path)?
                    .flatten()
                    .filter_map(|entry| {
                        let path = entry.path();
                        let file_stem = path.file_stem().unwrap().to_str().unwrap();
                        (path.is_file() && file_stem.starts_with("page")).then_some(entry.path())
                    })
                    .collect::<Vec<_>>();
                paths.sort();
                for entry in paths.into_iter().enumerate() {
                    on_page.send(entry)?;
                }
                return Ok(());
            } else {
                std::fs::create_dir_all(&path)?;
            }

            let server = client
                .get_at_home_server(chapter.as_ref(), false)
                .await
                .map_err(anyhow::Error::new)?;

            if server.chapter.hash.as_ref().is_empty() {
                return Err(tauri::Error::Anyhow(anyhow::anyhow!("the selected scanlation group does not have any pages")))
            }

            // Use async tasks to split up the workload of fetching and downloading the images.
            //
            // 5 images will be downloaded at any given moment.
            let queue = Arc::new(Mutex::new(
                server
                    .saver_images()
                    .into_iter()
                    .enumerate()
                    .collect::<VecDeque<_>>(),
            ));
            let mut workers = Vec::new();
            for _ in 0..5 {
                let queue = queue.clone();
                let on_page = on_page.clone();
                let path = path.clone();
                workers.push(tokio::spawn(async move {
                    loop {
                        let next = queue.lock().await.pop_front();
                        match next {
                            None => break,
                            Some((index, image)) => {
                                let stream = image.fetch().await.unwrap();
                                let ext = if stream.mime.as_str() == "image/jpeg" {
                                    ".jpg"
                                } else {
                                    ".png"
                                };
                                let mut file = tokio::fs::OpenOptions::new()
                                    .truncate(true)
                                    .create(true)
                                    .write(true)
                                    .open(path.join(format!("page-{index}{ext}")))
                                    .await
                                    .unwrap();
                                stream.stream_to(&mut file).await.unwrap();
                                let _ =
                                    on_page.send((index, path.join(format!("page-{index}{ext}"))));
                            }
                        }
                    }
                }));
            }

            for worker in workers {
                worker.await?;
            }

            Ok(())
        }
    }
}
