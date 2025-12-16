use crate::{args::{self, Args}, config::{CONFIG, Config}, log::{LogLevel, log_info, log_info_raw}, std::merge::Merge};
use clap::Parser;

pub fn process_config () {
  let args = Args::parse();

  log_info("Parsed args", &LogLevel::Debug);
  log_info_raw(&args, &LogLevel::Debug);

  let mut config = Config::from_args(&args);

  log_info("Parsed config", &LogLevel::Debug);
  log_info_raw(&config, &LogLevel::Debug);

  config = <&args::Args as Into<Config>>::into(&args).merge(
    config
  );

  log_info("Merged args + config", &LogLevel::Debug);
  log_info_raw(&config, &LogLevel::Debug);

  CONFIG.set(config.clone()).expect("Could not update config");
}
