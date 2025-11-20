use ::std::process;

use clap::Parser;

use crate::{commands::Args, config::{CONFIG, Config, ToExitCode, get_config}, conventions::handler::{generate_changelog, resolve_semver_type}, fs::write_plain_file, git::{log::get_logs, push::push_tag, tag::{create_tag, get_log_by_tag, get_tags}, util::find_latest_semver_in_tags}, log::{log_error, log_info, log_success, print_header}, semver::{SemVer}, std::Merge, webhooks::handler::handle_webhooks};

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

  config = <&commands::Args as Into<Config>>::into(&args).as_ref().merge(
    &config
  );

  CONFIG.set(config).expect("Could not update config");

  drop(args);

  print_header();

  log_info("Analyzing tags");
  
  let config = Config::inject();

  let tags = get_tags(&(&config).cwd);

  let mut from = None::<String>;
  let mut semver = None::<SemVer>;

  /* If we have tags, get log from latest tag to HEAD */
  if let Some(inner_tags) = tags {
    let latest = find_latest_semver_in_tags(&inner_tags);

    if let Some(inner_latest) = latest {
      let log = get_log_by_tag(&inner_latest);

      if let Some(inner_log) = log {
        log_info(
          &format!(
            "Found latest tag with SemVer {} at {}",
            inner_latest.semver.as_ref().to_string(),
            inner_log.abbr_hash
          )
        );

        from = Some(inner_log.hash);
        semver = Some(inner_latest.semver);
      }
    }
  }

  let logs = get_logs(&(&config).cwd, from, None);

  if logs.is_none() {
    log_error("No relevant commit found, aborting");
    process::exit(config.to_exit_code());
  }

  let logs = logs.unwrap();
  let semver_type = resolve_semver_type(&logs);

  log_info(
    &format!(
      "Next version will result in a {} upgrade",
      &semver_type.to_string().to_lowercase()
    )
  );

  let current_semver = semver.unwrap_or(SemVer::default());

  let resulting_semver = current_semver.bump(&semver_type);

  log_info(
    &format!(
      "Next version is {}",
      resulting_semver.as_ref().to_string()
    )
  );

  create_tag(&resulting_semver);
  log_info("Tag created");
  push_tag(&resulting_semver);
  log_info("Tag pushed");

  let mut changelog = None;

  if (&config).changelog.as_ref().map(|v| v.enabled).flatten().unwrap_or(true) {
    log_info("Generating changelog");
    changelog = Some(generate_changelog(&logs));
    log_info("Changelog generated");

    log_info("Writing changelog to file");
    if let Some(changelog_path) = (&config).changelog.as_ref().map(|v| v.path.clone()).flatten() {
      write_plain_file(&changelog_path, &changelog.as_ref().unwrap());
    }
    log_info("Changelog written to file");
  }

  log_info("Handle webhooks");
  handle_webhooks(&config, &resulting_semver, &changelog).await;
  log_info("Webhooks handled");
  log_success("Completed process");
}
