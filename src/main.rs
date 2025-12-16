use ::std::process;


use crate::{config::Config, conventions::handler::resolve_semver_type, fs::write_str_to_file, git::{push::push_tag, tag::create_tag}, log::{LogLevel, log_error, log_info, log_success, print_header}, metafile::handler::handle_metafile, procedures::{changelog::create_changelog, config::process_config, git::{analyze_logs, analyze_tags}, semver::get_semver}, semver::core::SemVer};

mod git;
mod config;
mod conventions;
mod semver;
mod std;
mod metafile;
mod args;
mod markdown;
mod fs;
mod webhooks;
mod http;
mod log;
mod changelog;
mod procedures;

#[tokio::main]
async fn main() {
  print_header();
  process_config();
  let analyze_tags_result = analyze_tags();
  let analyze_logs_result = analyze_logs(analyze_tags_result.map(|v| v.latest_log));
  let get_semver_result = get_semver(&analyze_logs_result.semver_type, analyze_tags_result.map(|v| v.latest_tag));

  handle_metafile(&get_semver_result.semver);
  
  let create_changelog_result = create_changelog(&analyze_logs_result.logs);

  let config = Config::inject();
  let config_semver = config.semver.clone().map(|v| v.to_semver_with_format());


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
  handle_webhooks(&resulting_semver, &changelog).await;
  log_info("Webhooks handled", &LogLevel::Info);
  log_success("Completed successfully", &LogLevel::Success);
}
