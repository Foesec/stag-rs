use clap::Parser;
use octocrab::{self, Octocrab};
use owo_colors::*;
use std::env;

mod errors;
mod cl_args;
mod fs;
mod model;
use errors::*;
use cl_args::ClArgs;

pub type StagResult<T> = Result<T, StagError>;

#[tokio::main]
async fn main() -> StagResult<()> {
    println!("Hello, world!");

    let args = ClArgs::parse();
    let token = get_personal_access_token(&args)?;

    println!(
        "Using token {}\nFetching {} tags",
        token.bright_green().on_blue(),
        args.n.bright_green().on_blue()
    );

    let client = initialize_github_client(token)?;

    let repos = client
        .current()
        .list_repos_for_authenticated_user()
        .send()
        .await?;

    println!("Received {} repos", repos.total_count.clone().get_or_insert(0));

    for repo in repos {
        let tags = match repo.owner.map(|owner| (owner.login, &repo.name)) {
            Some((owner, name)) => client.repos(owner, name).list_tags().per_page(3).send().await,
            None => todo!(),
        };
        println!("{}\t {:?}", repo.name.bright_red().on_blue(), tags.bright_white().on_blue());
    }

    Ok(())
}

fn get_personal_access_token(args: &ClArgs) -> Result<String, StagError> {
    match args.token {
        Some(ref t) => Ok(t.into()),
        None => env::var("GH_ACCESS_TOKEN")
            .or(env::var("TOKEN"))
            .or(Err(StagError::MissingToken)),
    }
}

fn initialize_github_client(token: String) -> Result<Octocrab, StagError> {
    octocrab::Octocrab::builder()
        .personal_token(token)
        .build()
        .map_err(|oc_err| StagError::BuildingClientFailure(oc_err))
}


