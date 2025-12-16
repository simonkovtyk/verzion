use crate::{changelog::generation::contract::{ChangelogData, ChangelogMessage}, conventions::conventional::types::{Message, Types}, std::option::ToOption};

pub fn get_changelog_data (messages: &Vec<Message>) -> ChangelogData {
  let mut features = Vec::new();
  let mut fixes = Vec::new();
  let mut breaking_changes = Vec::new();

  for message in messages.into_iter() {
    if message.header.breaking_change.detected || message.body.breaking_change.detected {
      breaking_changes.push(
        ChangelogMessage {
          content: Some(message.header.content.clone()),
          log: message.log.clone()
        }
      );

      continue;
    }

    match message.header.r#type {
      Types::Feat => {
        features.push(
          ChangelogMessage {
            content: Some(message.header.content.clone()),
            log: message.log.clone()
          }
        );
      },
      Types::Fix => {
        fixes.push(
          ChangelogMessage {
            content: Some(message.header.content.clone()),
            log: message.log.clone()
          }
        );
      },
      _ => {}
    }
  }

  ChangelogData {
    features: features.to_option(),
    fixes: fixes.to_option(),
    breaking_changes: breaking_changes.to_option(),
    contibutors: None
  }
}
