use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct AppState {
    pub home_repo: String,
    pub repo_path: PathBuf,
}

impl AppState {
    pub fn new(home_repo: &str, repo_path: &Path) -> Self {
        Self { 
            home_repo: home_repo.to_owned(),
            repo_path: repo_path.to_owned()
        }
    }
}
