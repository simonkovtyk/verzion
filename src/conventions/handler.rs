use crate::{config::Config, conventions::{config::ConvetionTypes, conventional}, git::log::GitLog, semver::r#type::SemVerType};

pub fn resolve_semver_type (logs: &Vec<GitLog>) -> SemVerType {
  let config = Config::inject();

  let convention = config.convention.as_ref().unwrap_or(&ConvetionTypes::Conventional);

  match convention {
    ConvetionTypes::Conventional => {
      let messages = conventional::parse::parse_logs(logs);

      return conventional::bump::get_semver_type(messages);
    }
  }
}
