use serde::{Deserialize, Serialize};

use crate::std::merge::Merge;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitOriginConfig {
  pub name: String,
  pub enabled: Option<bool>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitConfig {
  pub all_origins: Option<bool>,
  pub origins: Option<Vec<GitOriginConfig>>
}

impl GitConfig {
  pub fn is_empty (&self) -> bool {
    self.all_origins.is_none() && self.origins.is_none()
  }

  pub fn new (
    all_origins: Option<bool>,
    origins: Option<Vec<GitOriginConfig>>
  ) -> Option<Self> {
    let instance = Self {
      all_origins,
      origins
    };

    if instance.is_empty() {
      return None;
    }

    Some(instance)
  }
}

impl Merge for GitConfig {
  fn merge (self, other: Self) -> Self {
    Self {
      all_origins: self.all_origins.or(other.all_origins),
      origins: self.origins.merge(other.origins)
    }
  }
}
