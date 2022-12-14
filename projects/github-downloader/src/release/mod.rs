use crate::GithubDownloader;


import_types!(schema = "list-releases.json");


impl GithubDownloader {
    /// /repos/{owner}/{repo}/releases
    pub fn list_releases(&self, owner: &str, repo: &str) -> Result<Vec<Release>> {
        let url = self.get_host()?.join(&format!("repos/{}/{}/releases", owner, repo))?;
        let response = self.get(url)?;
        let releases: Vec<Release> = response.json()?;
        Ok(releases)
    }
}
