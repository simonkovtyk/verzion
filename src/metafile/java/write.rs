use serde::{Deserialize, Serialize};
use std::{fs};

use crate::semver::core::SemVer;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "project")]
pub struct Project {
  pub version: Option<String>,
  #[serde(flatten)]
  pub other: String
}

pub fn write_semver (path_to_metafile: &str, semver: &SemVer) -> () {
  let metafile_str = fs::read_to_string(path_to_metafile).expect("Couldn't read metafile");
  let mut metafile = quick_xml::de::from_str::<Project>(&metafile_str).expect("Couldn't parse metafile");

  metafile.version = Some(semver.to_string());

  fs::write(
    path_to_metafile,
    quick_xml::se::to_string(&metafile).expect("Couldn't serialize metafile")
  ).expect("Couldn't write metafile");
}
