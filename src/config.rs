use std::{env, fs};
use clap::{ValueEnum};
use serde::{Deserialize, Serialize};

use crate::{std::Merge, webhooks::config::WebhookConfig};

pub const CONFIG_FILE_NAME: &str = "nexlog.json";

#[derive(Serialize, Deserialize, Debug, Clone, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum BumpConvetion {
  Conventional
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BumpTypes {
  Java,
  Node,
  Plain
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BumpTarget {
  pub r#type: BumpTypes,
  pub path: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChangelogConfig {
  pub enabled: Option<bool>,
  pub path: Option<String>
}

impl Merge for ChangelogConfig {
  fn merge (&self, other: &Self) -> Self {
    Self {
      enabled: self.enabled.merge(&other.enabled),
      path: self.path.clone().or(other.path.clone())
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
  /* Accept multiple paths for e.g. monorepos */
  pub references: Option<Vec<String>>,
  pub graceful: Option<bool>,
  pub cwd: Option<String>,
  pub colored: Option<bool>,
  pub enabled: Option<bool>,
  pub convention: Option<BumpConvetion>,
  pub targets: Option<Vec<BumpTarget>>,
  pub changelog: Option<ChangelogConfig>,
  pub gitlab: Option<WebhookConfig>,
  pub github: Option<WebhookConfig>
}

impl Merge for Config {
  fn merge(&self, other: &Self) -> Self {
    Config {
      references: self.references.merge(&other.references),
      graceful: self.graceful.or(other.graceful.clone().or(Some(false))),
      cwd: self.cwd.clone().or(other.cwd.clone()),
      colored: self.colored.or(other.colored.clone()),
      enabled: self.enabled.or(other.enabled.clone()),
      convention: self.convention.clone().or(other.convention.clone()),
      targets: self.targets.merge(&other.targets),
      changelog: self.changelog.merge(&other.changelog),
      gitlab: self.gitlab.merge(&other.gitlab),
      github: self.github.merge(&other.github)
    }
  }
}

impl AsRef<Config> for Config {
  fn as_ref(&self) -> &Config {
    &self
  }
}

pub fn get_config (path: &Option<String>) -> Config {
  let mut resulting_path = env::current_dir().expect("Couldn't get current directory").join(CONFIG_FILE_NAME).to_str().unwrap().to_string();

  if let Some(path) = path {
    resulting_path = path.to_string();
  }

  let content_buf = fs::read(resulting_path).expect("Couldn't read config file");

  return serde_json::from_slice::<Config>(&content_buf).expect("Couldn't parse config file");
}
