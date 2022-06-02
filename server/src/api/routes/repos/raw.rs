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
    let service = Service::new(&state.repo_path, &repo, &state.home_repo).await?;
    let path = std::path::PathBuf::from(path);
    let branch = params.get("branch").map(String::as_ref);
    let home_repo_path = state.repo_path.join(&state.home_repo);
    let file = service.get_file_content(&path, branch, &home_repo_path).await?;

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
    let service = Service::new(&state.repo_path, &repo, &state.home_repo).await?;
    let path = std::path::PathBuf::from(path);
    let branch = params.get("branch").map(String::as_ref);
    let is_bin = service.is_file_binary(&path, branch).await?;

    let host_header = is_host_allowed(&state.allowed_origins, host.hostname());

    Ok((
        [(header::ACCESS_CONTROL_ALLOW_ORIGIN, host_header)],
        Json(serde_json::to_value(is_bin).map_err(Error::from)?)
    ))
}

// GET /repos/:repo/raw/readme
pub async fn get_readme_file(
    Extension(state): Extension<Arc<AppState>>,
    Path(repo): Path<String>,
    TypedHeader(host): TypedHeader<Host>,
) -> Result<impl IntoResponse, StatusCode> {
    let service = Service::new(&state.repo_path, &repo, &state.home_repo).await?;
    let home_repo = state.repo_path.join(&state.home_repo);
    let readme_file = service.get_readme(&home_repo).await?;

    let host_header = is_host_allowed(&state.allowed_origins, host.hostname());

    Ok((
        [(header::ACCESS_CONTROL_ALLOW_ORIGIN, host_header)],
        readme_file
    ))
}
