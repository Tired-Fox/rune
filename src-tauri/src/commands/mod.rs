use std::collections::HashSet;

use manrex::MangaId;
use serde::Deserialize;

use crate::PNAME;

pub mod account;
pub mod manga;
pub mod chapter;
pub mod list;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum CacheTarget {
    Id(String),
    Specific {
        manga: MangaId,

        #[serde(default)]
        cover: bool,
        #[serde(default)]
        chapters: HashSet<String>,
    },
}

#[tauri::command]
pub async fn remove_cache(targets: Vec<CacheTarget>) -> Result<(), tauri::Error> {
    Ok(rm_cache(targets)?)
}

/// Remove cached images and data given a manga, cover art, and/or chapter
pub(crate) fn rm_cache(targets: Vec<CacheTarget>) -> anyhow::Result<()> {
    for target in targets {
        match target {
            CacheTarget::Id(id) => {
                let base = std::env::temp_dir().join(PNAME).join(&id);
                if base.exists() {
                    std::fs::remove_dir_all(&base)?;
                }
            }
            CacheTarget::Specific {
                manga,
                cover,
                chapters,
            } => {
                let base = std::env::temp_dir().join(PNAME).join(manga.as_ref());
                for entry in std::fs::read_dir(&base)?.flatten() {
                    if entry.path().is_file() && entry.path().starts_with("cover") && cover {
                        std::fs::remove_file(entry.path())?;
                    } else if entry.path().is_dir() {
                        let id = entry.path();
                        let id = id.file_stem().unwrap().to_str().unwrap();
                        if chapters.contains(id) {
                            std::fs::remove_dir_all(base.join(id))?;
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
