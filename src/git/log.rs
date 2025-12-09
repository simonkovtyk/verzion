use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitLog {
  pub message: String,
  pub author_timestamp: u64,
  pub author_name: String,
  pub author_email: String,
  pub committer_name: String,
  pub committer_email: String,
  pub committer_timestamp: u64,
  pub hash: String,
  pub abbr_hash: String
}

pub fn get_log (hash: &str) -> Option<GitLog> {
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

  let config = Config::inject();

  if let Some(cwd) = config.cwd.clone() {
    command.current_dir(cwd);
  }

  let output = command.output().expect("Could not execute git show command");

  if output.stdout.is_empty() {
    return None;
  }

  let content = str::from_utf8(&output.stdout).expect("Content contained invalid UTF-8");

  return serde_json::from_str(
    &content
  ).expect("Failed to deserialize JSON");
}

const LOG_SEPARATOR: char = '\x1f';

pub fn get_logs (cwd: &Option<String>, from: Option<String>, to: Option<&str>) -> Option<Vec<GitLog>> {
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

  if let Some(cwd) = cwd {
    log_command.current_dir(cwd);
  }

  let log_output = log_command.output().expect("Failed to execute git log command");

  if log_output.stdout.is_empty() {
    return None;
  }

  let content = str::from_utf8(&log_output.stdout).expect("Content contained invalid UTF-8");


  let logs = content.lines().map(|line| {
    let items: Vec<&str> = line.split(LOG_SEPARATOR).collect();

    GitLog {
      message: items[0].to_string(),
      author_name: items[1].to_string(),
      author_email: items[2].to_string(),
      author_timestamp: items[3].parse().unwrap_or(0),
      committer_name: items[4].to_string(),
      committer_email: items[5].to_string(),
      committer_timestamp: items[6].parse().unwrap_or(0),
      hash: items[7].to_string(),
      abbr_hash: items[8].to_string()
    }
  }).collect();

  Some(logs)
}

