use crate::conventions::conventional::types::{Types};

pub struct ConventionalBuilder {
  pub header: Option<String>,
  pub body: Option<String>,
  pub footer: Option<Vec<String>>
}

impl ConventionalBuilder {
  pub fn new (
    header: Option<String>,
    body: Option<String>,
    footer: Option<Vec<String>>
  ) -> Self {
    Self {
      header,
      body,
      footer
    }
  }

  #[allow(dead_code)]
  pub fn set_header (&mut self, header: impl Into<String>) -> &mut Self {
    self.header = Some(header.into());
    self
  }

  #[allow(dead_code)]
  pub fn set_body (&mut self, body: impl Into<String>) -> &mut Self {
    self.body = Some(body.into());
    self
  }

  #[allow(dead_code)]
  pub fn set_footer (&mut self, footer: impl Into<Vec<String>>) -> &mut Self {
    self.footer = Some(footer.into());
    self
  }

  #[allow(dead_code)]
  pub fn add_footer (&mut self, footer: impl Into<String>) -> &mut Self {
    if let Some(inner_footer) = self.footer.as_mut() {
      inner_footer.push(footer.into());
    } else {
      self.footer = Some(vec![footer.into()]);
    }

    self
  }
}

impl TryInto<String> for ConventionalBuilder {
  type Error = String;

  fn try_into(self) -> Result<String, Self::Error> {
    let header = self.header.as_ref().ok_or("Header is required for Conventional Commit")?;
    let body = if let Some(inner_body) = self.body.as_ref() {
        format!("\n\n{}", inner_body)
      } else {
        "".to_string()
      };
    let footer = if let Some(inner_footer) = self.footer.as_ref() {
        format!("\n\n{}", inner_footer.join("\n"))
      } else {
        "".to_string()
      };
    
    Ok(format!("{}{}{}", header, body, footer))
  }
}

impl Default for ConventionalBuilder {
  fn default() -> Self {
    Self {
      header: None,
      body: None,
      footer: None
    }
  }
}

pub struct ConventionalHeader {
  pub r#type: Option<Types>,
  pub scope: Option<String>,
  pub content: Option<String>,
  pub breaking_change: Option<bool>
}

impl ConventionalHeader {
  pub fn new (
    r#type: Option<Types>,
    scope: Option<String>,
    content: Option<String>,
    breaking_change: Option<bool>
  ) -> Self {
    Self {
      r#type,
      scope,
      content,
      breaking_change
    }
  }

  #[allow(dead_code)]
  pub fn set_type (&mut self, r#type: impl Into<Types>) -> &mut Self {
    self.r#type = Some(r#type.into());
    self
  }

  #[allow(dead_code)]
  pub fn set_breaking_change (&mut self, breaking_change: impl Into<bool>) -> &mut Self {
    self.breaking_change = Some(breaking_change.into());
    self
  }

  #[allow(dead_code)]
  pub fn set_scope (&mut self, scope: impl Into<String>) -> &mut Self {
    self.scope = Some(scope.into());
    self
  }

  #[allow(dead_code)]
  pub fn set_content (&mut self, content: impl Into<String>) -> &mut Self {
    self.content = Some(content.into());
    self
  }
}

impl TryInto<String> for ConventionalHeader {
  type Error = String;

  fn try_into(self) -> Result<String, Self::Error> {
    let r#type = self.r#type
      .as_ref()
      .ok_or("Type is required for Conventional Header")?
      .to_string();
    let content = self.content
      .as_ref()
      .ok_or("Content is required for Conventional Header")?;
    let breaking_change = if self.breaking_change.unwrap_or(false) {
      "!"
    } else {
      ""
    };

    match self.scope.as_ref() {
      Some(scope) => {
        return Ok(format!("{}{}({}): {}", r#type, breaking_change, scope, content));
      },
      _ => {
        return Ok(format!("{}{}: {}", r#type, breaking_change, content));
      }
    }
  }
}

impl Default for ConventionalHeader {
  fn default() -> Self {
    Self {
      r#type: None,
      scope: None,
      content: None,
      breaking_change: None
    }
  }
}

pub struct ConventionalFooter {
  pub content: Option<String>,
  pub breaking_change: Option<bool>,
}

impl ConventionalFooter {
  pub fn new (
    content: Option<String>,
    breaking_change: Option<bool>,
  ) -> Self {
    Self {
      content,
      breaking_change
    }
  }

  #[allow(dead_code)]
  pub fn set_breaking_changes (&mut self, breaking_change: impl Into<bool>) -> &mut Self {
    self.breaking_change = Some(breaking_change.into());
    self
  }

  #[allow(dead_code)]
  pub fn set_content (&mut self, content: impl Into<String>) -> &mut Self {
    self.content = Some(content.into());
    self
  }
}

impl TryInto<String> for ConventionalFooter {
  type Error = String;

  fn try_into(self) -> Result<String, Self::Error> {
    let content = self.content
      .as_ref()
      .ok_or("Content is required for Conventional Footer")?;
    let breaking_change = if self.breaking_change.unwrap_or(false) {
      "BREAKING CHANGE: "
    } else {
      ""
    };

    Ok(format!("{}{}", breaking_change, content))
  }
}

impl Default for ConventionalFooter {
  fn default() -> Self {
    Self {
      content: None,
      breaking_change: None
    }
  }
}
