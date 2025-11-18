use crate::{conventions::conventional::types::{BODY_BREAKING_CHANGE_INDICATORS, BODY_BREAKING_CHANGE_SEPARATOR, Body, BreakingChange, Header, Message, Types}, git::log::GitLog};

pub fn parse_header (raw_header: &str) -> Option<Header> {
  let mut r#type = String::new();
  let mut scope = String::new();
  let mut content = String::new();
  let mut type_endend = false;
  let mut scope_detected = false;
  let mut scope_ended = false;
  let mut scope_closed = false;
  let mut has_explicit_breaking_change = false;
  let mut skip_next_char = false;

  // I hate Regex, I do it char by char
  for raw_header_char in raw_header.chars() {
    if skip_next_char {
      skip_next_char = false;
      continue;
    }

    if !type_endend {
      match raw_header_char {
        ':' => {
          type_endend = true;
        },
        '(' => {
          scope_detected = true;
          type_endend = true;
        },
        '!' => {
          has_explicit_breaking_change = true;
        },
        _ => {
          r#type.push(raw_header_char);
        }
      }

      continue;
    }

    if scope_detected && !scope_ended {
      match raw_header_char {
        ':' => {
          scope_ended = true;
        },
        '!' => {
          if scope_closed {
            has_explicit_breaking_change = true;
          }
        }
        ')' => {
          scope_closed = true;
        },
        _ => {
          scope.push(raw_header_char);
        }
      }

      continue;
    }

    content.push(raw_header_char);
  }

  /* Message could not be parsed, so exit without */
  if r#type.is_empty() || content.is_empty() {
    return None;
  }

  return Some(Header {
    r#type: Types::from(r#type.as_str()),
    content: content.trim().to_string(),
    scope: if scope.is_empty() { None } else { Some(scope) },
    breaking_change: BreakingChange {
      detected: has_explicit_breaking_change,
      /* Breaking change message cannot exist in header of message */
      message: None
    }
  });
}

pub fn parse_body (raw_body: &str) -> Body {
  let mut detected = false;
  let mut message = None;

  // I still hate regex, so line by line
  for line in raw_body.lines() {
    for indicator in BODY_BREAKING_CHANGE_INDICATORS {
      let index = line.find(indicator);

      if index.is_none() {
        continue;
      }

      detected = true;

      let message_byte_offset =
        index.unwrap() as u32
        + indicator.len() as u32
        + BODY_BREAKING_CHANGE_SEPARATOR.len_utf8() as u32
        + ' '.len_utf8() as u32;

      message = Some(line[(message_byte_offset as usize)..].to_string());

      break;
    }

    if detected {
      break;
    }
  }

  return Body {
    breaking_change: BreakingChange {
      detected,
      message
    }
  }
}

pub fn parse_logs (logs: &Vec<GitLog>) -> Vec<Message> {
  let mut messages = Vec::new();

  for log in logs {
    let mut lines = log.message.lines();
    let header;

    if let Some(raw_header) = lines.next() && let Some(parsed_header) = parse_header(raw_header) {
      header = parsed_header;
    } else {
      // Messages without/invalid header doesnt exist, so skip it
      continue;
    }

    let body = parse_body(&lines.collect::<Vec<&str>>().join("\n"));

    messages.push(
      Message {
        header,
        body,
        log: log.clone()
      }
    );
  }

  return messages;
}
