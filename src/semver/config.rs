use serde::{Deserialize, Serialize};

use crate::{semver::core::SemVer, std::merge::Merge};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SemVerConfig {
  pub semver: Option<String>,
  pub format: Option<String>,
  pub major: Option<u64>,
  pub minor: Option<u64>,
  pub patch: Option<u64>,
  pub pre_release: Option<String>,
  pub iteration: Option<u64>,
  pub metadata: Option<Vec<String>>
}

impl SemVerConfig {
  pub fn is_empty (&self) -> bool {
    self.semver.is_none()
      && self.format.is_none()
      && self.major.is_none()
      && self.minor.is_none()
      && self.patch.is_none()
      && self.pre_release.is_none()
      && self.iteration.is_none()
      && self.metadata.is_none()
  }

  pub fn new (
    semver: Option<String>,
    format: Option<String>,
    major: Option<u64>,
    minor: Option<u64>,
    patch: Option<u64>,
    pre_release: Option<String>,
    iteration: Option<u64>,
    metadata: Option<Vec<String>>
  ) -> Option<Self> {
    let instance = Self {
      semver,
      format,
      major,
      minor,
      patch,
      pre_release,
      iteration,
      metadata
    };

    if instance.is_empty() {
      None
    } else {
      Some(instance)
    }
  }
}

impl SemVerConfig {
  pub fn to_semver (self) -> Option<SemVer> {
    let mut semver = if let Some(inner_semver) = self.semver {
      SemVer::try_from_str(&inner_semver).ok()?
    } else {
      SemVer::default()
    };

    semver.major = self.major.or(semver.major);
    semver.minor = self.minor.or(semver.minor);
    semver.patch = self.patch.or(semver.patch);
    semver.pre_release = self.pre_release.or(semver.pre_release);
    semver.iteration = self.iteration.or(semver.iteration);
    semver.metadata = self.metadata.or(semver.metadata);

    Some(semver)
  }

  pub fn to_semver_with_format (self) -> Option<SemVer> {
    let mut semver = if let Some(inner_semver) = self.semver.as_ref() {
      SemVer::try_from_format(&inner_semver, &self.format).ok()?
    } else {
      SemVer::default()
    };

    semver.major = self.major.or(semver.major);
    semver.minor = self.minor.or(semver.minor);
    semver.patch = self.patch.or(semver.patch);
    semver.pre_release = self.pre_release.or(semver.pre_release);
    semver.iteration = self.iteration.or(semver.iteration);
    semver.metadata = self.metadata.or(semver.metadata);

    Some(semver)
  }
}

impl Merge for SemVerConfig {
  fn merge(self, other: Self) -> Self {
    Self {
      semver: self.semver.or(other.semver),
      format: self.format.or(other.format),
      major: self.major.or(other.major),
      minor: self.minor.or(other.minor),
      patch: self.patch.or(other.patch),
      pre_release: self.pre_release.or(other.pre_release),
      iteration: self.iteration.or(other.iteration),
      metadata: self.metadata.merge(other.metadata)
    }
  }
}
