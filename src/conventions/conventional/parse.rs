use crate::{conventions::conventional::types::{Message, Types}, git::GitLog};

pub fn parse_message (message: &str) -> Message {
  let mut r#type = String::new();
  let mut scope = String::new();
  let mut content = String::new();
  let mut type_endend = false;
  let mut scope_detected = false;
  let mut scope_ended = false;

  for message_char in message.chars() {
    if !type_endend {
      /* No scope detected, continue with content */
      if message_char == ':' {
        type_endend = true;
        continue;
      }

      /* Scope detected so continue with it */
      if message_char == '(' {
        scope_detected = true;
        type_endend = true;
        continue;
      }

      r#type.push(message_char);
      continue;
    }

    if scope_detected && !scope_ended {
      if message_char == ')' {
        scope_ended = true;
        continue;
      }

      scope.push(message_char);
      continue;
    }

    content.push(message_char);
  }

  if r#type.is_empty() || content.is_empty() {
    panic!("Could not parse message: Missing type or content. Ensure the message follows the 'type(scope): content' format. Example: 'feat(parser): Add support for parsing messages' or 'fix()")
  }

  return Message {
    r#type: Types::from(r#type.as_str()),
    content: content.trim().to_string(),
    scope: Some(scope)
  };
}

pub fn parse_logs (logs: Vec<GitLog>) -> Vec<Message> {
  let mut messages = Vec::new();

  for log in logs {
    let message = log.message.lines().next();

    if message.is_none() {
      continue;
    }

    messages.push(
      parse_message(message.unwrap())
    );
  }

  return messages;
}
