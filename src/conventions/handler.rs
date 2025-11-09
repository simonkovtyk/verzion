use crate::{config::{BumpConvetion, Config}, conventions::conventional, git::GitLog, semver::SemVerType};

pub fn resolve_semver_type (config: &Config, logs: &Vec<GitLog>) -> SemVerType {
  let convention = config.bump.convention.as_ref().unwrap_or(&BumpConvetion::Conventional);

  match convention {
    BumpConvetion::Conventional => {
      let messages = conventional::parse::parse_logs(logs);

      return conventional::bump::get_semver_type(messages);
    }
  }
}
