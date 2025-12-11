use std::process::Command;

use crate::{config::Config, git::config::GitOriginType};

pub struct GitRemote {
  pub name: String,
  pub url: String
}

pub fn get_remote_url (name: Option<&str>) -> Option<String> {
  let config = Config::inject();

  let mut command = Command::new("git");

  let origin_type = config.git.clone().map(|v| v.origin_type).flatten().unwrap_or(GitOriginType::Push);
  let origin = name.unwrap_or("origin");

  command.args(&[
    "remote",
    "get-url",
    &format!("--{}", origin_type.to_string()),
    origin
  ]);


  if let Some(cwd) = config.cwd.clone() {
    command.current_dir(cwd);
  }

  let output = command.output().expect("Could not execute git show command");

  if output.stdout.is_empty() {
    return None;
  }

  let content = String::from_utf8(output.stdout).expect("Content contained invalid UTF-8");

  Some(content)
}

pub fn get_remote_names () -> Option<Vec<String>> {
  let mut command = Command::new("git");

  command.args(&[
    "remote",
    "show"
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
  let names: Vec<String> = content
    .lines()
    .map(|line| line.trim().to_string())
    .collect();

  Some(names)
}

pub fn set_remote (remote: &GitRemote) {
  let mut command = Command::new("git");

  command.args(&[
    "remote",
    "set",
    &remote.name,
    &remote.url
  ]);

  let config = Config::inject();

  if let Some(cwd) = config.cwd.clone() {
    command.current_dir(cwd);
  }

  command.output().expect("Could not execute git show command");
}
