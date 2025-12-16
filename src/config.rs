use std::{env, fs, path::PathBuf, str::FromStr};
use once_cell::sync::{OnceCell};
use serde::{Deserialize, Serialize};

use crate::{args::Args, changelog::config::ChangelogConfig, conventions::config::ConventionConfig, git::config::GitConfig, log::LogLevel, metafile::config::MetafileConfig, semver::config::SemVerConfig, std::merge::Merge, webhooks::config::WebhookConfig};

pub const CONFIG_FILE_NAME: &str = "verzion.json";

pub static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
  pub enabled: Option<bool>,
  pub colored: Option<bool>,
  pub graceful: Option<bool>,
  pub cwd: Option<String>,
  /* Accept multiple paths for e.g. monorepos */
  pub references: Option<Vec<String>>,
  pub log_level: Option<LogLevel>,
  pub semver: Option<SemVerConfig>,
  pub metafiles: Option<MetafileConfig>,
  pub convention: Option<ConventionConfig>,
  pub changelog: Option<ChangelogConfig>,
  pub git: Option<GitConfig>,
  pub webhooks: Option<WebhookConfig>
}

impl Config {
  pub fn inject () -> &'static Self {
    CONFIG.get().expect("Could not retrieve config")
  }

  pub fn from_args (args: &Args) -> Self {
    let path_buf = args.config.clone()
      .map(|v|
        PathBuf::from_str(&v).expect("Could not parse")
      )
      .unwrap_or(
        PathBuf::from_str(&args.cwd.clone()
          .unwrap_or(
            env::current_dir().expect("Could not get current directory")
              .to_str()
              .expect("Contains invalid UTF-8")
              .to_string()
        )
      ).expect("Could not parse cwd").join(CONFIG_FILE_NAME)
    );

    let content_buf = fs::read(path_buf).expect("Couldn't read config file");

    serde_json::from_slice::<Config>(&content_buf).expect("Couldn't parse config file")
  }
}

pub trait ToExitCode {
  fn to_exit_code(&self) -> i32;
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
  fn merge(self, other: Self) -> Self {
    Config {
      references: self.references.merge(other.references),
      graceful: self.graceful.or(other.graceful.or(Some(false))),
      cwd: self.cwd.or(other.cwd),
      colored: self.colored.or(other.colored),
      enabled: self.enabled.or(other.enabled),
      semver: self.semver.merge(other.semver),
      convention: self.convention.or(other.convention),
      metafiles: self.metafiles.merge(other.metafiles),
      changelog: self.changelog.merge(other.changelog),
      log_level: self.log_level.or(other.log_level),
      git: self.git.merge(other.git),
      webhooks: self.webhooks.merge(other.webhooks),
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
      semver: None,
      convention: None,
      metafiles: None,
      changelog: None,
      log_level: None,
      git: None,
      webhooks: None
    }
  }
}

impl AsRef<Config> for Config {
  fn as_ref(&self) -> &Config {
    &self
  }
}
