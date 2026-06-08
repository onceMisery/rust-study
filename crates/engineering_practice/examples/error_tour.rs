use engineering_practice::errors::AppError;

fn main() {
    println!("=== 1. Option：有值或无值 ===");
    let result = engineering_practice::find_config("host");
    println!("查找 host: {:?}", result);
    let missing = engineering_practice::find_config("unknown");
    println!("查找 unknown: {:?}", missing);

    // Option 链式操作
    let port_str = engineering_practice::find_config("host").map(|h| format!("http://{}:8080", h));
    println!("URL: {:?}", port_str);

    println!("\n=== 2. Result：成功或失败 ===");
    let valid = engineering_practice::parse_port("8080");
    println!("解析 8080: {:?}", valid);
    let invalid = engineering_practice::parse_port("abc");
    println!("解析 abc: {:?}", invalid);

    println!("\n=== 3. ? 操作符与错误传播 ===");
    match engineering_practice::divide(10, 2) {
        Ok(v) => println!("10/2 = {}", v),
        Err(e) => println!("错误: {:?}", e),
    }
    match engineering_practice::divide(10, 0) {
        Ok(v) => println!("10/0 = {}", v),
        Err(e) => println!("错误: {:?}", e),
    }

    println!("\n=== 4. 自定义错误类型 ===");
    let err = AppError::InvalidPort;
    println!("InvalidPort: {:?}", err);
    let err = AppError::DivideByZero;
    println!("DivideByZero: {:?}", err);

    println!("\n=== 5. unwrap_or 默认值 ===");
    let port = engineering_practice::parse_port("abc").unwrap_or(8080);
    println!("解析失败时使用默认端口: {}", port);

    let host = engineering_practice::find_config("missing").unwrap_or("127.0.0.1");
    println!("查找失败时使用默认主机: {}", host);

    println!("\n=== 6. 错误处理最佳实践 ===");
    println!("✅ 库函数：返回 Result，让调用方决定如何处理");
    println!("✅ 应用代码：用 match 或 ? 处理错误");
    println!("✅ 测试/原型：可以用 unwrap() 或 expect()");
    println!("❌ 避免：在库函数中 panic!()");
}
