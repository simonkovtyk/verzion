use std::env;
use serde::{Deserialize, Serialize};

use crate::{config::Config, std::Merge, webhooks::{github::auth::GITHUB_TOKEN_ENV, gitlab::auth::GITLAB_TOKEN_ENV}};

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

impl Merge for WebhookConfig {
  fn merge(&self, other: &Self) -> Self {
    Self {
      enabled: self.enabled.or(other.enabled.clone()),
      url: self.url.clone().or(other.url.clone()),
      token: self.token.clone().or(other.token.clone()),
      token_env: self.token_env.clone().or(other.token_env.clone())
    }
  }
}

pub enum WebhookType {
  GitHub,
  GitLab
}

pub fn get_token (config: &Config, webhook_type: &WebhookType) -> String {
  let token = match webhook_type {
    WebhookType::GitHub => config.github.clone().unwrap().token.clone(),
    WebhookType::GitLab => config.gitlab.clone().unwrap().token.clone(),
  };

  if let Some(inner_token) = token {
    return inner_token;
  }

  let token_env = match webhook_type {
    WebhookType::GitHub => config.github.clone().unwrap().token_env.clone().unwrap_or(GITHUB_TOKEN_ENV.to_string()),
    WebhookType::GitLab => config.gitlab.clone().unwrap().token_env.clone().unwrap_or(GITLAB_TOKEN_ENV.to_string())
  };

  let env = env::var(token_env);

  env.expect(
    "Not token found! Please provide it as an argument, config, or as an environment variable."
  )
}
