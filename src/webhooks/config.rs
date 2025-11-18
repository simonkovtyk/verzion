use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookConfig {
  pub enabled: Option<bool>,
  pub url: Option<String>,
  pub token: Option<String>,
  pub token_env: Option<String>
}

impl WebhookConfig {
  pub fn is_enabled (&self) -> bool {
    self.enabled.unwrap_or(true)
  }
}
