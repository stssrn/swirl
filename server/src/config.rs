use crate::error::Error;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub home_repo: String,
    pub repo_path: PathBuf,
}

const ENV_PORT: &str = "SWIRL_PORT";
const ENV_HOME_REPO: &str = "SOFT_SERVE_HOME_REPO";
const ENV_REPO_PATH: &str = "SOFT_SERVE_REPO_PATH";

const DEFAULT_PORT: u16 = 34342;

// Default's below are the same as Soft Serve's defaults.
const DEFAULT_HOME_REPO: &str = "config";
const DEFAULT_REPO_PATH: &str = "./repos";

impl Config {
    pub fn load() -> Result<Self, Error> {
        dotenv::dotenv().ok();

        let port = std::env::var(ENV_PORT)
            .ok()
            .map_or(Ok(DEFAULT_PORT), |env_var| env_var.parse::<u16>())?;

        let home_repo = std::env::var(ENV_HOME_REPO)
            .unwrap_or_else(|_| DEFAULT_HOME_REPO.into());

        let repo_path = std::env::var(ENV_REPO_PATH)
            .unwrap_or_else(|_| DEFAULT_REPO_PATH.into());

        let repo_path = if let Some(home_dir) = dirs::home_dir() {
            let home_dir = home_dir.to_str().unwrap();
            repo_path
                .replace("$HOME", home_dir)
                .replace('~', home_dir)
        } else { repo_path };

        let repo_path = PathBuf::from(repo_path);

        Ok( Config { port, home_repo, repo_path })
    }
}
