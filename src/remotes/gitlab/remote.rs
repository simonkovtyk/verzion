use url::Url;

use crate::{config::Config, git::remote::GitRemote, remotes::config::{RemoteType, get_token}};

pub struct GitLabRemote {
  pub url: Url
}

impl GitLabRemote {
  pub fn to_origin (&mut self) -> String {
    let config = Config::inject();
    let token = get_token(config, &RemoteType::GitLab);

    if self.url.scheme() != "http" || self.url.scheme() != "https" {
      self.url.set_scheme("https");
    }

    self.url.set_username("verzion");
    self.url.set_password(Some(&token));

    self.url.to_string()
  }

  pub fn get_project_path (&self) -> String {
    let path = self.url.path();
    path[1..path.len() - 4].to_string()
  }

  pub fn get_api_url (&self) -> String {
    let mut url = self.url.clone();

    url.set_path("");

    let mut url_str = url.as_str();
    url_str = &url_str[..url_str.len() - 1];

    format!(
      "{}/{}/{}/{}",
      url_str,
      "api/v4/projects",
      urlencoding::encode(&self.get_project_path()),
      "releases"
    )
  }
}

impl TryFrom<&GitRemote> for GitLabRemote {
  type Error = &'static str;

  fn try_from(value: &GitRemote) -> Result<Self, Self::Error> {
    let url = Url::parse(&value.url);

    if url.is_err() {
      return Err("URL could not be parsed");
    }

    let url = url.unwrap();

    Ok(GitLabRemote {
      url
    })
  }
}
