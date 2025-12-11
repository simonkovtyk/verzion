use crate::git::log::GitLog;

/*
 * A message is constructed by:
 * Header (1st line)
 * Body (proceeding lines)
 */
#[derive(Debug, Clone)]
pub struct Message {
  pub header: Header,
  pub body: Body,
  pub log: GitLog
}

#[derive(Debug, Clone)]
pub struct Header {
  pub r#type: Types,
  pub scope: Option<String>,
  pub content: String,
  pub breaking_change: BreakingChange
}

#[derive(Debug, Clone)]
pub struct Body {
  pub breaking_change: BreakingChange
}

#[derive(Debug, Clone)]
pub struct BreakingChange {
  pub detected: bool,
  #[allow(dead_code)]
  pub message: Option<String>
}

pub const BODY_BREAKING_CHANGE_SEPARATOR: char = ':';

pub const BODY_BREAKING_CHANGE_INDICATORS: &[&str; 2] = &[
  "BREAKING CHANGE",
  "BREAKING CHANGES"
];

#[derive(Debug, Clone)]
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

impl TryFrom<&str> for Types {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "feat" => Ok(Self::Feat),
      "fix" => Ok(Self::Fix),
      "chore" => Ok(Self::Chore),
      "docs" => Ok(Self::Docs),
      "style" => Ok(Self::Style),
      "refactor" => Ok(Self::Refactor),
      "perf" => Ok(Self::Perf),
      "test" => Ok(Self::Test),
      "build" => Ok(Self::Build),
      "ci" => Ok(Self::Ci),
      "revert" => Ok(Self::Revert),
      _ => Err("Unknown type")
    }
  }
}
