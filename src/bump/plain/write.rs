use crate::semver::SemVer;
use std::{fs::{OpenOptions}, io::{BufWriter, Write}};

pub fn write_semver (path_to_metafile: &str, semver: &SemVer) -> () {
  let file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path_to_metafile)
    .expect("Could not open file");

  let mut writer = BufWriter::new(file);

  writer.write_all(&semver.as_bytes()).expect("Could not write to file");
  writer.flush().expect("Could not flush file");
}
