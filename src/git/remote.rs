use std::process::Command;

use crate::config::Config;

pub struct GitRemote {
  pub url: String
}

pub fn get_remote_url () -> Option<GitRemote> {
  let mut command = Command::new("git");

  command.args(&[
    "remote",
    "get-url",
    "--push",
    "origin"
  ]);

  let config = Config::inject();

  if let Some(cwd) = config.cwd.clone() {
    command.current_dir(cwd);
  }

  let output = command.output().expect("Could not execute git show command");

  if output.stdout.is_empty() {
    return None;
  }

  let content = String::from_utf8(output.stdout).expect("Content contained invalid UTF-8");

  Some(GitRemote {
    url: content
  })
}
