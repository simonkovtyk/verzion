#[allow(dead_code)]
pub struct H2 {
  value: String
}

impl H2 {
  #[allow(dead_code)]
  pub fn new (value: impl Into<String>) -> Self {
    H2 {
      value: value.into()
    }
  }
}

impl Into<String> for H2 {
  fn into(self) -> String {
    format!("## {}", self.value)
  }
}
