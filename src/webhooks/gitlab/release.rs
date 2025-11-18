use std::{collections::HashMap};
use reqwest;
use url::Url;
use urlencoding::encode;

use crate::{commands::Args, config::Config, git::remote::get_remote_url, semver::SemVer, webhooks::{config::get_token, gitlab::auth::GITLAB_TOKEN_ENV}};

pub fn get_project_path_from_git_remote (url: &Url) -> String {
  let path = url.path();

  encode(&path[..path.len() - 4]).to_string()
}

pub fn get_request_host (url: &Url, config: &Config) -> String {
  let config_host = config.gitlab.clone().map(|v| v.url).flatten();

  if let Some(config_host_inner) = config_host {
    return config_host_inner;
  }

  return url.host_str().expect("No host found in URL").to_string();
}

pub async fn create_release (
  semver: &SemVer,
  config: &Config,
  changelog: &Option<String>
) {
  let token = get_token(config, GITLAB_TOKEN_ENV);
  let remote = get_remote_url(&config.cwd).expect("Could not get git remote URL");

  let url = Url::parse(&remote.url).expect("Could not parse git remote URL");
  let path = get_project_path_from_git_remote(&url);
  let host = get_request_host(&url, config);

  let client = reqwest::Client::new();

  let mut body = HashMap::new();

  let semver = semver.to_string();

  body.insert("tag_name", semver.to_string());
  body.insert("name", semver.to_string());

  if let Some(inner_changelog) = changelog {
    body.insert("description", inner_changelog.to_string());
  }

  let response = client.post(
    format!(
      "{}/projects/{}/releases",
      host,
      path
    )
  ).json(&token)
  .send()
  .await
  .expect("Could not create release");

  if !response.status().is_success() {
    panic!("Something went wrong while creating the release");
  }
}

