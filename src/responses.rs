use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserResponse {
    pub(crate) login: String,
}

#[derive(Deserialize, Debug)]
pub struct RepoResponse {
    pub(crate) name: String,
    pub(crate) owner: UserResponse,
}

#[derive(Deserialize, Debug)]
pub struct ContributorResponse {
    pub(crate) author: Option<UserResponse>,
    pub(crate) weeks: Vec<WeeklyCommitsResponse>,
}

#[derive(Deserialize, Debug)]
pub struct WeeklyCommitsResponse {
    pub(crate) a: i64, // signed not because it makes sense, but because GH
    pub(crate) d: i64, // sometimes returns such nonsense
}

#[derive(Deserialize, Debug)]
pub(crate) struct FetchReposResponse {
    pub(crate) items: Vec<RepoResponse>,
}

#[derive(Deserialize, Debug)]
#[serde(transparent)]
pub(crate) struct FetchContributorsResponse(pub(crate) Vec<ContributorResponse>);
