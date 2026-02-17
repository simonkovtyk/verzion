use std::path::Path;

use crate::{config::Config, git::tracking::GitTrackingBatch, log::log_debug, metafile::{config::MetafileTypes, git::get_commit_msg, java, node, plain}, semver::core::SemVer};

pub struct HandleMetafilesResult {
  pub tracking_batch: GitTrackingBatch
}

pub fn handle_metafile (semver: &SemVer) -> Result<HandleMetafilesResult, String> {
  let config = Config::inject();

  let mut tracking_batch = Vec::new();

  if let Some(inner_metafiles) = config.metafiles.as_ref() {
    for metafile in inner_metafiles {
      let mut path = Path::new(&metafile.path).to_path_buf();

      if !path.is_absolute() && let Some(inner_cwd) = &config.cwd {
        let cwd_path = Path::new(&inner_cwd);

        path = cwd_path.join(&path);
      }

      let path_str = path.to_str().ok_or("Contains invalid UTF-8 in path");

      match path_str {
        Ok(inner_path_str) => {
          match metafile.r#type {
            MetafileTypes::Plain => {
              plain::write::write_semver(inner_path_str, semver)?;
            },
            MetafileTypes::Java => {
              java::write::write_semver(inner_path_str, semver)?;
            },
            MetafileTypes::Node => {
              node::write::write_semver(inner_path_str, semver)?;
            }
          }

          log_debug(
            &format!(
              "Updated metafile of type {:?} at path: {}",
              metafile.r#type,
              inner_path_str
            )
          );

          let tracking_path = metafile.tracking
            .as_ref()
            .map(|v| v.track(inner_path_str, &get_commit_msg()))
            .flatten();

          if let Some(inner_tracking_path) = tracking_path {
            tracking_batch.push(inner_tracking_path);
          }
        },
        Err(_) => {
          return Err("Contains invalid UTF-8 in path".to_string());
        }
      }
    }
  }

  return Ok(HandleMetafilesResult {
    tracking_batch: tracking_batch
  });
}
