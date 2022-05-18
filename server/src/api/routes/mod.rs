use std::sync::Arc;

use axum::{
    routing::get,
    Router,
    extract::Extension,
};

use repos::{
    get_branch_tree,
    get_branches,
    get_commits,
    get_commit,
    get_file,
    get_is_bin_file,
    get_repos,
};

mod repos;

use crate::api::AppState;

pub fn routes(app_state: Arc<AppState>) -> Router {
    let repos = Router::new()
        .route("/", get(get_repos))
        .route("/:repo/branches", get(get_branches))
        .route("/:repo/branches/:branch/tree", get(get_branch_tree))
        .route("/:repo/commits", get(get_commits))
        .route("/:repo/commits/:id", get(get_commit))
        .route("/:repo/raw/content/*path", get(get_file))
        .route("/:repo/raw/is_bin/*path", get(get_is_bin_file));

    Router::new()
        .nest("/repos", repos)
        .layer(Extension(app_state))
}
