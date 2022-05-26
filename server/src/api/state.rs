use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct AppState {
    pub home_repo: String,
    pub repo_path: PathBuf,
    pub allowed_origins: Vec<String>,
}

impl AppState {
    pub fn new(home_repo: &str, repo_path: &Path, allowed_origins: &[String]) -> Self {
        Self {
            home_repo: home_repo.to_owned(),
            repo_path: repo_path.to_owned(),
            allowed_origins: allowed_origins.to_owned()
        }
    }
}
