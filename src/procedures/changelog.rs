use crate::{changelog::{config::DEFAULT_CHANGELOG_PATH, git::get_commit_msg, handler::generate_changelog}, config::Config, fs::write_str_to_file, git::{log::GitLog, tracking::GitTrackingBatch}, log::log_debug};

pub struct CreateChangelogResult {
  pub changelog: String,
  pub tracking_batch: GitTrackingBatch
}

pub fn create_changelog (
  logs: &Vec<GitLog>
) -> Option<CreateChangelogResult> {
  let config = Config::inject();

  let changelog_config = config.changelog.clone()?;

  if !changelog_config.enabled.unwrap_or(false) {
    log_debug("Skipping changelog generation, because it is disabled");

    return None;
  }

  let changelog = generate_changelog(logs);
  let mut tracking_batch = Vec::new();
  let changelog_path = changelog_config.path.unwrap_or(DEFAULT_CHANGELOG_PATH.to_string());

  write_str_to_file(&changelog_path, changelog.as_str());

  if let Some(inner_tracking_batch)  = changelog_config.tracking
    .as_ref()
    .map(|v| v.track(&changelog_path, &get_commit_msg()).map(|v| vec![v]))
    .flatten() {
      tracking_batch.extend(inner_tracking_batch);
  }

  log_debug(
    &format!(
      "Generated changelog:\n{}",
      &changelog
    )
  );

  Some(CreateChangelogResult {
    changelog,
    tracking_batch
  })
}
