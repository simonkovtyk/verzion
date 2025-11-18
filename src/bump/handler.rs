use crate::{bump::{java, node, plain}, config::{BumpTypes, Config}, semver::SemVer};

pub fn handle_bump (config: &Config, semver: &SemVer) {
  if let Some(enabled) = config.enabled && !enabled {
    return;
  }

  if config.targets.is_none() {
    return;
  }

  for target in config.targets.as_ref().unwrap() {
    match target.r#type {
      BumpTypes::Plain => {
        plain::write::write_semver(&target.path, semver);
      },
      BumpTypes::Java => {
        java::write::write_semver(&target.path, semver);
      },
      BumpTypes::Node => {
        node::write::write_semver(&target.path, semver);
      }
    }
  }
}
