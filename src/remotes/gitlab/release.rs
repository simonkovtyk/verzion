use crate::{config::Config, git::remote::get_remote_url, semver::core::SemVer, remotes::{config::{RemoteType, get_token}, gitlab::{http::post_create_release}}};

pub async fn create_release (
  semver: &SemVer,
  changelog: &Option<String>
) {
  let config = Config::inject();
  let token = get_token(config, &RemoteType::GitLab);
  let remote = get_remote_url().expect("Could not get git remote URL");

  post_create_release(
    &remote,
    semver,
    &token,
    changelog
  ).await;
}

