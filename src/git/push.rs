use std::process::Command;

use crate::{std::command::CommandOptions};

pub fn push_tag (
  origin_name: &str,
  tag_name: &str,
  options: CommandOptions
) -> Result<(), String> {
  let mut command = Command::new("git");

  command.args(&[
    "push",
    origin_name,
    "tag",
    tag_name
  ]);

  if let Some(cwd) = options.cwd.as_ref() {
    command.current_dir(cwd);
  }

  command.output().map(|_| ()).map_err(|_| "Could not execute git push".to_string())
}

pub fn push (
  origin_name: &str,
  options: CommandOptions
) -> Result<(), String> {
  let mut command = Command::new("git");

  command.args(&[
    "push",
    origin_name
  ]);

  if let Some(cwd) = options.cwd.as_ref() {
    command.current_dir(cwd);
  }

  command.output().map(|_| ()).map_err(|_| "Could not execute git push".to_string())
}
