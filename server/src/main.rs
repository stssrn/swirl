use std::{net::SocketAddr, sync::Arc};
use anyhow::Result;
use tracing::info;

mod api;
mod config;
mod entities;
mod error;
mod repository;
mod service;

use config::Config;
use repository::Repository;
pub use error::Error;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::load()?;
    let app_state = Arc::new(api::AppState::new(&config.home_repo, &config.repo_path, &config.allowed_origins));
    let routes = api::routes::routes(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], config.port));
    info!("starting server on http://{addr} in directory {:?}", config.repo_path);

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await?;
    Ok(())
}
