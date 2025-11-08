use std::process::Command;

pub struct GitLog {
  pub message: String
}

pub fn get_log_messages (from: &str, to: &str) -> Vec<GitLog> {
  let log_output = Command::new("git")
    .args(&[
      "log",
      &format!("{}..{}", from, to)
    ]).output().expect("Failed to execute git log command");

  if log_output.stdout.is_empty() {
    panic!("No commits found between {} and {}", from, to);
  }

  let content = str::from_utf8(&log_output.stdout).expect("Content contained invalid UTF-8");

  return content.lines().map(|line| GitLog {
    message: line.to_string()
  }).collect();
}
