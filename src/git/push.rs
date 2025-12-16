use std::process::Command;

use crate::{semver::core::SemVer, std::command::CommandOptions};

pub fn push_tag (
  semver: &SemVer,
  options: CommandOptions
) -> Result<(), String> {
  let mut command = Command::new("git");

  command.args(&[
    "push",
    "origin",
    &semver.to_string()
  ]);

  if let Some(cwd) = options.cwd.as_ref() {
    command.current_dir(cwd);
  }

  command.output().map(|_| ()).map_err(|_| "Could not execute git push".to_string())
}
