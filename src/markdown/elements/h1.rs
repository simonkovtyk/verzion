#[allow(dead_code)]
pub struct H1 {
  value: String
}

impl H1 {
  #[allow(dead_code)]
  pub fn new (value: impl Into<String>) -> Self {
    H1 {
      value: value.into()
    }
  }
}

impl Into<String> for H1 {
  fn into(self) -> String {
    format!("# {}", self.value)
  }
}
