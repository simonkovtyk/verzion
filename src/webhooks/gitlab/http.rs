use std::collections::HashMap;

use reqwest::header::HeaderMap;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::RetryTransientMiddleware;

use crate::{config::Config, http::{get_retry_policy, get_user_agent}, semver::core::SemVer, webhooks::{config::WebhookItemConfig, gitlab::remote::GitLabRemote}};

pub async fn post_create_release (
  webhook_item: &WebhookItemConfig,
  remote: &GitLabRemote,
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

  client.post(
    url
  ).headers(headers)
    .body(serde_json::to_string(&body).expect("Could not serialize body"))
    .send()
    .await
    .expect("Failed to send request");
}
