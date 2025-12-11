use std::process::Command;

use crate::{config::Config, git::{log::{GitLog, get_log}, rev_parse::get_rev_parse}, semver::core::SemVer};

#[derive(Debug, Clone)]
pub struct GitTag {
  pub annotation: String,
  pub semver: SemVer
}

pub fn get_tags (cwd: &Option<String>) -> Option<Vec<GitTag>> {
  let mut tag_command = Command::new("git");

  tag_command.args(&[
    "tag",
    "-l"
  ]);

  if let Some(cwd) = cwd {
    tag_command.current_dir(cwd);
  }

  let log_output = tag_command.output().expect("Failed to execute git log command");

  if log_output.stdout.is_empty() {
    return None;
  }

  let content = str::from_utf8(&log_output.stdout).expect("Content contained invalid UTF-8");
  let mut tags = Vec::new();

  let config = Config::inject();

  for line in content.lines() {
    let annotation = line.to_string();
    let semver = SemVer::try_from_format(
      line,
      &config.semver.as_ref()
        .map(|v| v.format.clone())
        .flatten()
    );

    /* Semver not parsable, skip this tag */
    if semver.is_err() {
      continue;
    }

    tags.push(GitTag {
      annotation,
      semver: semver.unwrap()
    });
  }

  Some(tags)
}

pub fn create_tag (semver: &SemVer) {
  let config = Config::inject();
  let mut tag_command = Command::new("git");

  let format_semver = semver.format(
    &config.semver.as_ref()
      .map(|v| v.format.clone())
      .flatten()
  );

  tag_command.args(&[
    "tag",
    "-a",
    &format_semver,
    "-m",
    &format_semver
  ]);
  
  let config = Config::inject();

  if let Some(cwd) = config.cwd.clone() {
    tag_command.current_dir(cwd);
  }

  tag_command.output().expect("Could not execute git tag command");
}

pub fn get_log_by_tag (tag: &GitTag) -> Option<GitLog> {
  let hash = get_rev_parse(&tag.annotation);

  if hash.is_none() {
    return None;
  }

  return get_log(&hash.unwrap());
}
