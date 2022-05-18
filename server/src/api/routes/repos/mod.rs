use std::{collections::HashMap, sync::Arc};
use axum::{Extension, extract::Query, http::StatusCode, Json};
use serde_json::Value;

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

// GET /repos
pub async fn get_repos(
    Extension(state): Extension<Arc<AppState>>,
    Query(params): Query<HashMap<String, usize>>,
) -> Result<Json<Value>, StatusCode> {
    let (page, limit) = parse_page_queries(&params);
    let home_repo_path = state.repo_path.join(&state.home_repo);

    let repos: Vec<models::Repo> = Service::get_repos(&home_repo_path, page, limit).await?
        .into_iter()
        .map(models::Repo::from)
        .collect();
    Ok(Json(serde_json::to_value(repos).map_err(Error::from)?))
}

fn parse_page_queries(params: &HashMap<String, usize>) -> (Option<usize>, Option<usize>) {
    let page = params.get("page").map(|page| page.to_owned());
    let limit = params.get("limit").map(|n| n.to_owned());
    (page, limit)
}