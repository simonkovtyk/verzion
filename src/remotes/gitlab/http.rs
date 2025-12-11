use std::collections::HashMap;

use reqwest::header::HeaderMap;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::RetryTransientMiddleware;

use crate::{config::Config, git::remote::GitRemote, http::{get_retry_policy, get_user_agent}, semver::core::SemVer, remotes::gitlab::remote::GitLabRemote};

pub async fn post_create_release (
  remote: &GitRemote,
  semver: &SemVer,
  token: &str,
  changelog: &Option<String>
) {
  let config = Config::inject();

  let client = ClientBuilder::new(reqwest::Client::new())
    .with(
      RetryTransientMiddleware::new_with_policy(
        get_retry_policy(
          config.gitlab.clone().map(|v| v.retries).flatten()
        )
      )
    ).build();

  let remote = GitLabRemote::try_from(remote)
    .expect("Could not parse git remote as GitLab remote");
 
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
  headers.insert("PRIVATE-TOKEN", token.parse().unwrap());
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
