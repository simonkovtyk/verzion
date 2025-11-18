use std::ptr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemVer {
  pub major: Option<u64>,
  pub minor: Option<u64>,
  pub patch: Option<u64>
}

impl ToString for &SemVer {
  fn to_string(&self) -> String {
    format!(
      "{}.{}.{}",
      self.major.unwrap_or(0),
      self.minor.unwrap_or(0),
      self.patch.unwrap_or(0)
    )
  }
}

impl Into<String> for &SemVer {
  fn into(self) -> String {
    self.to_string()
  }
}

impl TryFrom<&str> for SemVer {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let mut instance = Self {
      major: None,
      minor: None,
      patch: None
    };

    let mut field = &mut instance.major as *mut Option<u64>;

    for value_char in value.chars() {
      match value_char {
        '.' => {
          match field {
            f if ptr::eq(f, &instance.major) => {
              field = &mut instance.minor as *mut Option<u64>;
            },
            f if ptr::eq(f, &instance.minor) => {
              field = &mut instance.patch as *mut Option<u64>;
            },
            /* else field stays pointing to current */
            _ => {}
          }
        },
        _ => {
          if !value_char.is_ascii_digit() {
            return Err("Not a valid semver version");
          }

          let value_char_as_digit = value_char.to_digit(10)
            .expect("Could not parse digit") as u64;

          unsafe {
            *field = if let Some(field_value) = *field {
              let new_field_value = field_value * 10 + value_char_as_digit;
              Some(new_field_value)
            } else {
              Some(value_char_as_digit)
            }
          }
        }
      }
    }

    Ok(instance)
  }
}

impl Ord for SemVer {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.major.cmp(&other.major)
      .then(self.minor.cmp(&other.minor))
      .then(self.patch.cmp(&other.patch))
  }
}

impl PartialOrd for SemVer {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
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

pub fn bump_semver(mut current: SemVer, target: SemVerType) -> SemVer {
  match target {
    SemVerType::Major => {
      current.major = current.major.map(|v| v + 1);
    },
    SemVerType::Minor => {
      current.minor = current.minor.map(|v| v + 1);
    },
    SemVerType::Patch => {
      current.patch = current.patch.map(|v| v + 1);
    }
  };

  current
}
