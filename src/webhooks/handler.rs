use crate::{commands::Args, config::Config, semver::SemVer, webhooks::github};

pub async fn handle_webhooks (
  config: &Config,
  semver: &SemVer,
  args: &Args,
  changelog: &str
) {
  println!("Creating GitHub release");
  if let Some(enabled) = config.github.clone().map(|v| v.is_enabled()) && enabled {
    println!("Creating GitHub release");
    github::release::create_release(semver, config, args, changelog).await;
  }

  if let Some(enabled) = config.gitlab.clone().map(|v| v.is_enabled()) && enabled {

  }
}
