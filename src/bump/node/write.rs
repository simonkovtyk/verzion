use std::{fs};
use serde_json::Value;

use crate::semver::SemVer;

pub fn write_semver (path_to_metafile: &str, semver: &SemVer) -> () {
  let metafile_buf = fs::read(path_to_metafile).expect("Couldn't read metafile");
  let mut metafile = serde_json::from_slice::<Value>(&metafile_buf).expect("Couldn't parse metafile");

  metafile["version"] = Value::from(semver.to_string());

  fs::write(
    path_to_metafile,
    serde_json::to_string(&metafile).expect("Couldn't serialize metafile")
  ).expect("Couldn't write metafile");
}
