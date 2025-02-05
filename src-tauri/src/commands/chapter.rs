use manrex::{
    model::{
        chapter::{Chapter, ChapterFilter},
        Paginated,
    },
    ChapterId, MangaId,
};
use tauri::State;
use crate::SharedClient;

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
pub async fn get_pages(
    client: State<'_, SharedClient>,
    chapter: ChapterId,
    save: Option<bool>,
) -> Result<Vec<String>, tauri::Error> {
    let save = save.unwrap_or_default();
    let mut client = client.lock().await;
    match client.as_mut() {
        None => Err(tauri::Error::Anyhow(anyhow::anyhow!(
            "client is not available"
        ))),
        Some(client) => {
            let server = client
                .get_at_home_server(chapter.as_ref(), false)
                .await
                .map_err(anyhow::Error::new)?;

            if server.chapter.hash.as_ref().is_empty() {
                return Err(tauri::Error::Anyhow(anyhow::anyhow!("the selected scanlation group does not have any pages")))
            }

            let pages = if save {
                server.saver_images()
                    .into_iter()
                    .map(|v| v.link().to_string())
                    .collect()
            } else {
                server.images()
                    .into_iter()
                    .map(|v| v.link().to_string())
                    .collect()
            };

            Ok(pages)
        }
    }
}
