use crate::{config::Config, log::log_debug, procedures::{changelog::CreateChangelogResult, semver::GetSemVerResult}, webhooks::{handler::handle_webhook}};

pub async fn call_webhooks (
  get_semver_result: &GetSemVerResult,
  create_changelog_result: &Option<CreateChangelogResult>
) {
  let config = Config::inject();
  let webhooks_enabled = config.webhooks.as_ref().map(|v| v.iter().find(|iv| iv.is_enabled())).flatten();

  if webhooks_enabled.is_none() {
    log_debug(
      "No enabled webhooks found, skipping webhook calls"
    );

    return;
  }

  handle_webhook(
    &get_semver_result.semver,
    &create_changelog_result.as_ref().map(|v| v.changelog.clone())
  ).await;
}
