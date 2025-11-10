struct H1 {
  value: String
}

impl H1 {
  pub fn new (value: &str) -> Self {
    H1 {
      value: value.to_string()
    }
  }
}

impl Into<String> for H1 {
  fn into(self) -> String {
    format!("# {}", self.value)
  }
}
