use std::env;

use crate::{commands::Args, config::Config};

const DEFAULT_TOKEN_ENV: &str = "GITLAB_TOKEN";

/* Resolve by priority: arg -> config -> env */
pub fn get_token (config: &Config, args: &Args) -> String {
  let gitlab_token_arg = args.gitlab_token.clone();

  if let Some(gitlab_token_arg_inner) = gitlab_token_arg {
    return gitlab_token_arg_inner;
  }

  let gitlab_config_token = config.gitlab.clone().map(|v| v.token).flatten();

  if let Some(gitlab_config_token_inner) = gitlab_config_token {
    return gitlab_config_token_inner;
  }

  let gitlab_token_env = config.gitlab.clone().map(|v| v.token_env).flatten().unwrap_or(DEFAULT_TOKEN_ENV.to_string());

  let gitlab_token_env_value = env::var(gitlab_token_env);

  if let Ok(gitlab_token_env_value_inner) = gitlab_token_env_value {
    return gitlab_token_env_value_inner;
  }

  panic!("Not GitLab token found! Please provide it as an argument, config, or as an environment variable.");
}
