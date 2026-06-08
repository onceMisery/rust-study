# 专题一：所有权系统深入

所有权是 Rust 最独特的特性，也是它能在无 GC 的情况下保证内存安全和并发安全的核心机制。

配套代码：

```powershell
cargo run -p advanced_features --example ownership_tour
```

---

## 1. 所有权三条核心规则

### 规则一：每个值同一时间只有一个所有者

```rust
let s1 = String::from("hello");  // s1 拥有这个 String
let s2 = s1;                      // 所有权移动到 s2，s1 不再有效
// println!("{}", s1);            // ❌ 编译错误
println!("{}", s2);               // ✅ s2 是唯一所有者
```

### 规则二：赋值、传参、返回值可能发生移动

```rust
fn take_ownership(text: String) -> usize {
    text.len()  // text 在函数结束时被 drop
}

let s = String::from("hello");
let len = take_ownership(s);  // s 移动进函数
// println!("{}", s);         // ❌ s 已无效
```

### 规则三：所有者离开作用域时自动释放

```rust
{
    let s = String::from("hello");
    // s 在这里可用
}   // s 离开作用域，自动调用 drop，堆内存被释放
```

---

## 2. 移动语义（Move）

### 栈数据 vs 堆数据的赋值行为

| 类型 | 赋值行为 | 原变量 | 原因 |
|------|----------|--------|------|
| `i32`、`bool`、`char` | **复制**（Copy） | 仍可用 | 栈上固定大小，复制成本极低 |
| `String`、`Vec<T>` | **移动**（Move） | 不可用 | 堆数据，避免双重释放 |
| `&T`（引用） | **复制**（Copy） | 仍可用 | 引用本身是栈上的胖指针 |

### 内存布局图解

```
移动前:
  s1: [ptr──┐, len=5, cap=5]     堆: [h, e, l, l, o]
          └───────────────────────▶

移动后:
  s1: [无效]                       堆: [h, e, l, l, o]
  s2: [ptr──┐, len=5, cap=5]         ▲
          └──────────────────────────┘
```

移动时只复制栈上的元数据（24 字节），不复制堆上的实际数据。

### 函数传参的移动

```rust
fn print_length(s: String) {
    println!("长度: {}", s.len());
}   // s 被 drop

let text = String::from("ownership");
print_length(text);     // text 移动进函数
// text 不再可用
```

### 返回值的所有权转移

```rust
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)  // 新建 String，所有权返回调用方
}

let greeting = create_greeting("Rust");  // greeting 获得所有权
println!("{}", greeting);                 // ✅ 可用
```

---

## 3. 克隆（Clone）：显式深拷贝

当确实需要两份独立数据时，使用 `clone`：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();   // 深拷贝：堆数据也被复制
println!("{} {}", s1, s2);  // 两者都可用
```

### Clone 的成本

| 操作 | 成本 | 说明 |
|------|------|------|
| `String::clone()` | O(n) | 复制全部字节 |
| `Vec<T>::clone()` | O(n) | 复制所有元素 |
| `i32` 赋值 | O(1) | Copy，零成本 |

> ⚠️ **新手常见误区**：遇到所有权错误就到处加 `.clone()`。更好的做法是先理解函数到底需不需要取得所有权——如果只需要读取，用 `&str` 或 `&[T]` 借用即可。

### 配套代码

```rust
println!("{}", advanced_features::ownership_move_demo());
// 长度是 5，现在创建新字符串

println!("{}", advanced_features::copy_vs_move_demo());
// a=42, b=42 (Copy); s2=hello (Move)
```

---

## 4. 借用与引用

### 不可变借用 `&T`

```rust
fn sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

let numbers = vec![1, 2, 3, 4, 5];
let total = sum(&numbers);  // 借用，不转移所有权
println!("总和: {}, 原数组: {:?}", total, numbers);  // numbers 仍可用
```

### 可变借用 `&mut T`

```rust
fn append_suffix(text: &mut String, suffix: &str) {
    text.push_str(suffix);
}

let mut title = String::from("Rust");
append_suffix(&mut title, " 入门");
println!("{}", title);  // Rust 入门
```

---

## 5. 借用检查器的两条规则

### 规则一：互斥借用

**任何时刻，要么只有一个可变引用，要么有多个不可变引用。**

```rust
let mut s = String::from("hello");

// ✅ 多个不可变借用可以同时存在
let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2);

// ✅ 不可变借用全部使用后，可以创建可变借用
s.push_str(" world");
```

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s;  // ❌ 不能同时有不可变和可变借用
```

### 规则二：引用必须有效

```rust
let r;
{
    let s = String::from("hello");
    r = &s;
}   // s 被释放
// println!("{}", r);  // ❌ r 指向已释放的数据
```

### NLL：Non-Lexical Lifetimes

Rust 2018+ 使用 NLL，借用持续到**最后一次使用**而非作用域结束：

```rust
let mut data = vec![1, 2, 3];
let r = &data[0];
println!("{}", r);   // r 最后一次使用
// r 在这里失效

data.push(4);        // ✅ 可以修改
println!("{:?}", data);
```

### 常见错误及修复

| 错误信息 | 原因 | 修复 |
|----------|------|------|
| `cannot borrow as mutable` | 存在不可变借用 | 确保不可变借用不再使用后，再创建可变借用 |
| `does not live long enough` | 引用超过数据寿命 | 调整变量声明顺序或改用所有权类型 |
| `cannot move out of borrowed content` | 从借用中移动 | 使用 `.clone()` 或重构 |

### 配套代码

```rust
println!("{}", advanced_features::borrow_rules_demo());
println!("{}", advanced_features::borrow_checker_demo());
println!("{}", advanced_features::nll_demo());
```

---

## 6. 生命周期

### 为什么需要生命周期

当函数返回引用时，编译器必须确认返回的引用不会指向已释放的数据。

### 函数中的生命周期标注

```rust
fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
```

`'a` 表示：返回引用的有效期不超过 `left` 和 `right` 的共同有效期。

### 生命周期省略规则

1. **每个引用参数获得独立生命周期**
2. **只有一个输入生命周期时**，自动赋给输出
3. **有 `&self` 时**，self 的生命周期自动赋给输出

```rust
fn first_word(s: &str) -> &str { ... }     // 规则 2 生效，无需标注
fn method(&self) -> &str { ... }            // 规则 3 生效，无需标注
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str  // 需要标注
```

### 结构体中的生命周期

```rust
struct Excerpt<'a> {
    part: &'a str,  // 必须标注：Excerpt 不能比 part 活得更久
}
```

### 配套代码

```rust
println!("{}", advanced_features::struct_lifetime_demo());
// 摘录: 'Call me Ishmael', level=3
```

---

## 7. 所有权的实际应用场景

### 场景一：所有权链式传递

```rust
fn process_and_return(text: String) -> String {
    let upper = text.to_uppercase();
    format!("[{}]", upper)  // 返回新的 String，所有权传给调用方
}

let result = process_and_return(String::from("hello"));
println!("{}", result);  // [HELLO]
```

### 场景二：借用优于克隆

```rust
// ❌ 不推荐：不必要的克隆
fn print_info(data: Vec<i32>) {
    let cloned = data.clone();
    println!("总和: {}", cloned.iter().sum::<i32>());
}

// ✅ 推荐：借用
fn print_info(data: &[i32]) {
    println!("总和: {}", data.iter().sum::<i32>());
}
```

---

## 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 内存管理 | 所有权，编译期释放 | GC 运行时回收 | GC 运行时回收 |
| 赋值行为 | Move（堆）/ Copy（栈） | 引用复制 | 值/引用复制 |
| 双重释放 | 编译期阻止 | 不可能（GC） | 不可能（GC） |
| 数据竞争 | 编译期阻止 | 运行时可能 | 运行时可能 |
| 性能开销 | 零运行时开销 | GC 暂停 | GC 暂停 |

---

## 最佳实践

1. **先想"谁拥有数据"**，不要急着 clone
2. **参数优先用借用**（`&T`、`&str`、`&[T]`）
3. **返回值用所有权类型**（`String`、`Vec<T>`）
4. **生命周期标注是描述关系**，不是延长寿命
5. **如果标注越来越复杂**，重新设计数据所有权边界

## 配套代码

```powershell
cargo run -p advanced_features --example ownership_tour
```
