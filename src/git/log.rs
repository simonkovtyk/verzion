use std::process::Command;

use serde::{Deserialize, Serialize};

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

pub fn get_log (cwd: &Option<String>, hash: &str) -> Option<GitLog> {
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

  if let Some(cwd) = cwd {
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

pub fn get_logs (cwd: &Option<String>, from: Option<String>, to: Option<&str>) -> Option<Vec<GitLog>> {
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
},";
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

  return Some(serde_json::from_str(
    &format!(
      "[{}]",
      &content[..content.len() - 1]
    )
  ).expect("Failed to deserialize JSON"));
}

