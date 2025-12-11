use serde::{Deserialize, Serialize};
use clap::{ValueEnum};

use crate::std::Merge;

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum GitOriginType {
  All,
  Push
}

impl ToString for GitOriginType {
  fn to_string(&self) -> String {
    match self {
      GitOriginType::All => "all".to_string(),
      GitOriginType::Push => "push".to_string()
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitConfig {
  pub all_origins: Option<bool>,
  pub origin_type: Option<GitOriginType>
}

impl GitConfig {
  pub fn is_empty (&self) -> bool {
    self.all_origins.is_none()
    && self.origin_type.is_none()
  }

  pub fn new (
    all_origins: Option<bool>,
    origin_type: Option<GitOriginType>
  ) -> Option<Self> {
    let instance = Self {
      all_origins,
      origin_type
    };

    if instance.is_empty() {
      None
    } else {
      Some(instance)
    }
  }
}

impl Merge for GitConfig {
  fn merge(&self, other: &Self) -> Self {
    Self {
      all_origins: self.all_origins.or(other.all_origins),
      origin_type: self.origin_type.clone().or(other.origin_type.clone())
    }
  }
}
