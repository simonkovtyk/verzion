use crate::{config::{Config, ToExitCode}, semver::core::SemVer, std::panic::ExpectWithStatusCode, webhooks::{config::WebhookItemConfig, github::{http::post_create_release, remote::GitHubRemote}}};

pub async fn create_release (
  webhook_item: &WebhookItemConfig,
  semver: &SemVer,
  changelog: &Option<String>
) {
  let config = Config::inject();

  let remote_url = webhook_item.get_url().expect_with_status_code("Remote URL absent", config.to_exit_code());

  let github_remote = GitHubRemote::try_from(
    remote_url.as_ref()
  ).expect_with_status_code("GitHub remote could not be parsed", config.to_exit_code());

  post_create_release(
    webhook_item,
    &github_remote,
    semver,
    changelog
  ).await;
}
