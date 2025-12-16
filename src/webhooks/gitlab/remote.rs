use url::Url;

pub struct GitLabRemote {
  pub url: Url
}

impl GitLabRemote {
  pub fn to_origin (&mut self, token: &Option<String>) -> String {
    if self.url.scheme() != "http" || self.url.scheme() != "https" {
      self.url.set_scheme("https");
    }

    self.url.set_username("verzion");
    self.url.set_password(token.as_deref());

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

impl TryFrom<&str> for GitLabRemote {
  type Error = &'static str;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let url = Url::parse(&value).map_err(|_| "URL could not be parsed")?;

    Ok(GitLabRemote {
      url
    })
  }
}
