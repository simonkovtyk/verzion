use std::collections::HashMap;

use reqwest::header::HeaderMap;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::RetryTransientMiddleware;

use crate::{config::Config, http::{get_retry_policy, get_user_agent}, semver::core::SemVer, webhooks::config::WebhookItemConfig};

pub async fn post_create_release (
  webhook_item: &WebhookItemConfig,
  semver: &SemVer,
  changelog: &Option<String>
) {
  let config = Config::inject();

  let client = ClientBuilder::new(reqwest::Client::new())
    .with(
      RetryTransientMiddleware::new_with_policy(
        get_retry_policy(
          webhook_item.http_retries
        )
      )
    ).build();

  let mut headers = HeaderMap::new();

  headers.insert("User-Agent", get_user_agent().parse().unwrap());

  let mut body = HashMap::new();

  let semver_format = config.semver.as_ref().map(|v| v.format.clone()).flatten();

  body.insert("semver", semver.format(&semver_format));
  body.insert("raw_semver", semver.to_string());

  if let Some(inner_changelog) = changelog {
    body.insert("changelog", inner_changelog.clone());
  }

  client.post(
    webhook_item.url.as_ref().expect("Webhook URL is not set")
  ).headers(headers)
    .body(
      serde_json::to_string(&body).expect("Failed to serialize body")
    )
    .send()
    .await
    .expect("Failed to send request");
}
