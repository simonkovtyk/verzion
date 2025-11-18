use std::path::Path;

use url::Url;

use crate::git::remote::GitRemote;

#[derive(Debug)]
pub struct GitHubRemote {
  pub owner: String,
  pub repository: String
}

impl TryFrom<GitRemote> for GitHubRemote {
  type Error = String;

  fn try_from(value: GitRemote) -> Result<Self, Self::Error> {
    let url = Url::parse(&value.url);

    if url.is_err() {
      return Err("URL could not be parsed".to_string());
    }

    let url = url.unwrap();
    let path = Path::new(url.path());
    let mut owner = None;
    let mut repository = None;

    for component in path.components() {
      match component {
        std::path::Component::Normal(value) => {
          let value = value.to_str();

          if value.is_none() {
            return Err("Invalid character in URL path".to_string());
          }

          let value = value.unwrap().to_string();

          if owner.is_none() {
            owner = Some(value);
            continue;
          }

          let len = value.chars().count().saturating_sub(4);
          let value = value.chars().take(len).collect::<String>();
          repository = Some(value);
        },
        _ => {}
      }
    }

    if owner.is_none() || repository.is_none() {
      return Err("Owner or repository not found in URL".to_string());
    }

    return Ok(Self {
      owner: owner.unwrap(),
      repository: repository.unwrap()
    })
  }
}
