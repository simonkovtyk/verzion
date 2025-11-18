#[allow(dead_code)]
pub struct Quote {
  value: String
}

impl Quote {
  #[allow(dead_code)]
  pub fn new (value: impl Into<String>) -> Self {
    Self {
      value: value.into()
    }
  }
}

impl Into<String> for Quote {
  fn into(self) -> String {
    format!("> {}", self.value)
  }
}
