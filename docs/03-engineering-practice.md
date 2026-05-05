# 阶段三：工程实践

这一章的目标是把前两章的语法、所有权、错误处理和集合能力组织成一个能运行、能维护的小项目。

配套命令：

```powershell
cargo run -p engineering_practice --example engineering_tour
cargo run -p engineering_practice --example task_tracker_demo
```

完整项目案例见：[05-build-task-tracker-from-zero.md](05-build-task-tracker-from-zero.md)

## 1. 模块系统：把代码按职责拆开

Rust 使用 `mod` 声明模块，用 `pub` 控制可见性。本项目的工程实践 crate：

```text
crates/engineering_practice/src/
  lib.rs
  config.rs
  concurrency.rs
  errors.rs
  math.rs
  task_tracker.rs
```

`lib.rs` 是 crate 的公共入口：

```rust
pub mod concurrency;
pub mod config;
pub mod errors;
pub mod math;
pub mod task_tracker;

pub use concurrency::{add_with_thread, shared_counter, sum_with_channel};
pub use config::{find_config, parse_port};
pub use errors::AppError;
pub use math::divide;
```

模块设计建议：

- `errors.rs` 放共享错误类型。
- `config.rs` 放配置读取和解析。
- `concurrency.rs` 放线程、channel、共享状态示例。
- `task_tracker.rs` 放完整任务看板案例。

新手常见问题是把所有代码都放进 `main.rs`。这样短期简单，长期会导致函数互相依赖、测试困难、职责不清。

## 2. Cargo：Rust 的工程入口

Cargo 负责构建、运行、测试和依赖管理。本项目是 workspace：

```toml
[workspace]
members = [
    "crates/basic_syntax",
    "crates/advanced_features",
    "crates/engineering_practice",
]
resolver = "2"
```

常用命令：

```powershell
cargo check
cargo test
cargo run -p engineering_practice --example task_tracker_demo
cargo fmt
```

建议开发节奏：

1. 写一点代码。
2. `cargo check` 快速看类型和借用问题。
3. `cargo run ...` 看主流程输出。
4. `cargo test` 确认示例和文档测试仍可运行。

## 3. 错误处理：Result、Option 与 panic

`Option<T>` 表示“可能没有值”：

```rust
pub fn find_config(key: &str) -> Option<&'static str> {
    match key {
        "host" => Some("localhost"),
        "env" => Some("dev"),
        _ => None,
    }
}
```

`Result<T, E>` 表示“可能失败”：

```rust
pub fn divide(left: i32, right: i32) -> Result<i32, AppError> {
    if right == 0 {
        Err(AppError::DivideByZero)
    } else {
        Ok(left / right)
    }
}
```

在任务看板案例中，错误被建模成 enum：

```rust
pub enum TaskError {
    EmptyTitle,
    InvalidCommand,
    InvalidTaskId,
    TaskNotFound(u64),
}
```

这样调用方可以根据错误分支做不同处理，而不是只能解析字符串。

`panic!` 的使用边界：

- 适合不可恢复错误，例如内部状态被破坏。
- 不适合用户输入错误、文件不存在、端口解析失败等可恢复场景。
- 库代码应尽量返回 `Result`，把决策权交给调用方。

## 4. 并发编程

Rust 标准库提供三类基础并发能力。

线程：

```rust
thread::spawn(move || left + right)
    .join()
    .expect("worker thread should finish")
```

消息传递：

```rust
let (sender, receiver) = mpsc::channel();
sender.send(value)?;
```

共享状态：

```rust
let counter = Arc::new(Mutex::new(0usize));
```

核心概念：

- `move` 把捕获值移动进线程，避免线程引用已经失效的外部变量。
- `Arc<T>` 是线程安全引用计数，适合多个线程共享所有权。
- `Mutex<T>` 提供互斥访问，确保同一时间只有一个线程修改数据。
- `Send` 和 `Sync` 是 Rust 并发安全的底层 trait。

实践建议：优先用 channel 表达“数据流动”；只有确实需要多个线程共享同一份状态时，再使用 `Arc<Mutex<T>>`。

## 5. 测试：够用即可，但要能守住主流程

本项目保留轻量测试，不追求大量测试代码：

- 集成测试放在 `crates/*/tests/`。
- 文档测试放在 `//!` 文档注释代码块中。
- 主流程通过 example 运行验证。

运行：

```powershell
cargo test
```

测试的目的不是代替学习，而是确保教程中的示例代码始终能编译和运行。

## 6. 工程案例：任务看板

任务看板案例包含：

- `TaskStatus`：任务状态 enum。
- `Task`：任务结构体。
- `TaskError`：业务错误 enum。
- `Command`：命令解析结果。
- `TaskBoard`：任务集合与业务操作。
- `parse_command`：把用户输入解析成命令。

示例命令：

```text
add 阅读 Rust 基础语法
add 完成所有权练习
start 2
done 1
list
```

运行：

```powershell
cargo run -p engineering_practice --example task_tracker_demo
```

输出会展示每条命令执行后的结果。这个案例虽然没有接入文件和数据库，但已经具备一个小项目的骨架：模型、错误、命令解析、业务状态、输出格式和测试。

## 与 Java、Go 的工程实践对比

| 主题 | Rust | Java | Go |
| --- | --- | --- | --- |
| 项目构建 | Cargo 原生统一构建、测试、依赖 | Maven / Gradle 功能强但配置重 | `go` 工具链简单统一 |
| 模块组织 | crate + mod + pub，可见性严格 | package + class，OOP 组织明显 | package 简洁，目录即包 |
| 错误处理 | `Result` 显式返回，类型化错误 | checked/unchecked exception、Optional | `error` 多返回值，显式但重复 |
| 并发 | 类型系统约束共享和移动 | 线程池、锁、CompletableFuture、虚拟线程 | goroutine + channel 简洁高效 |
| 测试 | 内置测试、文档测试、集成测试 | JUnit/TestNG 等生态成熟 | 内置 testing，表驱动测试常见 |

Java 在大型企业工程、框架生态、团队协作规范方面非常成熟；Go 在服务端工程和云原生工具上简单直接；Rust 的优势是把更多资源和并发安全问题提前到编译期处理，适合性能敏感、可靠性要求高、长期维护成本敏感的系统组件。
