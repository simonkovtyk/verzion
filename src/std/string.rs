pub trait Capitalize {
  fn capitalize (&self) -> Result<String, &'static str>;
}

impl Capitalize for String {
  fn capitalize (&self) -> Result<Self, &'static str> {
    let mut chars = self.chars();

    let first = chars.next().ok_or("String is empty")?;
    let rest = chars.collect::<String>();

    Ok(format!("{}{}", first.to_uppercase(), rest))
  }
}
