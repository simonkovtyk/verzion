struct H3 {
  value: String
}

impl H3 {
  pub fn new (value: &str) -> Self {
    H3 {
      value: value.to_string()
    }
  }
}

impl Into<String> for H3 {
  fn into(self) -> String {
    format!("### {}", self.value)
  }
}
