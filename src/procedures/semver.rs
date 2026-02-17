use crate::{config::Config, log::log_debug, semver::{core::SemVer, r#type::SemVerType}};

pub struct GetSemVerResult {
  pub semver: SemVer
}

pub fn get_semver (
  semver_type: &SemVerType,
  latest_semver: Option<SemVer>
) -> GetSemVerResult {
  let config = Config::inject();

  let config_semver = config.semver.clone().map(|v| v.to_semver_with_format()).flatten();

  if let Some(inner_config_semver) = config_semver && inner_config_semver.is_fullfilled() {
    log_debug(
      &format!("Using SemVer from config: {:?}", &inner_config_semver)
    );

    return GetSemVerResult {
      semver: inner_config_semver
    };
  }

  let base_semver = if let Some(inner_latest_semver) = latest_semver && inner_latest_semver.is_fullfilled() {
    inner_latest_semver
  } else {
    SemVer::default()
  };

  let semver = base_semver.bump(semver_type);

  log_debug(
    &format!(
      "Calculated SemVer: {:?}",
      &semver
    )
  );

  GetSemVerResult {
    semver
  }
}
