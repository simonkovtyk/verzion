pub enum LinebreakStyle {
  Slash,
  Break,
  Spaces,
  EmptyLine
}

pub struct Linebreak {
  style: LinebreakStyle
}

impl Linebreak {
  pub fn new (style: Option<impl Into<LinebreakStyle>>) -> Self {
    Self {
      style: style.map(|v| v.into()).unwrap_or(LinebreakStyle::Break)
    }
  }
}

impl Into<String> for Linebreak {
  fn into(self) -> String {
    match self.style {
      LinebreakStyle::Slash => {
        "\\".to_string()
      },
      LinebreakStyle::Break => {
        "<br/>".to_string()
      },
      LinebreakStyle::Spaces => {
        "  ".to_string()
      },
      LinebreakStyle::EmptyLine => {
        "".to_string()
      }
    }
  }
}
