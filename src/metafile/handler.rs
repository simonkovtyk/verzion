use std::path::Path;

use crate::{config::Config, metafile::{config::MetafileTypes, java, node, plain}, semver::core::SemVer};

pub fn handle_metafile (semver: &SemVer) {
  let config = Config::inject();

  if config.metafiles.is_none() {
    return;
  }

  for target in config.metafiles.as_ref().unwrap().targets.unwrap().iter_mut() {
    let mut path = Path::new(&target.path).to_path_buf();

    if !path.is_absolute() && let Some(inner_cwd) = &config.cwd {
      let cwd_path = Path::new(&inner_cwd);

      path = cwd_path.join(&path);
    }

    let path_str = path.to_str().expect("Contains invalid UTF-8 in path");

    match target.r#type {
      MetafileTypes::Plain => {
        plain::write::write_semver(path_str, semver);
      },
      MetafileTypes::Java => {
        java::write::write_semver(path_str, semver);
      },
      MetafileTypes::Node => {
        node::write::write_semver(path_str, semver);
      }
    }
  }
}
