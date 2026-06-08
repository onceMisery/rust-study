//! 异步编程示例。

use std::time::{Duration, Instant};

/// 模拟异步操作：延迟后返回结果。
pub async fn fetch_data(id: u32) -> String {
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("数据-{}", id)
}

/// 模拟可能失败的异步操作。
pub async fn fetch_with_result(id: u32) -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(50)).await;
    if id == 0 {
        Err("ID 不能为 0".to_string())
    } else {
        Ok(format!("成功获取数据-{}", id))
    }
}

/// 演示并发执行多个异步任务。
pub async fn concurrent_fetch(ids: &[u32]) -> Vec<String> {
    let mut handles = Vec::new();
    for &id in ids {
        handles.push(tokio::spawn(async move { fetch_data(id).await }));
    }

    let mut results = Vec::new();
    for handle in handles {
        if let Ok(result) = handle.await {
            results.push(result);
        }
    }
    results
}

/// 演示 tokio::select! 和超时控制。
pub async fn fetch_with_timeout(id: u32, timeout_ms: u64) -> Result<String, String> {
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

/// 演示异步性能：串行 vs 并行。
pub async fn async_performance_demo() -> String {
    let ids = vec![1, 2, 3, 4, 5];

    // 串行执行
    let start = Instant::now();
    let mut serial_results = Vec::new();
    for &id in &ids {
        serial_results.push(fetch_data(id).await);
    }
    let serial_time = start.elapsed();

    // 并行执行
    let start = Instant::now();
    let parallel_results = concurrent_fetch(&ids).await;
    let parallel_time = start.elapsed();

    format!(
        "串行: {:?} ({:?})\n并行: {:?} ({:?})",
        serial_results, serial_time, parallel_results, parallel_time
    )
}
