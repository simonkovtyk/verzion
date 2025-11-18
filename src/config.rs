use std::{env, fs};
use clap::{ValueEnum};
use serde::{Deserialize, Serialize};
use merge::Merge;

use crate::{util::merge_options, webhooks::config::WebhookConfig};

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

#[derive(Serialize, Deserialize, Debug, Clone, Merge)]
pub struct Config {
  /* Accept multiple paths for e.g. monorepos */
  #[merge(strategy = merge::option::overwrite_none)]
  pub references: Option<Vec<String>>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub graceful: Option<bool>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub cwd: Option<String>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub colored: Option<bool>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub enabled: Option<bool>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub convention: Option<BumpConvetion>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub targets: Option<Vec<BumpTarget>>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub changelog: Option<ChangelogConfig>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub gitlab: Option<WebhookConfig>,
  #[merge(strategy = merge::option::overwrite_none)]
  pub github: Option<WebhookConfig>
}

impl AsRef<Config> for Config {
  fn as_ref(&self) -> &Config {
    &self
  }
}

pub fn merge_config (a: &Config, b: &Config) -> Config {
  let mut config = a.clone();

  println!("{:?}", a);
  println!("{:?}", b);

  config.merge(b.clone());
  config.gitlab = merge_options(config.gitlab, b.clone().gitlab);
  config.github = merge_options(config.github, b.clone().github);

  config
}

pub fn get_config (path: &Option<String>) -> Config {
  let mut resulting_path = env::current_dir().expect("Couldn't get current directory").join(CONFIG_FILE_NAME).to_str().unwrap().to_string();

  if let Some(path) = path {
    resulting_path = path.to_string();
  }

  let content_buf = fs::read(resulting_path).expect("Couldn't read config file");

  return serde_json::from_slice::<Config>(&content_buf).expect("Couldn't parse config file");
}
