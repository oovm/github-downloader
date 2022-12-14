use diagnostic_quick::error_3rd::RequestClient;
use diagnostic_quick::error_3rd::Url;
use diagnostic_quick::QResult;
use octocrab::Octocrab;

use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Debug, Clone, Default)]
pub struct GithubDownloader {
    host: Option<String>,
    token: Option<String>,
}


impl GithubDownloader {
    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }
    pub fn get_host(&self) -> QResult<Url> {
        Ok(url)
    }
}

impl GithubDownloader {
    /// https://github.abskoop.workers.dev/https://github.com/oovm/wee/releases/download/v0.3.0/wee-win-amd64.exe
    /// /repos/{owner}/{repo}/releases
    pub async fn list_releases(&self, owner: &str, repo: &str) -> QResult {
        let client = match &self.host {
            Some(host) => Octocrab::builder().personal_token(host.clone()).build(),
            None => Octocrab::builder().build(),
        }.unwrap();
        let page = client
            .repos(owner, repo)
            .releases()
            .list()
            // Optional Parameters
            .per_page(100)
            .page(0u32)
            // Send the request
            .send()
            .await.unwrap();
        for item in page.items {
            println!("{:#?}", item);
        }
        Ok(())
    }
}
