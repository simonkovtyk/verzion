use crate::{semver::core::SemVer, webhooks::{config::WebhookItemConfig, custom::http::post_create_release}};

pub async fn create_release (
  webhook_item: &WebhookItemConfig,
  semver: &SemVer,
  changelog: &Option<String>
) {
  post_create_release(webhook_item, semver, changelog).await;
}
