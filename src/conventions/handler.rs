use crate::{config::Config, conventions::{config::{ConvetionTypes, DEFAULT_CONVENTION}, conventional}, git::log::GitLog, semver::r#type::SemVerType};

pub fn resolve_semver_type (logs: &Vec<GitLog>) -> SemVerType {
  let config = Config::inject();

  let convention = config.convention.as_ref().unwrap_or(&DEFAULT_CONVENTION);

  match convention {
    ConvetionTypes::Conventional => {
      let messages = conventional::parse::parse_logs(logs);

      return conventional::bump::get_semver_type(messages);
    }
  }
}
