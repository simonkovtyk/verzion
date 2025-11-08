use crate::{conventions::conventional::types::{Message, Types}, semver::{SemVerType, compare_semver_type}};

pub fn get_semver_type (messages: Vec<Message>) -> SemVerType {
  let mut current_semver_type = None;

  for message in messages {
    let semver_type = match message.r#type {
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

    current_semver_type = Some(compare_semver_type(
      current_semver_type.unwrap(),
      semver_type.unwrap()
    ));
  }

  if current_semver_type.is_none() {
    panic!("No suitable bump trigger found");
  }

  return current_semver_type.unwrap();
}
