use std::error::Error;

pub struct SemVer {
  pub major: Option<u64>,
  pub minor: Option<u64>,
  pub patch: Option<u64>
}

#[derive(Debug)]
pub struct SemVerError {}

impl TryInto<String> for SemVer {
  type Error = SemVerError;

  fn try_into(self) -> Result<String, Self::Error> {
    Ok(format!(
      "{}.{}.{}",
      self.major.ok_or(SemVerError {})?,
      self.minor.ok_or(SemVerError {})?,
      self.patch.ok_or(SemVerError {})?
    ))
  }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemVerType {
  Major = 3,
  Minor = 2,
  Patch = 1
}

pub fn compare_semver_type(current: SemVerType, against: SemVerType) -> SemVerType {
  return std::cmp::max(current, against);
}
