use std::process::Command;

use crate::{config::Config, semver::SemVer};

pub fn push_tag (semver: &SemVer) {
  let mut command = Command::new("git");

  command.args(&[
    "push",
    "origin",
    &semver.to_string()
  ]);

  let config = Config::inject();

  if let Some(cwd) = config.cwd.clone() {
    command.current_dir(cwd);
  }

  command.output().expect("Could not execute git push");
}
