use std::{path::Path, sync::{Arc, Mutex}};
use tracing::error;

use crate::error::Error;

pub mod git;
pub mod models;
pub mod soft_serve;

pub struct Repository {
    pub repo: Arc<Mutex<git2::Repository>>,
}

impl Repository {
    pub fn open(path: &Path) -> Result<Self, Error> {
        match git2::Repository::open_bare(path) {
            Err(e) => {
                error!("Opening repo: {e}");
                Err(e.into())
            }
            Ok(repo) => {
                let repo = Arc::new(Mutex::new(repo));
                Ok(Self { repo })
            }
        }
    }
}
