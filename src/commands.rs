use clap::Parser;

use crate::{config::{BumpConvetion, Config}, webhooks::config::WebhookConfig};

#[derive(Parser, Debug, Clone)]
#[command(arg_required_else_help = true, name = "verzion", version, about = "verzion - Commit Analyzer")]
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
  
  /* github */
  #[arg(long, help = "GitHub enabled", help_heading = "GitHub")]
  pub github_enabled: Option<bool>,
  #[arg(long, help = "GitHub token", help_heading = "GitHub")]
  pub github_token: Option<String>,
  #[arg(long, help = "GitHub token environment variable name", help_heading = "GitHub")]
  pub github_token_env: Option<String>,
  #[arg(long, help = "GitHub remote url", help_heading = "GitHub")]
  pub github_url: Option<String>
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
      gitlab: Some(WebhookConfig {
        enabled: self.gitlab_enabled,
        url: self.gitlab_url.clone(),
        token: self.gitlab_token.clone(),
        token_env: self.gitlab_token_env.clone()
      }),
      github: Some(WebhookConfig {
        enabled: self.github_enabled,
        url: self.github_url.clone(),
        token: self.github_token.clone(),
        token_env: self.github_token_env.clone()
      })
    }
  }
}
