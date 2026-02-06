use std::collections::HashMap;

use reqwest::header::HeaderMap;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::RetryTransientMiddleware;

use crate::{config::{Config, ToExitCode}, http::{get_retry_policy, get_user_agent}, semver::core::SemVer, std::panic::ExpectWithStatusCode, webhooks::config::WebhookItemConfig};

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
  
  if let Some(token) = webhook_item.get_token() {
    headers.insert(
      "Authorization",
      token.parse().unwrap()
    );
  }

  let mut body = HashMap::new();

  let semver_format = config.semver.as_ref().map(|v| v.format.clone()).flatten();

  body.insert("semver", semver.format(&semver_format));
  body.insert("raw_semver", semver.to_string());

  if let Some(inner_changelog) = changelog {
    body.insert("changelog", inner_changelog.clone());
  }

  client.post(
    webhook_item.url
      .as_ref()
      .expect_with_status_code(
        "Webhook URL is not set",
        config.to_exit_code()
      )
  ).headers(headers)
    .body(
      serde_json::to_string(&body).expect_with_status_code(
        "Failed to serialize body",
        config.to_exit_code()
      )
    )
    .send()
    .await
    .expect_with_status_code(
      "Failed to send request",
      config.to_exit_code()
    );
}
