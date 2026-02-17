use std::process::Command;

use crate::{git::{log::{GitLog, get_log}, rev_parse::get_rev_parse}, std::command::CommandOptions};

#[derive(Debug, Clone)]
pub struct GitTag {
  pub content: String
}

pub fn get_tags (options: CommandOptions) -> Result<Vec<GitTag>, String> {
  let mut tag_command = Command::new("git");

  tag_command.args(&[
    "tag",
    "-l"
  ]);

  if let Some(cwd) = options.cwd {
    tag_command.current_dir(cwd);
  }

  let log_output = tag_command.output().map_err(|_| "Failed to execute git log command")?;

  if log_output.stdout.is_empty() {
    return Err("No tags found".to_string());
  }

  let content = str::from_utf8(&log_output.stdout).map_err(|_| "Content contained invalid UTF-8".to_string())?;
  let mut tags = Vec::new();

  for line in content.lines() {
    tags.push(GitTag {
      content: line.to_string()
    });
  }

  Ok(tags)
}

pub fn create_tag (value: &str, options: CommandOptions) -> Result<(), String> {
  let mut tag_command = Command::new("git");

  tag_command.args(&[
    "tag",
    "-a",
    value,
    "-m",
    value
  ]);
  
  if let Some(cwd) = options.cwd.as_ref() {
    tag_command.current_dir(cwd);
  }

  tag_command.output().map(|_| ()).map_err(|_| "Could not execute git tag command".to_string())
}

pub fn get_log_by_tag (tag: &GitTag, options: CommandOptions) -> Result<GitLog, String> {
  let hash = get_rev_parse(&tag.content, options.clone())?;

  get_log(&hash, options)
}
