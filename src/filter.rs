use crate::AggregatedRepo;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct RepoWithMvp {
    name: String,
    owner: String,
    mvp: String,
    mvp_percent: f64,
}

impl fmt::Display for RepoWithMvp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:44}{:24}{:.2}",
            format!("{}/{}", self.owner, self.name),
            self.mvp,
            self.mvp_percent
        )
    }
}

pub fn filter(repo: AggregatedRepo) -> Option<RepoWithMvp> {
    let criterion = repo.top25_total_contributions * 3 / 4;
    // unwrap is ok, because a repo without any contributors will not be
    // top-starred ;)
    let mvp_contribution = repo.contributions.get(0).unwrap();

    if mvp_contribution.total < criterion {
        None
    } else {
        Some(RepoWithMvp {
            name: repo.name,
            owner: repo.owner,
            mvp: mvp_contribution.user.clone(),
            mvp_percent: f64::from(mvp_contribution.total)
                / f64::from(repo.top25_total_contributions),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! contribution {
        ($user:expr, $total:expr) => {
            crate::Contribution {
                user: $user.to_owned(),
                total: $total,
            }
        };
    }

    #[test]
    fn rejects_bus_factor_below_75() {
        let aggregated = AggregatedRepo {
            owner: "owner".to_owned(),
            name: "repo".to_owned(),
            top25_total_contributions: 100,
            contributions: vec![contribution!("owner", 74), contribution!("user", 26)],
        };
        assert_eq!(filter(aggregated), None);
    }

    #[test]
    fn accepts_bus_factor_above_75() {
        let aggregated = AggregatedRepo {
            owner: "owner".to_owned(),
            name: "repo".to_owned(),
            top25_total_contributions: 100,
            contributions: vec![contribution!("owner", 75), contribution!("user", 25)],
        };
        assert_eq!(
            filter(aggregated),
            Some(RepoWithMvp {
                name: "repo".to_owned(),
                owner: "owner".to_owned(),
                mvp: "owner".to_owned(),
                mvp_percent: 0.75,
            })
        );
    }
}
