use std::path::Path;

use crate::{bump::{java, node, plain}, config::{BumpTypes, Config}, semver::SemVer};

pub fn handle_bump (semver: &SemVer) {
  let config = Config::inject();

  if config.targets.is_none() {
    return;
  }

  for target in config.targets.as_ref().unwrap() {
    let mut path = Path::new(&target.path).to_path_buf();

    if !path.is_absolute() && let Some(inner_cwd) = &config.cwd {
      let cwd_path = Path::new(&inner_cwd);

      path = cwd_path.join(&path);
    }

    let path_str = path.to_str().expect("Contains invalid UTF-8 in path");

    println!("{}", &path_str);

    match target.r#type {
      BumpTypes::Plain => {
        plain::write::write_semver(path_str, semver);
      },
      BumpTypes::Java => {
        java::write::write_semver(path_str, semver);
      },
      BumpTypes::Node => {
        node::write::write_semver(path_str, semver);
      }
    }
  }
}
