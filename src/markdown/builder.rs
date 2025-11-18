#[allow(dead_code)]
pub struct MarkdownBuilder {
  pub content: Vec<String>
}

impl MarkdownBuilder {
  #[allow(dead_code)]
  pub fn new () -> Self {
    MarkdownBuilder {
      content: Vec::new()
    }
  }

  #[allow(dead_code)]
  pub fn add (&mut self, element: impl Into<String>) -> &Self {
    self.content.push(
      element.into()
    );

    self
  }

  #[allow(dead_code)]
  pub fn add_multiple (&mut self, elements: Vec<impl Into<String>>) -> &Self {
    for element in elements {
      self.content.push(
        element.into()
      );
    }

    self
  }
}

impl Into<String> for MarkdownBuilder {
  fn into(self) -> String {
    self.content.join("")
  }
}

impl ToString for MarkdownBuilder {
  fn to_string(&self) -> String {
    self.content.join("")
  }
}
