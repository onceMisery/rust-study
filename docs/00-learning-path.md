# Rust 系统学习路线

> **零基础入门**：如果你是第一次接触 Rust，请先阅读 [从零开始：环境搭建与第一个程序](00-getting-started.md)，完成工具链安装、VSCode 配置和第一个程序运行后再开始正式学习。

本项目按“先语法、再模型、后工程”的顺序组织。Rust
的难点不是关键字数量，而是它把内存安全、性能和并发安全放进了类型系统。学习时不要急着背规则，应该先写小程序，再观察编译器如何提示你修正所有权、借用和生命周期问题。

## 阶段一：基础语法

目标是能读懂并写出普通 Rust 函数。重点包括变量声明、可变性、遮蔽、标量类型、复合类型、函数、表达式、控制流和切片。

配套代码：

```powershell
cargo run -p basic_syntax --example basic_tour
```

## 阶段二：进阶特性

目标是理解 Rust 和 Java / Go 最大的不同：Rust 没有垃圾回收器，却能在编译期保证内存安全。重点包括所有权、移动、复制、借用、可变引用、生命周期、trait、默认方法、泛型和
trait bound。

配套代码：

```powershell
cargo run -p advanced_features --example advanced_tour
```

## 阶段三：工程实践

目标是能写一个结构清晰、可维护的 Rust 小项目。重点包括模块系统、Cargo、错误处理、panic 使用边界、线程、消息传递、共享状态、单元测试、集成测试和文档测试。

配套代码：

```powershell
cargo run -p engineering_practice --example engineering_tour
cargo run -p engineering_practice --example task_tracker_demo
```

## 阶段四：和 Java / Go 对比

目标是建立语言设计层面的判断力。每个章节末尾都有 Java / Go 对比，最后再通过独立对比章节做总览。Rust
适合对性能、资源控制、可靠性要求很高的系统；Java 适合成熟企业生态和大型业务系统；Go 适合网络服务、云原生工具和简单并发服务。

## 阶段五：完整 Web 工程案例

目标是从零构建一个个人本地博客系统，技术栈包括 Axum、SQLx、Tera、SQLite、Markdown 渲染和简单后台认证。

配套文档：[06-build-local-blog-with-axum.md](06-build-local-blog-with-axum.md)

配套代码：

```powershell
cargo run -p blog
```

## 建议节奏

1. 每读完一节，运行对应 example。
2. 修改示例代码，故意触发编译错误，阅读错误信息。
3. 不要绕过编译器。Rust 编译器的报错本身就是学习材料。
4. 每个阶段结束后，执行 `cargo test`，确认示例仍可运行。

## 前置知识

### `Cargo.toml` 是什么

`Cargo.toml` 是 Rust 的**项目配置文件**，相当于 Rust 项目的"身份证"和"构建说明书"。Cargo 是 Rust 的包管理器，而
`Cargo.toml` 就是它的核心配置文件。

### 类比理解

| 语言/生态         | 类似文件                                  | 作用          |
|---------------|---------------------------------------|-------------|
| Rust          | `Cargo.toml`                          | 项目配置 + 依赖管理 |
| Node.js       | `package.json`                        | npm 包配置     |
| Java (Maven)  | `pom.xml`                             | 项目对象模型      |
| Java (Gradle) | `build.gradle`                        | 构建脚本        |
| Python        | `pyproject.toml` / `requirements.txt` | 项目配置        |
| Go            | `go.mod`                              | Go 模块管理     |

### 基本结构示例

```toml
[package]
name = "my_project"          # 项目名称
version = "0.1.0"            # 版本号（语义化版本）
edition = "2021"             # Rust 版本（2018/2021/2024）
authors = ["Your Name <email@example.com>"]  # 作者
description = "这是一个示例项目"              # 描述
license = "MIT"               # 开源协议

[dependencies]                # 依赖的第三方库
rand = "0.8.5"               # 随机数库
serde = { version = "1.0", features = ["derive"] }  # 带特性的依赖
tokio = { version = "1.0", features = ["full"] }    # 异步运行时

[dependencies.reqwest]       # 更详细的依赖写法
version = "0.11"
features = ["json", "blocking"]

[dev-dependencies]           # 仅在测试/开发时需要的依赖
criterion = "0.5"            # 性能测试库

[build-dependencies]         # 仅在构建脚本 build.rs 中使用的依赖
cc = "1.0"

[features]                   # 可选特性（条件编译）
default = ["std"]            # 默认启用的特性
std = []                     # 标准库支持
serde-support = ["serde"]    # 启用 serde 支持

[profile.release]            # 优化配置（生产构建）
opt-level = 3                # 优化级别
lto = true                   # 链接时优化
```

### 常用字段说明

| 区域                     | 字段          | 说明                |
|------------------------|-------------|-------------------|
| `[package]`            | `name`      | 项目名称，会作为生成的二进制文件名 |
|                        | `version`   | 版本号，遵循语义化版本规范     |
|                        | `edition`   | Rust 语言版本，影响语法特性  |
| `[dependencies]`       | `库名 = "版本"` | 生产依赖，会被打包进最终产物    |
| `[dev-dependencies]`   | -           | 开发依赖（测试、基准测试）     |
| `[build-dependencies]` | -           | 构建脚本依赖            |
| `[features]`           | -           | 条件编译特性，可选启用       |
| `[profile.*]`          | -           | 不同构建模式的编译优化参数     |

### 实际操作示例

#### 1. 创建新项目（自动生成 Cargo.toml）

```bash
cargo new my_app
cd my_app
```

自动生成的 `Cargo.toml`：

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### 2. 添加依赖

```bash
# 添加随机数库
cargo add rand

# 添加特定版本
cargo add serde@1.0

# 添加带特性的依赖
cargo add tokio --features full
```

添加后 `Cargo.toml` 会自动更新：

```toml
[dependencies]
rand = "0.8.5"
serde = "1.0.203"
tokio = { version = "1.35", features = ["full"] }
```

#### 3. 使用依赖

```rust
// 代码中引入依赖
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(1..=100);
    println!("随机数: {}", num);
}
```

#### 4. 构建运行

```bash
cargo build    # 构建项目（会下载依赖）
cargo run      # 运行项目（自动构建）
cargo test     # 运行测试
```

### 版本号语法

| 写法             | 含义       | 允许更新范围                  |
|----------------|----------|-------------------------|
| `"0.8.5"`      | 确切版本     | 不允许自动更新                 |
| `"0.8"`        | 主要版本固定   | 0.8.x 的任何版本             |
| `"^0.8.5"`     | 兼容更新（默认） | 0.8.x 和 0.9.x（如果兼容）     |
| `"~0.8.5"`     | 补丁版本更新   | 仅 0.8.x                 |
| `"*"`          | 任意版本     | 任何版本（不推荐）               |
| `"0.8.5"`，路径依赖 | 本地路径     | `path = "../local-lib"` |
| Git 依赖         | Git 仓库   | `git = "https://..."`   |

### 实际项目结构

```
my_project/
├── Cargo.toml          # 项目配置文件
├── Cargo.lock          # 锁定依赖确切版本（自动生成）
├── src/
│   ├── main.rs         # 主程序入口
│   └── lib.rs          # 库入口（如果是库项目）
├── tests/              # 集成测试
├── benches/            # 基准测试
└── target/             # 编译输出（自动生成）
```

### 关键点总结

1. **`Cargo.toml` 是 Rust 项目的核心配置**，必须放在项目根目录
2. **用 TOML 格式**（Tom's Obvious, Minimal Language），类似 INI 但更规范
3. **声明依赖**：Cargo 会自动下载、编译、链接依赖
4. **管理版本**：遵循语义化版本，可以精确控制更新范围
5. **无需手动管理类路径**：Cargo 自动处理所有依赖关系
6. **`Cargo.lock`** 会锁定确切版本，确保团队/生产环境依赖一致
