pub trait Merge {
  fn merge (self, other: Self) -> Self;
}

impl <T: Merge + Clone> Merge for Option<T> {
  fn merge (self, other: Option<T>) -> Option<T> {
    match (self, other) {
      (Some(inner_self), Some(inner_other)) => Some(inner_self.merge(inner_other)),
      (matching_self, None) => matching_self,
      (None, matching_other) => matching_other
    }
  }
}

impl Merge for bool {
  fn merge (self, other: bool) -> bool {
    self.clone() || other.clone()
  }
}

impl <T: Clone> Merge for Vec<T> {
  fn merge (self, other: Vec<T>) -> Vec<T> {
    let mut data = Vec::with_capacity(self.len() + other.len());

    data.extend_from_slice(&self);
    data.extend_from_slice(&other);

    data
  }
}
