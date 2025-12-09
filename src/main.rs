use ::std::process;

use clap::Parser;

use crate::{commands::Args, config::{CONFIG, Config, LogLevel, ToExitCode}, conventions::handler::{generate_changelog, resolve_semver_type}, fs::write_str_to_file, git::{log::get_logs, push::push_tag, tag::{create_tag, get_log_by_tag, get_tags}, util::find_latest_semver_in_tags}, log::{log_error, log_info, log_success, print_header}, metafile::handler::handle_metafile, semver::SemVer, std::{Capitalize, Merge}, webhooks::handler::handle_webhooks};

mod git;
mod config;
mod conventions;
mod semver;
mod std;
mod metafile;
mod commands;
mod markdown;
mod fs;
mod webhooks;
mod http;
mod log;

#[tokio::main]
async fn main() {
  let args = Args::parse();

  let mut config = Config::from_args(&args);

  config = <&commands::Args as Into<Config>>::into(&args).as_ref().merge(
    &config
  );
  CONFIG.set(config.clone()).expect("Could not update config");
  drop(args);

  print_header();

  log_info("Analyzing tags", &LogLevel::Debug);
  
  let config = Config::inject();
  let config_semver = config.semver.clone().map(|v| v.to_semver_with_format());
  let mut semver = None::<SemVer>;
  let mut from = None::<String>;
  let tags = get_tags(&config.cwd);

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
          ),
          &LogLevel::Info
        );

        from = Some(inner_log.hash);

        if let Some(inner_semver) = &semver && !inner_semver.is_fullfilled() {
          semver = Some(inner_latest.semver);
        }
      }
    }
  }

  let logs = get_logs(&config.cwd, from, None);

  if logs.is_none() {
    log_error("No relevant commit found, aborting", &LogLevel::Error);
    process::exit(config.to_exit_code());
  }

  let logs = logs.unwrap();
  let semver_type = resolve_semver_type(&logs);

  log_info(
    &format!(
      "Upgrade: {}",
      &semver_type.to_string().to_lowercase().capitalize()
    ),
    &LogLevel::Info
  );

  let mut current_semver = semver.unwrap_or(SemVer::default());

  if let Some(inner_config_semver) = config_semver {
    current_semver = inner_config_semver.merge(&current_semver);
  }

  let resulting_semver = current_semver.bump(&semver_type);

  log_info(
    &format!(
      "Version: {}",
      resulting_semver.as_ref().to_string()
    ),
    &LogLevel::Info
  );

  create_tag(&resulting_semver);
  log_info("Tag created", &LogLevel::Info);
  push_tag(&resulting_semver);
  log_info("Tag pushed", &LogLevel::Info);

  handle_metafile(&resulting_semver);

  let mut changelog = None;

  if (&config).changelog.as_ref().map(|v| v.enabled).flatten().unwrap_or(false) {
    log_info("Generating changelog", &LogLevel::Info);
    changelog = Some(generate_changelog(&logs));
    log_info("Changelog generated", &LogLevel::Info);

    log_info("Writing changelog to file", &LogLevel::Info);

    if let Some(changelog_path) = (&config).changelog.as_ref().map(|v| v.path.clone()).flatten() {
      write_str_to_file(&changelog_path, &changelog.as_ref().unwrap());
    }

    log_info("Changelog written to file", &LogLevel::Info);
  }

  log_info("Handle webhooks", &LogLevel::Info);
  handle_webhooks(&config, &resulting_semver, &changelog).await;
  log_info("Webhooks handled", &LogLevel::Info);
  log_success("Completed successfully", &LogLevel::Success);
}
