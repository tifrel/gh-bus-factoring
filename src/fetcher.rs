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

    async fn send_request(&self, url: &str, what: &str) -> Result<reqwest::Response, Error> {
        let res = self
            .client
            .get(url)
            .header("User-Agent", "golem-task")
            .header("Authorization", &format!("token {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|err| Error::Fetch(what.to_string(), err))?;

        if res.status().is_success() {
            Ok(res)
        } else {
            Err(Error::BadStatus(what.to_string(), res.status()))
        }
    }

    async fn get_request_body(&self, url: &str, what: &str) -> Result<String, Error> {
        let res = self.send_request(url, what).await?;
        match res
            .text()
            .await
            .map_err(|err| Error::ResponseBody(what.to_string(), err))
        {
            Ok(body) if &body == "{}" => Err(Error::EmptyBody(what.to_string())),
            result => result,
        }
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

        let body = self.get_request_body(&url, "top repos").await?;
        match serde_json::from_str::<FetchReposResponse>(&body) {
            Err(err) => Err(Error::Deserialization("top repos".to_string(), err)),
            Ok(FetchReposResponse { items }) => Ok(items),
        }
    }

    pub async fn get_contributors(
        &self,
        repo: &RepoResponse,
    ) -> Result<Vec<ContributorResponse>, Error> {
        let url = format!(
            "{}/repos/{}/{}/stats/contributors",
            GH_REST_API_URL, repo.owner.login, repo.name,
        );

        let body = self
            .get_request_body(&url, &format!("{}/{}", repo.owner.login, repo.name))
            .await?;
        match serde_json::from_str::<FetchContributorsResponse>(&body) {
            Err(err) => Err(Error::Deserialization(repo.name.clone(), err)),
            Ok(FetchContributorsResponse(contributors)) => Ok(contributors),
        }
    }
}
