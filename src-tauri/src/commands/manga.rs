use base64::Engine;
use futures_util::StreamExt;
use manrex::{model::{cover::CoverSize, manga::{Manga, MangaFilter}, Paginated}, Client};
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn list_manga(client: State<'_, Mutex<Option<Client>>>, filters: Option<MangaFilter>) -> Result<Paginated<Vec<Manga>>, tauri::Error> {
  let mut client = client.lock().await;
  match client.as_mut() {
      None => Err(tauri::Error::Anyhow(anyhow::anyhow!("client is not available"))),
      Some(client) => {
        Ok(client
          .list_manga(filters)
          .await
          .map_err(anyhow::Error::new)?)
      }
  }
}

#[tauri::command]
pub async fn get_cover_image(manga: Manga, size: Option<CoverSize>) -> Result<(Option<String>, String), tauri::Error> {
    let (mime, mut stream) = manga
        .get_cover_art(size)
        .map_err(anyhow::Error::new)?
        .fetch()
        .await
        .map_err(anyhow::Error::new)?;

    let mut bytes = Vec::new();
    while let Some(Ok(chunk)) = stream.next().await {
        bytes.extend(chunk); 
    }

    Ok((mime, base64::engine::general_purpose::STANDARD.encode(&bytes)))
}
