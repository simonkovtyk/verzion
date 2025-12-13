use std::{process};

use crate::{config::Config, conventions::handler::resolve_semver_type, git::{log::{GitLog, get_logs}, push::push_tag, remote::{GitRemote, get_remote_names, get_remote_url}, tag::{GitTag, create_tag, get_log_by_tag, get_tags}, util::find_latest_semver_in_tags}, semver::{core::SemVer, r#type::SemVerType}, std::ToExitCode};

pub struct AnalyzeTagsResult {
  pub latest_tag: GitTag,
  pub latest_log: GitLog
}

pub fn analyze_tags () -> Option<AnalyzeTagsResult> {
  let config = Config::inject();
  let tags = get_tags(&config.cwd);

  if tags.is_none() {
    return None;
  }

  let latest_tag = find_latest_semver_in_tags(&tags.unwrap());

  if latest_tag.is_none() {
    return None;
  }

  let latest_log = get_log_by_tag(&latest_tag.as_ref().unwrap());

  if latest_log.is_none() {
    return None;
  }

  Some(AnalyzeTagsResult {
    latest_tag: latest_tag.unwrap(),
    latest_log: latest_log.unwrap()
  })
}

pub struct AnalyzeLogsResult {
  pub semver_type: SemVerType
}

pub fn analyze_logs (from: Option<GitLog>) -> AnalyzeLogsResult {
  let config = Config::inject();
  let logs = get_logs(&config.cwd, from.map(|v| v.hash), None);

  if logs.is_none() {
    process::exit(config.to_exit_code());
  }

  let semver_type = resolve_semver_type(&logs.unwrap());

  AnalyzeLogsResult {
    semver_type
  }
}

struct PreparePublishResult {
  pub remotes: Vec<GitRemote>
}

pub fn prepare_publish (
  semver: &SemVer
) -> PreparePublishResult {
  create_tag(semver);
  push_tag(semver);

  let remote_names = get_remote_names();

  let config = Config::inject();

  if remote_names.is_none() {
    process::exit(config.to_exit_code());
  }

  let mut remotes: Vec<GitRemote> = Vec::new();

  for remote_name in remote_names.unwrap().iter_mut() {
    let url = get_remote_url(Some(remote_name));

    if url.is_none() {
      continue;
    }

    remotes.push(
      GitRemote {
        name: remote_name.clone(),
        url: url.unwrap()
      }
    );
  }

  PreparePublishResult {
    remotes
  }
}
