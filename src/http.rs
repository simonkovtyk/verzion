pub fn get_user_agent () -> String {
  let version = env!("CARGO_PKG_VERSION");

  format!("Verzion/{} (Compatible; Minimal)", version)
}
