use crate::{changelog::{generation::{simple, template}, utils::get_contributors}, config::Config, conventions::{config::ConvetionTypes, conventional::{self}}, fs::write_str_to_file, git::log::GitLog};

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

  if let Some(template_path) = config.changelog.clone().map(|v| v.template_path).flatten() {
    return template::generate(template_path, data);
  }

  simple::generate(data)
}

pub fn write_changelog (changelog: &str) {
  let config = Config::inject();
  let path = config.changelog.clone().unwrap().path.unwrap_or(DEFAULT_CHANGELOG_FILENAME.to_string());

  write_str_to_file(path.as_str(), changelog);
}
