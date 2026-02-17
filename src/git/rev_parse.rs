use std::process::Command;

use crate::std::command::CommandOptions;

pub fn get_rev_parse (rev: &str, options: CommandOptions) -> Result<String, String> {
  let mut rev_parse_command = Command::new("git");

  rev_parse_command.args(&[
    "rev-parse",
    rev
  ]);

  if let Some(cwd) = options.cwd.as_ref() {
    rev_parse_command.current_dir(cwd);
  }

  let output = rev_parse_command.output().map_err(|_| "Could not execute git rev-parse command")?;

  if output.stdout.is_empty() {
    return Err("No output from git rev-parse".to_string());
  }

  String::from_utf8(output.stdout).map_err(|_| "Output contained invalid UTF-8".to_string())
}
