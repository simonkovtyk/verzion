use std::{env};

use clap::Parser;

use crate::{changelog::config::{ChangelogConfig, ChangelogType}, config::{Config, ToExitCode}, conventions::config::ConvetionTypes, git::tracking::{GitTracking, GitTrackingRoot, GitTrackingStrategy}, log::LogLevel, semver::config::SemVerConfig, std::panic::{EXIT_ERROR, EXIT_SUCCESS, ExpectWithStatusCode}};

#[derive(Parser, Debug, Clone)]
#[command(
  arg_required_else_help = false,
  name = "verzion",
  version,
  about = "verzion - Commit Analyzer"
)]
pub struct Args {
  /* general */
  #[arg(long, help = "Path to configuration file", help_heading = "General")]
  pub config: Option<String>,
  #[arg(long, help = "Dir of configuration file", help_heading = "General")]
  pub config_dir: Option<String>,
  #[arg(long, help = "Path to run onto", help_heading = "General")]
  pub cwd: Option<String>,
  #[arg(long, help = "Exit gracefully", help_heading = "General")]
  pub graceful: Option<bool>,
  #[arg(long, help = "Colored output", help_heading = "General")]
  pub colored: Option<bool>,
  #[arg(long, help = "Convention to use", help_heading = "General")]
  pub convention: Option<ConvetionTypes>,
  #[arg(long, help = "References to other configs", help_heading = "General")]
  pub references: Option<Vec<String>>,
  #[arg(long, help = "Exits on false without doing something", help_heading = "General")]
  pub enabled: Option<bool>,
  #[arg(long, help = "Log level for outputs", help_heading = "General")]
  pub log_level: Option<LogLevel>,

  /* git tracking */
  #[arg(long, help = "Track all dynamic files", help_heading = "Tracking")]
  pub tracking_enabled: Option<bool>,
  #[arg(long, help = "Origins for tracking", help_heading = "Tracking")]
  pub tracking_origins: Option<Vec<String>>,
  #[arg(long, help = "Custom message used while tracking", help_heading = "Tracking")]
  pub tracking_message: Option<String>,

  /* changelog */
  #[arg(long, help = "Should create a changelog", help_heading = "Changelog")]
  pub changelog_enabled: Option<bool>,
  #[arg(long, help = "Type of the changelog to generate", help_heading = "Changelog")]
  pub changelog_type: Option<ChangelogType>,
  #[arg(long, help = "Output path of changelog", help_heading = "Changelog")]
  pub changelog_path: Option<String>,
  #[arg(long, help = "Path to changelog template", help_heading = "Changelog")]
  pub changelog_template_path: Option<String>,
  #[arg(long, help = "Wether to track changelogs by Git", help_heading = "Changelog")]
  pub changelog_tracking_enabled: Option<bool>,
  #[arg(long, help = "Strategy to use while tracking changelogs", help_heading = "Changelog")]
  pub changelog_tracking_strategy: Option<GitTrackingStrategy>,
  #[arg(long, help = "Message to use while tracking changelogs", help_heading = "Changelog")]
  pub changelog_tracking_message: Option<String>,

  /* semver */
  #[arg(long, help = "Force SemVer (e.g. 1.2.0)", help_heading = "SemVer")]
  pub semver: Option<String>,
  #[arg(long, help = "Format SemVer (e.g. \"v{}\")", help_heading = "SemVer")]
  pub semver_format: Option<String>,
  #[arg(long, help = "Force SemVer Major", help_heading = "SemVer")]
  pub semver_major: Option<u64>,
  #[arg(long, help = "Force SemVer Minor", help_heading = "SemVer")]
  pub semver_minor: Option<u64>,
  #[arg(long, help = "Force SemVer Patch", help_heading = "SemVer")]
  pub semver_patch: Option<u64>,
  #[arg(long, help = "Force SemVer Pre-Release (e.g. alpha, beta)", help_heading = "SemVer")]
  pub semver_pre_release: Option<String>,
  #[arg(long, help = "Force SemVer Iteration", help_heading = "SemVer")]
  pub semver_iteration: Option<u64>,
  #[arg(long, help = "Force SemVer Metadata", help_heading = "SemVer")]
  pub semver_metadata: Option<Vec<String>>
}

impl Args {
  pub fn get_cwd (&self) -> String {
    self.cwd.clone().unwrap_or(
      env::current_dir()
      .expect_with_status_code(
        "Could not get current working directory",
        self.to_exit_code()
      )
      .to_str()
      .expect_with_status_code(
        "Could not convert cwd path since it contains invalid charset",
        self.to_exit_code()
      )
      .to_string()
    )
  }
}

impl ToExitCode for Args {
  fn to_exit_code(&self) -> i32 {
    self.graceful.map(|v| if v {
      EXIT_SUCCESS
    } else {
      EXIT_ERROR
    }).unwrap_or(EXIT_ERROR)
  }
}

impl Into<Config> for Args {
  fn into(self) -> Config {
    Config {
      graceful: self.graceful,
      cwd: self.cwd,
      references: self.references,
      colored: self.colored,
      enabled: self.enabled,
      convention: self.convention,
      log_level: self.log_level,
      semver: SemVerConfig::new(
        self.semver,
        self.semver_format,
        self.semver_major,
        self.semver_minor,
        self.semver_patch,
        self.semver_pre_release,
        self.semver_iteration,
        self.semver_metadata
      ),
      tracking: GitTrackingRoot::new(
        self.tracking_origins,
        self.tracking_enabled,
        self.tracking_message
      ),
      metafiles: None,
      changelog: ChangelogConfig::new(
        self.changelog_enabled,
        self.changelog_type,
        self.changelog_path,
        self.changelog_template_path,
        GitTracking::new(
          self.changelog_tracking_enabled,
          self.changelog_tracking_strategy,
          self.changelog_tracking_message
        )
      ),
      webhooks: None
    }
  }
}
