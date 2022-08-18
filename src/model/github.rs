use std::fmt::Display;

use anyhow::{anyhow, Result};
use octocrab::models::Repository;

pub struct RepoIdentifier {
    pub owner: String,
    pub name: String,
}

impl RepoIdentifier {
    pub fn parse(s: &str) -> Result<RepoIdentifier> {
        let lr: Vec<&str> = s.split([' ', '/']).collect();

        if lr.len() == 2 {
            Ok(RepoIdentifier {
                owner: lr[0].into(),
                name: lr[1].into(),
            })
        } else {
            Err(anyhow!("Unable to parse string to repo {}", s))
        }
    }

    pub fn len(&self) -> usize {
        self.owner.len() + self.name.len() + 1
    }
}

impl TryFrom<Repository> for RepoIdentifier {
    type Error = anyhow::Error;

    fn try_from(repo: Repository) -> std::result::Result<Self, Self::Error> {
        if let Some(owner) = repo.owner {
            Ok(RepoIdentifier {
                owner: owner.login.clone(),
                name: repo.name.clone(),
            })
        } else {
            Err(anyhow!("No owner found in repository {}", &repo.name))
        }
    }
}

impl Display for RepoIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

pub struct SimpleTag(pub String);

pub struct TaggedRepository {
    pub identifier: RepoIdentifier,
    pub tags: Vec<SimpleTag>,
}

impl Display for SimpleTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
