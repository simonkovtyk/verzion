#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SemVerType {
  Major = 4,
  Minor = 3,
  Patch = 2,
  PreRelease = 1
}

impl SemVerType {
  pub fn max_or (self: Self, against: Self) -> Self {
    std::cmp::max(self, against)
  }
}

impl ToString for SemVerType {
  fn to_string(&self) -> String {
    match self {
      Self::Major => "major".to_string(),
      Self::Minor => "minor".to_string(),
      Self::Patch => "patch".to_string(),
      Self::PreRelease => "pre-release".to_string()
    }
  }
}
