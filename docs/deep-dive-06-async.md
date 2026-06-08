# 专题六：异步编程

Rust 的异步编程基于 `async/await` 语法和 `Future` trait，配合 Tokio 运行时实现高效的并发 I/O。

配套代码：

```powershell
cargo run -p engineering_practice --example async_tour
```

---

## 1. async/await 基础

### 定义异步函数

```rust
async fn fetch_data(id: u32) -> String {
    // 模拟网络请求
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("数据-{}", id)
}
```

### 调用异步函数

```rust
#[tokio::main]
async fn main() {
    let data = fetch_data(1).await;  // .await 等待异步操作完成
    println!("{}", data);  // 数据-1
}
```

### async fn 的本质

`async fn` 返回一个 `Future`——它不会立即执行，只有被 `.await` 时才真正运行：

```rust
let future = fetch_data(1);   // 不会执行！只是创建了 Future
let data = future.await;       // 现在才执行
```

---

## 2. 并发执行

### 串行 vs 并行

```rust
// 串行：总耗时 = 100ms × 5 = 500ms
for id in 1..=5 {
    fetch_data(id).await;
}

// 并行：总耗时 ≈ 100ms（同时执行）
let results = concurrent_fetch(&[1, 2, 3, 4, 5]).await;
```

### tokio::spawn 创建任务

```rust
async fn concurrent_fetch(ids: &[u32]) -> Vec<String> {
    let mut handles = Vec::new();
    for &id in ids {
        handles.push(tokio::spawn(async move {
            fetch_data(id).await
        }));
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }
    results
}
```

> `tokio::spawn` 创建独立的异步任务，可以在不同的线程上并行执行。`async move` 将变量所有权移动进闭包。

---

## 3. 异步错误处理

### async fn 返回 Result

```rust
async fn fetch_with_result(id: u32) -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(50)).await;
    if id == 0 {
        Err("ID 不能为 0".to_string())
    } else {
        Ok(format!("成功获取数据-{}", id))
    }
}

// 使用 ? 传播错误
async fn process() -> Result<(), String> {
    let data = fetch_with_result(1).await?;
    println!("{}", data);
    Ok(())
}
```

---

## 4. 超时控制

```rust
async fn fetch_with_timeout(id: u32, timeout_ms: u64) -> Result<String, String> {
    let result = tokio::time::timeout(
        Duration::from_millis(timeout_ms),
        fetch_data(id),
    )
    .await;

    match result {
        Ok(data) => Ok(data),
        Err(_) => Err(format!("请求超时（{}ms）", timeout_ms)),
    }
}

// 200ms 超时：成功（fetch 只需 100ms）
let ok = fetch_with_timeout(1, 200).await;   // Ok("数据-1")

// 50ms 超时：失败（fetch 需要 100ms）
let err = fetch_with_timeout(1, 50).await;    // Err("请求超时")
```

---

## 5. tokio::select! 多路复用

```rust
tokio::select! {
    result1 = fetch_data(1) => println!("任务1完成: {}", result1),
    result2 = fetch_data(2) => println!("任务2完成: {}", result2),
}
// 哪个先完成就执行哪个分支，其他任务被取消
```

---

## 6. 性能对比

### 实际测量

```rust
// 串行执行 5 个 100ms 任务
let serial_time = ...;   // ≈ 550ms

// 并行执行 5 个 100ms 任务
let parallel_time = ...; // ≈ 110ms
```

并行执行比串行快 **5 倍**，因为所有 I/O 操作同时进行。

### async vs 线程

| 特性 | async (Tokio) | OS 线程 |
|------|-------------|---------|
| 创建开销 | 极小（~100字节） | 大（~1MB 栈） |
| 并发数量 | 数十万 | 数百~数千 |
| 适用场景 | I/O 密集型 | CPU 密集型 |
| 编程复杂度 | 需要 async 传染 | 传统同步编程 |

---

## 7. 异步最佳实践

### 适用场景

- ✅ 网络请求（HTTP、gRPC、WebSocket）
- ✅ 数据库查询
- ✅ 文件 I/O
- ✅ 大量并发 I/O 操作

### 不适用场景

- ❌ CPU 密集型计算（用 `tokio::spawn_blocking`）
- ❌ 简单的同步程序（直接用同步代码更简单）

### 常见问题

| 问题 | 解决方案 |
|------|----------|
| async 函数中调用同步阻塞代码 | 用 `tokio::spawn_blocking` |
| 在 async 中使用 `Rc` | 改用 `Arc`（线程安全） |
| 忘记 `.await` | 编译器会警告 |
| Future 不执行 | 确保调用 `.await` 或用 `tokio::spawn` |

---

## 与 Java/Go 对比

| 特性 | Rust (Tokio) | Java (CompletableFuture) | Go (goroutine) |
|------|-------------|-------------------------|----------------|
| 并发模型 | async/await + 事件循环 | 线程池 + 回调 | goroutine + channel |
| 创建开销 | ~100 字节 | ~1 MB | ~2 KB |
| 语法 | `async`/`await` | 链式回调 | `go func()` |
| 错误处理 | `Result` + `?` | 异常 | 多返回值 |
| 运行时 | Tokio（显式引入） | JVM 内置 | Go 运行时内置 |

---

## 配套代码

```powershell
cargo run -p engineering_practice --example async_tour
```

代码示例涵盖：基础 async/await、并发执行、Result 错误处理、超时控制、串行 vs 并行性能对比。
