use std::path::{Path, PathBuf};
use anyhow::Context;
use tracing::debug;

use crate::{
    service::Service,
    entities::{TreeNode, Commit, Repo, Oid},
    error::Error
};

impl Service {
    pub async fn get_repos(
        home_repo_path: &Path, page: Option<usize>, limit: Option<usize>
    ) -> Result<Vec<Repo>, Error> {
        let page = page.unwrap_or(0);
        let limit = limit.unwrap_or(usize::MAX);
        let offset = page * limit;
        debug!("returning repos (page {page}, limit {limit})");
        crate::Repository::get_all_repos(home_repo_path).await
            .map(|repos| {
                repos.into_iter()
                    .skip(offset)
                    .filter(|repo| !repo.private)
                    .take(limit)
                    .collect()
            })
    }

    pub async fn get_tree(
        &self, branch: Option<&str>, page: Option<usize>, limit: Option<usize>
    ) -> Result<TreeNode, Error> {
        if self.private {
            return Err(Error::Forbidden("Repo is private".to_string()))
        }

        let reference = branch.map(branch_to_ref).unwrap_or_else(|| "HEAD".to_string());
        let page = page.unwrap_or(0);
        let limit = limit.unwrap_or(50);
        let offset = page * limit;
        self.repo.get_tree(&reference, (offset, limit)).await
    }

    pub async fn get_branches(
        &self, page: Option<usize>, limit: Option<usize>
    ) -> Result<Vec<String>, Error> {
        if self.private {
            return Err(Error::Forbidden("Repo is private".to_string()))
        }

        let page = page.unwrap_or(0);
        let limit = limit.unwrap_or(20);
        let offset = page * limit;

        debug!("getting branches in {}", self.name);
        self.repo.get_branches((offset, limit)).await
    }

    pub async fn get_commits(
        &self, branch: Option<&str>, page: Option<usize>, limit: Option<usize>
    ) -> Result<Vec<Commit>, Error> {
        if self.private {
            return Err(Error::Forbidden("Repo is private".to_string()))
        }

        let reference = branch.map(branch_to_ref).unwrap_or_else(|| "HEAD".to_string());
        let page = page.unwrap_or(0);
        let limit = limit.unwrap_or(20);
        let start = page * limit;
        debug!("returning commits of {} in {reference} (page {page}, limit {limit})", self.name);
        self.repo.get_commits(&reference, (start, limit)).await
    }

    pub async fn get_commit(&self, branch: Option<&str>, oid: &Oid) -> Result<Commit, Error> {
        if self.private {
            return Err(Error::Forbidden("Repo is private".to_string()))
        }

        let reference = branch.map(branch_to_ref).unwrap_or_else(|| "HEAD".to_string());
        debug!("getting commit {} in repo {}", oid.to_string(), self.name);
        self.repo.get_commit(&reference, oid).await.ok_or_else(|| {
            tracing::error!("couldn't find commit {}", oid.to_string());
            Error::NotFound(format!("Couldn't find OID {}", oid.to_string()))
        })
    }

    pub async fn get_file_content(&self, path: &Path, branch: Option<&str>, home_repo_path: &Path) -> Result<Vec<u8>, Error> {
        let all_repos = crate::Repository::get_all_repos(home_repo_path).await?;
        let readme_path = all_repos.into_iter()
            .find_map(|repo| if repo.repo == self.name { repo.readme } else { None })
            .map(|path| PathBuf::from(&path)).context("couldn't find readme")?;

        if self.private && self.name != self.home_repo_name && path != readme_path {
            return Err(Error::Forbidden("Repo is private".to_string()))
        }

        let reference = branch.map(branch_to_ref).unwrap_or_else(|| "HEAD".to_string());
        debug!("getting file content of {path:?} in repo {} ({reference})", self.name);
        let oid = self.repo.get_file_id_from_path(path, &reference).await?;
        self.repo.get_object_content(&oid)
    }

    pub async fn is_file_binary(&self, target: &Path, branch: Option<&str>) -> Result<bool, Error> {
        if self.private {
            return Err(Error::Forbidden("Repo is private".to_string()))
        }

        let reference = branch.map(branch_to_ref).unwrap_or_else(|| "HEAD".to_string());
        let oid = self.repo.get_file_id_from_path(target, &reference).await?;
        self.repo.is_object_binary(&oid).await
    }

    pub async fn get_readme(&self, home_repo_path: &Path) -> Result<Vec<u8>, Error> {
        debug!("getting readme content of repo {}", self.name);
        let all_repos = crate::Repository::get_all_repos(home_repo_path).await?;
        let readme_path = all_repos.into_iter()
            .find_map(|repo| if repo.repo == self.name { repo.readme } else { None })
            .map(|path| PathBuf::from(&path)).context("couldn't find readme")?;
        debug!(?readme_path);
        self.get_file_content(&readme_path, None, home_repo_path).await
    }
}

fn branch_to_ref(branch: &str) -> String {
    format!("refs/heads/{branch}")
}
