use serde::{Deserialize, Serialize};

use crate::{git::config::GitRuleset, std::merge::Merge};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangelogConfig {
  pub enabled: Option<bool>,
  pub path: Option<String>,
  pub template_path: Option<String>,
  pub git_ruleset: Option<GitRuleset>
}

impl Merge for ChangelogConfig {
  fn merge (self, other: Self) -> Self {
    Self {
      enabled: self.enabled.merge(other.enabled),
      path: self.path.clone().or(other.path.clone()),
      template_path: self.template_path.clone().or(other.template_path.clone()),
      git_ruleset: self.git_ruleset.merge(other.git_ruleset)
    }
  }
}

