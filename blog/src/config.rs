use std::env;

/// 应用配置。
///
/// 初学阶段直接从环境变量读取，部署时可以用 `.env` 文件或系统环境变量覆盖。
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
    pub admin_token: String,
    pub site_title: String,
    pub site_base_url: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            database_url: read_env("DATABASE_URL", "sqlite://blog.db"),
            bind_addr: read_env("BIND_ADDR", "127.0.0.1:3000"),
            admin_token: read_env("ADMIN_TOKEN", "change-me-local-admin-token"),
            site_title: read_env("SITE_TITLE", "Rust 本地博客"),
            site_base_url: read_env("SITE_BASE_URL", "http://127.0.0.1:3000"),
        })
    }
}

fn read_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}
