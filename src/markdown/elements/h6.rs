struct H6 {
  value: String
}

impl H6 {
  pub fn new (value: &str) -> Self {
    H6 {
      value: value.to_string()
    }
  }
}

impl Into<String> for H6 {
  fn into(self) -> String {
    format!("###### {}", self.value)
  }
}
