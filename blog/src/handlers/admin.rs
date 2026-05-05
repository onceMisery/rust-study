use crate::{
    handlers::render,
    middleware::auth::{login_cookie, logout_cookie, require_admin, validate_login},
    models::post::{Post, PostForm},
    AppState,
};
use axum::{
    extract::{Form, Path, State},
    http::{header, HeaderMap, StatusCode},
    middleware,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use tera::Context;

pub fn routes(state: AppState) -> Router<AppState> {
    let protected = Router::new()
        .route("/admin", get(dashboard))
        .route("/admin/posts/new", get(new_post_form))
        .route("/admin/posts", post(create_post))
        .route("/admin/posts/:id/edit", get(edit_post_form))
        .route("/admin/posts/:id", post(update_post))
        .route("/admin/posts/:id/delete", post(delete_post))
        .route_layer(middleware::from_fn_with_state(state, require_admin));

    Router::new()
        .route("/admin/login", get(login_form).post(login))
        .route("/admin/logout", post(logout))
        .merge(protected)
}

pub async fn dashboard(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY created_at DESC")
        .fetch_all(&state.pool)
        .await
        .map_err(|error| {
            tracing::error!(%error, "failed to list admin posts");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let mut context = Context::new();
    context.insert("site_title", &state.config.site_title);
    context.insert("posts", &posts);
    render(&state.templates, "admin/dashboard.html", &context)
}

pub async fn new_post_form(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    context.insert("site_title", &state.config.site_title);
    context.insert("mode", "new");
    render(&state.templates, "admin/new.html", &context)
}

pub async fn create_post(
    State(state): State<AppState>,
    Form(form): Form<PostForm>,
) -> Result<impl IntoResponse, StatusCode> {
    sqlx::query(
        "INSERT INTO posts (title, slug, summary, markdown, published) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(form.title.trim())
    .bind(form.slug.trim())
    .bind(form.summary.trim())
    .bind(form.markdown.trim())
    .bind(form.published_flag())
    .execute(&state.pool)
    .await
    .map_err(|error| {
        tracing::error!(%error, "failed to create post");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Redirect::to("/admin"))
}

pub async fn edit_post_form(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, StatusCode> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|error| {
            tracing::error!(%error, id, "failed to load admin post");
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    let mut context = Context::new();
    context.insert("site_title", &state.config.site_title);
    context.insert("post", &post);
    render(&state.templates, "admin/edit.html", &context)
}

pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<PostForm>,
) -> Result<impl IntoResponse, StatusCode> {
    sqlx::query(
        "UPDATE posts
         SET title = ?, slug = ?, summary = ?, markdown = ?, published = ?, updated_at = datetime('now')
         WHERE id = ?",
    )
    .bind(form.title.trim())
    .bind(form.slug.trim())
    .bind(form.summary.trim())
    .bind(form.markdown.trim())
    .bind(form.published_flag())
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|error| {
        tracing::error!(%error, id, "failed to update post");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Redirect::to("/admin"))
}

pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, StatusCode> {
    sqlx::query("DELETE FROM posts WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|error| {
            tracing::error!(%error, id, "failed to delete post");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Redirect::to("/admin"))
}

pub async fn login_form(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let mut context = Context::new();
    context.insert("site_title", &state.config.site_title);
    render(&state.templates, "admin/login.html", &context)
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    token: String,
}

pub async fn login(
    State(state): State<AppState>,
    Form(form): Form<LoginForm>,
) -> Result<impl IntoResponse, StatusCode> {
    validate_login(&form.token, &state.config.admin_token)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        login_cookie(&state.config.admin_token)
            .parse()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    Ok((headers, Redirect::to("/admin")))
}

pub async fn logout() -> Result<impl IntoResponse, StatusCode> {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::SET_COOKIE,
        logout_cookie()
            .parse()
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );
    Ok((headers, Redirect::to("/")))
}
