use crate::{models::post::Post, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse};

pub async fn feed(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let posts = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts WHERE published = 1 ORDER BY created_at DESC LIMIT 20",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|error| {
        tracing::error!(%error, "failed to build rss feed");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let items = posts
        .iter()
        .map(|post| {
            format!(
                "<item><title>{}</title><link>{}/posts/{}</link><description>{}</description><pubDate>{}</pubDate></item>",
                escape_xml(&post.title),
                state.config.site_base_url,
                escape_xml(&post.slug),
                escape_xml(&post.summary),
                escape_xml(&post.created_at),
            )
        })
        .collect::<Vec<_>>()
        .join("");

    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<rss version="2.0">
  <channel>
    <title>{}</title>
    <link>{}</link>
    <description>个人本地 Rust 博客</description>
    {}
  </channel>
</rss>"#,
        escape_xml(&state.config.site_title),
        escape_xml(&state.config.site_base_url),
        items
    );

    Ok((
        [("content-type", "application/rss+xml; charset=utf-8")],
        xml,
    ))
}

fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
