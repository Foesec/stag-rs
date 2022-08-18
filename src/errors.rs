use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum StagError {
    #[error("No token specified or passed via env variable")]
    MissingToken,
    #[error("Error when building octocrab client: {0}")]
    BuildingClientFailure(#[source] octocrab::Error),
    #[error(transparent)]
    OctocrabError(#[from] octocrab::Error),
    #[error("Error when reading repo list")]
    RepoListError
}

#[derive(Error, Debug)]
#[error("Failed to parse model from string `{0}`")]
pub struct ParseError(pub String);
