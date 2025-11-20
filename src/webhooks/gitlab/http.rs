use std::collections::HashMap;

use reqwest::header::HeaderMap;
use url::Url;

use crate::{git::remote::GitRemote, http::get_user_agent, semver::SemVer, webhooks::gitlab::release::get_project_path_from_git_remote};

pub async fn post_create_release (
  remote: &GitRemote,
  semver: &SemVer,
  token: &str,
  changelog: &Option<String>
) {
  let client = reqwest::Client::new();
  let mut remote_url = Url::parse(&remote.url).expect("Could not parse remote URL");
  let project_path = get_project_path_from_git_remote(&remote_url);
  
  remote_url.set_path("");

  let mut remote_url_str = remote_url.as_str();
  remote_url_str = &remote_url_str[..remote_url_str.len() - 1];

  let url = format!(
    "{}/{}/{}/{}",
    remote_url_str,
    "api/v4/projects",
    urlencoding::encode(&project_path),
    "releases"
  );

  let mut headers = HeaderMap::new();
  headers.insert("PRIVATE-TOKEN", token.parse().unwrap());
  headers.insert("User-Agent", get_user_agent().parse().unwrap());
  
  let mut body = HashMap::new();

  let semver = semver.to_string();

  body.insert("tag_name", semver.to_string());
  body.insert("name", semver.to_string());

  if let Some(inner_changelog) = changelog {
    body.insert("description", inner_changelog.to_string());
  }

  client.post(
    url
  ).headers(headers)
    .json(&body)
    .send()
    .await
    .expect("Failed to send request");
}
