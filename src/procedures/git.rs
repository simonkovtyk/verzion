use crate::{config::{Config, ToExitCode}, conventions::handler::resolve_semver_type, git::{log::{GitLog, get_logs}, push::push_tag, remote::{GitRemote, get_remote_names, get_remote_url}, tag::{GitTag, create_tag, get_log_by_tag, get_tags}, util::find_latest_semver_in_tags}, semver::{core::SemVer, r#type::SemVerType}, std::panic::ExpectWithStatusCode};

pub struct AnalyzeTagsResult {
  pub latest_tag: GitTag,
  pub latest_log: GitLog
}

pub fn analyze_tags () -> Option<AnalyzeTagsResult> {
  let config = Config::inject();
  let tags = get_tags(&config.cwd)?;
  let latest_tag = find_latest_semver_in_tags(&tags)?;
  let latest_log = get_log_by_tag(&latest_tag)?;

  Some(AnalyzeTagsResult {
    latest_tag: latest_tag,
    latest_log: latest_log
  })
}

pub struct AnalyzeLogsResult {
  pub semver_type: SemVerType,
  pub logs: Vec<GitLog>
}

pub fn analyze_logs (from: Option<GitLog>) -> AnalyzeLogsResult {
  let config = Config::inject();
  let logs = get_logs(
    &config.cwd,
    from.map(|v| v.hash),
    None
  ).expect_with_status_code("No logs found", config.to_exit_code());

  let semver_type = resolve_semver_type(&logs);

  AnalyzeLogsResult {
    semver_type,
    logs: logs
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

  let config = Config::inject();

  let remote_names = get_remote_names()
    .expect_with_status_code("No remote names found", config.to_exit_code());

  let mut remotes: Vec<GitRemote> = Vec::new();

  for remote_name in remote_names {
    let url = get_remote_url(Some(&remote_name))
      .expect_with_status_code("Remote url not found", config.to_exit_code());

    remotes.push(
      GitRemote {
        name: remote_name,
        url: url
      }
    );
  }

  PreparePublishResult {
    remotes
  }
}
