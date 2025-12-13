use crate::{semver::core::SemVer, webhooks::{config::WebhookItemConfig, github::{http::post_create_release, remote::GitHubRemote}}};

pub async fn create_release (
  webhook_item: &WebhookItemConfig,
  semver: &SemVer,
  changelog: &Option<String>
) {
  let remote_url = webhook_item.get_url();

  if remote_url.is_none() {
    return;
  }

  let github_remote = GitHubRemote::try_from(
    remote_url.unwrap().as_ref()
  );

  if github_remote.is_err() {
    return;
  }

  post_create_release(
    webhook_item,
    &github_remote.unwrap(),
    semver,
    changelog
  ).await;
}
