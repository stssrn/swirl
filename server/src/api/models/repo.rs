use serde::Serialize;

use crate::entities;

#[derive(Debug, Serialize)]
pub struct Repo {
    name: String,
    repo: String,
    note: String,
    readme: Option<String>,
}

impl std::convert::From<entities::Repo> for Repo {
    fn from(repo: entities::Repo) -> Self {
        Self {
            name: repo.name,
            repo: repo.repo,
            note: repo.note,
            readme: repo.readme,
        }
    }
}