use serde::{Deserialize, Serialize};
use clap::{ValueEnum};

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum ConvetionTypes {
  Conventional
}

pub type ConventionConfig = ConvetionTypes;
