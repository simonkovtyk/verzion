use crate::{config::{Config, ToExitCode}, conventions::handler::resolve_semver_type, git::{log::{GitLog, get_logs}, push::push_tag, remote::{GitRemote, get_remote_names, get_remote_url}, tag::{GitTag, create_tag, get_log_by_tag, get_tags}}, semver::{core::SemVer, r#type::SemVerType, utils::{SemVerWithTag, find_latest_semver}}, std::{command::CommandOptions, panic::ExpectWithStatusCode}};

pub struct AnalyzeTagsResult {
  pub latest_tag: GitTag,
  pub latest_log: GitLog,
  pub latest_semver: SemVer
}

pub fn analyze_tags () -> Result<AnalyzeTagsResult, String> {
  let config = Config::inject();
  let tags = get_tags(CommandOptions {
    cwd: config.cwd.clone()
  })?;

  let mut semver_with_tags: Vec<SemVerWithTag> = Vec::new();

  for tag in tags {
    if let Ok(inner_semver) = SemVer::try_from_format(
      &tag.content,
      &config.semver.as_ref().map(|v| v.format.clone()).flatten()
    ) {
      semver_with_tags.push(
        SemVerWithTag {
          semver: inner_semver,
          tag: tag
        }
      );
    }
  }

  let latest_semver_with_tags = find_latest_semver(semver_with_tags)
    .expect_with_status_code(
      "Found no latest semver",
      config.to_exit_code()
    );
  let latest_log = get_log_by_tag(
    &latest_semver_with_tags.tag,
    CommandOptions {
    cwd: config.cwd.clone()
    }
  )?;

  Ok(AnalyzeTagsResult {
    latest_log: latest_log,
    latest_tag: latest_semver_with_tags.tag,
    latest_semver: latest_semver_with_tags.semver
  })
}

pub struct AnalyzeLogsResult {
  pub semver_type: SemVerType,
  pub logs: Vec<GitLog>
}

pub fn analyze_logs (from: Option<GitLog>) -> AnalyzeLogsResult {
  let config = Config::inject();
  let logs = get_logs(
    from.map(|v| v.hash),
    None,
    CommandOptions {
      cwd: config.cwd.clone()
    }
  ).expect_with_status_code("No logs found", config.to_exit_code());

  let semver_type = resolve_semver_type(&logs);

  AnalyzeLogsResult {
    semver_type,
    logs: logs
  }
}

pub struct PreparePublishResult {
  pub remotes: Vec<GitRemote>
}

pub fn publish (
  semver: &SemVer
) -> PreparePublishResult {
  let config = Config::inject();

  create_tag(&semver.to_string(), CommandOptions {
    cwd: config.cwd.clone()
  }).expect_with_status_code(
    "Could not create tag",
    config.to_exit_code()
  );

  let remote_names = get_remote_names(CommandOptions {
    cwd: config.cwd.clone()
  }).expect_with_status_code("No remote names found", config.to_exit_code());

  let mut remotes: Vec<GitRemote> = Vec::new();

  for remote_name in remote_names {
    let url = get_remote_url(
      Some(&remote_name),
      CommandOptions {
        cwd: config.cwd.clone()
      }
    ).expect_with_status_code("Remote url not found", config.to_exit_code());

    let remote = GitRemote {
      url: url.clone(),
      name: remote_name
    };

    push_tag(
      &remote.name,
      &semver.to_string(),
      CommandOptions {
        cwd: config.cwd.clone()
      }
    ).expect_with_status_code(
      "Could not push tag to remote",
      config.to_exit_code()
    );

    remotes.push(
      remote
    );
  }

  PreparePublishResult {
    remotes
  }
}
