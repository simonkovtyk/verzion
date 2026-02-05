use crate::{changelog::{config::{ChangelogType, DEFAULT_CHANGELOG_TYPE, DEFAULT_TEMPLATE_PATH}, generation::{simple, template}, utils::get_contributors}, config::Config, conventions::{config::ConvetionTypes, conventional::{self}}, fs::write_str_to_file, git::log::GitLog};

#[allow(dead_code)]
const DEFAULT_CHANGELOG_FILENAME: &str = "CHANGELOG.md";

pub fn generate_changelog (logs: &Vec<GitLog>) -> String {
  let config = Config::inject();
  let convention = config.convention.as_ref().unwrap_or(&ConvetionTypes::Conventional);
  let data;

  match convention {
    &ConvetionTypes::Conventional => {
      let messages = conventional::parse::parse_logs(logs);
      data = conventional::changelog::get_changelog_data(&messages);
    }
  }

  let _contributors = get_contributors(logs);

  match config.changelog.as_ref()
    .map(|v| v.r#type.clone())
    .flatten()
    .unwrap_or(DEFAULT_CHANGELOG_TYPE) {
    ChangelogType::Simple => {
      return simple::generate(data);
    },
    ChangelogType::Template => {
      return template::generate(
        config.changelog.as_ref()
          .map(|v| v.template_path.clone())
          .flatten()
          .unwrap_or(DEFAULT_TEMPLATE_PATH.to_string()),
        data
      );
    }
  }
}

#[allow(dead_code)]
pub fn write_changelog (changelog: &str) {
  let config = Config::inject();
  let path = config.changelog.clone().unwrap().path.unwrap_or(DEFAULT_CHANGELOG_FILENAME.to_string());

  write_str_to_file(path.as_str(), changelog);
}
