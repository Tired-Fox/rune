use manrex::{model::manga::Tag, Client, TagId};

use crate::PNAME;


/// Fetches manga tags
///
/// Optionally filter and only retrieve the listed tag ids
pub async fn get_tags(client: &mut Client, filter: Option<Vec<TagId>>) -> anyhow::Result<Vec<Tag>> {
    let base = dirs::cache_dir().unwrap().join(PNAME);

    if base.join("tags.json").exists() {
        let content = std::fs::read_to_string(base.join("tags.json"))?;
        Ok(serde_json::from_str::<Vec<Tag>>(&content)?
          .into_iter()
          .filter(|t| filter.as_ref().map(|v| v.contains(&t.id)).unwrap_or(true))
          .collect())
    } else {
        let tags = client.get_manga_tag_list().await?;
        if !base.exists() {
            std::fs::create_dir_all(&base)?;
        }
        std::fs::write(base.join("tags.json"), serde_json::to_string(&tags.data)?)?;
        Ok(
          tags.data
              .into_iter()
              .filter(|t| filter.as_ref().map(|v| v.contains(&t.id)).unwrap_or(true))
              .collect()
        )
    }
}
