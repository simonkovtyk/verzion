use std::env;
use serde::{Deserialize, Serialize};

use crate::{config::Config, git::remote::get_remote_url, std::command::CommandOptions, webhooks::{custom, github, gitlab}};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
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
  pub http_retries: Option<u32>,
  pub http_timeout: Option<u64>
}

impl WebhookItemConfig {
  #[allow(dead_code)]
  pub fn is_enabled (&self) -> bool {
    let is_empty = self.is_empty();

    if is_empty {
      return false;
    }

    self.enabled.unwrap_or(true)
  }

  pub fn is_empty (&self) -> bool {
    self.origin.is_none() && self.enabled.is_none() && self.url.is_none() && self.token.is_none() && self.token_env.is_none() && self.http_retries.is_none()
  }

  #[allow(dead_code)]
  pub fn new (
    r#type: Option<WebhookType>,
    origin: Option<String>,
    enabled: Option<bool>,
    url: Option<String>,
    token: Option<String>,
    token_env: Option<String>,
    http_retries: Option<u32>,
    http_timeout: Option<u64>
  ) -> Option<Self> {
    let instance = Self {
      r#type,
      origin,
      enabled,
      url,
      token,
      token_env,
      http_retries,
      http_timeout
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

    let token_env = match self.token_env.as_ref() {
      Some(token_env) => token_env.as_str(),
      None => match self.r#type {
        Some(WebhookType::GitHub) => github::config::TOKEN_ENV,
        Some(WebhookType::GitLab) => gitlab::config::TOKEN_ENV,
        Some(WebhookType::Custom) => custom::config::TOKEN_ENV,
        None => return None
      }
    };

    env::var(token_env).ok()
  }

  pub fn get_url (&self) -> Result<String, String> {
    if let Some(inner_url) = self.url.as_ref() {
      return Ok(inner_url.clone());
    }

    let config = Config::inject();

    get_remote_url(self.origin.as_deref(), CommandOptions {
      cwd: config.cwd.clone()
    })
  }
}

pub type WebhookConfig = Vec<WebhookItemConfig>;
