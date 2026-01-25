use serde::{Deserialize, Serialize};

use crate::{git::config::GitRuleset};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MetafileTypes {
  Java,
  Node,
  Plain
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetafileTarget {
  pub r#type: MetafileTypes,
  pub path: String,
  pub git_ruleset: GitRuleset
}
