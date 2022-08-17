use crate::errors;

pub struct RepoIdentifier {
  pub owner: String,
  pub name: String,
}

impl RepoIdentifier {
  pub fn parse(s: &str) -> Result<RepoIdentifier, errors::ParseError> {
    let lr: Vec<&str> = s.split([' ', '/']).collect();

    if lr.len() == 2 {
      Ok(RepoIdentifier{
        owner: lr[0].into(),
        name: lr[1].into()
      })
    } else {
      Err(errors::ParseError(s.into()))
    }
  }
}