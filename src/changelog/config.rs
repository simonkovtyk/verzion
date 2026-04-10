use serde::{Deserialize, Serialize};
use clap::ValueEnum;
use crate::{git::tracking::{GitTracking}, std::merge::Merge};

pub const DEFAULT_CHANGELOG_TYPE: ChangelogType = ChangelogType::Simple;
pub const DEFAULT_TEMPLATE_PATH: &str = ".verzion/changelog_template.md";
pub const DEFAULT_CHANGELOG_PATH: &str = "CHANGELOG.md";

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum)]
#[serde(rename_all = "lowercase", tag = "type")]
pub enum ChangelogType {
  Simple,
  Template
}

impl Merge for ChangelogType {
  fn merge (self, other: Self) -> Self {
    other
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangelogConfig {
  pub enabled: Option<bool>,
  pub r#type: Option<ChangelogType>,
  pub path: Option<String>,
  pub tracking: Option<GitTracking>,
  pub template_path: Option<String>,
  pub use_in_webhooks: Option<bool>
}

impl ChangelogConfig {
  pub fn is_empty (&self) -> bool {
    self.enabled.is_none()
      && self.r#type.is_none()
      && self.path.is_none()
      && self.tracking.is_none()
      && self.template_path.is_none()
  }

  pub fn new (
    enabled: Option<bool>,
    r#type: Option<ChangelogType>,
    path: Option<String>,
    template_path: Option<String>,
    use_in_webhooks: Option<bool>,
    tracking: Option<GitTracking>
  ) -> Option<Self> {
    let instance = Self {
      enabled,
      r#type,
      path,
      tracking,
      template_path,
      use_in_webhooks
    };

    if instance.is_empty() {
      return None;
    }

    Some(instance)
  }
}

impl Merge for ChangelogConfig {
  fn merge (self, other: Self) -> Self {
    Self {
      enabled: self.enabled.merge(other.enabled),
      r#type: self.r#type.merge(other.r#type),
      path: self.path.or(other.path),
      tracking: self.tracking.merge(other.tracking),
      template_path: other.template_path,
      use_in_webhooks: self.use_in_webhooks.merge(other.use_in_webhooks)
    }
  }
}
