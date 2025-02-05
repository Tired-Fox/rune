use std::collections::BTreeMap;

use manrex::{
    model::{
        cover::CoverSize,
        manga::{Manga, MangaFilter, MangaInclude, Tag, Volume},
        Paginated,
    }, GroupId, MangaId, TagId
};
use tauri::State;
use tokio::fs::OpenOptions;

use crate::{SharedClient, PNAME};

use super::{rm_cache, CacheTarget};

#[tauri::command]
pub async fn list_manga(
    client: State<'_, SharedClient>,
    filters: Option<MangaFilter>,
) -> Result<Paginated<Manga>, tauri::Error> {
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

            let ids = response.data
                .iter()
                .map(|v| v.id.as_ref().to_string())
                .collect::<Vec<_>>();

            rm_cache(
                std::fs::read_dir(&base)?
                    .flatten()
                    .filter_map(|m| {
                        let id = m.path().file_stem().unwrap().to_str().unwrap().to_string();
                        (!ids.contains(&id)).then_some(CacheTarget::Id(id))
                    })
                    .collect()
            )?;

            Ok(response)
        }
    }
}

#[tauri::command]
pub async fn get_cover_art(
    manga: Manga,
    size: Option<CoverSize>,
) -> Result<String, tauri::Error> {
    let image = manga
        .get_cover_art(size)
        .map_err(anyhow::Error::new)?;

    Ok(image.link().to_string())
}

#[tauri::command]
pub async fn get_manga(
    client: State<'_, SharedClient>,
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

#[tauri::command]
pub async fn get_tags(
    client: State<'_, SharedClient>,
    tags: Vec<TagId>,
) -> Result<Vec<Tag>, tauri::Error> {
    let mut client = client.lock().await;
    match client.as_mut() {
        None => Err(tauri::Error::Anyhow(anyhow::anyhow!(
            "client is not available"
        ))),
        Some(client) => {
            Ok(crate::cache::tag::get_tags(client, Some(tags)).await?)
        },
    }
}

#[tauri::command]
pub async fn get_volumes_and_chapters(
    client: State<'_, SharedClient>,
    id: MangaId,
    translated_language: Option<Vec<String>>,
    group: Option<Vec<GroupId>>,
) -> Result<BTreeMap<String, Volume>, tauri::Error> {
    let mut client = client.lock().await;
    match client.as_mut() {
        None => Err(tauri::Error::Anyhow(anyhow::anyhow!(
            "client is not available"
        ))),
        Some(client) => Ok(client
            .get_manga_volumes_and_chapters(id, translated_language, group)
            .await
            .map_err(anyhow::Error::new)?),
    }
}
