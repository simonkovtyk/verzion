use crate::{config::{Config, ToExitCode}, semver::core::SemVer, std::panic::ExpectWithStatusCode, webhooks::{config::WebhookItemConfig, gitlab::{http::post_create_release, remote::GitLabRemote}}};

pub async fn create_release (
  webhook_item: &WebhookItemConfig,
  semver: &SemVer,
  changelog: &Option<String>
) {
  let config = Config::inject();

  let remote_url = webhook_item.get_url().expect_with_status_code(
    "Could not get webhook URL",
    config.to_exit_code()
  );

  let gitlab_remote = GitLabRemote::try_from(
    remote_url.as_ref()
  ).expect_with_status_code(
    "Could not parse GitLab remote",
    config.to_exit_code()
  );

  post_create_release(
    webhook_item,
    &gitlab_remote,
    semver,
    changelog
  ).await;
}

