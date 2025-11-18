#[allow(dead_code)]
pub enum ParagraphStyle {
  None,
  Bold,
  Italic,
  Strikethrough,
  Subscript,
  Superscript,
  Underline,
  AllBoldAndItalic
}

#[allow(dead_code)]
const BOLD_TAG: &str = "**";
#[allow(dead_code)]
const BOLD_ALT_TAG: &str = "__";
#[allow(dead_code)]
const ITALIC_TAG: &str = "*";
#[allow(dead_code)]
const ITALIC_ALT_TAG: &str = "_";
#[allow(dead_code)]
const STRIKETHROUGH_TAG: &str = "~~";
#[allow(dead_code)]
const STRIKETHROUGH_ALT_TAG: &str = "~";
#[allow(dead_code)]
const SUBSCRIPT_OPEN_TAG: &str = "<sub>";
#[allow(dead_code)]
const SUBSCRIPT_CLOSE_TAG: &str = "</sub>";
#[allow(dead_code)]
const SUPERSCRIPT_OPEN_TAG: &str = "<sup>";
#[allow(dead_code)]
const SUPERSCRIPT_CLOSE_TAG: &str = "</sup>";
#[allow(dead_code)]
const UNDERLINE_OPEN_TAG: &str = "<ins>";
#[allow(dead_code)]
const UNDERLINE_CLOSE_TAG: &str = "</ins>";
#[allow(dead_code)]
const ALL_BOLD_AND_ITALIC_TAG: &str = "***";

#[allow(dead_code)]
pub struct Paragraph {
  value: String,
  style: ParagraphStyle,
  use_alt: bool
}

impl Paragraph {
  #[allow(dead_code)]
  pub fn new (value: impl Into<String>, style: Option<impl Into<ParagraphStyle>>, use_alt: Option<impl Into<bool>>) -> Self {
    Self {
      value: value.into(),
      style: style.map(|v| v.into()).unwrap_or(ParagraphStyle::None),
      use_alt: use_alt.map(|v| v.into()).unwrap_or(false)
    }
  }
}

impl Into<String> for Paragraph {
  fn into(self) -> String {
    match self.style {
      ParagraphStyle::None => {
        self.value
      },
      ParagraphStyle::Bold => {
        let tag = if self.use_alt { BOLD_ALT_TAG } else { BOLD_TAG };
        format!(
          "{}{}{}",
          tag,
          self.value,
          tag
        )
      },
      ParagraphStyle::Italic => {
        let tag = if self.use_alt { ITALIC_ALT_TAG } else { ITALIC_TAG };
        format!(
          "{}{}{}",
          tag,
          self.value,
          tag
        )
      },
      ParagraphStyle::Strikethrough => {
        let tag = if self.use_alt { STRIKETHROUGH_ALT_TAG } else { STRIKETHROUGH_TAG };
        format!(
          "{}{}{}",
          tag,
          self.value,
          tag
        )
      },
      ParagraphStyle::Subscript => {
        format!(
          "{}{}{}",
          SUBSCRIPT_OPEN_TAG,
          self.value,
          SUBSCRIPT_CLOSE_TAG
        )
      },
      ParagraphStyle::Superscript => {
        format!(
          "{}{}{}",
          SUPERSCRIPT_OPEN_TAG,
          self.value,
          SUPERSCRIPT_CLOSE_TAG
        )
      },
      ParagraphStyle::Underline => {
        format!(
          "{}{}{}",
          UNDERLINE_OPEN_TAG,
          self.value,
          UNDERLINE_CLOSE_TAG
        )
      },
      ParagraphStyle::AllBoldAndItalic => {
        format!(
          "{}{}{}",
          ALL_BOLD_AND_ITALIC_TAG,
          self.value,
          ALL_BOLD_AND_ITALIC_TAG
        )
      }
    }
  }
}
