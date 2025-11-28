use reqwest_retry::policies::ExponentialBackoff;

const DEFAULT_RETRIES: u32 = 3;

pub fn get_user_agent () -> String {
  let version = env!("CARGO_PKG_VERSION");

  format!("Verzion/{} (Compatible; Minimal)", version)
}

pub fn get_retry_policy (retries: Option<u32>) -> ExponentialBackoff {
  ExponentialBackoff::builder().build_with_max_retries(retries.unwrap_or(DEFAULT_RETRIES))
}
