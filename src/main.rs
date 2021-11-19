use std::sync::Arc;
use tokio::sync::mpsc;
#[macro_use]
extern crate clap;

struct Options {
    token: String,
    language: String,
    project_count: u32,
}

const ABOUT: &str = "\
Fetches the top (by stars) `project_count` github repos for programming \
language `language`, investigates contributions, and prints it out if an \
individual contributor is responsible for at least 75% of the total \
contributions from all top 25 contributions";

fn init() -> Options {
    let matches = clap::clap_app!(myapp =>
        (version: "1.0")
        (author: "Till Friesewinkel <till.friesewinkel@gmail.com>")
        (about: ABOUT)
        (@arg language: -l --language +takes_value +required "Sets the programming language to query")
        (@arg project_count: -c --project_count +takes_value +required "Sets the input file to use")
        (@arg token: -t --token +takes_value "Github Authentication Token required to communicate with the Github REST API")
    ).get_matches();

    // we can unwrap, as clap takes care of required arguments
    let language = matches.value_of("language").unwrap().to_owned();
    let project_count = matches
        .value_of("project_count")
        .unwrap()
        .parse()
        .expect("Failed to parse project count as integer");
    let token = match matches.value_of("token") {
        Some(token) => token.to_owned(),
        None => std::env::var("GH_AUTHTOKEN")
            .expect("Did not find an environment variable `GH_AUTHTOKEN`"),
    };

    Options {
        token,
        language,
        project_count,
    }
}

#[tokio::main]
async fn main() {
    let opts = init();

    let fetcher = Arc::new(ghbf_lib::GithubFetcher::new(opts.token));
    let (repo_tx, mut repo_rx) = mpsc::channel(20);
    let (err_tx, mut err_rx) = mpsc::channel(20);

    let repos = match fetcher
        .get_top_repos(&opts.language, opts.project_count)
        .await
    {
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
        Ok(repos) => repos,
    };

    for repo in repos {
        let fetcher = fetcher.clone();
        let repo_tx = repo_tx.clone();
        let err_tx = err_tx.clone();
        tokio::spawn(async move {
            let contributors = match fetcher.get_contributors(&repo).await {
                Err(err) => {
                    err_tx.send(err).await.unwrap();
                    return;
                }
                Ok(contributors) => contributors,
            };

            let aggregation = ghbf_lib::aggregate(repo, contributors).await;
            if let Some(repo) = ghbf_lib::filter(aggregation) {
                // unwrap ok, capacity is ensured
                repo_tx.send(repo).await.unwrap();
            }
        });
    }

    // drops needed as `repo_rx` will otherwise never yield `None`, and thus
    // the program would never exit.
    drop(repo_tx);
    drop(err_tx);

    // print header
    println!("{:24}{:24}{:24}", "project", "user", "percentage");
    println!(
        "{:24}{:24}{:24}",
        "-".repeat(22),
        "-".repeat(22),
        "-".repeat(22),
    );

    // process data
    loop {
        tokio::select! {
            // because of the borrowing rules, we need to nest the match
            // instead of using the pattern matching from the `tokio::select!`
            // macro
            res = repo_rx.recv() => {
                match res {
                    Some(repo) => println!("{}", repo),
                    None => std::process::exit(0),
                }
            },
            Some(err) = err_rx.recv() => eprintln!("{}", err),
        }
    }
}
