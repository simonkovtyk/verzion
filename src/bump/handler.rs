use crate::{bump::{java, node, plain}, config::{BumpTypes, Config}, semver::{self, SemVer}};

pub fn handle_bump (config: &Config, semver: &SemVer) {
  if config.bump.targets.is_none() {
    return;
  }

  for target in config.bump.targets.as_ref().unwrap() {
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
