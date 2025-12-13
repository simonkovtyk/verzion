use std::env;
use serde::{Deserialize, Serialize};

use crate::git::remote::get_remote_url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum WebhookType {
  Custom,
  GitHub,
  GitLab
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookItemConfig {
  pub r#type: Option<WebhookType>,
  pub origin: Option<String>,
  pub enabled: Option<bool>,
  pub url: Option<String>,
  pub token: Option<String>,
  pub token_env: Option<String>,
  pub http_retries: Option<u32>
}

impl WebhookItemConfig {
  pub fn is_enabled (&self) -> bool {
    let is_empty = self.is_empty();

    if is_empty {
      return false;
    }

    self.enabled.unwrap_or(true)
  }

  pub fn is_empty (&self) -> bool {
    self.r#type.is_none() && self.origin.is_none() && self.enabled.is_none() && self.url.is_none() && self.token.is_none() && self.token_env.is_none() && self.http_retries.is_none()
  }

  pub fn new (
    r#type: Option<WebhookType>,
    origin: Option<String>,
    enabled: Option<bool>,
    url: Option<String>,
    token: Option<String>,
    token_env: Option<String>,
    http_retries: Option<u32>
  ) -> Option<Self> {
    let instance = Self {
      r#type,
      origin,
      enabled,
      url,
      token,
      token_env,
      http_retries
    };

    if instance.is_empty() {
      None
    } else {
      Some(instance)
    }
  }

  pub fn get_token (&self) -> Option<String> {
    if let Some(token) = self.token.clone() {
      return Some(token);
    }

    if let Some(token_env) = self.token_env.clone() {
      return env::var(token_env).ok();
    }

    None
  }

  pub fn get_url (&self) -> Option<String> {
    if self.url.is_some() {
      return self.url.clone();
    }

    return get_remote_url(self.origin.as_deref());
  }
}

pub type WebhookConfig = Vec<WebhookItemConfig>;

