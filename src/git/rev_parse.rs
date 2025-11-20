use std::process::Command;

use crate::config::Config;

pub fn get_rev_parse (rev: &str) -> Option<String> {
  let mut rev_parse_command = Command::new("git");

  rev_parse_command.args(&[
    "rev-parse",
    rev
  ]);

  let config = Config::inject();

  if let Some(cwd) = config.cwd.clone() {
    rev_parse_command.current_dir(cwd);
  }

  let output = rev_parse_command.output().expect("Could not execute git rev-parse command");

  if output.stdout.is_empty() {
    return None;
  }

  String::from_utf8(output.stdout).ok()
}
