use crate::{config::{Config, ToExitCode}, semver::core::SemVer, std::panic::ExpectWithStatusCode, webhooks::{config::{WebhookItemConfig, WebhookType}, custom, github, gitlab}};

async fn handle_webhook_item (
  webhook_item: &WebhookItemConfig,
  semver: &SemVer,
  changelog: &Option<String>
) {
  let config = Config::inject();

  match webhook_item.r#type.as_ref().expect_with_status_code("No type for webhook item provided", config.to_exit_code()) {
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

  for webhook_item in config.webhooks.clone().expect_with_status_code("No webhook item in webhooks found", config.to_exit_code()) {
    handle_webhook_item(&webhook_item, semver, changelog).await;
  }
}

