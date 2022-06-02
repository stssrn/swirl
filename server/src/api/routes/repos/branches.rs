use std::{collections::HashMap, sync::Arc};
use axum::{Extension, extract::{Query, Path}, http::{StatusCode, header}, Json, response::IntoResponse, TypedHeader, headers::Host};

use crate::{api::{AppState, models}, service::Service, Error};

use super::{parse_page_queries, is_host_allowed};

// GET /repos/:repo/branches
pub async fn get_branches(
    Extension(state): Extension<Arc<AppState>>,
    Path(repo): Path<String>,
    Query(params): Query<HashMap<String, usize>>,
    TypedHeader(host): TypedHeader<Host>,
) -> Result<impl IntoResponse, StatusCode> {
    let (page, limit) = parse_page_queries(&params);

    let service = Service::new(&state.repo_path, &repo, &state.home_repo).await?;
    let branches = service.get_branches(page, limit).await?;

    let host_header = is_host_allowed(&state.allowed_origins, host.hostname());

    Ok((
        [(header::ACCESS_CONTROL_ALLOW_ORIGIN, host_header)],
        Json(serde_json::to_value(branches).map_err(Error::from)?)
    ))
}

// GET /repos/:repo/branches/:branch/tree
pub async fn get_branch_tree(
    Extension(state): Extension<Arc<AppState>>,
    Path((repo, branch)): Path<(String, String)>,
    Query(params): Query<HashMap<String, usize>>,
    TypedHeader(host): TypedHeader<Host>,
) -> Result<impl IntoResponse, StatusCode> {
    let (page, limit) = parse_page_queries(&params);

    let service = Service::new(&state.repo_path, &repo, &state.home_repo).await?;
    let tree: models::TreeEntry = service.get_tree(Some(&branch), page, limit).await?.into();

    let host_header = is_host_allowed(&state.allowed_origins, host.hostname());

    Ok((
        [(header::ACCESS_CONTROL_ALLOW_ORIGIN, host_header)],
        Json(serde_json::to_value(tree).map_err(Error::from)?)
    ))
}
