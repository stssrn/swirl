use std::path::Path;
use anyhow::Context;
use tracing::error;

use crate::{repository::Repository, Error};

mod git;

pub struct Service {
    repo: Repository,
    name: String,
    home_repo_name: String,
    private: bool,
}

impl Service {
    pub async fn new(root_path: &Path, repo_name: &str, home_repo_name: &str) -> Result<Self, Error> {
        match Repository::open(&root_path.join(repo_name)) {
            Err(e) => {
                error!("Failed to open repository {repo_name}: {e}");
                Err(e)
            },
            Ok(repo) =>  {
                let home_repo_path = root_path.join(home_repo_name);
                let private = Repository::get_all_repos(&home_repo_path).await?
                    .into_iter()
                    .find_map(|repo| if repo.repo == repo_name { Some(repo.private) } else { None })
                    .context("couldn't determine wether repo is private or not")?;

                Ok(Self
                    {
                        repo,
                        name: repo_name.to_string(),
                        home_repo_name: home_repo_name.to_string(),
                        private
                    }
                )
            }
        }
    }
}
