use clap::Parser;

use crate::{commands::Args, config::get_config, conventions::handler::{generate_changelog, resolve_semver_type}, fs::write_plain_file, git::{log::get_logs, tag::{get_log_by_tag, get_tags}, util::find_latest_semver_in_tags}, log::{log_error, log_info, log_success, log_warn, print_header}, semver::{SemVer, bump_semver}, webhooks::handler::handle_webhooks};

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

#[tokio::main]
async fn main() {

  let args = Args::parse();
  let config = get_config(&args.config);

  print_header(&config.colored);

  log_warn("Test", &config.colored);
  log_info("Test", &config.colored);
  log_error("Test", &config.colored);
  log_success("Test", &config.colored);

  let tags = get_tags(&args.cwd);

  let mut from = None::<String>;
  let mut semver = None::<SemVer>;

  /* If we have tags, get log from latest tag to HEAD */
  if let Some(inner_tags) = tags {
    let latest = find_latest_semver_in_tags(&inner_tags);

    if let Some(inner_latest) = latest {
      let log = get_log_by_tag(&args.cwd, &inner_latest);

      if let Some(inner_log) = log {
        from = Some(inner_log.hash);
        semver = Some(inner_latest.semver);
      }
    }
  }

  let logs = get_logs(&args.cwd, from, None);

  if logs.is_none() {
    return;
  }
  
  let logs = logs.unwrap();
  let semver_type = resolve_semver_type(&config, &logs);
  let current_semver = semver.unwrap_or(SemVer {
    major: Some(0),
    minor: Some(0),
    patch: Some(0)
  });

  let resulting_semver = bump_semver(current_semver, semver_type);

  println!("{:?}", &resulting_semver);

  let changelog = generate_changelog(&config, &logs, &resulting_semver);

  handle_webhooks(&config, &resulting_semver, &args, &changelog).await;

  if let Some(changelog_path) = config.changelog.map(|v| v.path).flatten() {
    write_plain_file(&changelog_path, &changelog);
  }
}
