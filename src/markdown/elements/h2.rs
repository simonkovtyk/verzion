struct H2 {
  value: String
}

impl H2 {
  pub fn new (value: &str) -> Self {
    H2 {
      value: value.to_string()
    }
  }
}

impl Into<String> for H2 {
  fn into(self) -> String {
    format!("## {}", self.value)
  }
}
