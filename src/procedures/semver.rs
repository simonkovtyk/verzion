use crate::{config::Config, git::tag::GitTag, semver::{core::SemVer, r#type::SemVerType}};

pub struct GetSemVerResult {
  pub semver: SemVer
}

pub fn get_semver (
  semver_type: &SemVerType,
  latest_tag: Option<GitTag>
) -> GetSemVerResult {
  let config = Config::inject();

  let config_semver = config.semver.clone().map(|v| v.to_semver_with_format());

  if let Some(inner_config_semver) = config_semver && inner_config_semver.is_fullfilled() {
    return GetSemVerResult {
      semver: inner_config_semver
    };
  }

  let base_semver = if let Some(inner_latest_tag) = latest_tag && inner_latest_tag.semver.is_fullfilled() {
    inner_latest_tag.semver.clone()
  } else {
    SemVer::default()
  };

  GetSemVerResult {
    semver: base_semver.bump(semver_type)
  }
}
