use std::{collections::HashMap, sync::Arc};
use axum::{Extension, extract::{Query, Path}, http::StatusCode, Json};
use serde_json::Value;

use crate::{api::{AppState, models}, service::Service, Error};

use super::parse_page_queries;

// GET /repos/:repo/branches
pub async fn get_branches(
    Extension(state): Extension<Arc<AppState>>,
    Path(repo): Path<String>,
    Query(params): Query<HashMap<String, usize>>,
) -> Result<Json<Value>, StatusCode> {
    let (page, limit) = parse_page_queries(&params);

    let service = Service::new(&state.repo_path, &repo)?;
    let branches = service.get_branches(page, limit).await?;

    Ok(Json(serde_json::to_value(branches).map_err(Error::from)?))
}

// GET /repos/:repo/branches/:branch/tree
pub async fn get_branch_tree(
    Extension(state): Extension<Arc<AppState>>,
    Path((repo, branch)): Path<(String, String)>,
    Query(params): Query<HashMap<String, usize>>,
) -> Result<Json<Value>, StatusCode> {
    let (page, limit) = parse_page_queries(&params);

    let service = Service::new(&state.repo_path, &repo)?;
    let tree: models::TreeEntry = service.get_tree(Some(&branch), page, limit).await?.into();

    Ok(Json(serde_json::to_value(tree).map_err(Error::from)?))
}