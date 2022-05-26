use std::{collections::HashMap, sync::Arc, str::FromStr};
use axum::{Extension, extract::{Query, Path}, http::{StatusCode, header}, Json, TypedHeader, headers::Host, response::IntoResponse};

use crate::{api::{AppState, models}, service::Service, Error, entities};

use super::is_host_allowed;

// GET /repos/:repo/commits
pub async fn get_commits(
    Extension(state): Extension<Arc<AppState>>,
    Path(repo): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    TypedHeader(host): TypedHeader<Host>,
) -> Result<impl IntoResponse, StatusCode> {
    let page = params.get("page").and_then(|page| page.parse::<usize>().ok());
    let limit = params.get("limit").and_then(|limit| limit.parse::<usize>().ok());
    let branch = params.get("branch").map(String::as_ref);

    let service = Service::new(&state.repo_path, &repo)?;
    let commits: Vec<models::CommitListItem> = service.get_commits(branch, page, limit).await?
        .into_iter()
        .map(models::CommitListItem::from)
        .collect();

    let host_header = is_host_allowed(&state.allowed_origins, host.hostname());

    Ok((
        [(header::ACCESS_CONTROL_ALLOW_ORIGIN, host_header)],
        Json(serde_json::to_value(commits).map_err(Error::from)?)
    ))
}

// GET /repos/:repo/commits/:commit
pub async fn get_commit(
    Extension(state): Extension<Arc<AppState>>,
    Path((repo, id)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    TypedHeader(host): TypedHeader<Host>,
) -> Result<impl IntoResponse, StatusCode> {
    let branch = params.get("branch").map(String::as_ref);

    let service = Service::new(&state.repo_path, &repo)?;
    let id = entities::Oid::from_str(&id).map_err(Error::from)?;
    let commit: models::Commit = service.get_commit(branch, &id).await?.into();

    let host_header = is_host_allowed(&state.allowed_origins, host.hostname());

    Ok((
        [(header::ACCESS_CONTROL_ALLOW_ORIGIN, host_header)],
        Json(serde_json::to_value(commit).map_err(Error::from)?)
    ))
}
