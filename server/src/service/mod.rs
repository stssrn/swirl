use std::path::Path;
use tracing::error;

use crate::{repository::Repository, Error};

mod git;

pub struct Service {
    repo: Repository,
    name: String,
}

impl Service {
    pub fn new(root_path: &Path, repo_name: &str) -> Result<Self, Error> {
        match Repository::open(&root_path.join(repo_name)) {
            Err(e) => {
                error!("Failed to open repository {repo_name}: {e}");
                Err(e)
            },
            Ok(repo) =>  Ok(Self
                {
                    repo,
                    name: repo_name.to_string()
                }
            )
        }
    }
}
