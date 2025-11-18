use std::process::Command;

pub struct GitRemote {
  pub url: String
}

pub fn get_remote_url (cwd: &Option<String>) -> Option<GitRemote> {
  let mut command = Command::new("git");

  command.args(&[
    "remote",
    "get-url",
    "--push",
    "origin"
  ]);

  if let Some(cwd) = cwd {
    command.current_dir(cwd);
  }

  let output = command.output().expect("Could not execute git show command");

  if output.stdout.is_empty() {
    return None;
  }

  let content = String::from_utf8(output.stdout).expect("Content contained invalid UTF-8");

  Some(GitRemote {
    url: content
  })
}
