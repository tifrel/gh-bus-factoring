use crate::{responses::*, Error};

const GH_REST_API_URL: &str = "https://api.github.com";

pub struct GithubFetcher {
    token: String,
    client: reqwest::Client,
}

impl GithubFetcher {
    pub fn new(token: String) -> Self {
        let client = reqwest::Client::new();
        Self { token, client }
    }

    async fn send_request(&self, url: &str) -> Result<reqwest::Response, Error> {
        self.client
            .get(url)
            .header("User-Agent", &self.token)
            // .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|err| Error::Fetch(err))
    }

    pub async fn get_top_repos(
        &self,
        language: &str,
        n_projects: u32,
    ) -> Result<Vec<RepoResponse>, Error> {
        let url = format!(
            "{}/search/repositories?q=+language:{}&sort=stars&order=desc&per_page={}",
            GH_REST_API_URL, language, n_projects
        );

        let res: FetchReposResponse = self
            .send_request(&url)
            .await?
            .json()
            .await
            .map_err(|err| Error::Deserialization(err))?;

        Ok(res.items)
    }

    pub async fn get_contributors(
        &self,
        repo: &RepoResponse,
    ) -> Result<Vec<ContributorResponse>, Error> {
        let url = format!(
            "{}/repos/{}/{}/stats/contributors",
            GH_REST_API_URL, repo.owner.login, repo.name,
        );

        let res: FetchContributorsResponse = self
            .send_request(&url)
            .await
            .unwrap()
            .json()
            .await
            .map_err(|err| Error::Deserialization(err))?;
        Ok(res.0)
    }
}
