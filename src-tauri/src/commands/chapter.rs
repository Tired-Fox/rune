use std::{
    collections::{HashSet, VecDeque},
    path::PathBuf,
    sync::Arc,
};

use manrex::{
    model::{
        chapter::{Chapter, ChapterFilter},
        Paginated,
    },
    ChapterId, MangaId,
};
use tauri::{async_runtime, ipc::Channel, Manager, ResourceId, Runtime, State, Webview};
use tokio::{sync::Mutex, task::JoinSet};

use crate::{commands::{rm_cache, CacheTarget}, natural_compare, task::AbortSender, SharedClient, PNAME};

#[tauri::command]
pub async fn get_chapters(
    client: State<'_, SharedClient>,
    filters: Option<ChapterFilter>,
) -> Result<Paginated<Chapter>, tauri::Error> {
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
    client: State<'_, SharedClient>,
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
pub async fn get_pages<R: Runtime>(
    webview: Webview<R>,
    client: State<'_, SharedClient>,
    manga: MangaId,
    chapter: ChapterId,
    on_page: Channel<Vec<(usize, PathBuf)>>,
) -> Result<ResourceId, tauri::Error> {
    let (tx, mut rx) = tokio::sync::oneshot::channel::<()>();
    let abort_sender = AbortSender(tx);
    let mut rt = webview.resources_table();
    let rid = rt.add(abort_sender);

    let client = client.inner().clone();
    async_runtime::spawn(async move {
        let client = client.clone();
        let base = std::env::temp_dir().join(PNAME).join(manga.as_ref());
        let mut client = client.lock().await;
        let mut workers = JoinSet::new();
        let mut fetched = false;

        match client.as_mut() {
            None => return Err(tauri::Error::Anyhow(anyhow::anyhow!(
                "client is not available"
            ))),
            Some(client) => {
                let path = base.join(chapter.as_ref());

                if path.exists() {
                    workers.spawn(async move {
                        let mut paths = std::fs::read_dir(&path)?
                            .flatten()
                            .filter_map(|entry| {
                                let path = entry.path();
                                let file_stem = path.file_stem().unwrap().to_str().unwrap();
                                (path.is_file() && file_stem.starts_with("page")).then_some(entry.path())
                            })
                            .collect::<Vec<_>>();
                        paths.sort_by(|a, b| natural_compare(a.to_str().unwrap(), b.to_str().unwrap()));
                        on_page.send(paths.into_iter().enumerate().collect::<Vec<_>>())?;

                        Ok::<(), anyhow::Error>(())
                    });
                } else {
                    fetched = true;
                    std::fs::create_dir_all(&path)?;
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
                    for _ in 0..5 {
                        let queue = queue.clone();
                        let on_page = on_page.clone();
                        let path = path.clone();
                        workers.spawn(async move {
                            loop {
                                let next = queue.lock().await.pop_front();
                                match next {
                                    None => break,
                                    Some((index, image)) => {
                                        let stream = image.fetch().await?;
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
                                            .await?;
                                        stream.stream_to(&mut file).await?;
                                        on_page.send(vec![(index, path.join(format!("page-{index}{ext}")))])?;
                                    }
                                }
                            }

                            Ok::<(), anyhow::Error>(())
                        });
                    }
                }
            }
        };

        loop {
            let manga = manga.clone();
            tokio::select! {
                _ = &mut rx => {
                    workers.abort_all();
                    if fetched {
                        rm_cache(vec![CacheTarget::Specific {
                            manga,
                            cover: false,
                            chapters: HashSet::from([chapter.to_string()]),
                        }])?;
                    }
                },
                results = workers.join_next() => {
                    match results {
                        Some(result) => result??,
                        None => break,
                    }
                }
            }
        }

        Ok(())
    });

    Ok(rid)
}
