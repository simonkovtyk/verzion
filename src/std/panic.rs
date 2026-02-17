use crate::log::log_error;

pub const EXIT_ERROR: i32 = 1;
pub const EXIT_SUCCESS: i32 = 0;

pub trait ExpectWithStatusCode<T> {
  fn expect_with_status_code (self, msg: &str, code: i32) -> T;
}

impl <T> ExpectWithStatusCode<T> for Option<T> {
  fn expect_with_status_code (self, msg: &str, code: i32) -> T {
    match self {
      Some(value) => value,
      None => {
        log_error(msg);
        std::process::exit(code);
      }
    }
  }
}

impl <T, E> ExpectWithStatusCode<T> for Result<T, E> {
  fn expect_with_status_code (self, msg: &str, code: i32) -> T {
    match self {
      Ok(value) => value,
      Err(_) => {
        log_error(msg);
        std::process::exit(code);
      }
    }
  }
}
