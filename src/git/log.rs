use std::process::Command;
use serde::{Deserialize, Serialize};

use crate::{config::Config, std::command::CommandOptions};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitLogStakeholder {
  name: String,
  email: String,
  timestamp: Option<u64>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitLog {
  pub message: String,
  pub author: GitLogStakeholder,
  pub comitter: GitLogStakeholder,
  pub hash: String,
  pub abbr_hash: String
}

pub fn get_log (hash: &str, options: CommandOptions) -> Result<GitLog, String> {
  let mut command = Command::new("git");
  let pretty_format = "{
\"message\":\"%s\",
\"author_name\":\"%an\",
\"author_email\":\"%ae\",
\"author_timestamp\":%at,
\"committer_name\":\"%cn\",
\"committer_email\":\"%ce\",
\"committer_timestamp\":%ct,
\"hash\":\"%H\",
\"abbr_hash\":\"%h\"
}";

  command.args(&[
    "log",
    "-1",
    hash.trim(),
    &format!("--pretty=format:{}", pretty_format),
    "--no-patch",
  ]);

  if let Some(cwd) = options.cwd.as_ref() {
    command.current_dir(cwd);
  }

  let output = command.output().map_err(|_| "Invoking Git failed")?;

  if output.stdout.is_empty() {
    return Err("No log found for the given hash".to_string());
  }

  let content = str::from_utf8(&output.stdout).map_err(|_| "Content contained invalid UTF-8")?;

  return serde_json::from_str(
    &content
  ).map_err(|_| "Failed to deserialize JSON".to_string());
}

const LOG_SEPARATOR: char = '\x1f';

pub fn get_logs (
  from: Option<String>,
  to: Option<&str>,
  options: CommandOptions
) -> Result<Vec<GitLog>, String> {
  let mut command = Command::new("git");
  let pretty_format = format!(
    "%s{sep}%an{sep}%ae{sep}%at{sep}%cn{sep}%ce{sep}%ct{sep}%H{sep}%h",
    sep = LOG_SEPARATOR
  );
  let log_command = command
    .args(&[
      "log",
      &format!("--pretty=format:{}", pretty_format)
    ]);

  if let Some(inner_from) = from {
    let _to = if let Some(inner_to) = to {
      inner_to
    } else {
      "HEAD"
    };

    log_command.arg(
      format!("{}..{}", inner_from, _to)
    );
  }

  if let Some(cwd) = options.cwd.as_ref() {
    log_command.current_dir(cwd);
  }

  let log_output = log_command.output()
    .map_err(|_| "Failed to execute git log command")?;

  if log_output.stdout.is_empty() {
    return Err("No logs found".to_string());
  }

  let content = str::from_utf8(&log_output.stdout).map_err(|_| "Content contained invalid UTF-8")?;

  let logs = content.lines().map(|line| {
    let items: Vec<&str> = line.split(LOG_SEPARATOR).collect();

    GitLog {
      message: items[0].to_string(),
      hash: items[7].to_string(),
      abbr_hash: items[8].to_string(),
      author: GitLogStakeholder {
        name: items[1].to_string(),
        email: items[2].to_string(),
        timestamp: items[3].parse().ok()
      },
      comitter: GitLogStakeholder {
        name: items[4].to_string(),
        email: items[5].to_string(),
        timestamp: items[6].parse().ok()
      }
    }
  }).collect();

  Ok(logs)
}

