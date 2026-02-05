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

pub fn write_semver (path_to_metafile: &str, semver: &SemVer) -> Result<(), String> {
  let metafile_str = fs::read_to_string(path_to_metafile).map_err(|_| "Couldn't read metafile")?;
  let mut metafile = quick_xml::de::from_str::<Project>(&metafile_str).map_err(|_| "Couldn't parse metafile")?;

  metafile.version = Some(semver.to_string());

  fs::write(
    path_to_metafile,
    quick_xml::se::to_string(&metafile).map_err(|_| "Couldn't serialize metafile")?
  ).map_err(|_| "Couldn't write metafile")?;

  Ok(())
}
