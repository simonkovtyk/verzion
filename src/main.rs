use crate::{config::Config, log::{log_debug, log_info, print_header}, metafile::handler::handle_metafile, procedures::{changelog::create_changelog, config::process_config, git::{analyze_logs, analyze_tags, handle_tracking_batch, publish_tag}, semver::get_semver, webhooks::call_webhooks}};

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
mod package;

#[tokio::main]
async fn main() {
  process_config();
  print_header();

  log_debug(
    &format!(
      "Parsed config: {:?}",
      Config::inject()
    )
  );

  // Analyze
  let analyze_tags_result = analyze_tags();
  let analyze_logs_result = analyze_logs(analyze_tags_result.as_ref().map(|v| v.latest_log.clone()));
  let get_semver_result = get_semver(&analyze_logs_result.semver_type, analyze_tags_result.as_ref().map(|value| value.latest_semver.clone()));
  let create_changelog_result = create_changelog(&analyze_logs_result.logs);
  let handle_metafile_result = handle_metafile(&get_semver_result.semver);

  // Git
  handle_tracking_batch(
    &get_semver_result.semver,
    &create_changelog_result,
    &handle_metafile_result.ok()
  );
  publish_tag(&get_semver_result.semver);

  // Remote
  call_webhooks(
    &get_semver_result,
    &create_changelog_result
  ).await;

  log_info(
    "Successfully terminated"
  );
}
