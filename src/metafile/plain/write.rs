use crate::semver::core::SemVer;
use std::{fs::{OpenOptions}, io::{BufWriter, Write}};

pub fn write_semver (path_to_metafile: &str, semver: &SemVer) -> Result<(), String> {
  let file = OpenOptions::new()
    .write(true)
    .create(true)
    .truncate(true)
    .open(path_to_metafile)
    .map_err(|_| "Could not open file")?;

  let mut writer = BufWriter::new(file);

  writer.write_all(&semver.as_bytes()).map_err(|_| "Could not write to file")?;
  writer.flush().map_err(|_| "Could not flush file")?;

  Ok(())
}
