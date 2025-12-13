use std::path::Path;

use url::Url;

use crate::{git::remote::GitRemote};

#[derive(Debug)]
pub struct GitHubRemote {
  pub url: Url,
  pub owner: String,
  pub repository: String
}

impl GitHubRemote {
  pub fn to_origin (&self, token: &Option<String>) -> String {
    let mut url = self.url.clone();

    if self.url.scheme() != "https" {
      url.set_scheme("https");
    }

    url.set_username("verzion");
    url.set_password(token.as_deref());

    url.to_string()
  }
}

impl TryFrom<&str> for GitHubRemote {
  type Error = String;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let url = Url::parse(&value);

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
      url,
      owner: owner.unwrap(),
      repository: repository.unwrap()
    })
  }
}
