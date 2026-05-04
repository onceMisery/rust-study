# 阶段三：工程实践

## 模块系统

Rust 使用 `mod` 声明模块，使用 `pub` 控制可见性。本项目的工程实践 crate 结构如下：

```text
crates/engineering_practice/src/
  lib.rs
  config.rs
  concurrency.rs
  errors.rs
  math.rs
```

`lib.rs` 负责组织模块并导出公共 API：

```rust
pub mod concurrency;
pub mod config;
pub mod errors;
pub mod math;

pub use concurrency::{add_with_thread, shared_counter, sum_with_channel};
```

最佳实践：模块边界应该围绕职责划分，而不是围绕语法概念划分。错误类型放在 `errors.rs`，并发示例放在 `concurrency.rs`，配置解析放在 `config.rs`。

## Cargo 包管理

Cargo 同时负责构建、测试、运行示例和管理依赖。本项目是 workspace：

```toml
[workspace]
members = [
    "crates/basic_syntax",
    "crates/advanced_features",
    "crates/engineering_practice",
]
```

常用命令：

```powershell
cargo test
cargo run -p engineering_practice --example engineering_tour
cargo check
cargo fmt
```

实际开发中，`cargo check` 用于快速检查类型和借用问题，`cargo test` 用于验证行为，`cargo fmt` 用于统一格式。

## 错误处理：Option、Result 与 panic

`Option<T>` 表示可能有值，也可能没有值：

```rust
pub fn find_config(key: &str) -> Option<&'static str> {
    match key {
        "host" => Some("localhost"),
        _ => None,
    }
}
```

`Result<T, E>` 表示可能成功，也可能失败：

```rust
pub fn divide(left: i32, right: i32) -> Result<i32, AppError> {
    if right == 0 {
        Err(AppError::DivideByZero)
    } else {
        Ok(left / right)
    }
}
```

`panic!` 适合不可恢复错误，例如违反内部不变量；业务错误、IO 错误、解析错误通常应该返回 `Result`。

注意事项：库代码少用 `unwrap()`，因为它会把错误处理权从调用方手里夺走。示例代码、测试代码或明确不可能失败的内部路径可以谨慎使用 `expect()`。

## 并发编程

Rust 标准库支持线程：

```rust
thread::spawn(move || left + right).join()
```

消息传递使用 channel：

```rust
let (sender, receiver) = mpsc::channel();
sender.send(value)?;
```

共享状态常用 `Arc<Mutex<T>>`：

```rust
let counter = Arc::new(Mutex::new(0usize));
```

设计原理：Rust 的所有权和 trait 约束会检查跨线程数据是否安全，例如 `Send` 表示值可以在线程间移动，`Sync` 表示引用可以在线程间共享。

最佳实践：优先使用消息传递表达所有权转移；确实需要共享可变状态时，再使用 `Arc<Mutex<T>>`。

## 测试

Rust 常见测试类型：

1. 单元测试：写在模块内部，适合测试私有实现细节。
2. 集成测试：写在 crate 的 `tests/` 目录，像外部用户一样调用公共 API。
3. 文档测试：写在文档注释的代码块中，`cargo test` 会编译并运行。

本项目为了保持学习主流程清晰，只保留了轻量集成测试和少量文档测试。它们的目标不是覆盖所有边界，而是确认示例代码能运行。
