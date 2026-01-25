use crate::{changelog::{git::get_commit_msg, handler::generate_changelog}, config::Config, fs::write_str_to_file, git::{log::GitLog}};

pub struct CreateChangelogResult {
  pub changelog: String,
  pub tracking_batch: Option<String>
}

pub fn create_changelog (
  logs: &Vec<GitLog>
) -> Option<CreateChangelogResult> {
  let config = Config::inject();

  let changelog_config = config.changelog.clone()?;

  if changelog_config.enabled.unwrap_or(false) {
    return None;
  }

  let changelog = generate_changelog(logs);

  if let Some(changelog_path) = changelog_config.path {
    write_str_to_file(&changelog_path, changelog.as_str());

    let tracking_batch = changelog_config.tracking
      .as_ref()
      .map(|v| v.track(&changelog_path, &get_commit_msg()))
      .flatten();

    return Some(CreateChangelogResult {
      changelog,
      tracking_batch
    });
  }

  Some(CreateChangelogResult {
    changelog,
    tracking_batch: None
  })
}
