use std::fmt::Debug;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use clap::{ValueEnum};

use crate::config::{Config};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd, Eq, Ord, ValueEnum)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum LogLevel {
  None = 0,
  Error = 1,
  Warn = 2,
  Info = 3,
  Success = 4,
  Debug = 5
}

#[allow(dead_code)]
const WARN_PREFIX: &str = "WARN";
#[allow(dead_code)]
const INFO_PREFIX: &str = "INFO";
#[allow(dead_code)]
const ERROR_PREFIX: &str = "ERROR";
#[allow(dead_code)]
const SUCCESS_PREFIX: &str = "SUCCESS";

#[allow(dead_code)]
pub fn log_info_raw <T: Debug> (value: T, log_level: &LogLevel) {
  let config = Config::inject();
  let mut prefix = create_prefix(INFO_PREFIX);

  if config.colored.unwrap_or(true) {
    prefix = prefix.blue().bold().to_string();
  }

  let config_log_level = config.log_level.clone().unwrap_or(LogLevel::Success);

  if log_level > &config_log_level {
    return;
  }

  println!(
    "{} {:#?}",
    prefix,
    value
  );
}

#[allow(dead_code)]
pub fn log_info (value: &str, log_level: &LogLevel) {
  let config = Config::inject();
  let mut prefix = create_prefix(INFO_PREFIX);

  if config.colored.unwrap_or(true) {
    prefix = prefix.blue().bold().to_string();
  }

  let config_log_level = config.log_level.clone().unwrap_or(LogLevel::Success);

  if log_level > &config_log_level {
    return;
  }

  println!(
    "{} {}",
    prefix,
    value
  );
}

#[allow(dead_code)]
pub fn log_error (value: &str, log_level: &LogLevel) {
  let config = Config::inject();
  let mut prefix = create_prefix(ERROR_PREFIX);

  if config.colored.unwrap_or(true) {
    prefix = prefix.red().bold().to_string();
  }

  let config_log_level = config.log_level.clone().unwrap_or(LogLevel::Success);

  if log_level > &config_log_level {
    return;
  }

  println!(
    "{} {}",
    prefix,
    value
  );
}

#[allow(dead_code)]
pub fn log_success (value: &str, log_level: &LogLevel) {
  let config = Config::inject();
  let mut prefix = create_prefix(SUCCESS_PREFIX);

  if config.colored.unwrap_or(true) {
    prefix = prefix.green().bold().to_string();
  }

  let config_log_level = config.log_level.clone().unwrap_or(LogLevel::Success);

  if log_level > &config_log_level {
    return;
  }

  println!(
    "{} {}",
    prefix,
    value
  );
}

#[allow(dead_code)]
pub fn log_warn (value: &str, log_level: &LogLevel) {
  let config = Config::inject();
  let mut prefix = create_prefix(WARN_PREFIX);

  if config.colored.unwrap_or(true) {
    prefix = prefix.yellow().bold().to_string();
  }

  let config_log_level = config.log_level.clone().unwrap_or(LogLevel::Success);

  if log_level > &config_log_level {
    return;
  }

  println!(
    "{} {}",
    prefix,
    value
  );
}

const LOGO: &str = r"
                          .__               
___  __ __________________|__| ____   ____  
\  \/ // __ \_  __ \___   /  |/  _ \ /    \ 
 \   /\  ___/|  | \//    /|  (  <_> )   |  \
  \_/  \___  >__|  /_____ \__|\____/|___|  /
           \/            \/              \/ 
";

pub fn create_prefix (value: &str) -> String {
  format!("[{}]", value)
}

pub fn print_header () {
  let config = Config::inject();
  let mut logo = LOGO.to_string();
  let mut version = env!("CARGO_PKG_VERSION").to_string();

  if config.colored.unwrap_or(true) {
    logo = logo.purple().bold().to_string();
    version = format!(" {} ", version).on_purple().white().bold().to_string();
  }

  println!(
    "{}\n{}\n",
    logo,
    version
  );
}
