use crate::AppState;
use axum::{
    body::Body,
    extract::State,
    http::{header, HeaderMap, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

const AUTH_COOKIE: &str = "blog_admin_token";

/// 管理后台认证中间件。
///
/// 这个示例使用本地 token cookie，适合个人本地部署学习。
/// 真实公网系统应使用 HTTPS、密码哈希、CSRF 防护和更完善的 session 管理。
pub async fn require_admin(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> Response {
    if has_valid_cookie(&headers, &state.config.admin_token) {
        next.run(request).await
    } else {
        Redirect::to("/admin/login").into_response()
    }
}

pub fn login_cookie(token: &str) -> String {
    format!("{AUTH_COOKIE}={token}; Path=/; HttpOnly; SameSite=Lax")
}

pub fn logout_cookie() -> String {
    format!("{AUTH_COOKIE}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax")
}

pub fn validate_login(input: &str, expected: &str) -> Result<(), StatusCode> {
    if input == expected {
        Ok(())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn has_valid_cookie(headers: &HeaderMap, expected: &str) -> bool {
    headers
        .get(header::COOKIE)
        .and_then(|value| value.to_str().ok())
        .map(|cookies| {
            cookies
                .split(';')
                .map(str::trim)
                .any(|cookie| cookie == format!("{AUTH_COOKIE}={expected}"))
        })
        .unwrap_or(false)
}
