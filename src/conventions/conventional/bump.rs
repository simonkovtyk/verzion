use std::process;

use crate::{config::{Config, LogLevel, ToExitCode}, conventions::conventional::types::{Message, Types}, log::log_error, semver::SemVerType};

pub fn get_semver_type (messages: Vec<Message>) -> SemVerType {
  let mut current_semver_type = None;

  for message in messages {
    // We could know if a breaking change occurred. Since it is the highest semver type, we can skip any other message.
    if message.header.breaking_change.detected || message.body.breaking_change.detected {
      current_semver_type = Some(SemVerType::Major);
      break;
    }

    let semver_type = match message.header.r#type {
      Types::Fix => {
        Some(SemVerType::Patch)
      },
      Types::Feat => {
        Some(SemVerType::Minor)
      },
      _ => {
        None
      }
    };

    if current_semver_type.is_none() {
      current_semver_type = semver_type;
      
      continue;
    }

    if semver_type.is_none() {
      continue;
    }

    current_semver_type = Some(current_semver_type.unwrap().max_or(semver_type.unwrap()));
  }

  if current_semver_type.is_none() {
    let config = Config::inject();

    log_error("No suitable bump trigger found", &LogLevel::Error);
    process::exit(config.to_exit_code());
  }

  return current_semver_type.unwrap();
}
