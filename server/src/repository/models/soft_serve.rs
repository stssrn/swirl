use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub repos: Vec<Repo>,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    pub name: String,
    pub repo: String,
    pub private: bool,
    pub note: String,
    pub readme: Option<String>,
}

impl std::convert::From<Repo> for crate::entities::Repo {
    fn from(repo: Repo) -> Self {
        Self {
            name: repo.name,
            repo: repo.repo,
            note: repo.note,
            readme: repo.readme
        }
    }
}
