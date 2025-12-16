use crate::{changelog::handler::generate_changelog, config::{Config, ToExitCode}, fs::write_str_to_file, git::log::GitLog, std::panic::ExpectWithStatusCode};

pub struct CreateChangelogResult {
  pub changelog: String
}

pub fn create_changelog (
  logs: &Vec<GitLog>
) -> Option<CreateChangelogResult> {
  let config = Config::inject();

  if !config.changelog.as_ref()?.enabled? {
    return None;
  }

  let changelog = generate_changelog(logs);

  if let Some(changelog_path) = config.changelog
    .clone()
    .map(|v| v.path)
    .flatten() {
    write_str_to_file(&changelog_path, changelog.as_str());
  }

  Some(
    CreateChangelogResult {
      changelog
    }
  )
}
