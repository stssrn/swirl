use std::path::{Path, PathBuf};

use anyhow::Context;
use tracing::trace;

use crate::{entities::{self, Oid, TreeNode}, Error};
use super::{models, Repository};

impl Repository {
    pub async fn get_all_repos(
        home_repo_path: &Path
    ) -> Result<Vec<entities::Repo>, Error> {
        let home_repo = Repository::open(home_repo_path)?;

        let config: models::soft_serve::Config = home_repo
            .get_tree("HEAD", (0, usize::MAX)).await?.children.unwrap()
            .iter()
            .find_map(|entry|  {
                if entry.name == "config.yaml" {
                    let oid = &entry.oid;
                    let object = &home_repo.get_object_content(oid).unwrap();
                    let config: models::soft_serve::Config = serde_yaml::from_slice(object).ok()?;
                    Some(config)
                } else { None }
            }).unwrap();
        let repos: Vec<entities::Repo> = config.repos.into_iter().map(|repo| repo.into()).collect();
        Ok(repos)
    }

    pub async fn get_file_id_from_path(&self, target: &Path, reference: &str) -> Result<Oid, Error> {
        let tree = self.get_tree(reference, (0, usize::MAX)).await?;

        let oid = find_tree(&tree, target).context("Couldn't find target")?.oid.to_owned();
        trace!(?oid);
        Ok(oid)
    }
}


fn find_tree<'a>(tree: &'a TreeNode, target_path: &Path) -> Option<&'a TreeNode> {
    let target_path = target_path.strip_prefix("/").unwrap_or(target_path);
    trace!("target {target_path:?}");
    let target = target_path.iter().next()?.to_str()?;


    if tree.children.is_none() && tree.name == target {
        return Some(tree)
    }

    if let Some(tree) = tree.children.as_ref()?.iter().find(|node| node.name == target) {
        match tree.children {
            None => Some(tree),
            Some(_) => find_tree(tree, &target_path.iter().skip(1).collect::<PathBuf>())
        }
    } else {
        tracing::error!("Didn't find target {target:?}");
        None
    }
}
