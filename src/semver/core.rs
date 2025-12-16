use crate::{semver::r#type::SemVerType, std::merge::Merge};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SemVer {
  pub major: Option<u64>,
  pub minor: Option<u64>,
  pub patch: Option<u64>,
  pub pre_release: Option<String>,
  pub iteration: Option<u64>,
  pub metadata: Option<Vec<String>>
}

enum SemVerField {
  Major,
  Minor,
  Patch,
  PreRelease,
  Iteration,
  Metadata
}

impl SemVer {
  pub fn is_fullfilled (&self) -> bool {
    let is_fullfilled = self.major.is_some()
      && self.minor.is_some()
      && self.patch.is_some();

    if self.iteration.is_some() {
      return is_fullfilled && self.pre_release.is_some();
    }

    is_fullfilled
  }
  pub fn bump (mut self: Self, r#type: &SemVerType) -> Self {
    match r#type {
      SemVerType::Major => {
        self.major = self.major.map(|v| v + 1);
        self.minor = Some(0);
        self.patch = Some(0);
      },
      SemVerType::Minor => {
        self.minor = self.minor.map(|v| v + 1);
        self.patch = Some(0);
      },
      SemVerType::Patch => {
        self.patch = self.patch.map(|v| v + 1);
      },
      SemVerType::PreRelease => {
        if self.iteration.is_some() {
          self.iteration = self.iteration.map(|v| v + 1);
        } else {
          self.patch = self.patch.map(|v| v + 1);
        }
      }
    };

    self.clone()
  }

  pub fn format (&self, format: &Option<String>) -> String {
    let semver_str = self.to_string();
    
    if let Some(inner_semver_format) = format.clone() {
      return inner_semver_format.replace("{}", &semver_str);
    }

    semver_str
  }

  pub fn try_from_str (value: &str) -> Result<Self, &'static str> {
    let mut instance = Self {
      major: None,
      minor: None,
      patch: None,
      metadata: None,
      pre_release: None,
      iteration: None
    };

    let mut field = SemVerField::Major;

    for (index, value_char) in value.chars().enumerate() {
      match value_char {
        '-' => {
          field = SemVerField::PreRelease;
        },
        '.' => {
          match field {
            SemVerField::Major => {
              field = SemVerField::Minor;
            },
            SemVerField::Minor => {
              field = SemVerField::Patch;
            },
            SemVerField::PreRelease => {
              field = SemVerField::Iteration;
            },
            _ => {}
          }
        },
        '+' => {
          field = SemVerField::Metadata;
        },
        _ => {
          match field {
            SemVerField::Major => {
              let value_char_as_digit = value_char.to_digit(10).ok_or("Could not convert value to digit")? as u64;

              if let Some(major) = instance.major {
                instance.major = Some(
                  major * 10 + value_char_as_digit);
              } else {
                instance.major = Some(value_char_as_digit);
              }
            },
            SemVerField::Minor => {
              let value_char_as_digit = value_char.to_digit(10).ok_or("Could not convert value to digit")? as u64;

              if let Some(minor) = instance.minor {
                instance.minor = Some(minor * 10 + value_char_as_digit);
              } else {
                instance.minor = Some(value_char_as_digit);
              }
            },
            SemVerField::Patch => {
              let value_char_as_digit = value_char.to_digit(10).ok_or("Could not convert value to digit")? as u64;

              if let Some(patch) = instance.patch {
                instance.patch = Some(patch * 10 + value_char_as_digit);
              } else {
                instance.patch = Some(value_char_as_digit);
              }
            },
            SemVerField::PreRelease => {
              if let Some(pre_release) = instance.pre_release {
                instance.pre_release = Some(pre_release + &value_char.to_string());
              } else {
                instance.pre_release = Some(value_char.to_string());
              }
            },
            SemVerField::Iteration => {
              let value_char_as_digit = value_char.to_digit(10).ok_or("Could not convert value to digit")? as u64;

              if let Some(iteration) = instance.iteration {
                instance.iteration = Some(iteration * 10 + value_char_as_digit);
              } else {
                instance.iteration = Some(value_char_as_digit);
              }
            },
            SemVerField::Metadata => {
              let metadata = &value[index..];

              let metadata_split = metadata.split(".").map(|v| v.to_string()).collect::<Vec<String>>();

              if !metadata_split.is_empty() {
                instance.metadata = Some(metadata_split)
              }

              break;
            }
          }
        }
      }
    }

    Ok(instance)
  }

  pub fn try_from_format (value: &str, format: &Option<String>) -> Result<Self, &'static str> {
    let deformat = SemVer::try_deformat(value, format)?;

    SemVer::try_from_str(deformat.as_str())
  }

  pub fn try_deformat (value: &str, format: &Option<String>) -> Result<String, &'static str> {
    if let Some(inner_format) = format {
      let open_index = inner_format.find("{").ok_or("Expected opening '{' in semver format")?;
      let close_index = inner_format.rfind("}").ok_or("Expected closing '}' in semver format")?;
      let close_index_right = inner_format.len() - 1 - close_index;

      if value.len() - 1 <= close_index_right + open_index {
        return Err("Expected semver value to match be at least as long as semver format");
      }

      Ok(value[open_index..(value.len() - close_index_right)].to_string())
    } else {
      Ok(value.to_string())
    }
  }
  
  pub fn as_bytes (&self) -> Vec<u8> {
    format!("{}\n", self.to_string()).as_bytes().to_vec()
  }
}

impl Merge for SemVer {
  fn merge (self, other: Self) -> Self {
    Self {
      major: self.major.or(other.major),
      minor: self.minor.or(other.minor),
      patch: self.patch.or(other.patch),
      pre_release: self.pre_release.or(other.pre_release),
      iteration: self.iteration.or(other.iteration),
      metadata: self.metadata.or(other.metadata)
    }
  }
}

impl Default for SemVer {
  fn default() -> Self {
    Self {
      major: Some(0),
      minor: Some(0),
      patch: Some(0),
      pre_release: None,
      iteration: None,
      metadata: None
    }
  }
}

impl ToString for &SemVer {
  fn to_string(&self) -> String {
    let mut value = format!(
      "{}.{}.{}",
      self.major.unwrap_or(0),
      self.minor.unwrap_or(0),
      self.patch.unwrap_or(0)
    );

    if let Some(pre_release) = self.pre_release.as_ref() {
      value = value
        + &format!("-{}", pre_release);

      if let Some(iteration) = self.iteration {
        value = value
          + &format!(".{}", iteration);
      }
    }

    return value;
  }
}

impl AsRef<SemVer> for SemVer {
  fn as_ref(&self) -> &Self {
    self
  }
}

impl Into<String> for &SemVer {
  fn into(self) -> String {
    self.to_string()
  }
}

impl Ord for SemVer {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    self.major.cmp(&other.major)
      .then(self.minor.cmp(&other.minor))
      .then(self.patch.cmp(&other.patch))
      .then(
        self.pre_release.is_none().cmp(&other.pre_release.is_none())
      ).
      then(self.iteration.cmp(&other.iteration))
  }
}

impl PartialOrd for SemVer {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

