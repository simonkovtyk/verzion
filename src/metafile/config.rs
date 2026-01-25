use serde::{Deserialize, Serialize};

use crate::git::tracking::GitTracking;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MetafileTypes {
  Java,
  Node,
  Plain
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metafile {
  pub r#type: MetafileTypes,
  pub path: String,
  pub tracking: Option<GitTracking>
}
