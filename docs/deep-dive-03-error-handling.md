# 专题三：错误处理深入

Rust 没有异常机制，而是用类型系统表达错误——这迫使开发者在编译期就思考"如果出错怎么办"。

配套代码：

```powershell
cargo run -p engineering_practice --example error_tour
cargo run -p basic_syntax --example basic_tour
```

---

## 1. Option：有值或无值

### 基本使用

```rust
fn find_user(id: u32) -> Option<String> {
    match id {
        1 => Some("Alice".to_string()),
        2 => Some("Bob".to_string()),
        _ => None,
    }
}

match find_user(1) {
    Some(name) => println!("找到: {}", name),
    None => println!("用户不存在"),
}
```

### 链式操作

```rust
let result = find_user(1)
    .map(|name| name.to_uppercase())    // Some("ALICE")
    .filter(|name| name.len() > 3)      // Some("ALICE")
    .unwrap_or("UNKNOWN".to_string());  // "ALICE"
```

---

## 2. Result：成功或失败

### 基本使用

```rust
fn parse_port(raw: &str) -> Result<u16, String> {
    raw.parse::<u16>().map_err(|e| format!("解析失败: {}", e))
}

match parse_port("8080") {
    Ok(port) => println!("端口: {}", port),
    Err(e) => eprintln!("错误: {}", e),
}
```

### ? 操作符

`?` 是错误传播的语法糖，出错时提前返回 `Err`：

```rust
fn read_config(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;  // 出错则提前返回
    Ok(content)
}
```

等价于：

```rust
fn read_config(path: &str) -> Result<String, std::io::Error> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };
    Ok(content)
}
```

---

## 3. 自定义错误类型

### 基础定义

```rust
#[derive(Debug)]
enum AppError {
    EmptyInput,
    InvalidNumber(std::num::ParseIntError),
    OutOfRange { value: i32, min: i32, max: i32 },
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::EmptyInput => write!(f, "输入不能为空"),
            AppError::InvalidNumber(e) => write!(f, "数字解析失败: {}", e),
            AppError::OutOfRange { value, min, max } =>
                write!(f, "值 {} 超出范围 [{}, {}]", value, min, max),
        }
    }
}

impl std::error::Error for AppError {}

// From trait 允许 ? 自动转换
impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::InvalidNumber(e)
    }
}
```

### 使用自定义错误

```rust
type AppResult<T> = Result<T, AppError>;

fn parse_and_validate(raw: &str) -> AppResult<i32> {
    if raw.trim().is_empty() {
        return Err(AppError::EmptyInput);
    }
    let value: i32 = raw.parse()?;  // 自动通过 From 转换为 AppError
    if !(0..=100).contains(&value) {
        return Err(AppError::OutOfRange { value, min: 0, max: 100 });
    }
    Ok(value)
}
```

---

## 4. 错误处理策略

### 策略对比

| 策略 | 适用场景 | 示例 |
|------|----------|------|
| `Result<T, E>` + `?` | 库函数、业务逻辑 | 大多数情况 |
| `match` 手动处理 | 需要针对每种错误做不同处理 | 用户交互 |
| `unwrap_or(default)` | 有合理默认值 | 配置解析 |
| `unwrap()` / `expect()` | 测试、原型 | 竞赛代码 |
| `anyhow` crate | 应用级错误处理 | Web 服务 |
| `thiserror` crate | 库级错误类型定义 | 公共库 |

### unwrap 的安全替代

```rust
// ❌ 危险：None/Err 时 panic
let value = some_option.unwrap();

// ✅ 安全：提供默认值
let value = some_option.unwrap_or(0);
let value = some_option.unwrap_or_else(|| expensive_default());

// ✅ 安全：明确错误信息
let value = some_option.expect("配置文件中必须包含端口号");
```

---

## 5. Option 与 Result 互转

```rust
// Option → Result
let result: Result<i32, &str> = Some(42).ok_or("没有值");

// Result → Option
let option: Option<i32> = Ok(42).ok();
let option: Option<i32> = Err("error").ok();  // None
```

---

## 6. panic!：不可恢复的错误

```rust
// 仅在真正无法继续时使用
fn get_config() -> Config {
    let file = std::fs::read_to_string("config.toml")
        .expect("配置文件必须存在");  // 文件不存在时 panic
    // ...
}
```

**何时用 panic**：
- 程序逻辑不应该到达的分支（如 `unreachable!()`）
- 启动阶段配置缺失
- 测试代码

**何时不用 panic**：
- 用户输入验证（用 Result）
- 网络请求失败（用 Result）
- 文件读写错误（用 Result）

---

## 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 错误机制 | `Result<T, E>` 类型 | 异常（try/catch） | 多返回值 + error |
| 编译期检查 | ✅ 必须处理 Result | ❌ 仅 checked exception | ❌ 容易忽略 error |
| 错误传播 | `?` 操作符 | `throw` | `return err` |
| 性能 | 零开销抽象 | 异常有栈追踪开销 | error 是普通值 |
| 可恢复性 | 类型系统区分 | 全部是异常 | 靠约定 |

---

## 最佳实践

1. **库函数返回 Result**，让调用方决定如何处理
2. **应用代码用 `?` 传播错误**，在最上层统一处理
3. **自定义错误类型实现 `Display` + `Error` + `From`**
4. **用 `unwrap_or` 替代 `unwrap`**，除非你确定不会出错
5. **不要吞掉错误**——至少用 `eprintln!` 记录

## 配套代码

```powershell
cargo run -p engineering_practice --example error_tour
cargo run -p basic_syntax --example basic_tour  # 主题 9-16 包含错误处理
```
