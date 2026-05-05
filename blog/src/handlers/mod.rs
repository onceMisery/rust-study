pub mod admin;
pub mod post;
pub mod rss;

use axum::{http::StatusCode, response::Html};
use tera::{Context, Tera};

pub fn render(templates: &Tera, name: &str, context: &Context) -> Result<Html<String>, StatusCode> {
    templates.render(name, context).map(Html).map_err(|error| {
        tracing::error!(%error, "template render failed");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
