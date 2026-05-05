CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    summary TEXT NOT NULL DEFAULT '',
    markdown TEXT NOT NULL,
    published INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS comments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    post_id INTEGER NOT NULL,
    author TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (post_id) REFERENCES posts(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_posts_slug ON posts(slug);
CREATE INDEX IF NOT EXISTS idx_posts_published_created_at ON posts(published, created_at);
CREATE INDEX IF NOT EXISTS idx_comments_post_id ON comments(post_id);

INSERT OR IGNORE INTO posts (id, title, slug, summary, markdown, published)
VALUES (
    1,
    'Hello Rust Blog',
    'hello-rust-blog',
    '第一篇本地博客文章，介绍这个 Axum + SQLite 示例。',
    '# Hello Rust Blog

这是第一篇文章。

- 使用 **Axum** 提供 Web 路由
- 使用 **SQLx + SQLite** 保存文章和评论
- 使用 **Tera** 渲染 HTML
- 使用 **pulldown-cmark** 渲染 Markdown',
    1
);
