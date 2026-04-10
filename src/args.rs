use std::{env};

use clap::Parser;

use crate::{changelog::config::{ChangelogConfig, ChangelogType}, config::{Config, ToExitCode}, conventions::config::ConvetionTypes, git::tracking::{GitTracking, GitTrackingRoot, GitTrackingStrategy}, log::LogLevel, semver::config::SemVerConfig, std::{option::ToOption, panic::{EXIT_ERROR, EXIT_SUCCESS, ExpectWithStatusCode}}, webhooks::config::{WebhookItemConfig, WebhookType}};

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
  #[arg(long, help = "Use changelog in Webhooks", help_heading = "Changelog")]
  pub changelog_use_in_webhooks: Option<bool>,
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
  pub semver_metadata: Option<Vec<String>>,

  /* webhook */
  #[arg(long, help = "Create a Webhook with origin", help_heading = "Webhook")]
  pub webhook_custom_origin: Option<String>,
  #[arg(long, help = "Create a Webhook with enablement", help_heading = "Webhook")]
  pub webhook_custom_enabled: Option<bool>,
  #[arg(long, help = "Create a Webhook with url", help_heading = "Webhook")]
  pub webhook_custom_url: Option<String>,
  #[arg(long, help = "Create a Webhook with token", help_heading = "Webhook")]
  pub webhook_custom_token: Option<String>,
  #[arg(long, help = "Create a Webhook with token env", help_heading = "Webhook")]
  pub webhook_custom_token_env: Option<String>,
  #[arg(long, help = "Create a Webhook with HTTP retries", help_heading = "Webhook")]
  pub webhook_custom_http_retries: Option<u32>,
  #[arg(long, help = "Create a Webhook with HTTP retries", help_heading = "Webhook")]
  pub webhook_custom_http_timeout: Option<u64>,

  #[arg(long, help = "Create a Webhook with origin", help_heading = "Webhook")]
  pub webhook_gitlab_origin: Option<String>,
  #[arg(long, help = "Create a Webhook with enablement", help_heading = "Webhook")]
  pub webhook_gitlab_enabled: Option<bool>,
  #[arg(long, help = "Create a Webhook with url", help_heading = "Webhook")]
  pub webhook_gitlab_url: Option<String>,
  #[arg(long, help = "Create a Webhook with token", help_heading = "Webhook")]
  pub webhook_gitlab_token: Option<String>,
  #[arg(long, help = "Create a Webhook with token env", help_heading = "Webhook")]
  pub webhook_gitlab_token_env: Option<String>,
  #[arg(long, help = "Create a Webhook with HTTP retries", help_heading = "Webhook")]
  pub webhook_gitlab_http_retries: Option<u32>,
  #[arg(long, help = "Create a Webhook with HTTP retries", help_heading = "Webhook")]
  pub webhook_gitlab_http_timeout: Option<u64>,

  #[arg(long, help = "Create a Webhook with enablement", help_heading = "Webhook")]
  pub webhook_github_origin: Option<String>,
  #[arg(long, help = "Create a Webhook with enablement", help_heading = "Webhook")]
  pub webhook_github_enabled: Option<bool>,
  #[arg(long, help = "Create a Webhook with url", help_heading = "Webhook")]
  pub webhook_github_url: Option<String>,
  #[arg(long, help = "Create a Webhook with token", help_heading = "Webhook")]
  pub webhook_github_token: Option<String>,
  #[arg(long, help = "Create a Webhook with token env", help_heading = "Webhook")]
  pub webhook_github_token_env: Option<String>,
  #[arg(long, help = "Create a Webhook with HTTP retries", help_heading = "Webhook")]
  pub webhook_github_http_retries: Option<u32>,
  #[arg(long, help = "Create a Webhook with HTTP retries", help_heading = "Webhook")]
  pub webhook_github_http_timeout: Option<u64>
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
    let mut webhook_config = Vec::new();

    let custom = WebhookItemConfig::new(
      Some(WebhookType::Custom),
      self.webhook_custom_origin,
      self.webhook_custom_enabled,
      self.webhook_custom_url,
      self.webhook_custom_token,
      self.webhook_custom_token_env,
      self.webhook_custom_http_retries,
      self.webhook_custom_http_timeout
    );

    if let Some(value) = custom {
      webhook_config.push(value);
    }

    let gitlab = WebhookItemConfig::new(
      Some(WebhookType::GitLab),
      self.webhook_gitlab_origin,
      self.webhook_gitlab_enabled,
      self.webhook_gitlab_url,
      self.webhook_gitlab_token,
      self.webhook_gitlab_token_env,
      self.webhook_gitlab_http_retries,
      self.webhook_gitlab_http_timeout
    );

    if let Some(value) = gitlab {
      webhook_config.push(value);
    }

    let github = WebhookItemConfig::new(
      Some(WebhookType::GitHub),
      self.webhook_github_origin,
      self.webhook_github_enabled,
      self.webhook_github_url,
      self.webhook_github_token,
      self.webhook_github_token_env,
      self.webhook_github_http_retries,
      self.webhook_github_http_timeout
    );

    if let Some(value) = github {
      webhook_config.push(value);
    }

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
        self.changelog_use_in_webhooks,
        GitTracking::new(
          self.changelog_tracking_enabled,
          self.changelog_tracking_strategy,
          self.changelog_tracking_message
        )
      ),
      webhooks: webhook_config.to_option()
    }
  }
}
