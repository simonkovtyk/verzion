#[allow(dead_code)]
pub struct Image {
  src: String,
  alt: Option<String>
}

impl Image {
  #[allow(dead_code)]
  pub fn new (src: impl Into<String>, alt: Option<impl Into<String>>) -> Self {
    Image {
      src: src.into(),
      alt: alt.map(|v| v.into())
    }
  }
}

impl Into<String> for Image {
  fn into(self) -> String {
    format!(
      "![{}]({})",
      self.alt.clone().unwrap_or("".to_string()),
      self.src
    )
  }
}
