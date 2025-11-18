use crate::{commands::Args, config::Config, git::remote::get_remote_url, semver::SemVer, webhooks::github::{auth::get_token, http::post_create_release, remote::GitHubRemote}};

pub async fn create_release (semver: &SemVer, config: &Config, args: &Args, changelog: &str) {
  let remote = get_remote_url(&args.cwd);

  if remote.is_none() {
    return;
  }

  let github_remote = GitHubRemote::try_from(remote.unwrap());

  if github_remote.is_err() {
    return;
  }

  let github_remote = github_remote.unwrap();
  let token = get_token(config, args);

  post_create_release(&github_remote, semver, &token, changelog).await;
}
