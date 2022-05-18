use std::{collections::HashMap, sync::Arc, str::FromStr};
use axum::{Extension, extract::{Query, Path}, http::StatusCode, Json};
use serde_json::Value;

use crate::{api::{AppState, models}, service::Service, Error, entities};

// GET /repos/:repo/commits
pub async fn get_commits(
    Extension(state): Extension<Arc<AppState>>,
    Path(repo): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, StatusCode> {
    let page = params.get("page").and_then(|page| page.parse::<usize>().ok());
    let limit = params.get("limit").and_then(|page| page.parse::<usize>().ok());
    let branch = params.get("branch").map(String::as_ref);

    let service = Service::new(&state.repo_path, &repo)?;
    let commits: Vec<models::CommitListItem> = service.get_commits(branch, page, limit).await?
        .into_iter()
        .map(models::CommitListItem::from)
        .collect();

    Ok(Json(serde_json::to_value(commits).map_err(Error::from)?))
}

// GET /repos/:repo/commits/:commit
pub async fn get_commit(
    Extension(state): Extension<Arc<AppState>>,
    Path((repo, id)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, StatusCode> {
    let branch = params.get("branch").map(String::as_ref);

    let service = Service::new(&state.repo_path, &repo)?;
    let id = entities::Oid::from_str(&id).map_err(Error::from)?;
    let commit: models::Commit = service.get_commit(branch, &id).await?.into();

    Ok(Json(serde_json::to_value(commit).map_err(Error::from)?))
}