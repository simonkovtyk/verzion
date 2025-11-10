pub struct MarkdownBuilder {
  content: Vec<String>
}

impl MarkdownBuilder {
  pub fn new () -> Self {
    MarkdownBuilder {
      content: Vec::new()
    }
  }

  pub fn add<T> (&mut self, element: T) -> &Self
    where T: ToString {
    self.content.push(
      element.to_string()
    );

    self
  }
}

impl ToString for MarkdownBuilder {
  fn to_string(&self) -> String {
    self.content.join("\n")
  }
}
