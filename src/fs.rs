use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use anyhow::Result;

use crate::model::github::RepoIdentifier;

pub fn read_repo_list<P: AsRef<Path>>(path: P) -> Result<Vec<RepoIdentifier>> {
    let file = fs::File::open(path)?;
    let mut repo_ids = vec![];
    for (i, line) in io::BufReader::new(file).lines().enumerate() {
        match line {
            Ok(line) => match RepoIdentifier::parse(&line) {
                Ok(repo_id) => repo_ids.push(repo_id),
                Err(e) => eprintln!("Failed to parse line {} to repo identified: {}", i, e),
            },
            Err(e) => eprintln!("Failed to read line {}: {}", i, e),
        }
    }
    Ok(repo_ids)
}
