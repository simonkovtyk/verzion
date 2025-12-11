use clap::Parser;

use crate::{config::Config, conventions::config::ConvetionTypes, git::config::{GitConfig, GitOriginType}, log::LogLevel, remotes::config::RemoteConfig, semver::config::SemVerConfig};

#[derive(Parser, Debug, Clone)]
#[command(arg_required_else_help = false, name = "verzion", version, about = "verzion - Commit Analyzer")]
pub struct Args {
  /* general */
  #[arg(long, help = "Path to configuration file", help_heading = "General")]
  pub config: Option<String>,
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

  /* changelog */
  #[arg(long, help = "Should create a changelog", help_heading = "Changelog")]
  pub changelog_enabled: Option<bool>,
  #[arg(long, help = "Push changelog to remote", help_heading = "Changelog")]
  pub changelog_push: Option<bool>,
  #[arg(long, help = "Output path of changelog", help_heading = "Changelog")]
  pub changelog_path: Option<String>,
  #[arg(long, help = "Path to changelog template", help_heading = "Changelog")]
  pub changelog_template_path: Option<String>,

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

  /* git */
  #[arg(long, help = "Handle all git origins", help_heading = "Git")]
  pub git_all_origins: Option<bool>,
  #[arg(long, help = "Origin types to handle", help_heading = "Git")]
  pub git_origin_type: Option<GitOriginType>,

  /* gitlab */
  #[arg(long, help = "GitLab enabled", help_heading = "GitLab")]
  pub gitlab_enabled: Option<bool>,
  #[arg(long, help = "GitLab token", help_heading = "GitLab")]
  pub gitlab_token: Option<String>,
  #[arg(long, help = "GitLab token environment variable name", help_heading = "GitLab")]
  pub gitlab_token_env: Option<String>,
  #[arg(long, help = "GitLab remote url", help_heading = "GitLab")]
  pub gitlab_url: Option<String>,
  #[arg(long, help = "GitLab HTTP retries", help_heading = "GitLab")]
  pub gitlab_retries: Option<u32>,
  
  /* github */
  #[arg(long, help = "GitHub enabled", help_heading = "GitHub")]
  pub github_enabled: Option<bool>,
  #[arg(long, help = "GitHub token", help_heading = "GitHub")]
  pub github_token: Option<String>,
  #[arg(long, help = "GitHub token environment variable name", help_heading = "GitHub")]
  pub github_token_env: Option<String>,
  #[arg(long, help = "GitHub remote url", help_heading = "GitHub")]
  pub github_url: Option<String>,
  #[arg(long, help = "GitHub HTTP retries", help_heading = "GitHub")]
  pub github_retries: Option<u32>
}

impl Into<Config> for &Args {
  fn into(self) -> Config {
    Config {
      graceful: self.graceful,
      cwd: self.cwd.clone(),
      references: self.references.clone(),
      colored: self.colored,
      enabled: self.enabled,
      convention: self.convention.clone(),
      semver: SemVerConfig::new(
        self.semver.clone(),
        self.semver_format.clone(),
        self.semver_major.clone(),
        self.semver_minor.clone(),
        self.semver_patch.clone(),
        self.semver_pre_release.clone(),
        self.semver_iteration.clone(),
        self.semver_metadata.clone()
      ),
      metafiles: None,
      changelog: None,
      log_level: self.log_level.clone(),
      git: GitConfig::new(
        self.git_all_origins,
        self.git_origin_type.clone()
      ),
      gitlab: RemoteConfig::new(
        self.gitlab_enabled,
        self.gitlab_url.clone(),
        self.gitlab_token.clone(),
        self.gitlab_token_env.clone(),
        self.gitlab_retries.clone()
      ),
      github: RemoteConfig::new(
        self.github_enabled,
        self.github_url.clone(),
        self.github_token.clone(),
        self.github_token_env.clone(),
        self.github_retries.clone()
      )
    }
  }
}
