use diagnostic_quick::QResult;
use reqwest::Url;

mod release;


#[derive(Debug, Copy, Clone)]
pub struct GithubDownloader {
    host: Option<String>,
    token: Option<String>,
}


impl GithubDownloader {
    pub fn with_token(token: String) -> Self {
        Self {
            host: None,
            token: Some(token),
        }
    }
    pub fn get_host(&self) -> QResult<Url> {
        let url = match &self.host {
            Some(host) => Url::parse(host)?,
            None => Url::parse("https://api.github.com")?,
        };
        Ok(url)
    }
}