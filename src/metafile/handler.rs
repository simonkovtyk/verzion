use std::path::Path;

use crate::{config::Config, git::{tracking::GitTrackingBatch}, metafile::{config::MetafileTypes, git::get_commit_msg, java, node, plain}, semver::core::SemVer};

pub struct HandleMetafilesResult {
  pub tracking_batch: Option<GitTrackingBatch>
}

pub fn handle_metafile (semver: &SemVer) -> Option<HandleMetafilesResult> {
  let config = Config::inject();

  let mut tracking_batch = Vec::new();

  if let Some(inner_metafiles) = config.metafiles.as_ref() {
    for metafile in inner_metafiles {
      let mut path = Path::new(&metafile.path).to_path_buf();

      if !path.is_absolute() && let Some(inner_cwd) = &config.cwd {
        let cwd_path = Path::new(&inner_cwd);

        path = cwd_path.join(&path);
      }

      let path_str = path.to_str().expect("Contains invalid UTF-8 in path");

      match metafile.r#type {
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

      let tracking_path = metafile.tracking
        .as_ref()
        .map(|v| v.track(path_str, &get_commit_msg()))
        .flatten();

      if let Some(inner_tracking_path) = tracking_path {
        tracking_batch.push(inner_tracking_path);
      }
    }
  }

  return Some(HandleMetafilesResult {
    tracking_batch: if tracking_batch.is_empty() {
      None
    } else {
      Some(tracking_batch)
    }
  });
}
