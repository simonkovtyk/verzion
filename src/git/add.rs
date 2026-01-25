use std::process::Command;

use crate::std::command::CommandOptions;

pub fn add(
  path: &str,
  options: CommandOptions
) -> Result<(), String> {
  let mut command = Command::new("git");

  command.args(&[
    "add",
    path
  ]);

  if let Some(cwd) = options.cwd.as_ref() {
    command.current_dir(cwd);
  }

  command.output().map(|_| ()).map_err(|_| "Could not execute git add".to_string())
}
