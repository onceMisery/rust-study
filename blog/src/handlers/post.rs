use crate::{
    handlers::render,
    models::{
        comment::{Comment, CommentForm},
        post::Post,
    },
    utils::markdown::render_markdown,
    AppState,
};
use axum::{
    extract::{Form, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use tera::Context;

pub async fn index(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let posts = sqlx::query_as::<_, Post>(
        "SELECT * FROM posts WHERE published = 1 ORDER BY created_at DESC",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|error| {
        tracing::error!(%error, "failed to list posts");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut context = Context::new();
    context.insert("site_title", &state.config.site_title);
    context.insert("posts", &posts);
    render(&state.templates, "index.html", &context)
}

pub async fn show(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE slug = ? AND published = 1")
        .bind(&slug)
        .fetch_optional(&state.pool)
        .await
        .map_err(|error| {
            tracing::error!(%error, slug, "failed to load post");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let comments = sqlx::query_as::<_, Comment>(
        "SELECT * FROM comments WHERE post_id = ? ORDER BY created_at",
    )
    .bind(post.id)
    .fetch_all(&state.pool)
    .await
    .map_err(|error| {
        tracing::error!(%error, post_id = post.id, "failed to list comments");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut context = Context::new();
    context.insert("site_title", &state.config.site_title);
    context.insert("post", &post);
    context.insert("post_html", &render_markdown(&post.markdown));
    context.insert("comments", &comments);
    render(&state.templates, "post.html", &context)
}

pub async fn create_comment(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Form(form): Form<CommentForm>,
) -> Result<impl IntoResponse, StatusCode> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE slug = ? AND published = 1")
        .bind(&slug)
        .fetch_optional(&state.pool)
        .await
        .map_err(|error| {
            tracing::error!(%error, slug, "failed to load post for comment");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    if form.author.trim().is_empty() || form.body.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    sqlx::query("INSERT INTO comments (post_id, author, body) VALUES (?, ?, ?)")
        .bind(post.id)
        .bind(form.author.trim())
        .bind(form.body.trim())
        .execute(&state.pool)
        .await
        .map_err(|error| {
            tracing::error!(%error, post_id = post.id, "failed to create comment");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Redirect::to(&format!("/posts/{slug}")))
}
