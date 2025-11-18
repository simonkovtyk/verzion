use colored::Colorize;

const WARN_PREFIX: &str = "WARN";
const INFO_PREFIX: &str = "INFO";
const ERROR_PREFIX: &str = "ERROR";
const SUCCESS_PREFIX: &str = "SUCCESS";

pub fn log_info (value: &str, colored: &Option<bool>) {
  let mut prefix = create_prefix(INFO_PREFIX);

  if colored.unwrap_or(true) {
    prefix = prefix.blue().bold().to_string();
  }

  println!(
    "{} {}",
    prefix,
    value
  );
}

pub fn log_error (value: &str, colored: &Option<bool>) {
  let mut prefix = create_prefix(ERROR_PREFIX);

  if colored.unwrap_or(true) {
    prefix = prefix.red().bold().to_string();
  }

  println!(
    "{} {}",
    prefix,
    value
  );
}

pub fn log_success (value: &str, colored: &Option<bool>) {
  let mut prefix = create_prefix(SUCCESS_PREFIX);

  if colored.unwrap_or(true) {
    prefix = prefix.green().bold().to_string();
  }

  println!(
    "{} {}",
    prefix,
    value
  );
}

pub fn log_warn (value: &str, colored: &Option<bool>) {
  let mut prefix = create_prefix(WARN_PREFIX);

  if colored.unwrap_or(true) {
    prefix = prefix.yellow().bold().to_string();
  }

  println!(
    "{} {}",
    prefix,
    value
  );
}

const LOGO: &str = r"
                          .__               
___  __ __________________|__| ____   ____  
\  \/ // __ \_  __ \___   /  |/  _ \ /    \ 
 \   /\  ___/|  | \//    /|  (  <_> )   |  \
  \_/  \___  >__|  /_____ \__|\____/|___|  /
           \/            \/              \/ 
";

pub fn create_prefix (value: &str) -> String {
  format!("[{}]", value)
}

pub fn print_header (colored: &Option<bool>) {
  let mut logo = LOGO.to_string();
  let mut version = env!("CARGO_PKG_VERSION").to_string();

  if colored.unwrap_or(true) {
    logo = logo.purple().bold().to_string();
    version = format!(" {} ", version).on_purple().white().bold().to_string();
  }

  println!(
    "{}\n{}\n",
    logo,
    version
  );
}
