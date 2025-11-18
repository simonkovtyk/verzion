use ::std::process;

use clap::Parser;
use merge::Merge;

use crate::{commands::Args, config::{Config, get_config, merge_config}, conventions::handler::{generate_changelog, resolve_semver_type}, fs::write_plain_file, git::{log::get_logs, push::push_tag, tag::{create_tag, get_log_by_tag, get_tags}, util::find_latest_semver_in_tags}, log::{log_error, log_info, log_success, print_header}, semver::{SemVer, bump_semver}, webhooks::handler::handle_webhooks};

mod git;
mod config;
mod conventions;
mod semver;
mod std;
mod bump;
mod commands;
mod markdown;
mod fs;
mod webhooks;
mod math;
mod http;
mod log;
mod util;

#[tokio::main]
async fn main() {
  let args = Args::parse();
  let mut config = get_config(&args.config);

  merge_config(
    &config,
    <&commands::Args as Into<Config>>::into(&args).as_ref()
  );

  drop(args);

  print_header(&config.colored);

  log_info("Analyzing tags", &config.colored);

  let tags = get_tags(&config.cwd);

  let mut from = None::<String>;
  let mut semver = None::<SemVer>;

  /* If we have tags, get log from latest tag to HEAD */
  if let Some(inner_tags) = tags {
    let latest = find_latest_semver_in_tags(&inner_tags);

    if let Some(inner_latest) = latest {
      let log = get_log_by_tag(&config.cwd, &inner_latest);

      if let Some(inner_log) = log {
        log_info(
          &format!(
            "Found latest tag with SemVer {} at {}",
            inner_latest.semver.as_ref().to_string(),
            inner_log.abbr_hash
          ),
          &config.colored
        );

        from = Some(inner_log.hash);
        semver = Some(inner_latest.semver);
      }
    }
  }

  let logs = get_logs(&config.cwd, from, None);

  if logs.is_none() {
    log_error("No relevant commit found, aborting", &config.colored);
    process::exit(
      config.graceful.map(|v| if v {
        0
      } else {
        1
      }).unwrap_or(1)
    );
  }

  let logs = logs.unwrap();
  let semver_type = resolve_semver_type(&config, &logs);

  log_info(
    &format!(
      "Next version will result in a {} upgrade",
      &semver_type.to_string().to_lowercase()
    ),
    &config.colored
  );

  let current_semver = semver.unwrap_or(SemVer {
    major: Some(0),
    minor: Some(0),
    patch: Some(0)
  });

  let resulting_semver = bump_semver(current_semver, semver_type);

  log_info(
    &format!(
      "Next version is {}",
      resulting_semver.as_ref().to_string()
    ),
    &config.colored
  );

  create_tag(&config.cwd, &resulting_semver);
  log_info("Tag created", &config.colored);
  push_tag(&config.cwd, &resulting_semver);
  log_info("Tag pushed", &config.colored);

  let mut changelog = None;

  if config.changelog.as_ref().map(|v| v.enabled).flatten().unwrap_or(true) {
    log_info("Generating changelog", &config.colored);
    changelog = Some(generate_changelog(&config, &logs));
    log_info("Changelog generated", &config.colored);

    log_info("Writing changelog to file", &config.colored);
    if let Some(changelog_path) = config.changelog.as_ref().map(|v| v.path.clone()).flatten() {
      write_plain_file(&changelog_path, &changelog.as_ref().unwrap());
    }
    log_info("Changelog written to file", &config.colored);
  }

  log_info("Handle webhooks", &config.colored);
  handle_webhooks(&config, &resulting_semver, &changelog).await;
  log_info("Webhooks handled", &config.colored);
  log_success("Completed process", &config.colored);
}
