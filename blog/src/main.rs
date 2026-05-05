mod config;
mod db;
mod handlers;
mod middleware;
mod models;
mod utils;

use axum::Router;
use config::Config;
use db::connect;
use handlers::{admin, post, rss};
use std::sync::Arc;
use tera::Tera;
use tower_http::{services::ServeDir, trace::TraceLayer};

const PROJECT_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: sqlx::SqlitePool,
    pub templates: Arc<Tera>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename(format!("{PROJECT_DIR}/.env")).ok();
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter("blog=debug,tower_http=debug")
        .init();

    let config = Config::from_env()?;
    let pool = connect(&config.database_url).await?;
    let templates = Arc::new(Tera::new(&format!("{PROJECT_DIR}/templates/**/*.html"))?);

    let state = AppState {
        config: config.clone(),
        pool,
        templates,
    };

    let app = Router::new()
        .route("/", axum::routing::get(post::index))
        .route("/posts/:slug", axum::routing::get(post::show))
        .route(
            "/posts/:slug/comments",
            axum::routing::post(post::create_comment),
        )
        .route("/rss.xml", axum::routing::get(rss::feed))
        .merge(admin::routes(state.clone()))
        .nest_service("/static", ServeDir::new(format!("{PROJECT_DIR}/static")))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&config.bind_addr).await?;
    tracing::info!("blog server listening on http://{}", config.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
