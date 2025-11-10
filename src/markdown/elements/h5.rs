struct H5 {
  value: String
}

impl H5 {
  pub fn new (value: &str) -> Self {
    H5 {
      value: value.to_string()
    }
  }
}

impl Into<String> for H5 {
  fn into(self) -> String {
    format!("##### {}", self.value)
  }
}
