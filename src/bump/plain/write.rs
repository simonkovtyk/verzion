use crate::semver::SemVer;
use std::{fs};

pub fn write_semver (path_to_metafile: &str, semver: &SemVer) -> () {
  let version: String = semver.try_into().expect("Failed to convert SemVer to string");

  fs::write(path_to_metafile, version).expect("Could not write to file");
}
