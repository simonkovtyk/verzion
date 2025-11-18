use std::env;

use crate::{commands::Args, config::Config};

const DEFAULT_TOKEN_ENV: &str = "GITHUB_TOKEN";

/* Resolve by priority: arg -> config -> env */
pub fn get_token (config: &Config, args: &Args) -> String {
  let github_token_arg = args.github_token.clone();

  if let Some(github_token_arg_inner) = github_token_arg {
    return github_token_arg_inner;
  }

  let github_config_token = config.github.clone().map(|v| v.token).flatten();

  if let Some(github_config_token_inner) = github_config_token {
    return github_config_token_inner;
  }

  let github_token_env = config.gitlab.clone().map(|v| v.token_env).flatten().unwrap_or(DEFAULT_TOKEN_ENV.to_string());

  let github_token_env_value = env::var(github_token_env);

  if let Ok(github_token_env_value_inner) = github_token_env_value {
    return github_token_env_value_inner;
  }

  panic!("Not GitHub token found! Please provide it as an argument, config, or as an environment variable.");
}
