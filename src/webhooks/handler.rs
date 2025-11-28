use crate::{config::Config, semver::SemVer, webhooks::{github, gitlab}};

pub async fn handle_webhooks (
  config: &Config,
  semver: &SemVer,
  changelog: &Option<String>
) {
  if let Some(inner_config) = config.github.clone() && inner_config.is_enabled() {
    println!("{:?}", inner_config);
    github::release::create_release(semver, config, changelog).await;
  }

  println!("{:?}", config.gitlab.clone());

  if let Some(inner_config) = config.gitlab.clone() && inner_config.is_enabled() {
    println!("Hit");
    gitlab::release::create_release(semver, config, changelog).await;
  }
}
