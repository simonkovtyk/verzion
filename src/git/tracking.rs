use serde::{Deserialize, Serialize};
use clap::{ValueEnum};

use crate::{config::Config, git::{add::add, commit::commit}, std::{command::CommandOptions, merge::Merge}};

pub const DEFAULT_TRACKED: bool = false;
pub const DEFAULT_MESSAGE: Option<String> = None;
pub const DEFAULT_STRATEGY: GitTrackingStrategy = GitTrackingStrategy::Batch;

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
    message: Option<String>
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

        return None;
      }
      GitTrackingStrategy::Batch => {
        return Some(path.to_string());
      }
    }
  }
}

impl Merge for GitTracking {
  fn merge(self, other: Self) -> Self {
    Self {
      enabled: self.enabled.merge(other.enabled),
      strategy: self.strategy.merge(other.strategy),
      message: self.message.or(other.message),
    }
  }
}

/*
 * If tracking is enabled for a certain file, but no atomic commit should be made, we need to collect all paths for adding a single commit later.
 *
 */
pub type GitTrackingBatch = Vec<String>;
