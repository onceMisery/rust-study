# 专题八：模式匹配深入

模式匹配是 Rust 最强大的特性之一。它不仅是 `switch` 的增强版，更是解构数据、验证条件的核心工具。

配套代码：

```powershell
cargo run -p advanced_features --example pattern_matching_tour
```

---

## 1. match 基础

### 穷举所有可能

```rust
let grade = match score {
    90..=100 => "A",
    80..=89  => "B",
    70..=79  => "C",
    60..=69  => "D",
    _        => "F",  // 必须覆盖所有情况
};
```

`match` 的**穷举检查**是编译期强制的——漏掉任何情况都会报错。这是 Rust 的安全保证。

---

## 2. 解构模式

### 解构元组

```rust
let point = (3, 7);
match point {
    (0, 0) => "原点",
    (_x, 0) => "x轴上",
    (0, _y) => "y轴上",
    (x, y) if x == y => "对角线上",
    (_x, _y) => "普通点",
};
```

### 解构结构体

```rust
struct Color { r: u8, g: u8, b: u8 }

match color {
    Color { r: 255, g: 0, b: 0 } => "纯红",
    Color { r: 0, g: 255, b: 0 } => "纯绿",
    Color { r, g, b } if r > 200 && g > 100 && b < 50 => "暖色",
    Color { .. } => "其他颜色",  // .. 忽略剩余字段
}
```

### 解构枚举

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

match msg {
    Message::Quit => println!("退出"),
    Message::Move { x, y } => println!("移动到 ({}, {})", x, y),
    Message::Write(text) => println!("写入: {}", text),
    Message::Color(r, g, b) => println!("颜色({}, {}, {})", r, g, b),
}
```

---

## 3. 守卫（Guard）

守卫是 `match` 分支中的额外条件：

```rust
let numbers = vec![1, -2, 3, -4, 5];

let classified: Vec<String> = numbers.iter().map(|&n| match n {
    n @ 1..=10 => format!("{} 是小正数", n),
    n @ -10..=-1 => format!("{} 是小负数", n),
    n if n > 10 => format!("{} 是大正数", n),
    n => format!("{} 超出范围", n),
}).collect();
```

> 注意：守卫在模式匹配之后执行，所以优先级低于字面量模式和范围模式。

---

## 4. @ 绑定

`@` 允许你**匹配一个模式的同时把整个值绑定到一个变量**：

```rust
let msg = Some("hello");
match msg {
    some @ Some(s) if s.len() > 3 => {
        // some 是整个 Some("hello")
        // s 是内部的 "hello"
        println!("长消息: {:?}", some);
    }
    Some(s) => println!("短消息: {}", s),
    None => println!("无消息"),
}
```

---

## 5. if let：只关心一种匹配

当只关心一种匹配情况时，`if let` 比 `match` 更简洁：

```rust
let value: Option<i32> = Some(42);

// match 写法（需要处理所有分支）
match value {
    Some(v) => println!("找到: {}", v),
    None => {},
}

// if let 写法（更简洁）
if let Some(v) = value {
    println!("找到: {}", v);
}
```

---

## 6. while let：循环解构

`while let` 在模式持续匹配时循环：

```rust
let mut stack = vec![1, 2, 3];
while let Some(top) = stack.pop() {
    println!("弹出: {}", top);
}
// 输出: 3, 2, 1
```

---

## 7. 嵌套模式

```rust
let data = Some(vec![1, 2, 3]);
match data {
    Some(vec) if vec.len() > 2 => println!("长列表: {:?}", vec),
    Some(vec) => println!("短列表: {:?}", vec),
    None => println!("无数据"),
}
```

---

## 8. let-else 语法（Rust 1.65+）

匹配失败时执行代码块（必须 diverge，即不返回）：

```rust
fn process(input: &str) -> i32 {
    let Ok(number) = input.parse::<i32>() else {
        return -1;  // 匹配失败时提前返回
    };
    number * 2  // 匹配成功，继续执行
}
```

---

## 模式匹配适用场景

| 场景 | 推荐方式 | 说明 |
|------|----------|------|
| 枚举变体穷举 | `match` | 编译器强制覆盖所有变体 |
| Option/Result 单分支 | `if let` | 比 match 简洁 |
| 循环弹出/迭代 | `while let` | 适合栈、队列等 |
| 匹配失败提前返回 | `let-else` | Rust 1.65+ |
| 复杂条件匹配 | `match` + 守卫 | 守卫提供额外条件 |
| 匹配并捕获整体 | `@` 绑定 | 同时获得整体和部分 |

---

## 与 Java/Go 对比

| 特性 | Rust | Java (14+) | Go |
|------|------|-----------|-----|
| 模式匹配 | `match`（穷举） | `switch` 表达式 | `switch` |
| 解构 | 元组、结构体、枚举 | record pattern (预览) | ❌ |
| 守卫 | ✅ `if` 条件 | ❌ | ❌ |
| 绑定 | `@` | ❌ | ❌ |
| 穷举检查 | ✅ 编译期强制 | 部分（sealed class） | ❌ |

---

## 最佳实践

1. **优先用 `match` 处理枚举**——编译器的穷举检查是最好的安全网
2. **简单场景用 `if let`**——比 match 简洁得多
3. **善用 `@` 绑定**——既匹配又捕获，避免二次取值
4. **守卫用于额外条件**——模式无法表达的条件放在守卫中
5. **`_` 忽略不需要的值**——明确表示"我不关心这个"

## 配套代码

```powershell
cargo run -p advanced_features --example pattern_matching_tour
```
