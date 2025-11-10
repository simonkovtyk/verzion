struct H4 {
  value: String
}

impl H4 {
  pub fn new (value: &str) -> Self {
    H4 {
      value: value.to_string()
    }
  }
}

impl Into<String> for H4 {
  fn into(self) -> String {
    format!("#### {}", self.value)
  }
}
