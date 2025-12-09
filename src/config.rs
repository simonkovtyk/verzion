use std::{env, fs, path::PathBuf, str::FromStr};
use clap::{ValueEnum};
use once_cell::sync::{OnceCell};
use serde::{Deserialize, Serialize};

use crate::{commands::Args, semver::SemVer, std::Merge, webhooks::config::WebhookConfig};

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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum LogLevel {
  None = 0,
  Error = 1,
  Warn = 2,
  Info = 3,
  Success = 4,
  Debug = 5
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SemVerConfig {
  pub semver: Option<String>,
  pub format: Option<String>,
  pub major: Option<u64>,
  pub minor: Option<u64>,
  pub patch: Option<u64>,
  pub pre_release: Option<String>,
  pub iteration: Option<u64>,
  pub metadata: Option<Vec<String>>
}

impl SemVerConfig {
  pub fn is_empty (&self) -> bool {
    self.semver.is_none()
      && self.format.is_none()
      && self.major.is_none()
      && self.minor.is_none()
      && self.patch.is_none()
      && self.pre_release.is_none()
      && self.iteration.is_none()
      && self.metadata.is_none()
  }

  pub fn new (
    semver: Option<String>,
    format: Option<String>,
    major: Option<u64>,
    minor: Option<u64>,
    patch: Option<u64>,
    pre_release: Option<String>,
    iteration: Option<u64>,
    metadata: Option<Vec<String>>
  ) -> Option<Self> {
    let instance = Self {
      semver,
      format,
      major,
      minor,
      patch,
      pre_release,
      iteration,
      metadata
    };

    if instance.is_empty() {
      None
    } else {
      Some(instance)
    }
  }
}

impl SemVerConfig {
  pub fn to_semver (self) -> SemVer {
    let mut semver = if let Some(inner_semver) = self.semver {
      SemVer::try_from_str(&inner_semver).expect("Expect valid semver")
    } else {
      SemVer::default()
    };

    semver.major = self.major.or(semver.major);
    semver.minor = self.minor.or(semver.minor);
    semver.patch = self.patch.or(semver.patch);
    semver.pre_release = self.pre_release.or(semver.pre_release);
    semver.iteration = self.iteration.or(semver.iteration);
    semver.metadata = self.metadata.or(semver.metadata);

    semver
  }

  pub fn to_semver_with_format (self) -> SemVer {
    let mut semver = if let Some(inner_semver) = self.semver {
      SemVer::try_from_format(&inner_semver, &self.format).expect("Expect valid semver")
    } else {
      SemVer::default()
    };

    semver.major = self.major.or(semver.major);
    semver.minor = self.minor.or(semver.minor);
    semver.patch = self.patch.or(semver.patch);
    semver.pre_release = self.pre_release.or(semver.pre_release);
    semver.iteration = self.iteration.or(semver.iteration);
    semver.metadata = self.metadata.or(semver.metadata);

    semver
  }
}

impl Merge for SemVerConfig {
  fn merge(&self, other: &Self) -> Self {
    Self {
      semver: self.semver.clone().or(other.semver.clone()),
      format: self.format.clone().or(other.format.clone()),
      major: self.major.clone().or(other.major.clone()),
      minor: self.minor.clone().or(other.minor.clone()),
      patch: self.patch.clone().or(other.patch.clone()),
      pre_release: self.pre_release.clone().or(other.pre_release.clone()),
      iteration: self.iteration.clone().or(other.iteration.clone()),
      metadata: self.metadata.merge(&other.metadata)
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
  pub semver: Option<SemVerConfig>,
  pub changelog: Option<ChangelogConfig>,
  pub log_level: Option<LogLevel>,
  pub gitlab: Option<WebhookConfig>,
  pub github: Option<WebhookConfig>
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
      semver: self.semver.merge(&other.semver),
      convention: self.convention.clone().or(other.convention.clone()),
      targets: self.targets.merge(&other.targets),
      changelog: self.changelog.merge(&other.changelog),
      log_level: self.log_level.clone().or(other.log_level.clone()),
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
      semver: None,
      convention: None,
      targets: None,
      changelog: None,
      log_level: None,
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
