#[allow(dead_code)]
pub struct ListItem {
  value: String
}

impl ListItem {
  #[allow(dead_code)]
  pub fn new (value: impl Into<String>) -> Self {
    Self { value: value.into() }
  }
}

impl Into<String> for ListItem {
  fn into(self) -> String {
    self.value
  }
}

impl ToString for ListItem {
  fn to_string(&self) -> String {
    self.value.clone()
  }
}

#[allow(dead_code)]
pub struct List {
  items: Vec<ListItem>,
  ordered: bool
}

impl List {
  #[allow(dead_code)]
  pub fn new (ordered: Option<bool>) -> Self {
    Self {
      items: Vec::new(),
      ordered: ordered.unwrap_or(false)
    }
  }

  #[allow(dead_code)]
  pub fn add (&mut self, item: ListItem) -> &Self {
    self.items.push(item);

    self
  }
}

impl Into<String> for List {
  fn into(self) -> String {
    let mut items_str = Vec::new();

    for (index, item) in self.items.iter().enumerate() {
      let formatted = if self.ordered {
        format!("{}. {}", index + 1, item.to_string())
      } else {
        format!("- {}", item.to_string())
      };

      items_str.push(formatted);
    }

    items_str.join("\n")
  }
}
