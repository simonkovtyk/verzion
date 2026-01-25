use crate::{log::print_header, metafile::handler::handle_metafile, procedures::{changelog::create_changelog, config::process_config, git::{analyze_logs, analyze_tags, publish}, semver::get_semver}, webhooks::handler::handle_webhook};

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

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

#[tokio::main]
async fn main() {
  process_config();
  print_header();

  let analyze_tags_result = analyze_tags().ok();
  let analyze_logs_result = analyze_logs(analyze_tags_result.as_ref().map(|v| v.latest_log.clone()));
  let get_semver_result = get_semver(&analyze_logs_result.semver_type, analyze_tags_result.as_ref().map(|v| v.latest_semver.clone()));
  let create_changelog_result = create_changelog(&analyze_logs_result.logs);

  handle_metafile(&get_semver_result.semver);
  publish(&get_semver_result.semver);

  handle_webhook(
    &get_semver_result.semver,
    &create_changelog_result.as_ref().map(|v| v.changelog.clone())
  ).await;
}
