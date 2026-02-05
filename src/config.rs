use std::{fs, path::PathBuf, str::FromStr};
use once_cell::sync::{OnceCell};
use serde::{Deserialize, Serialize};

use crate::{args::Args, changelog::config::ChangelogConfig, conventions::config::ConventionConfig, git::tracking::GitTrackingRoot, log::LogLevel, metafile::config::Metafile, package::NAME, semver::config::SemVerConfig, std::{merge::Merge, panic::{EXIT_ERROR, EXIT_SUCCESS, ExpectWithStatusCode}}, webhooks::config::WebhookConfig};

pub const DEFAULT_CONFIG_FILE_EXTENSION: &str = "json";
pub const DEFAULT_CONFIG_FILE_BASE: &str = "config";

pub fn get_default_config_file_name () -> String {
  format!(
    "{}.{}",
    DEFAULT_CONFIG_FILE_BASE,
    DEFAULT_CONFIG_FILE_EXTENSION
  )
}

pub fn get_default_config_dir () -> String {
  format!(
    ".{}",
    NAME.to_lowercase()
  )
}

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
  pub metafiles: Option<Vec<Metafile>>,
  pub convention: Option<ConventionConfig>,
  pub changelog: Option<ChangelogConfig>,
  pub webhooks: Option<WebhookConfig>,
  pub tracking: Option<GitTrackingRoot>
}

impl Config {
  pub fn inject () -> &'static Self {
    CONFIG.get().expect("Could not retrieve config")
  }

  pub fn from_args (args: &Args) -> Self {
    let path_buf = args.config.as_ref()
      .map(|v|
        PathBuf::from_str(&v)
          .expect_with_status_code(
            "Could not parse",
            args.to_exit_code()
          )
      )
      .unwrap_or(
        PathBuf::from_str(&args.get_cwd())
        .expect_with_status_code(
          "Could not parse cwd",
          args.to_exit_code()
        )
        .join(get_default_config_dir())
        .join(get_default_config_file_name())
      );

    let content_buf = fs::read(path_buf)
      .expect_with_status_code(
        "Couldn't read config file",
        args.to_exit_code()
      );

    serde_json::from_slice::<Config>(&content_buf)
      .expect_with_status_code(
        "Couldn't parse config file",
        args.to_exit_code()
      )
  }
}

pub trait ToExitCode {
  fn to_exit_code(&self) -> i32;
}

impl ToExitCode for &Config {
  fn to_exit_code(&self) -> i32 {
    self.graceful.map(|v| if v {
      EXIT_SUCCESS
    } else {
      EXIT_ERROR
    }).unwrap_or(EXIT_SUCCESS)
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
      webhooks: self.webhooks.merge(other.webhooks),
      tracking: self.tracking.merge(other.tracking)
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
      webhooks: None,
      tracking: None
    }
  }
}

impl AsRef<Config> for Config {
  fn as_ref(&self) -> &Config {
    &self
  }
}
