use crate::{config::Config, conventions::{config::{ConvetionTypes, DEFAULT_CONVENTION}, conventional::{advertise::get_commit_msg_footer, builder::{ConventionalBuilder, ConventionalHeader}, types::Types}}};

pub fn get_conventional_commit_msg () -> String {
  let conventional_header = ConventionalHeader::new(
    Some(Types::Chore),
    Some("metafile".to_string()),
    Some("update semver".to_string()),
    Some(false)
  );

  return ConventionalBuilder::new(
    Some(conventional_header.to_string()),
    None,
    Some(vec![get_commit_msg_footer()])
  ).to_string();
}

pub fn get_commit_msg () -> String {
  let config = Config::inject();

  let convention = config.convention.as_ref().unwrap_or(&DEFAULT_CONVENTION);

  match convention {
    ConvetionTypes::Conventional => get_conventional_commit_msg()
  }
}
