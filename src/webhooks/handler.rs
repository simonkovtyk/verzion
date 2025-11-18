use crate::{config::Config, semver::SemVer, webhooks::{github, gitlab}};

pub async fn handle_webhooks (
  config: &Config,
  semver: &SemVer,
  changelog: &Option<String>
) {
  if let Some(enabled) = config.github.clone().map(|v| v.is_enabled()) && enabled {
    github::release::create_release(semver, config, changelog).await;
  }

  if let Some(enabled) = config.gitlab.clone().map(|v| v.is_enabled()) && enabled {
    gitlab::release::create_release(semver, config, changelog).await;
  }
}
