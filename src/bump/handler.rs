use crate::{bump::{java, node, plain}, config::{BumpTypes, Config}, semver::{self, SemVer}};

pub fn handle_bump (config: &Config, semver: &SemVer) {
  if config.bump.is_none() {
    return;
  }

  let bump_config = config.bump.as_ref().unwrap();

  if let Some(enabled) = bump_config.enabled && !enabled {
    return;
  }

  if bump_config.targets.is_none() {
    return;
  }

  for target in bump_config.targets.as_ref().unwrap() {
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
