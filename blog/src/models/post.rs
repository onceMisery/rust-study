use serde::Serialize;
use sqlx::FromRow;

/// 数据库中的文章。
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub markdown: String,
    pub published: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// 创建或更新文章时使用的表单。
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PostForm {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub markdown: String,
    pub published: Option<String>,
}

impl PostForm {
    pub fn published_flag(&self) -> bool {
        self.published.as_deref() == Some("on")
    }
}
