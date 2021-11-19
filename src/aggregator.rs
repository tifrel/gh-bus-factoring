use crate::responses::{ContributorResponse, RepoResponse};

#[derive(Debug, PartialEq)]
pub struct Contribution {
    pub(crate) user: String,
    pub(crate) total: u32,
}

#[derive(Debug, PartialEq)]
pub struct AggregatedRepo {
    pub(crate) owner: String,
    pub(crate) name: String,
    pub(crate) top25_total_contributions: u32,
    pub(crate) contributions: Vec<Contribution>,
}

pub async fn aggregate(
    repo: RepoResponse,
    contributors: Vec<ContributorResponse>,
) -> AggregatedRepo {
    let mut contributions = Vec::new();
    for contributor in contributors {
        let mut total = 0;
        for week in contributor.weeks {
            total += week.a + week.d;
        }

        contributions.push(Contribution {
            user: contributor.author.login,
            total,
        });
    }

    // arguments swapped so we get the top contributor first
    contributions.sort_by(|c1, c2| c2.total.cmp(&c1.total));

    let top25_total_contributions = contributions
        .iter()
        .take(25)
        .fold(0, |total, contribution| total + contribution.total);

    AggregatedRepo {
        owner: repo.owner.login,
        name: repo.name,
        top25_total_contributions,
        contributions,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{ContributorResponse, RepoResponse, UserResponse, WeeklyCommitsResponse};

    macro_rules! repo_response {
        ($name:expr, $owner:expr) => {
            RepoResponse {
                name: $name.to_owned(),
                owner: UserResponse {
                    login: $owner.to_owned(),
                },
            }
        };
    }

    macro_rules! contributor_response {
		($author:expr, $(($a:expr, $d:expr)),+) => {
			ContributorResponse {
				author: UserResponse { login: $author.to_owned() },
				weeks: vec![$(WeeklyCommitsResponse {a: $a, d: $d}),+],
			}
		};
	}

    #[tokio::test]
    async fn aggregation_ordered() {
        let repo = repo_response!("repo", "owner");
        let contributors = vec![
            contributor_response!("owner", (20, 0), (10, 10)),
            contributor_response!("user", (2, 1), (3, 4)),
        ];
        let aggregated = AggregatedRepo {
            owner: "owner".to_owned(),
            name: "repo".to_owned(),
            top25_total_contributions: 50,
            contributions: vec![
                Contribution {
                    user: "owner".to_owned(),
                    total: 40,
                },
                Contribution {
                    user: "user".to_owned(),
                    total: 10,
                },
            ],
        };

        assert_eq!(aggregate(repo, contributors).await, aggregated);
    }

    #[tokio::test]
    async fn aggregation_unordered() {
        let repo = repo_response!("repo", "owner");
        let contributors = vec![
            contributor_response!("owner", (2, 1), (3, 4)),
            contributor_response!("user", (20, 0), (10, 10)),
        ];
        let aggregated = AggregatedRepo {
            owner: "owner".to_owned(),
            name: "repo".to_owned(),
            top25_total_contributions: 50,
            contributions: vec![
                Contribution {
                    user: "user".to_owned(),
                    total: 40,
                },
                Contribution {
                    user: "owner".to_owned(),
                    total: 10,
                },
            ],
        };

        assert_eq!(aggregate(repo, contributors).await, aggregated);
    }

    #[tokio::test]
    async fn aggregation_gt25() {
        let repo = repo_response!("repo", "owner");
        let contributors = (1..30)
            .map(|i| contributor_response!(format!("user{}", i), (i, 0)))
            .collect();
        let aggregated = AggregatedRepo {
            owner: "owner".to_owned(),
            name: "repo".to_owned(),
            top25_total_contributions: (5..30).reduce(|a, b| a + b).unwrap(),
            contributions: (1..30)
                .rev()
                .map(|i| Contribution {
                    user: format!("user{}", i),
                    total: i,
                })
                .collect(),
        };

        assert_eq!(aggregate(repo, contributors).await, aggregated);
    }
}
