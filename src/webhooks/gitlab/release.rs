use crate::{semver::core::SemVer, webhooks::{config::WebhookItemConfig, gitlab::{http::post_create_release, remote::GitLabRemote}}};

pub async fn create_release (
  webhook_item: &WebhookItemConfig,
  semver: &SemVer,
  changelog: &Option<String>
) {
  let remote_url = webhook_item.get_url();

  if remote_url.is_none() {
    return;
  }

  let gitlab_remote = GitLabRemote::try_from(
    remote_url.unwrap().as_ref()
  );

  if gitlab_remote.is_err() {
    return;
  }

  post_create_release(
    webhook_item,
    &gitlab_remote.unwrap(),
    semver,
    changelog
  ).await;
}

