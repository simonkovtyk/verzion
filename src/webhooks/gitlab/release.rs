use url::Url;

use crate::{config::Config, git::remote::get_remote_url, semver::SemVer, webhooks::{config::{WebhookType, get_token}, gitlab::{http::post_create_release}}};

pub fn get_project_path_from_git_remote (url: &Url) -> String {
  let path = url.path();

  path[1..path.len() - 4].to_string()
}

pub async fn create_release (
  semver: &SemVer,
  config: &Config,
  changelog: &Option<String>
) {
  let token = get_token(config, &WebhookType::GitLab);
  let remote = get_remote_url(&config.cwd).expect("Could not get git remote URL");

  post_create_release(
    &remote,
    semver,
    &token,
    changelog
  ).await;
}

