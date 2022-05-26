use std::{collections::HashMap, sync::Arc};

use axum::{Extension, extract::{Query, Path}, http::{StatusCode, header}, Json, response::IntoResponse, TypedHeader, headers::Host};

use crate::{api::AppState, service::Service, Error};

use super::is_host_allowed;

// GET /repos/:repo/raw/content/*path
pub async fn get_file(
    Extension(state): Extension<Arc<AppState>>,
    Path((repo, path)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    TypedHeader(host): TypedHeader<Host>,
) -> Result<impl IntoResponse, StatusCode> {
    let service = Service::new(&state.repo_path, &repo)?;
    let path = std::path::PathBuf::from(path);
    let branch = params.get("branch").map(String::as_ref);
    let file = service.get_file_content(&path, branch).await?;

    let host_header = is_host_allowed(&state.allowed_origins, host.hostname());

    Ok((
        [(header::ACCESS_CONTROL_ALLOW_ORIGIN, host_header)],
        file
    ))
}

// GET /repos/:repo/raw/is_bin/*path
pub async fn get_is_bin_file(
    Extension(state): Extension<Arc<AppState>>,
    Path((repo, path)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    TypedHeader(host): TypedHeader<Host>,
) -> Result<impl IntoResponse, StatusCode> {
    let service = Service::new(&state.repo_path, &repo)?;
    let path = std::path::PathBuf::from(path);
    let branch = params.get("branch").map(String::as_ref);
    let is_bin = service.is_file_binary(&path, branch).await?;

    let host_header = is_host_allowed(&state.allowed_origins, host.hostname());

    Ok((
        [(header::ACCESS_CONTROL_ALLOW_ORIGIN, host_header)],
        Json(serde_json::to_value(is_bin).map_err(Error::from)?)
    ))
}
