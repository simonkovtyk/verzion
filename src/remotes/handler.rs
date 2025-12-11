use crate::{config::Config, remotes::{github, gitlab}, semver::core::{SemVer}};

pub async fn handle_webhooks (
  semver: &SemVer,
  changelog: &Option<String>
) {
  let config = Config::inject();

  if let Some(inner_config) = config.github.clone() && inner_config.is_enabled() {
    github::release::create_release(semver, changelog).await;
  }

  if let Some(inner_config) = config.gitlab.clone() && inner_config.is_enabled() {
    gitlab::release::create_release(semver, changelog).await;
  }
}

pub async fn setup_remotes () {
  let config = Config::inject();

  if let Some(inner_github) = config.github.clone() && inner_github.is_enabled() {
  }
}
