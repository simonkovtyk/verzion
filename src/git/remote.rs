use std::{result::Result, process::Command};

use crate::std::command::CommandOptions;

#[derive(Debug)]
pub struct GitRemote {
  pub name: String,
  #[allow(dead_code)]
  pub url: String
}

pub fn get_remote_url (
  name: Option<&str>,
  options: CommandOptions
) -> Result<String, String> {
  let mut command = Command::new("git");

  let origin = name.unwrap_or("origin");

  command.args(&[
    "remote",
    "get-url",
    "--push",
    origin
  ]);


  if let Some(cwd) = options.cwd.as_ref() {
    command.current_dir(cwd);
  }

  let output = command.output().map_err(|_| "Could not execute git show command".to_string())?;

  if output.stdout.is_empty() {
    return Err("No remote URL found".to_string());
  }

  String::from_utf8(output.stdout).map_err(|_| "Content contained invalid UTF-8".to_string())
}

pub fn get_remote_names (options: CommandOptions) -> Result<Vec<String>, String> {
  let mut command = Command::new("git");

  command.args(&[
    "remote",
    "show"
  ]);

  if let Some(cwd) = options.cwd.as_ref() {
    command.current_dir(cwd);
  }

  let output = command.output().map_err(|_| "Could not execute git show command".to_string())?;

  if output.stdout.is_empty() {
    return Err("No remote names found".to_string());
  }

  let content = String::from_utf8(output.stdout).map_err(|_| "Content contained invalid UTF-8".to_string())?;
  let names: Vec<String> = content
    .lines()
    .map(|line| line.trim().to_string())
    .collect();

  Ok(names)
}

#[allow(dead_code)]
pub fn set_remote (remote: &GitRemote, options: CommandOptions) -> Result<(), String> {
  let mut command = Command::new("git");

  command.args(&[
    "remote",
    "set",
    &remote.name,
    &remote.url
  ]);

  if let Some(cwd) = options.cwd.as_ref() {
    command.current_dir(cwd);
  }

  command.output().map(|_| ()).map_err(|_| "Could not execute git show command".to_string())
}
