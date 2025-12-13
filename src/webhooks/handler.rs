use crate::{config::Config, semver::core::SemVer, webhooks::{config::{WebhookItemConfig, WebhookType}, custom, github, gitlab}};

pub async fn handle_webhook_item (
  webhook_item: &WebhookItemConfig,
  semver: &SemVer,
  changelog: &Option<String>
) {
  if webhook_item.r#type.is_none() {
    return;
  }

  match webhook_item.r#type.as_ref().unwrap() {
    WebhookType::GitLab => {
      gitlab::release::create_release(webhook_item, semver, changelog).await;
    },
    WebhookType::GitHub => {
      github::release::create_release(webhook_item, semver, changelog).await;
    },
    WebhookType::Custom => {
      custom::release::create_release(webhook_item, semver, changelog).await;
    }
  }
}

pub async fn handle_webhook (
  semver: &SemVer,
  changelog: &Option<String>
) {
  let config = Config::inject();

  if config.webhook.is_none() {
    return;
  }

  for webhook_item in config.webhook.clone().unwrap() {
    handle_webhook_item(&webhook_item, semver, changelog).await;
  }
}

pub async fn setup_remotes () {
}
