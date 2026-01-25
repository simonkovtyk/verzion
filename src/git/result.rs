pub struct GitRulesetResult {
  pub needs_push: bool
}

impl Default for GitRulesetResult {
  fn default() -> Self {
    Self {
      needs_push: false
    }
  }
}
