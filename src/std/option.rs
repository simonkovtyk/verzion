pub trait ToOption {
  fn to_option (&self) -> Option<Self> where Self: Sized;
}

impl <T: Clone> ToOption for Vec<T> {
  fn to_option (&self) -> Option<Self> where Self: Sized {
    if self.is_empty() {
      return None
    }

    Some(self.clone())
  }
}
