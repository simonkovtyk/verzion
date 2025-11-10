pub struct Code {
  language: Option<String>,
  value: String
}

const TAG: &str = "```";

impl Code {
  pub fn new (value: String, language: Option<impl Into<String>>) -> Self {
    Code {
      language: language.map(|v| v.into()),
      value: value.into()
    }
  }
}

impl Into<String> for Code {
  fn into(self) -> String {
    format!(
      "{}{}\n{}\n{}",
      TAG,
      self.language.clone().unwrap_or("".to_string()),
      self.value,
      TAG
    )
  }
}
