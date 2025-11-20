use std::{env, fs};
use clap::{ValueEnum};
use once_cell::sync::{OnceCell};
use serde::{Deserialize, Serialize};

use crate::{std::Merge, webhooks::config::WebhookConfig};

pub const CONFIG_FILE_NAME: &str = "verzion.json";

pub static CONFIG: OnceCell<Config> = OnceCell::new();

pub trait ToExitCode {
  fn to_exit_code(&self) -> i32;
}

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
  pub semver_format: Option<String>,
  pub changelog: Option<ChangelogConfig>,
  pub gitlab: Option<WebhookConfig>,
  pub github: Option<WebhookConfig>
}

impl Config {
  pub fn inject () -> &'static Self {
    CONFIG.get().expect("Could not retrieve config")
  }
}

impl ToExitCode for &Config {
  fn to_exit_code(&self) -> i32 {
    self.graceful.map(|v| if v {
      0
    } else {
      1
    }).unwrap_or(1)
  }
}

impl Merge for Config {
  fn merge(&self, other: &Self) -> Self {
    Config {
      references: self.references.merge(&other.references),
      graceful: self.graceful.or(other.graceful.clone().or(Some(false))),
      cwd: self.cwd.clone().or(other.cwd.clone()),
      colored: self.colored.or(other.colored.clone()),
      enabled: self.enabled.or(other.enabled.clone()),
      semver_format: self.semver_format.clone().or(other.semver_format.clone()),
      convention: self.convention.clone().or(other.convention.clone()),
      targets: self.targets.merge(&other.targets),
      changelog: self.changelog.merge(&other.changelog),
      gitlab: self.gitlab.merge(&other.gitlab),
      github: self.github.merge(&other.github)
    }
  }
}

impl Default for Config {
  fn default() -> Self {
    Self {
      references: None,
      graceful: None,
      cwd: None,
      colored: None,
      enabled: None,
      semver_format: None,
      convention: None,
      targets: None,
      changelog: None,
      gitlab: None,
      github: None
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
