use crate::{args::{self, Args}, config::{CONFIG, Config}, std::merge::Merge};
use clap::Parser;

pub fn process_config () {
  let args = Args::parse();

  let mut config = Config::from_args(&args);

  config = <args::Args as Into<Config>>::into(args).merge(
    config
  );

  CONFIG.set(config.clone())
    .expect("Could not update config");
}
