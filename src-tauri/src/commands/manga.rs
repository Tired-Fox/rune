use manrex::{
    model::{
        cover::CoverSize,
        manga::{Manga, MangaFilter, MangaInclude},
        Paginated,
    },
    Client, MangaId,
};
use tauri::State;
use tokio::{fs::OpenOptions, sync::Mutex};

use crate::PNAME;

#[tauri::command]
pub async fn list_manga(
    client: State<'_, Mutex<Option<Client>>>,
    filters: Option<MangaFilter>,
) -> Result<Paginated<Vec<Manga>>, tauri::Error> {
    let base = std::env::temp_dir().join(PNAME);

    let mut client = client.lock().await;
    match client.as_mut() {
        None => Err(tauri::Error::Anyhow(anyhow::anyhow!(
            "client is not available"
        ))),
        Some(client) => {
            let response = client
                .list_manga(filters)
                .await
                .map_err(anyhow::Error::new)?;

            let ids = response.data.iter().map(|v| v.id.as_ref().to_string()).collect::<Vec<_>>();
            for manga in std::fs::read_dir(&base)?.flatten() {
                let id = manga.path().file_stem().unwrap().to_str().unwrap().to_string();
                if !ids.contains(&id) {
                  let _ = std::fs::remove_dir_all(manga.path());
                }
            }

            Ok(response)
        }
    }
}

#[tauri::command]
pub async fn get_cover_art(
    manga: Manga,
    size: Option<CoverSize>,
) -> Result<String, tauri::Error> {
    let id = manga.id.as_ref();
    let base = std::env::temp_dir().join(PNAME);


    let base_name = match size {
        Some(size) => format!("cover-{size}.png"),
        None => "cover.jpg".to_string(),
    };

    let cover_path = base.join(id).join(&base_name);
    if cover_path.exists() {
        println!("HIT [COVER-ART]");
        return Ok(cover_path.display().to_string())
    }

    if !base.join(id).exists() {
        std::fs::create_dir_all(base.join(id))?;
    }

    let image = manga
        .get_cover_art(size)
        .map_err(anyhow::Error::new)?;
    let stream = image
        .fetch()
        .await
        .map_err(anyhow::Error::new)?;

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&cover_path)
        .await?;

    stream.stream_to(&mut file).await.map_err(anyhow::Error::new)?;

    Ok(cover_path.display().to_string())
}

#[tauri::command]
pub async fn get_manga(
    client: State<'_, Mutex<Option<Client>>>,
    id: MangaId,
    includes: Option<Vec<MangaInclude>>,
) -> Result<Manga, tauri::Error> {
    let mut client = client.lock().await;
    match client.as_mut() {
        None => Err(tauri::Error::Anyhow(anyhow::anyhow!(
            "client is not available"
        ))),
        Some(client) => Ok(client
            .get_manga(id, includes)
            .await
            .map_err(anyhow::Error::new)?),
    }
}
