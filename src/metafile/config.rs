use serde::{Deserialize, Serialize};

use crate::std::merge::Merge;

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
  pub path: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetafileConfig {
  pub targets: Option<Vec<MetafileTarget>>,
  pub push: Option<bool>
}

impl Merge for MetafileConfig {
  fn merge(self, other: Self) -> Self {
    Self {
      targets: self.targets.merge(other.targets),
      push: self.push.or(other.push)
    }
  }
}
