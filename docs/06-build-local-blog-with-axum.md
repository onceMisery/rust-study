# 从零构建本地博客系统：Axum + SQLx + Tera + SQLite

这一节实现一个个人本地部署的博客系统。它比前面的任务看板更接近真实 Web 项目：有 HTTP 路由、HTML 模板、SQLite 数据库、SQL 迁移、Markdown 渲染、后台管理、简单认证、评论和 RSS。

对应目录：

```text
blog/
├── Cargo.toml
├── .env
├── migrations/
│   └── 001_init.sql
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── db.rs
│   ├── models/
│   ├── handlers/
│   ├── middleware/
│   └── utils/
├── templates/
├── static/
└── content/
```

运行：

```powershell
cargo run -p blog
```

浏览器访问：

```text
http://127.0.0.1:3000
http://127.0.0.1:3000/admin/login
```

默认管理 token 在 `blog/.env`：

```text
ADMIN_TOKEN=change-me-local-admin-token
```

## 1. 项目目标

我们要实现：

- 首页文章列表。
- 文章详情页。
- 评论提交。
- RSS 输出。
- 管理后台登录。
- 新建、编辑、删除文章。
- Markdown 渲染 HTML。
- SQLite 文件数据库。
- SQL migrations 初始化表结构。

暂不实现：

- 多用户账号系统。
- 图片上传。
- 复杂权限。
- 公网安全加固。
- 全文搜索。

这是故意控制范围。初学者第一步应该先掌握 Web 项目主干：请求进入 Axum，handler 查询数据库，Tera 渲染 HTML，浏览器拿到响应。

## 2. 依赖选择

`blog/Cargo.toml`：

```toml
[dependencies]
anyhow = "1"
axum = "0.7"
chrono = { version = "0.4", features = ["serde"] }
dotenvy = "0.15"
pulldown-cmark = "0.10"
serde = { version = "1", features = ["derive"] }
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite", "chrono"] }
tera = "1.20"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["fs", "trace"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

每个依赖的作用：

| crate | 用途 |
| --- | --- |
| `axum` | Web 框架，负责路由、提取参数、返回响应 |
| `tokio` | 异步运行时，Axum 和 SQLx 都运行在 Tokio 上 |
| `sqlx` | 异步数据库访问，这里使用 SQLite |
| `tera` | HTML 模板引擎 |
| `pulldown-cmark` | Markdown 转 HTML |
| `dotenvy` | 读取 `.env` 环境变量 |
| `tower-http` | 静态文件服务和 HTTP trace |
| `tracing` | 日志 |
| `serde` | 表单和模板数据序列化 |
| `anyhow` | main 函数里简化错误传播 |

## 3. 配置管理

`src/config.rs` 负责读取环境变量：

```rust
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub bind_addr: String,
    pub admin_token: String,
    pub site_title: String,
    pub site_base_url: String,
}
```

为什么不把配置写死在代码里？

- 本地运行、测试、部署可能使用不同端口。
- 数据库文件位置可能不同。
- 管理 token 不能写死在源码里。
- 网站标题和基础 URL 应该可配置。

`.env` 示例：

```text
DATABASE_URL=sqlite://blog.db
BIND_ADDR=127.0.0.1:3000
ADMIN_TOKEN=change-me-local-admin-token
SITE_TITLE=Rust 本地博客
SITE_BASE_URL=http://127.0.0.1:3000
```

## 4. 数据库与迁移

`migrations/001_init.sql` 创建两张表：

```sql
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
```

`posts` 保存文章，`comments` 保存评论。`slug` 用于 URL，比如 `/posts/hello-rust-blog`。

`src/db.rs`：

```rust
static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn connect(database_url: &str) -> anyhow::Result<SqlitePool> {
    let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    MIGRATOR.run(&pool).await?;
    Ok(pool)
}
```

启动时会自动：

1. 创建 SQLite 文件。
2. 建立连接池。
3. 执行 migrations。

## 5. 模型层

文章模型：

```rust
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
```

`FromRow` 让 SQLx 能把查询结果转成结构体。`Serialize` 让 Tera 能读取字段。

表单模型：

```rust
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PostForm {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub markdown: String,
    pub published: Option<String>,
}
```

HTML checkbox 未选中时不会提交字段，所以 `published` 用 `Option<String>`。

## 6. 应用状态 AppState

`src/main.rs` 定义共享状态：

```rust
#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub pool: sqlx::SqlitePool,
    pub templates: Arc<Tera>,
}
```

Axum handler 通过 `State<AppState>` 读取它：

```rust
pub async fn index(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    // 使用 state.pool 查询数据库
    // 使用 state.templates 渲染模板
}
```

为什么 `Tera` 放进 `Arc`？

Tera 模板集合会被多个请求共享，`Arc` 可以让多个 handler 持有同一份模板对象。

## 7. 路由设计

公开路由：

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| GET | `/` | 首页文章列表 |
| GET | `/posts/:slug` | 文章详情 |
| POST | `/posts/:slug/comments` | 提交评论 |
| GET | `/rss.xml` | RSS |
| GET | `/admin/login` | 登录表单 |
| POST | `/admin/login` | 登录 |

后台路由：

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| GET | `/admin` | 后台首页 |
| GET | `/admin/posts/new` | 新建文章表单 |
| POST | `/admin/posts` | 创建文章 |
| GET | `/admin/posts/:id/edit` | 编辑文章表单 |
| POST | `/admin/posts/:id` | 更新文章 |
| POST | `/admin/posts/:id/delete` | 删除文章 |
| POST | `/admin/logout` | 退出 |

`src/main.rs` 中组合路由：

```rust
let app = Router::new()
    .route("/", axum::routing::get(post::index))
    .route("/posts/:slug", axum::routing::get(post::show))
    .route("/posts/:slug/comments", axum::routing::post(post::create_comment))
    .route("/rss.xml", axum::routing::get(rss::feed))
    .merge(admin::routes(state.clone()))
    .nest_service("/static", ServeDir::new("blog/static"))
    .with_state(state);
```

## 8. Handler 写法

首页查询已发布文章：

```rust
let posts = sqlx::query_as::<_, Post>(
    "SELECT * FROM posts WHERE published = 1 ORDER BY created_at DESC",
)
.fetch_all(&state.pool)
.await?;
```

文章详情：

1. 根据 `slug` 查文章。
2. 根据 `post.id` 查评论。
3. 把 Markdown 渲染成 HTML。
4. 调用 Tera 模板。

Markdown 渲染在 `src/utils/markdown.rs`：

```rust
pub fn render_markdown(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}
```

注意：这个示例适合本地博客。公网系统要对 HTML 做清洗，避免 XSS。

## 9. 管理认证

`src/middleware/auth.rs` 使用 cookie 保存本地管理 token：

```rust
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
```

这是简单教学方案，不是公网安全方案。

真实部署需要考虑：

- HTTPS。
- 密码哈希。
- CSRF token。
- session 过期。
- 登录失败限制。
- 安全 cookie 标记。

## 10. 模板结构

模板使用 Tera：

```text
templates/
  base.html
  index.html
  post.html
  admin/
    dashboard.html
    edit.html
    login.html
    new.html
```

`base.html` 定义公共布局：

```html
{% block content %}{% endblock content %}
```

页面模板继承它：

```html
{% extends "base.html" %}
{% block content %}
  <h1>文章</h1>
{% endblock content %}
```

这种模式类似 Java 的 Thymeleaf layout，也类似 Go template 中复用公共模板。

## 11. 静态资源

静态文件放在：

```text
static/
  style.css
  highlight.js
```

Axum 使用 `ServeDir` 暴露：

```rust
.nest_service("/static", ServeDir::new("blog/static"))
```

因此浏览器可以访问：

```text
/static/style.css
/static/highlight.js
```

## 12. 本地运行步骤

1. 确认 Rust 已安装：

```powershell
rustc --version
cargo --version
```

2. 检查 `.env`：

```text
DATABASE_URL=sqlite://blog.db
BIND_ADDR=127.0.0.1:3000
ADMIN_TOKEN=change-me-local-admin-token
```

3. 启动：

```powershell
cargo run -p blog
```

4. 打开首页：

```text
http://127.0.0.1:3000
```

5. 登录后台：

```text
http://127.0.0.1:3000/admin/login
```

输入 `.env` 中的 `ADMIN_TOKEN`。

6. 新建文章：

```text
http://127.0.0.1:3000/admin/posts/new
```

填写标题、slug、摘要、Markdown 内容，勾选发布。

## 13. 初学者应该重点理解什么

不要一开始就背所有 Axum API。先理解请求流：

```text
浏览器请求
  -> Axum Router 匹配路径
  -> handler 提取 Path/Form/State
  -> SQLx 查询 SQLite
  -> Tera 渲染 HTML
  -> Axum 返回响应
```

然后理解数据流：

```text
HTML 表单
  -> PostForm / CommentForm
  -> SQL INSERT / UPDATE
  -> 查询 Post / Comment
  -> 模板展示
```

最后理解工程边界：

- `config.rs` 不关心路由。
- `db.rs` 不关心 HTML。
- `models/` 不关心 HTTP。
- `handlers/` 连接 HTTP、数据库和模板。
- `middleware/` 处理横切逻辑，比如认证。
- `utils/` 放通用工具，比如 Markdown 渲染。

## 14. 后续扩展路线

建议按这个顺序继续：

1. 给评论增加审核状态。
2. 给文章增加标签表 `tags` 和关联表 `post_tags`。
3. 使用 `serde` 从 `content/` 导入 Markdown 文件。
4. 增加分页。
5. 增加搜索。
6. 使用 `clap` 添加命令行管理命令。
7. 把认证改成用户名 + 密码 + session。
8. 使用 Docker 打包部署。

## 与 Java、Go 的 Web 开发对比

| 主题 | Rust/Axum | Java/Spring Boot | Go/net/http 或 Gin |
| --- | --- | --- | --- |
| 路由 | 类型化 extractor，组合式 Router | 注解和控制器常见 | handler 函数直接，框架轻 |
| 数据库 | SQLx 编译/运行期类型映射，异步 | JPA/MyBatis/JdbcTemplate 生态成熟 | database/sql 简洁，ORM 可选 |
| 模板 | Tera 类似 Jinja2 | Thymeleaf/Freemarker | html/template |
| 错误处理 | `Result` 显式传播 | exception 为主 | `error` 显式返回 |
| 部署 | 单二进制 + SQLite 很轻 | JVM 运行时较重但生态强 | 单二进制，部署简单 |
| 学习曲线 | 所有权 + async 需要适应 | 框架概念多但资料丰富 | 语法简单，上手快 |

如果目标是个人本地博客，Rust 的优势是运行时占用低、部署产物清晰、类型约束强。Java 的优势是后台管理、权限、ORM 等成熟方案非常多。Go 的优势是 Web 服务实现简单、部署同样轻量。
