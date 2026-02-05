use serde::{Deserialize, Serialize};
use clap::{ValueEnum};

use crate::{config::{Config, ToExitCode}, conventions::{config::{ConvetionTypes, DEFAULT_CONVENTION}, conventional::{advertise::get_commit_msg_footer, builder::{ConventionalBuilder, ConventionalHeader}, types::Types}}, git::{add::add, commit::commit, push::push}, semver::core::SemVer, std::{command::CommandOptions, merge::Merge, panic::ExpectWithStatusCode}};

pub const DEFAULT_TRACKED: bool = false;
#[allow(dead_code)]
pub const DEFAULT_MESSAGE: Option<String> = None;
pub const DEFAULT_STRATEGY: GitTrackingStrategy = GitTrackingStrategy::Batch;
pub const DEFAULT_ORIGINS: [&str; 1] = ["origin"];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum GitTrackingStrategy {
  /* At the end, git will be invoked once to track the file */
  Batch,
  /* Creates a separate commit for a certain file */
  Individual
}

impl Merge for GitTrackingStrategy {
  fn merge(self, other: Self) -> Self {
    match self {
      Self::Batch => other,
      Self::Individual => self
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitTrackingRoot {
  /* Wether to commit the relating change */
  pub enabled: Option<bool>,
  /* Git remote origins to track against */
  pub origins: Option<Vec<String>>,
  /* Git commit message customization */
  pub message: Option<String>
}

impl Merge for GitTrackingRoot {
  fn merge(self, other: Self) -> Self {
    Self {
      enabled: self.enabled.merge(other.enabled),
      origins: self.origins.or(other.origins),
      message: self.message.or(other.message)
    }
  }
}

impl GitTrackingRoot {
  pub fn is_empty (&self) -> bool {
    self.origins.is_none()
      && self.enabled.is_none()
      && self.message.is_none()
  }

  pub fn new (
    origins: Option<Vec<String>>,
    enabled: Option<bool>,
    message: Option<String>
  ) -> Option<Self> {
    let instance = Self {
      origins,
      enabled,
      message
    };
    if instance.is_empty() {
      None
    } else {
      Some(instance)
    }
  }

  pub fn is_enabled (&self) -> bool {
    self.enabled.unwrap_or(DEFAULT_TRACKED)
  }

  pub fn track_batch (
    &self,
    semver: &SemVer,
    batch: GitTrackingBatch
  ) -> Result<(), String> {
    if !self.is_enabled() {
      return Ok(());
    }

    let config = Config::inject();

    for path in batch.iter() {
     add(path, CommandOptions {
      cwd: config.cwd.clone()
      })?;
    }

    let message = self.message.clone().unwrap_or(get_commit_msg(semver));

    commit(message.as_str(), CommandOptions {
      cwd: config.cwd.clone()
    })?;

    for origin in self.origins
      .as_ref()
      .map(|v| v.iter().map(|v| v.as_str()).collect())
      .unwrap_or(DEFAULT_ORIGINS.to_vec()) {
      push(origin, CommandOptions {
        cwd: config.cwd.clone()
      })?;
    }

    Ok(())
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GitTracking {
  /* Wether to commit the relating change */
  pub enabled: Option<bool>,
  /* Strategy to use while tracking a file */
  pub strategy: Option<GitTrackingStrategy>,
  /* Git commit message customization */
  pub message: Option<String>,
}

impl GitTracking {
  pub fn is_empty (&self) -> bool {
    self.enabled.is_none()
      && self.strategy.is_none()
      && self.message.is_none()
  }

  pub fn new (
    enabled: Option<bool>,
    strategy: Option<GitTrackingStrategy>,
    message: Option<String>,
  ) -> Option<Self> {
    let instance = Self {
      enabled,
      strategy,
      message
    };

    if instance.is_empty() {
      None
    } else {
      Some(instance)
    }
  }

  pub fn is_enabled (&self) -> bool {
    self.enabled.unwrap_or(DEFAULT_TRACKED)
  }

  pub fn get_strategy (&self) -> GitTrackingStrategy {
    self.strategy.clone().unwrap_or(DEFAULT_STRATEGY)
  }

  pub fn track (
    &self,
    path: &str,
    default_message: &str
  ) -> Option<String> {
    if !self.is_enabled() {
      return None;
    }

    match self.get_strategy() {
      GitTrackingStrategy::Individual => {
        let config = Config::inject();

        add(path, CommandOptions {
          cwd: config.cwd.clone()
        }).ok()?;

        let message = self.message.as_ref().map_or(default_message, |v| v.as_str());

        commit(message, CommandOptions {
          cwd: config.cwd.clone()
        }).ok()?;

        None
      }
      GitTrackingStrategy::Batch => {
        Some(path.to_string())
      }
    }
  }
}

impl Merge for GitTracking {
  fn merge(self, other: Self) -> Self {
    Self {
      enabled: self.enabled.merge(other.enabled),
      strategy: self.strategy.merge(other.strategy),
      message: self.message.or(other.message)
    }
  }
}

/*
 * If tracking is enabled for a certain file, but no atomic commit should be made, we need to collect all paths for adding a single commit later.
 */
pub type GitTrackingBatch = Vec<String>;

pub fn get_conventional_commit_msg (
  semver: &SemVer
) -> String {
  let config = Config::inject();

  let semver_format = config.semver.as_ref().map(|v| v.format.clone()).flatten();

  let conventional_header = ConventionalHeader::new(
    Some(Types::Chore),
    None,
    Some(
      format!("release {}", semver.format(&semver_format))
    ),
    Some(false)
  );

  return ConventionalBuilder::new(
    conventional_header.try_into().ok(),
    None,
    Some(vec![get_commit_msg_footer()])
  ).try_into()
    .expect_with_status_code(
      "Could not get conventional commit message",
      config.to_exit_code()
    );
}

pub fn get_commit_msg (
  semver: &SemVer
) -> String {
  let config = Config::inject();

  match config.convention.as_ref().unwrap_or(&DEFAULT_CONVENTION) {
    ConvetionTypes::Conventional => get_conventional_commit_msg(semver)
  }
}
