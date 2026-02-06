use crate::{git::tag::GitTag, semver::core::SemVer};

#[derive(Debug, Clone)]
pub struct SemVerWithTag {
  pub semver: SemVer,
  pub tag: GitTag
}

pub fn find_latest_semver (semver_with_tags: Vec<SemVerWithTag>) -> Option<SemVerWithTag> {
  semver_with_tags.into_iter().max_by_key(|v| v.semver.clone())
}
