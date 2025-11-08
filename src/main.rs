use crate::config::get_config;

mod git;
mod config;
mod conventions;
mod semver;
mod std;
mod bump;

fn main() {
  let config = get_config();

  println!("{:?}", config);
}
