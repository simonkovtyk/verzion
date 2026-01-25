use crate::std::command::CommandOptions;

pub fn commit (
  commit_msg: &str,
  command_options: CommandOptions
) -> Result<(), String> {
  let mut command = std::process::Command::new("git");

  command.args(&[
    "commit",
    "-m",
    commit_msg
  ]);

  if let Some(cwd) = command_options.cwd {
    command.current_dir(cwd);
  }

  command.output().map(|_| ()).map_err(|_| "Could not execute git commit".to_string())
}
