use std::{fs};
use serde_json::Value;

use crate::{config::{Config, ToExitCode}, semver::core::SemVer, std::panic::ExpectWithStatusCode};

pub fn write_semver (path_to_metafile: &str, semver: &SemVer) -> Result<(), String> {
  let config = Config::inject();
  let metafile_buf = fs::read(path_to_metafile).map_err(|_| "Couldn't read metafile")?;
  let mut metafile = serde_json::from_slice::<Value>(&metafile_buf).map_err(|_| "Couldn't parse metafile")?;

  metafile["version"] = Value::from(semver.to_string());

  fs::write(
    path_to_metafile,
    serde_json::to_string(&metafile).map_err(|_| "Couldn't serialize metafile")?
  ).expect_with_status_code("Couldn't write metafile", config.to_exit_code());

  Ok(())
}
