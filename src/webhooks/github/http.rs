use std::collections::HashMap;

use reqwest::header::{HeaderMap};

use crate::{http::get_user_agent, semver::SemVer, webhooks::github::remote::GitHubRemote};

pub async fn post_create_release (
  remote: &GitHubRemote,
  semver: &SemVer,
  token: &str,
  changelog: &Option<String>
) {
  let client = reqwest::Client::new();

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

  let mut body = HashMap::new();
  body.insert("tag_name", semver.to_string());
  body.insert("name", semver.to_string());

  if let Some(inner_changelog) = changelog {
    body.insert("body", inner_changelog.to_string());
  }

  client.post(
    url
  ).headers(headers)
    .json(&body)
    .send()
    .await
    .expect("Failed to send request");
}
