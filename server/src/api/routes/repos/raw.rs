use std::{collections::HashMap, sync::Arc};

use axum::{Extension, extract::{Query, Path}, http::StatusCode, Json};
use serde_json::Value;

use crate::{api::AppState, service::Service, Error};

// GET /repos/:repo/raw/content/*path
pub async fn get_file(
    Extension(state): Extension<Arc<AppState>>,
    Path((repo, path)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Vec<u8>, StatusCode> {
    let service = Service::new(&state.repo_path, &repo)?;
    let path = std::path::PathBuf::from(path);
    let branch = params.get("branch").map(String::as_ref);
    let file = service.get_file_content(&path, branch).await?;

    Ok(file)
}

// GET /repos/:repo/raw/is_bin/*path
pub async fn get_is_bin_file(
    Extension(state): Extension<Arc<AppState>>,
    Path((repo, path)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Value>, StatusCode> {
    let service = Service::new(&state.repo_path, &repo)?;
    let path = std::path::PathBuf::from(path);
    let branch = params.get("branch").map(String::as_ref);
    let is_bin = service.is_file_binary(&path, branch).await?;

    Ok(Json(serde_json::to_value(is_bin).map_err(Error::from)?))
}