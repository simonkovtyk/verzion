use serde::{Deserialize, Serialize};
use std::{fs};

use crate::semver::SemVer;

#[derive(Serialize, Deserialize)]
struct Metafile {
  project: MetafileProject
}

#[derive(Serialize, Deserialize)]
struct MetafileProject {
  version: String
}

pub fn write_semver (path_to_metafile: &str, semver: SemVer) -> () {
  let metafile_str = fs::read_to_string(path_to_metafile).expect("Couldn't read metafile");
  let mut metafile = serde_xml_rs::from_str::<Metafile>(&metafile_str).expect("Couldn't parse metafile");

  metafile.project.version = semver.try_into().expect("Couldn't convert SemVer to string");

  fs::write(
    path_to_metafile,
    serde_xml_rs::to_string(&metafile).expect("Couldn't serialize metafile")
  ).expect("Couldn't write metafile");
}
