use crate::{commands::Args, config::Config, git::remote::get_remote_url, semver::SemVer, webhooks::{config::get_token, github::{auth::GITHUB_TOKEN_ENV, http::post_create_release, remote::GitHubRemote}}};

pub async fn create_release (
  semver: &SemVer,
  config: &Config,
  changelog: &Option<String>
) {
  let remote = get_remote_url(&config.cwd);

  if remote.is_none() {
    return;
  }

  let github_remote = GitHubRemote::try_from(remote.unwrap());

  if github_remote.is_err() {
    return;
  }

  let github_remote = github_remote.unwrap();
  let token = get_token(config, GITHUB_TOKEN_ENV);

  post_create_release(&github_remote, semver, &token, changelog).await;
}
