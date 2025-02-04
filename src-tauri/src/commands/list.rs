#[derive(Serialize)]
pub struct List {
    manga: HashSet<String>,
    user: Option<String>,
    id: String,
    visibility: Visibility,
    version: usize,
}

impl From<&CustomList> for List {
    fn from(value: &CustomList) -> Self {
        let manga = value.relationships
            .iter()
            .filter_map(|relation| {
                relation.attributes.as_ref().and_then(|attributes| {
                    if attributes.is_manga() { Some(relation.id.to_string()) } else { None }
                })
            })
            .collect::<HashSet<_>>();

        let user = value.relationships
            .iter()
            .find(|relation| {
                relation.attributes.as_ref().map(|attributes| {
                    attributes.is_user()
                }).unwrap_or_default()
            })
            .map(|r| r.id.to_string());

        Self {
            manga,
            user,
            id: value.id.to_string(),
            visibility: value.attributes.visibility,
            version: value.attributes.version,
        }
    }
}

use std::collections::{BTreeMap, HashSet};

use manrex::{model::{custom_list::{CustomList, ListInclude}, Visibility}, ListId};
use serde::Serialize;
use tauri::State;

use crate::SharedClient;

#[tauri::command]
pub async fn get_list(
    client: State<'_, SharedClient>,
    id: ListId,
) -> tauri::Result<CustomList> {
    let mut client = client.lock().await;
    match client.as_mut() {
        None => Err(tauri::Error::Anyhow(anyhow::anyhow!(
            "client is not available"
        ))),
        Some(client) => {
            Ok(client.get_list(id, [ListInclude::User]).await.map_err(anyhow::Error::new)?)
        }
    }
}

#[tauri::command]
pub async fn get_lists(
    client: State<'_, SharedClient>,
    lists: Vec<ListId>,
) -> tauri::Result<BTreeMap<String, List>> {
    let mut client = client.lock().await;
    match client.as_mut() {
        None => Err(tauri::Error::Anyhow(anyhow::anyhow!(
            "client is not available"
        ))),
        Some(client) => {
            let mut result = BTreeMap::new();
            for list in lists {
                let list = client.get_list(list, [ListInclude::User]).await.map_err(anyhow::Error::new)?;
                result.insert(list.attributes.name.clone(), List::from(&list));
            }
            Ok(result)
        }
    }
}
