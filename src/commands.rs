use clap::Parser;

use crate::{config::{BumpConvetion, Config}, webhooks::config::WebhookConfig};

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
  pub convention: Option<BumpConvetion>,
  #[arg(long, help = "References to other configs", help_heading = "General")]
  pub references: Option<Vec<String>>,
  #[arg(long, help = "Exits on false without doing something", help_heading = "General")]
  pub enabled: Option<bool>,
  #[arg(long, help = "Format SemVer", help_heading = "General")]
  pub semver_format: Option<String>,

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
      semver_format: self.semver_format.clone(),
      targets: None,
      changelog: None,
      log_level: None,
      gitlab: WebhookConfig::new(
        self.gitlab_enabled,
        self.gitlab_url.clone(),
        self.gitlab_token.clone(),
        self.gitlab_token_env.clone(),
        self.gitlab_retries.clone()
      ),
      github: WebhookConfig::new(
        self.github_enabled,
        self.github_url.clone(),
        self.github_token.clone(),
        self.github_token_env.clone(),
        self.github_retries.clone()
      )
    }
  }
}
