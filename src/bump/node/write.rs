use std::{fs};
use serde::{Deserialize, Serialize};

use crate::semver::SemVer;

#[derive(Serialize, Deserialize)]
struct Metafile {
  version: String
}

pub fn write_semver (path_to_metafile: &str, semver: &SemVer) -> () {
  let metafile_buf = fs::read(path_to_metafile).expect("Couldn't read metafile");
  let mut metafile = serde_json::from_slice::<Metafile>(&metafile_buf).expect("Couldn't parse metafile");

  metafile.version = semver.try_into().expect("Couldn't convert SemVer to string");

  fs::write(
    path_to_metafile,
    serde_json::to_string(&metafile).expect("Couldn't serialize metafile")
  ).expect("Couldn't write metafile");
}
