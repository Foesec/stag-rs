use std::{cmp, vec};

use anyhow::Result;
use clap::Parser;
use octocrab::{self, Octocrab};
use owo_colors::*;

mod cl_args;
mod errors;
mod fs;
mod model;
use cl_args::ClArgs;

use crate::model::github::{SimpleTag, TaggedRepository};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    let args = ClArgs::parse();

    println!("Running command with parameters {:?}", &args);

    let client = initialize_github_client(args.token.as_ref())?;

    let repos = match args.repos_file.as_ref() {
        Some(path) => fs::read_repo_list(path),
        None => {
            let rs = client
                .current()
                .list_repos_for_authenticated_user()
                .send()
                .await?;
            rs.into_iter()
                .map(model::github::RepoIdentifier::try_from)
                .collect()
        }
    }?;

    println!("Fetching tags for {} repos", repos.len());
    // for repo in &repos {
    //     println!("{}", repo.bright_green().on_black());
    // }

    let mut tagged_repos = vec![];

    for repo in repos {
        let tags = client
            .repos(&repo.owner, &repo.name)
            .list_tags()
            .per_page(3)
            .send()
            .await?;
        let tags: Vec<SimpleTag> = tags.into_iter().map(|tag| SimpleTag(tag.name)).collect();
        tagged_repos.push(TaggedRepository {
            identifier: repo,
            tags,
        })
    }

    pretty_print_tagged_repos(&tagged_repos);

    Ok(())
}

fn initialize_github_client(token: Option<&String>) -> Result<Octocrab> {
    let mut builder = octocrab::Octocrab::builder();

    if let Some(tk) = token {
        builder = builder.personal_token(tk.into())
    }
    builder.build().map_err(|err| err.into())
}

fn pretty_print_tagged_repos(repos: &[TaggedRepository]) {
    if repos.is_empty() {
        println!("No repos to print...");
        return;
    }
    let mut longest_repo_len = 0;
    for repo in repos {
        let l = repo.identifier.len();
        longest_repo_len = cmp::max(l, longest_repo_len);
    }
    for repo in repos {
        let padded_repo = format!("{:1$}", format!("{}", &repo.identifier), longest_repo_len);
        let concat_tags = if repo.tags.is_empty() {
            format!("{}", "-".bright_magenta().on_black())
        } else {
            let first = format!("{}", &repo.tags[0].green().on_black());
            repo.tags[1..]
            .iter()
            .fold(first, |acc, next| format!("{}, {}", acc, next))
        };
        println!("{}\t[{}]", padded_repo, concat_tags)
    }
}
