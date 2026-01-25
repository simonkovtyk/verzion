use serde::{Deserialize, Serialize};

use crate::std::merge::Merge;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitRuleset {
  pub commit: Option<bool>,
  pub commit_msg: Option<String>,
  pub push: Option<bool>
}

impl Merge for GitRuleset {
  fn merge(self, other: Self) -> Self {
    Self {
      commit: self.commit.merge(other.commit),
      commit_msg: self.commit_msg.clone().or(other.commit_msg.clone()),
      push: self.push.merge(other.push)
    }
  }
}

pub struct GitRulesetResult {
  pub to_add_paths: Vec<String>,
  pub needs_push: bool
}

impl GitRulesetResult {
  pub fn from_ruleset (ruleset: &GitRuleset, to_add_paths: Vec<String>) -> Self {
    Self {
      to_add_paths,
      needs_push: ruleset.push.unwrap_or(false)
    }
  }
}

