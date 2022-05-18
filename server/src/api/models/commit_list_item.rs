use serde::Serialize;

use crate::entities;

#[derive(Debug, Serialize)]
pub struct CommitListItem {
    id: String,
    message: String,
}

impl std::convert::From<entities::Commit> for CommitListItem {
    fn from(commit: entities::Commit) -> Self {
        Self {
            id: commit.oid.to_string(),
            message: commit.summary,
        }
    }
}