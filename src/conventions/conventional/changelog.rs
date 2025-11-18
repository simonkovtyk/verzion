use crate::{conventions::conventional::types::{Message, Types}, markdown::{builder::MarkdownBuilder, elements::{h3::H3, linebreak::{Linebreak, LinebreakStyle}, list::{List, ListItem}}}};

pub fn get_changelog_message_section (title: &str, messages: &Vec<Message>) -> MarkdownBuilder {
  let mut builder = MarkdownBuilder::new();
  let heading = H3::new(title);

  builder.add(heading);

  builder.add(
    Linebreak::new(Some(LinebreakStyle::Newline))
  );

  let mut list = List::new(Some(false));

  for message in messages {
    let list_item = ListItem::new(
      if let Some(scope) = message.header.scope.clone() {
        format!("{}: {} ({}) by ",
          scope,
          message.header.content,
          /* TODO: Add commit link */
          message.log.abbr_hash
        )
      } else {
        message.header.content.clone()
      }
    );

    list.add(list_item);
  }

  builder.add(list);

  builder
}

pub struct FilteredMessages {
  pub feat: Vec<Message>,
  pub fix: Vec<Message>,
  pub breaking_changes: Vec<Message>
}

pub fn filter_messages (messages: &Vec<Message>) -> FilteredMessages {
  let mut feat = Vec::new();
  let mut fix = Vec::new();
  let mut breaking_changes = Vec::new();


  for message in messages.into_iter() {
    if message.header.breaking_change.detected || message.body.breaking_change.detected {
      breaking_changes.push(message.clone());

      continue;
    }

    match message.header.r#type {
      Types::Feat => {
        feat.push(message.clone());
      },
      Types::Fix => {
        fix.push(message.clone());
      },
      _ => {}
    }
  }

  FilteredMessages {
    feat,
    fix,
    breaking_changes
  }
}

pub fn get_changelog (messages: &Vec<Message>) -> String {
  let mut builder = MarkdownBuilder::new();
  
  let filtered_messages = filter_messages(messages);

  if !filtered_messages.feat.is_empty() {
    let features = get_changelog_message_section("Features", &filtered_messages.feat);
    builder.add(features);
  }

  if !filtered_messages.fix.is_empty() {
    let fixes = get_changelog_message_section("Fixes", &filtered_messages.fix);
    builder.add(
      Linebreak::new(Some(LinebreakStyle::Newline))
    );
    builder.add(
      Linebreak::new(Some(LinebreakStyle::Newline))
    );
    builder.add(fixes);
  }

  if !filtered_messages.breaking_changes.is_empty() {
    let breaking_changes = get_changelog_message_section("Breaking Changes", &filtered_messages.breaking_changes);
    builder.add(
      Linebreak::new(Some(LinebreakStyle::Newline))
    );
    builder.add(
      Linebreak::new(Some(LinebreakStyle::Newline))
    );
    builder.add(breaking_changes);
  }

  builder.into()
}
