use tracing::error;

use crate::{
    Error,
    entities::{Commit, TreeNode, Oid},
};

use super::Repository;

impl Repository {
    pub async fn get_tree(
        &self, reference: &str, (offset, limit): (usize, usize),
    ) -> Result<TreeNode, Error> {
        let repo = self.repo.lock()?;
        let tree = repo
            .revparse_single(reference)?
            .peel_to_tree()?;
        let oid: Oid = tree.id().into();
        let children = tree
            .iter()
            .skip(offset)
            .map(|node| {
                parse_tree_node(
                    &repo,
                    node.to_object(&repo).unwrap(),
                    node.name().unwrap_or("invalid_name")
                )
            })
            .take(limit)
            .collect::<Vec<TreeNode>>();

        Ok(TreeNode {
            name: "/".to_string(),
            oid,
            children: Some(children),
        })
    }

    pub async fn get_branches(&self, (offset, limit): (usize, usize)) -> Result<Vec<String>, Error> {
        let repo = self.repo.lock()?;
        let branches = repo.branches(None)?
            .filter_map(Result::ok)
            .skip(offset)
            .map(|(branch, _)| {
                branch.name()
                    .map(|name| name.unwrap_or("[INVALID UTF-8 STRING]"))
                    .unwrap()
                    .to_string()
            })
            .take(limit)
            .collect::<Vec<_>>();

            Ok(branches)
    }

    pub async fn get_commits(
        &self, reference: &str, (offset, limit): (usize, usize)
    ) -> Result<Vec<Commit>, Error>{
        let repo = self.repo.lock()?;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_ref(reference)?;
        revwalk.set_sorting(git2::Sort::TIME)?;
        let commits: Vec<Commit> = revwalk
            .filter_map(|oid| {
                repo.find_object(oid.ok()?, None)
                    .ok()?
                    .into_commit()
                    .ok()
            })
            .skip(offset)
            .map(|commit| commit.into())
            .take(limit)
            .collect();

        Ok(commits)
    }

    pub async fn get_commit(&self, reference: &str, oid: &Oid) -> Option<Commit> {
        self.get_commits(reference, (0, usize::MAX)).await.map(|commits| {
            commits.into_iter()
                .find(|commit| &commit.oid == oid)
        }).ok().flatten()
    }

    pub async fn get_commit_signature(&self, commit: &Commit) -> Option<String> {
        let repo = self.repo.lock().ok()?;
        let oid = git2::Oid::from_bytes(commit.oid.as_bytes()).ok()?;
        let signature = repo.extract_signature(&oid, None).ok()?;
        Some(signature.0.as_str()?.to_string())
    }

    pub fn get_object_content(&self, oid: &Oid) -> Result<Vec<u8>, Error> {
        Ok(self.repo.lock()?.find_blob(oid.to_owned().into())?.content().to_vec())
    }

    pub async fn is_object_binary(&self, oid: &Oid) -> Result<bool, Error> {
        Ok(self.repo.lock()?.find_blob(oid.to_owned().into())?.is_binary())
    }
}


fn parse_tree_node(repo: &git2::Repository, object: git2::Object, name: &str ) -> TreeNode {
    if let Some(kind) = object.kind() {
        match kind {
            git2::ObjectType::Blob => {
                TreeNode {
                    name: name.to_string(),
                    oid: object.id().into(),
                    children: None,
                }
            }
            git2::ObjectType::Tree => {
                let children: Vec<TreeNode> = object.as_tree().unwrap() // Can't fail
                    .iter()
                    .map(|node| {
                        parse_tree_node(
                            repo,
                            node.to_object(repo).unwrap(),
                            node.name().unwrap_or("invalid_name")
                    )})
                    .collect();

                TreeNode {
                    name: name.to_string(),
                    oid: object.id().into(),
                    children: Some(children)
                }
            }
            _ => {
                error!("Invalid tree node {kind}");
                panic!("oops")
            }
        }
    } else {
        panic!("uh uh idk")
    }
}

impl std::convert::From<Oid> for git2::Oid {
    fn from(oid: Oid) -> Self {
        git2::Oid::from_bytes(oid.as_bytes()).unwrap()
    }
}
