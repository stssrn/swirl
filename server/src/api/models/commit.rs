use serde::Serialize;

use crate::entities;

#[derive(Debug, Serialize)]
pub struct Commit {
    author: Author,
    timestamp: i64,
    message: String,
    // diff: String
}

impl std::convert::From<entities::Commit> for Commit {
    fn from(commit: entities::Commit) -> Self {
        Self {
            author: commit.author.into(),
            timestamp: commit.timestamp,
            message: commit.summary,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

impl std::convert::From<entities::Author> for Author {
    fn from(author: entities::Author) -> Self {
        Self {
            name: author.name,
            email: author.email,
        }
    }
}