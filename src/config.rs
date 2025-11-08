use std::{env, fs};
use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_NAME: &str = "nexlog.json";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BumpTypes {
  Java,
  Node,
  Plain
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BumpConfig {
  path: String,
  r#type: BumpTypes
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  /* Accept multiple paths for e.g. monorepos */
  paths: Vec<String>,
  bump: BumpConfig 
}

pub fn get_config () -> Config {
  let cwd = env::current_dir().expect("Couldn't get current directory");

  let config_path = cwd.join(CONFIG_FILE_NAME);
  let content_buf = fs::read(config_path).expect("Couldn't read config file");

  return serde_json::from_slice::<Config>(&content_buf).expect("Couldn't parse config file");
}
