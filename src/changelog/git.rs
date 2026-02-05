use crate::{config::{Config, ToExitCode}, conventions::{config::{ConvetionTypes, DEFAULT_CONVENTION}, conventional::{advertise::get_commit_msg_footer, builder::{ConventionalBuilder, ConventionalHeader}, types::Types}}, std::panic::ExpectWithStatusCode};

pub fn get_conventional_commit_msg () -> String {
  let config = Config::inject();

  let conventional_header = ConventionalHeader::new(
    Some(Types::Chore),
    Some("changelog".to_string()),
    Some("update changelog".to_string()),
    Some(false)
  );

  return ConventionalBuilder::new(
    conventional_header.try_into().ok(),
    None,
    Some(vec![get_commit_msg_footer()])
  ).try_into()
    .expect_with_status_code(
      "Could not get conventional commit msg",
      config.to_exit_code()
    );
}

pub fn get_commit_msg () -> String {
  let config = Config::inject();

  let convention = config.convention.as_ref().unwrap_or(&DEFAULT_CONVENTION);

  match convention {
    ConvetionTypes::Conventional => get_conventional_commit_msg()
  }
}
