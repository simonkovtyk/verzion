use crate::{config::{BumpConvetion, Config}, conventions::conventional, git::log::GitLog, semver::SemVerType};

pub fn resolve_semver_type (logs: &Vec<GitLog>) -> SemVerType {
  let config = Config::inject();

  let convention = config.convention.as_ref().unwrap_or(&BumpConvetion::Conventional);

  match convention {
    BumpConvetion::Conventional => {
      let messages = conventional::parse::parse_logs(logs);

      return conventional::bump::get_semver_type(messages);
    }
  }
}

pub fn generate_changelog (logs: &Vec<GitLog>) -> String {
  let config = Config::inject();

  let convention = config.convention.as_ref().unwrap_or(&BumpConvetion::Conventional);

  match convention {
    BumpConvetion::Conventional => {
      let messages = conventional::parse::parse_logs(logs);

      return conventional::changelog::get_changelog(&messages);
    }
  }
}
