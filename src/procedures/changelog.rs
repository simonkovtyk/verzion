use crate::{changelog::handler::generate_changelog, config::Config, fs::write_str_to_file, git::log::GitLog};

pub struct CreateChangelogResult {
  pub changelog: String
}

pub fn create_changelog (
  logs: &Vec<GitLog>
) -> Option<CreateChangelogResult> {
  let config = Config::inject();
  
  if !&config.changelog.as_ref().map(|v| v.enabled).flatten().unwrap_or(false) {
    return None;
  }

  let changelog = generate_changelog(logs);

  if let Some(changelog_path) = (&config).changelog.as_ref().map(|v| v.path.clone()).flatten() {
    write_str_to_file(&changelog_path, changelog.as_str());
  }

  Some(
    CreateChangelogResult {
      changelog
    }
  )
}
