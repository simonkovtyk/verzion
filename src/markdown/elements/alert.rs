pub enum AlertType {
  Note,
  Tip,
  Important,
  Warning,
  Caution
}

impl ToString for AlertType {
  fn to_string(&self) -> String {
    match self {
      AlertType::Note => "NOTE".to_string(),
      AlertType::Tip => "TIP".to_string(),
      AlertType::Important => "IMPORTANT".to_string(),
      AlertType::Warning => "WARNING".to_string(),
      AlertType::Caution => "CAUTION".to_string()
    }
  }
}

pub struct Alert {
  r#type: AlertType,
  value: String
}

impl Alert {
  pub fn new (r#type: AlertType, value: &str) -> Self {
    Alert {
      r#type,
      value: value.to_string()
    }
  }
}

impl Into<String> for Alert {
  fn into(self) -> String {
    format!("> {}\n> {}", self.r#type.to_string(), self.value)
  }
}
