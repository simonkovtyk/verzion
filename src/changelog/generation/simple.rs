use crate::{changelog::generation::contract::{ChangelogData, ChangelogMessage}, markdown::{builder::MarkdownBuilder, elements::{h3::H3, linebreak::{Linebreak, LinebreakStyle}, list::{List, ListItem}}}};

fn generate_section (title: &str, messages: &Vec<ChangelogMessage>) -> MarkdownBuilder {
  let mut builder = MarkdownBuilder::new();
  let heading = H3::new(title);

  builder.add(heading);

  builder.add(
    Linebreak::new(Some(LinebreakStyle::Newline))
  );

  let mut list = List::new(Some(false));

  for message in messages {
    let list_item = ListItem::new(
      if let Some(message_content) = message.content.clone() {
        message_content
      } else {
        message.log.message.clone()
      }
    );

    list.add(list_item);
  }

  builder.add(list);

  builder
}

pub fn generate (data: ChangelogData) -> String {
  let mut builder = MarkdownBuilder::new();
  
  if let Some(data_features) = data.features && !data_features.is_empty() {
    let features = generate_section("Features", &data_features);
    builder.add(features);
  }

  if let Some(data_fixes) = data.fixes && !data_fixes.is_empty() {
    let fixes = generate_section("Fixes", &data_fixes);
    builder.add(
      Linebreak::new(Some(LinebreakStyle::Newline))
    );
    builder.add(
      Linebreak::new(Some(LinebreakStyle::Newline))
    );
    builder.add(fixes);
  }

  if let Some(data_breaking_changes) = data.breaking_changes && !data_breaking_changes.is_empty() {
    let breaking_changes = generate_section("Breaking Changes", &data_breaking_changes);
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
