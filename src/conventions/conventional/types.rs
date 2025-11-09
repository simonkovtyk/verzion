/*
 * A message is constructed by:
 * Header (1st line)
 * Body (proceeding lines)
 */
#[derive(Debug)]
pub struct Message {
  pub header: Header,
  pub body: Body
}

#[derive(Debug)]
pub struct Header {
  pub r#type: Types,
  pub scope: Option<String>,
  pub content: String,
  pub breaking_change: BreakingChange
}

#[derive(Debug)]
pub struct Body {
  pub breaking_change: BreakingChange
}

#[derive(Debug)]
pub struct BreakingChange {
  pub detected: bool,
  pub message: Option<String>
}

pub const BODY_BREAKING_CHANGE_SEPARATOR: char = ':';

pub const BODY_BREAKING_CHANGE_INDICATORS: &[&str; 2] = &[
  "BREAKING CHANGE",
  "BREAKING CHANGES"
];

#[derive(Debug)]
pub enum Types {
  Feat,
  Fix,
  Chore,
  Docs,
  Style,
  Refactor,
  Perf,
  Test,
  Build,
  Ci,
  Revert
}

impl From<&str> for Types {
  fn from(value: &str) -> Self {
    match value {
      "feat" => Self::Feat,
      "fix" => Self::Fix,
      "chore" => Self::Chore,
      "docs" => Self::Docs,
      "style" => Self::Style,
      "refactor" => Self::Refactor,
      "perf" => Self::Perf,
      "test" => Self::Test,
      "build" => Self::Build,
      "ci" => Self::Ci,
      "revert" => Self::Revert,
      _ => {
        panic!("Cannot parse message type: {}", value)
      }
    }
  }
}
