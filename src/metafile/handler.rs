use std::path::Path;

use crate::{config::Config, git::result::GitRulesetResult, metafile::{config::MetafileTypes, java, node, plain}, semver::core::SemVer};

pub struct HandleMetafilesResult {
  pub git_ruleset: GitRulesetResult
}

impl Default for HandleMetafilesResult {
  fn default() -> Self {
    Self {
      git_ruleset: GitRulesetResult::default()
    }
  }
}

pub fn handle_metafile (semver: &SemVer) -> HandleMetafilesResult {
  let config = Config::inject();

  let mut result = HandleMetafilesResult::default();

  if let Some(inner_targets) = config.metafiles.as_ref() {
    for target in inner_targets {
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

      result.git_ruleset.needs_push = match target.git_ruleset.push {
        Some(true) => true,
        _ => result.git_ruleset.needs_push
      };
    }
  }

  return result;
}
