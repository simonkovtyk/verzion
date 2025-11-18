use std::env;
use serde::{Deserialize, Serialize};
use merge::Merge;

use crate::{config::Config};

#[derive(Serialize, Deserialize, Debug, Clone, Merge)]
pub struct WebhookConfig {
  #[merge(strategy = merge::option::overwrite_none)]
  pub enabled: Option<bool>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub url: Option<String>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub token: Option<String>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub token_env: Option<String>
}

impl WebhookConfig {
  pub fn is_enabled (&self) -> bool {
    self.enabled.unwrap_or(true)
  }
}

pub enum WebhookType {
  GitHub,
  GitLab
}

/* Resolve by priority: arg -> config -> env */
pub fn get_token (config: &Config, default: &str) -> String {
  let github_token_arg = config.github.clone().unwrap().token.clone();

  if let Some(github_token_arg_inner) = github_token_arg {
    return github_token_arg_inner;
  }

  let github_config_token = config.github.clone().map(|v| v.token).flatten();

  if let Some(github_config_token_inner) = github_config_token {
    return github_config_token_inner;
  }

  let github_token_env = config.gitlab.clone().map(|v| v.token_env).flatten().unwrap_or(default.to_string());

  let github_token_env_value = env::var(github_token_env);

  if let Ok(github_token_env_value_inner) = github_token_env_value {
    return github_token_env_value_inner;
  }

  panic!("Not GitHub token found! Please provide it as an argument, config, or as an environment variable.");
}
