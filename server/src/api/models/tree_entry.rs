use serde::Serialize;

use crate::entities;

#[derive(Debug, Serialize)]
pub struct TreeEntry {
    name: String,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none", rename = "entries")]
    entries: Option<Vec<TreeEntry>>,
}

impl std::convert::From<entities::TreeNode> for TreeEntry {
    fn from(node: entities::TreeNode) -> Self {
        let entries = node.children
            .map(|nodes| {
                nodes.into_iter()
                    .map(Self::from)
                    .collect()
            });

        Self {
            name: node.name,
            id: node.oid.to_string(),
            entries
        }
    }
}