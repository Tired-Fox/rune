use manrex::{auth::OAuth, Client};
use tauri::State;
use tokio::sync::Mutex;

use crate::{model::account::{Account, Creds}, PNAME};

#[tauri::command]
pub async fn fetch_account(account: State<'_, Mutex<Account>>) -> Result<Account, tauri::Error> {
  Ok(account.lock().await.clone())
}

#[tauri::command]
pub async fn login(client: State<'_, Mutex<Option<Client>>>, creds: Creds, username: String, password: String) -> Result<(), tauri::Error> {
  let mut client = client.lock().await;
  if client.is_none() {
    println!("INIT CLIENT");
    client.replace(Client::new(OAuth::new_with_cache(creds.decode(), dirs::cache_dir().unwrap().join(PNAME))));
  }

  let c = client.as_mut().unwrap();

  let oauth = c.oauth_mut();
  let credentials = creds.decode();

  if oauth.credentials() != &credentials {
    println!("REPLACE CREDENTIALS");
    std::fs::write(dirs::cache_dir().unwrap().join(PNAME).join("client.json"), serde_json::to_string(&creds)?)?;
    oauth.set_credentials(credentials);
  }

  c
    .oauth_mut()
    .login_with(username, password)
    .await
    .map_err(anyhow::Error::new)?;

  Ok(())
}

#[tauri::command]
pub async fn logout(client: State<'_, Mutex<Option<Client>>>, account: State<'_, Mutex<Account>>) -> Result<(), tauri::Error> {
  account.lock().await.logged_in = false;
  if let Some(client) = client.lock().await.as_mut() {
    client.oauth().logout().map_err(anyhow::Error::new)?;
  }
  Ok(())
}
