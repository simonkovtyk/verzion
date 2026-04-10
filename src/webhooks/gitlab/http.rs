use std::{collections::HashMap};

use reqwest::{header::HeaderMap};
use reqwest_middleware::ClientWithMiddleware;

use crate::{config::{Config, ToExitCode}, http::{get_user_agent}, semver::core::SemVer, std::{panic::ExpectWithStatusCode, reqwest::FromWebhookItemConfig}, webhooks::{config::WebhookItemConfig, gitlab::remote::GitLabRemote}};

pub async fn post_create_release (
  webhook_item: &WebhookItemConfig,
  remote: &GitLabRemote,
  semver: &SemVer,
  changelog: &Option<String>
) {
  let config = Config::inject();

  let remote_url = &mut remote.url.clone();
  remote_url.set_path("");

  let mut remote_url_str = remote_url.as_str();
  remote_url_str = &remote_url_str[..remote_url_str.len() - 1];

  let url = format!(
    "{}/{}/{}/{}",
    remote_url_str,
    "api/v4/projects",
    urlencoding::encode(&remote.get_project_path()),
    "releases"
  );

  let mut headers = HeaderMap::new();
  headers.insert("Content-Type", "application/json".parse().unwrap());
  headers.insert("PRIVATE-TOKEN", webhook_item.get_token().expect("Could not get token").parse().unwrap());
  headers.insert("User-Agent", get_user_agent().parse().unwrap());

  let mut body = HashMap::new();

  let semver_format = semver.format(
    &config.semver.as_ref()
      .map(|v| v.format.clone())
      .flatten()
  );

  body.insert("tag_name", semver_format.as_str());
  body.insert("name", semver_format.as_str());

  if let Some(inner_changelog) = changelog {
    body.insert("description", inner_changelog.as_str());
  }

  let client = ClientWithMiddleware::from_webhook_item_config(webhook_item);

  client.post(
    url
  ).headers(headers)
    .body(serde_json::to_string(&body).expect("Could not serialize body"))
    .send()
    .await
    .expect_with_status_code(
      "Failed to send request",
      config.to_exit_code()
    );
}
