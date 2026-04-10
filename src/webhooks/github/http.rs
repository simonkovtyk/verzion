use std::collections::HashMap;

use reqwest::header::{HeaderMap};
use reqwest_middleware::ClientWithMiddleware;

use crate::{config::{Config, ToExitCode}, http::{get_user_agent}, semver::core::SemVer, std::{panic::ExpectWithStatusCode, reqwest::FromWebhookItemConfig}, webhooks::{config::WebhookItemConfig, github::remote::GitHubRemote}};

pub async fn post_create_release (
  webhook_item: &WebhookItemConfig,
  remote: &GitHubRemote,
  semver: &SemVer,
  changelog: &Option<String>
) {
  let config = Config::inject();

  let url = format!(
    "https://api.github.com/repos/{}/{}/releases",
    remote.owner,
    remote.repository
  );
  let mut headers = HeaderMap::new();

  headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
  headers.insert(
    "Authorization",
    format!(
      "Bearer {}",
      webhook_item.get_token()
        .expect_with_status_code(
          "Could not get token",
          config.to_exit_code()
        )
    ).parse().unwrap()
  );
  headers.insert("X-GitHub-Api-Version", "2022-11-28".parse().unwrap());
  headers.insert("User-Agent", get_user_agent().parse().unwrap());

  let semver_format = semver.format(
    &config.semver.as_ref()
      .map(|v| v.format.clone())
      .flatten()
  );

  let mut body = HashMap::new();
  body.insert("tag_name", semver_format.as_str());
  body.insert("name", semver_format.as_str());

  if let Some(inner_changelog) = changelog {
    body.insert("body", inner_changelog.as_str());
  }

  let client = ClientWithMiddleware::from_webhook_item_config(webhook_item);

  client.post(
    url
  ).headers(headers)
    .body(
      serde_json::to_string(&body).expect_with_status_code("Failed to serialize body", config.to_exit_code())
    )
    .send()
    .await
    .ok();
}
