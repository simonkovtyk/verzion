use std::process::Command;

use crate::semver::SemVer;

pub fn push_tag (cwd: &Option<String>, semver: &SemVer) {
  let mut command = Command::new("git");

  command.args(&[
    "push",
    "origin",
    &semver.to_string()
  ]);

  if let Some(cwd) = cwd {
    command.current_dir(cwd);
  }

  command.output().expect("Could not execute git push");
}
