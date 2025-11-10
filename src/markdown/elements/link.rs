pub struct Link {
  href: String,
  text: String
}

impl Link {
  pub fn new (href: impl Into<String>, text: impl Into<String>) -> Self {
    Link {
      href: href.into(),
      text: text.into()
    }
  }
}

impl Into<String> for Link {
  fn into(self) -> String {
    format!(
      "[{}]({})",
      self.text,
      self.href
    )
  }
}
