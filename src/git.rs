use std::process::Command;

#[derive(Debug)]
pub struct GitLog {
  pub message: String
}

pub fn get_log_messages (from: &str, to: &str, cwd: Option<String>) -> Vec<GitLog> {
  let mut command = Command::new("git");
  let log_command = command
    .args(&[
      "log",
      "--pretty=format:%s",
      &format!("{}..{}", from, to)
    ]);

  if let Some(cwd) = cwd {
    log_command.current_dir(cwd);
  }

  let log_output = log_command.output().expect("Failed to execute git log command");

  if log_output.stdout.is_empty() {
    panic!("No commits found between {} and {}", from, to);
  }

  let content = str::from_utf8(&log_output.stdout).expect("Content contained invalid UTF-8");

  return content.lines().map(|line| GitLog {
    message: line.to_string()
  }).collect();
}
