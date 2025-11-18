#[allow(dead_code)]
pub struct TaskItem {
  value: String,
  description: Option<String>,
  checked: Option<bool>
}

impl TaskItem {
  #[allow(dead_code)]
  pub fn new (value: impl Into<String>, description: Option<impl Into<String>>, checked: Option<impl Into<bool>>) -> Self {
    Self {
      value: value.into(),
      description: description.map(|v| v.into()),
      checked: checked.map(|v| v.into())
    }
  }
}

impl Into<String> for TaskItem {
  fn into(self) -> String {
    format!(
      "- [{}]{}{}",
      self.checked.map(|v| if v { "x" } else { "" }).unwrap_or(""),
      self.description.map(|v| format!(" \\({}) ", v)).unwrap_or("".to_string()),
      self.value
    )
  }
}

impl ToString for TaskItem {
  fn to_string(&self) -> String {
    format!(
      "- [{}]{}{}",
      self.checked.map(|v| if v { "x" } else { "" }).unwrap_or(""),
      self.description.as_ref().map(|v| format!(" \\({}) ", v)).unwrap_or("".to_string()),
      self.value
    )
  }
}

#[allow(dead_code)]
pub struct Tasks {
  items: Vec<TaskItem>
}

impl Tasks {
  #[allow(dead_code)]
  pub fn new () -> Self {
    Self {
      items: Vec::new(),
    }
  }

  #[allow(dead_code)]
  pub fn add (&mut self, item: TaskItem) -> &Self {
    self.items.push(item);

    self
  }
}

impl Into<String> for Tasks {
  fn into(self) -> String {
    self.items.iter().map(|item| item.to_string()).collect::<Vec<_>>().join("\n")
  }
}
