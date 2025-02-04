use manrex::{auth::{Credentials, OAuth}, Client};
use model::account::Account;
use tauri::Manager;
use tokio::sync::Mutex;

pub mod commands;
pub mod model;
pub mod cache;

pub static PNAME: &str = "com.rune.manga";

fn load_account() -> Result<(Account, Option<Client>), manrex::Error> {
  let path = dirs::cache_dir().unwrap().join(PNAME).join("client.json");
  match Credentials::from_env() {
    // Prioritize environment variables for credentials
    Ok(creds) => {
      let client = Client::new(OAuth::new_with_cache(creds.clone(), dirs::cache_dir().unwrap().join(PNAME)));
      Ok((
        Account {
          logged_in: client.oauth().logged_in(),
          creds: model::account::Creds::encode(creds),
        },
        Some(client),
      ))
    },
    Err(_) => {
      // If no environment variables, then allow the user to specify them in a stashed
      // location
      if path.exists() {
        let content = std::fs::read_to_string(&path)?;
        let uclient = serde_json::from_str::<model::account::Creds>(&content)?;

        let creds = uclient.clone().decode();
        let client = Client::new(OAuth::new_with_cache(creds, dirs::cache_dir().unwrap().join(PNAME)));
        Ok((
          Account {
            logged_in: client.oauth().logged_in(),
            creds: uclient,
          },
          Some(client)
        ))
      } else {
        Ok(Default::default())
      }
    }
  }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  //setup();

  tauri::Builder::default()
    .setup(|app| {
      let (account, client) = load_account().unwrap_or_default();
      app.manage(Mutex::new(account));
      app.manage(Mutex::new(client));

      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
        commands::account::fetch_account,
        commands::account::login,
        commands::account::logout,

        commands::manga::list_manga,
        commands::manga::get_cover_art,
        commands::manga::get_manga,
        commands::manga::get_tags,
        commands::manga::get_volumes_and_chapters,

        commands::chapter::get_chapters,
        commands::chapter::get_chapter,
        commands::chapter::get_pages,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  //cleanup()
}

fn setup() {
  let path = std::env::temp_dir().join(PNAME);
  if !path.exists() {
    std::fs::create_dir_all(path).expect("failed to setup temporary directory");
  }
}

fn cleanup() {
  println!("Cleanup");
  let path = std::env::temp_dir().join(PNAME);
  if path.exists() {
    std::fs::remove_dir_all(path).expect("failed to remove temporary directory")
  }
}
