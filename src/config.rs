use std::{env, fs};
use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_NAME: &str = "nexlog.json";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BumpConvetion {
  Conventional
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BumpTypes {
  Java,
  Node,
  Plain
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BumpTarget {
  pub r#type: BumpTypes,
  pub path: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BumpConfig {
  pub enabled: Option<bool>,
  pub convention: Option<BumpConvetion>,
  pub targets: Option<Vec<BumpTarget>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
  /* Accept multiple paths for e.g. monorepos */
  pub paths: Option<Vec<String>>,
  pub bump: Option<BumpConfig>
}

pub fn get_config (path: Option<String>) -> Config {
  let mut resulting_path = env::current_dir().expect("Couldn't get current directory").join(CONFIG_FILE_NAME).to_str().unwrap().to_string();

  if let Some(path) = path {
    resulting_path = path;
  }

  println!("{}", resulting_path);
  let content_buf = fs::read(resulting_path).expect("Couldn't read config file");

  return serde_json::from_slice::<Config>(&content_buf).expect("Couldn't parse config file");
}
