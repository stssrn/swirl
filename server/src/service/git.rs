use std::path::Path;
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
                    .take(limit)
                    .collect()
            })
    }

    pub async fn get_tree(
        &self, branch: Option<&str>, page: Option<usize>, limit: Option<usize>
    ) -> Result<TreeNode, Error> {
        let reference = branch.map(branch_to_ref).unwrap_or("HEAD".to_string());
        let page = page.unwrap_or(0);
        let limit = limit.unwrap_or(50);
        let offset = page * limit;
        self.repo.get_tree(&reference, (offset, limit)).await
    }

    pub async fn get_branches(
        &self, page: Option<usize>, limit: Option<usize>
    ) -> Result<Vec<String>, Error> {
        let page = page.unwrap_or(0);
        let limit = limit.unwrap_or(20);
        let offset = page * limit;

        debug!("getting branches in {}", self.name);
        self.repo.get_branches((offset, limit)).await
    }

    pub async fn get_commits(
        &self, branch: Option<&str>, page: Option<usize>, limit: Option<usize>
    ) -> Result<Vec<Commit>, Error> {
        let reference = branch.map(branch_to_ref).unwrap_or("HEAD".to_string());
        let page = page.unwrap_or(0);
        let limit = limit.unwrap_or(20);
        let start = page * limit;
        debug!("returning commits of {} in {reference} (page {page}, limit {limit})", self.name);
        self.repo.get_commits(&reference, (start, limit)).await
    }

    pub async fn get_commit(&self, branch: Option<&str>, oid: &Oid) -> Result<Commit, Error> {
        let reference = branch.map(branch_to_ref).unwrap_or("HEAD".to_string());
        debug!("getting commit {} in repo {}", oid.to_string(), self.name);
        self.repo.get_commit(&reference, oid).await.ok_or(Error::NotFound(format!("Couldn't find OID {}", oid.to_string())))
    }

    pub async fn get_file_content(&self, path: &Path, branch: Option<&str>) -> Result<Vec<u8>, Error> {
        let reference = branch.map(branch_to_ref).unwrap_or("HEAD".to_string());
        debug!("getting file content of {path:?} in repo {} ({reference})", self.name);
        let oid = self.repo.get_file_id_from_path(path, &reference).await?;
        self.repo.get_object_content(&oid)
    }

    pub async fn is_file_binary(&self, target: &Path, branch: Option<&str>) -> Result<bool, Error> {
        let reference = branch.map(branch_to_ref).unwrap_or("HEAD".to_string());
        let oid = self.repo.get_file_id_from_path(target, &reference).await?;
        self.repo.is_object_binary(&oid).await
    }
}

fn branch_to_ref(branch: &str) -> String {
    format!("refs/heads/{branch}")
}
