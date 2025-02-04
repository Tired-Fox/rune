use base64::{Engine, engine::general_purpose::STANDARD};
use manrex::auth::Credentials;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Creds {
  pub(crate) id: Option<String>,
  /// Base64 Encoded Secret
  pub(crate) secret: Option<String>,
}

impl Creds {
  pub fn decode(&self) -> Credentials {
    Credentials::new(
      self.id.clone().unwrap_or_default(),
      String::from_utf8_lossy(&STANDARD.decode(self.secret.clone().unwrap_or_default()).unwrap())
    )
  }

  pub fn encode(creds: Credentials) -> Self {
    Self {
      id: Some(creds.id().into()),
      secret: Some(STANDARD.encode(creds.secret())),
    }
  }
}

#[derive(Default, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Account {
  pub(crate) logged_in: bool,
  pub(crate) creds: Creds,
}
