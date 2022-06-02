use std::{collections::HashMap, sync::Arc};
use axum::TypedHeader;
use axum::http::header;
use axum::headers::Host;
use axum::response::IntoResponse;
use axum::{Extension, extract::Query, http::StatusCode, Json};

use crate::api::models;
use crate::{api::AppState, service::Service, Error};

mod branches;
mod commits;
mod raw;

pub use branches::get_branch_tree;
pub use branches::get_branches;
pub use commits::get_commits;
pub use commits::get_commit;
pub use raw::get_file;
pub use raw::get_is_bin_file;
pub use raw::get_readme_file;

// GET /repos
pub async fn get_repos(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<HashMap<String, usize>>,
    TypedHeader(host): TypedHeader<Host>,
) -> Result<impl IntoResponse, StatusCode> {
    let (page, limit) = parse_page_queries(&params);
    let home_repo_path = state.repo_path.join(&state.home_repo);

    let repos: Vec<models::Repo> = Service::get_repos(&home_repo_path, page, limit).await?
        .into_iter()
        .map(models::Repo::from)
        .collect();

    let host_header = is_host_allowed(&state.allowed_origins, host.hostname());

    Ok((
        [(header::ACCESS_CONTROL_ALLOW_ORIGIN, host_header)],
        Json(serde_json::to_value(repos).map_err(Error::from)?)
    ))
}

fn parse_page_queries(params: &HashMap<String, usize>) -> (Option<usize>, Option<usize>) {
    let page = params.get("page").map(|page| page.to_owned());
    let limit = params.get("limit").map(|n| n.to_owned());
    (page, limit)
}

fn is_host_allowed(allowed_hosts: &[String], request_host: &str) -> String {
    allowed_hosts.iter()
        .find(|&host| {
            host == "*" || host == request_host
        })
        .map(|host| {
            tracing::debug!("host {host} is allowed");
            host.to_string()
        })
        .unwrap_or_else(|| {
            tracing::warn!("host \"{request_host}\" is not in allowed hosts {allowed_hosts:?}");
            String::new()
        })
}
