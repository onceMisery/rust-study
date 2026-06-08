use engineering_practice::async_demo::{
    async_performance_demo, concurrent_fetch, fetch_data, fetch_with_result, fetch_with_timeout,
};

#[tokio::main]
async fn main() {
    println!("=== 1. 基础 async/await ===");
    let data = fetch_data(1).await;
    println!("获取数据: {}", data);

    println!("\n=== 2. 异步 Result 处理 ===");
    match fetch_with_result(1).await {
        Ok(msg) => println!("成功: {}", msg),
        Err(e) => println!("失败: {}", e),
    }
    match fetch_with_result(0).await {
        Ok(msg) => println!("成功: {}", msg),
        Err(e) => println!("失败: {}", e),
    }

    println!("\n=== 3. 并发执行 ===");
    let results = concurrent_fetch(&[1, 2, 3]).await;
    println!("并发结果: {:?}", results);

    println!("\n=== 4. 超时控制 ===");
    match fetch_with_timeout(1, 200).await {
        Ok(data) => println!("成功（200ms 超时）: {}", data),
        Err(e) => println!("超时: {}", e),
    }
    match fetch_with_timeout(1, 50).await {
        Ok(data) => println!("成功（50ms 超时）: {}", data),
        Err(e) => println!("超时: {}", e),
    }

    println!("\n=== 5. 性能对比：串行 vs 并行 ===");
    println!("{}", async_performance_demo().await);

    println!("\n=== 6. async 最佳实践 ===");
    println!("✅ I/O 密集型任务适合 async（网络、文件）");
    println!("✅ CPU 密集型任务用 tokio::spawn_blocking");
    println!("✅ 错误处理：async fn 返回 Result");
    println!("✅ 超时控制：tokio::time::timeout");
    println!("❌ 避免：在 async 函数中执行阻塞操作");
}
