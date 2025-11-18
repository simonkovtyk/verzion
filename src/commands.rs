use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(arg_required_else_help = true, name = "nexlog", version, about = "commit analyzer")]
pub struct Args {
  #[arg(long, help = "Path to configuration file")]
  pub config: Option<String>,
  #[arg(long, help = "Path to run onto")]
  pub cwd: Option<String>,
  #[arg(long, help = "GitLab token")]
  pub gitlab_token: Option<String>,
  #[arg(long, help = "GitLab token")]
  pub github_token: Option<String>
}
