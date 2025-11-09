use clap::Parser;

use crate::{commands::Args, config::get_config, conventions::handler::resolve_semver_type, git::get_log_messages};

mod git;
mod config;
mod conventions;
mod semver;
mod std;
mod bump;
mod commands;

fn main() {
  let args = Args::parse();

  let config = get_config(args.config);
  //println!("{:?}", config);
  let logs = get_log_messages("master", "develop", args.cwd);
  //println!("{:?}", logs);

  let semver_type = resolve_semver_type(&config, &logs);

  println!("{:?}", semver_type);
}
