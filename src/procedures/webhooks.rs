use crate::{procedures::{changelog::CreateChangelogResult, semver::GetSemVerResult}, webhooks::handler::handle_webhook};

pub async fn call_webhooks (
  get_semver_result: &GetSemVerResult,
  create_changelog_result: &Option<CreateChangelogResult>
) {
  handle_webhook(
    &get_semver_result.semver,
    &create_changelog_result.as_ref().map(|v| v.changelog.clone())
  ).await;
}
