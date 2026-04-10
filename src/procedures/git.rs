use crate::{config::{Config, ToExitCode}, conventions::handler::resolve_semver_type, git::{log::{GitLog, get_logs}, push::push_tag, remote::{GitRemote, get_remote_names, get_remote_url}, tag::{GitTag, create_tag, get_log_by_tag, get_tags}}, log::log_debug, metafile::handler::HandleMetafilesResult, procedures::changelog::CreateChangelogResult, semver::{core::SemVer, r#type::SemVerType, utils::{SemVerWithTag, find_latest_semver}}, std::{command::CommandOptions, panic::ExpectWithStatusCode}};

#[derive(Debug, Clone)]
pub struct AnalyzeTagsResult {
  #[allow(dead_code)]
  pub latest_tag: GitTag,
  pub latest_log: GitLog,
  pub latest_semver: SemVer
}

pub fn analyze_tags () -> Option<AnalyzeTagsResult> {
  let config = Config::inject();
  let tags = get_tags(CommandOptions {
    cwd: config.cwd.clone()
  }).ok()?;

  let mut semver_with_tags: Vec<SemVerWithTag> = Vec::new();

  for tag in tags {
    if let Ok(inner_semver) = SemVer::try_from_format(
      &tag.content,
      &config.semver.as_ref().map(|v| v.format.clone()).flatten()
    ) {
      log_debug(
        &format!(
          "Found SemVer: {:?}",
          &inner_semver
        )
      );
      semver_with_tags.push(
        SemVerWithTag {
          semver: inner_semver,
          tag: tag
        }
      );
    }
  }

  let latest_semver_with_tags = find_latest_semver(semver_with_tags)?;

  log_debug(
    &format!(
      "Latest SemVer with tag: {:?}",
      &latest_semver_with_tags
    )
  );

  let latest_log = get_log_by_tag(
    &latest_semver_with_tags.tag,
    CommandOptions {
      cwd: config.cwd.clone()
    }
  ).ok()?;

  log_debug(
    &format!(
      "Latest log by tag: {:?}",
      &latest_log
    )
  );

  Some(
    AnalyzeTagsResult {
      latest_log: latest_log,
      latest_tag: latest_semver_with_tags.tag,
      latest_semver: latest_semver_with_tags.semver
    }
  )
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
  ).expect_with_status_code(
    "No logs found",
    config.to_exit_code()
  );

  log_debug(
    &format!(
      "Analyzed logs: {:?}",
      &logs
    )
  );

  let semver_type = resolve_semver_type(&logs);

  log_debug(
    &format!(
      "Resolved SemVer type: {:?}",
      semver_type
    )
  );

  AnalyzeLogsResult {
    semver_type,
    logs: logs
  }
}

pub struct PreparePublishResult {
  #[allow(dead_code)]
  pub remotes: Vec<GitRemote>
}

pub fn publish_tag (
  semver: &SemVer
) -> PreparePublishResult {
  let config = Config::inject();

  log_debug(
    &format!(
      "Publishing tag for SemVer: {:?}",
      &semver.to_string()
    )
  );

  create_tag(
    &semver.format(
      &config.semver.as_ref().map(|v| v.format.clone()).flatten()
    ), CommandOptions {
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

    log_debug(
      &format!(
        "Pushing to remote: {:?}",
        &remote
      )
    );

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

pub fn handle_tracking_batch (
  semver: &SemVer,
  create_changelog_result: &Option<CreateChangelogResult>,
  handle_metafile_result: &Option<HandleMetafilesResult>
) {
  let config = Config::inject();
  let mut tracking_batch = Vec::<String>::new();

  if let Some(inner_tracking_batch) = create_changelog_result.as_ref()
    .map(|v| v.tracking_batch.clone()) {
    tracking_batch.extend(inner_tracking_batch);
  }

  if let Some(inner_tracking_batch) = handle_metafile_result.as_ref()
    .map(|v| v.tracking_batch.clone()) {
    tracking_batch.extend(inner_tracking_batch);
  }

  if let Some(inner_tracking) = config.tracking.as_ref() {
    log_debug(
      &format!(
        "Handling tracking batch: {:?}",
        &tracking_batch
      )
    );

    inner_tracking
      .track_batch(
        semver,
        tracking_batch
      )
      .expect_with_status_code(
        "Could not handle git tracking batch",
        config.to_exit_code()
      );
  }
}
