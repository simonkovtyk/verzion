use std::collections::HashMap;

use reqwest::header::{HeaderMap};
use reqwest_middleware::ClientBuilder;
use reqwest_retry::RetryTransientMiddleware;

use crate::{config::Config, http::{get_retry_policy, get_user_agent}, semver::core::SemVer, remotes::github::remote::GitHubRemote};

pub async fn post_create_release (
  remote: &GitHubRemote,
  semver: &SemVer,
  token: &str,
  changelog: &Option<String>
) {
  let config = Config::inject();

  let client = ClientBuilder::new(reqwest::Client::new())
    .with(
      RetryTransientMiddleware::new_with_policy(
        get_retry_policy(
          config.github.clone().map(|v| v.retries).flatten()
        )
      )
    ).build();

  let url = format!(
    "https://api.github.com/repos/{}/{}/releases",
    remote.owner,
    remote.repository
  );
  let mut headers = HeaderMap::new();

  headers.insert("Accept", "application/vnd.github+json".parse().unwrap());
  headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());
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

  client.post(
    url
  ).headers(headers)
    .body(
      serde_json::to_string(&body).expect("Failed to serialize body")
    )
    .send()
    .await
    .expect("Failed to send request");
}
