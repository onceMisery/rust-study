# 阶段一：基础语法

本章覆盖 Rust 最基础的核心概念。学完后你应该能读懂并写出常见的 Rust 函数，理解所有权的基本规则，以及使用 Result/Option 处理错误。

配套代码：

```powershell
cargo run -p basic_syntax --example basic_tour
```

---

## 1. 变量声明与不可变性

### let：不可变绑定（默认）

Rust 中所有变量默认都是**不可变的**。这不是限制，而是一种安全保证——一旦赋值，值就不会被意外修改。

```rust
let x = 5;
// x = 6;  // ❌ 编译错误：cannot assign twice to immutable variable
println!("{}", x);  // 5
```

> **为什么默认不可变？** 在大型项目中，如果某个变量在多处被修改，很难追踪变化的时机和原因。不可变默认强制开发者明确标记"哪些值会变化"，减少隐蔽的 bug。

### mut：可变绑定

如果确实需要修改，必须显式加 `mut`：

```rust
let mut total = 0;
total += 1;
total += 2;
println!("{}", total);  // 3
```

> `mut` 是 mutable 的缩写。反义词是 immutable（不可变的）。

### 遮蔽（Shadowing）

遮蔽是用同名的新绑定覆盖旧绑定，**不是修改原变量**。遮蔽允许改变类型：

```rust
let value = 5;           // i32
let value = value * 2;   // 遮蔽：新的 i32
let value = "hello";     // 遮蔽：类型变成了 &str
```

实际应用场景：逐步转换数据时保持变量名语义一致。

```rust
let port = "8080";                // &str
let port: u16 = port.parse().unwrap();  // u16
// 两个 port 语义相同，但类型不同
```

对应库函数演示：

```rust
println!("{}", basic_syntax::immutable_then_shadow(5));
// 输出: 原始值: 5, 遮蔽后: 10

println!("{:?}", basic_syntax::shadowing_type_change(" 42 "));
// 输出: Ok(420)
```

**第一个函数 `immutable_then_shadow(5)` 的执行过程：**

```rust
pub fn immutable_then_shadow(input: i32) -> String {
    let value = input;        // value = 5（不可变绑定）
    let value = value * 2;    // 遮蔽：创建新的 value = 10（旧的 value 被覆盖）
    format!("原始值: {input}, 遮蔽后: {value}")
}
```

- 第一步：`let value = input` 把参数 5 绑定到 `value`，此时 `value` 是 i32 类型，值为 5。
- 第二步：`let value = value * 2` 用同名变量遮蔽——这不是修改原来的 `value`，而是创建了一个全新的 `value`，值是 5 × 2 = 10。
- `format!` 宏中 `{input}` 引用的是函数参数（始终是 5），`{value}` 引用的是遮蔽后的新变量（10），所以输出 `原始值: 5, 遮蔽后: 10`。

**第二个函数 `shadowing_type_change(" 42 ")` 的执行过程：**

```rust
pub fn shadowing_type_change(raw: &str) -> Result<u32, ParseIntError> {
    let value = raw.trim();        // value: &str = "42"（去空白）
    let value: u32 = value.parse()?;  // 遮蔽：value: u32 = 42（类型从 &str 变成了 u32）
    let value = value * 10;        // 遮蔽：value: u32 = 420
    Ok(value)
}
```

- 第一步：`raw.trim()` 去掉前后空白，`value` 的类型是 `&str`，值为 `"42"`。
- 第二步：`value.parse()?` 把字符串解析成 `u32`，遮蔽后的 `value` **类型变成了 `u32`**，值为 42。注意 `?` 会在解析失败时提前返回错误。
- 第三步：`value * 10` 计算 42 × 10 = 420，再次遮蔽产生新的 `value`。
- 最终返回 `Ok(420)`，所以 `{:?}`（Debug 格式）打印出 `Ok(420)`。

这两个例子展示了遮蔽的两个核心能力：**不修改原变量就能用新值覆盖旧名**，以及**允许在遮蔽时改变类型**（从 `&str` 到 `u32`）。

### 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 变量默认行为 | 不可变（`let`） | 可变（需 `final` 固定） | 可变（无不可变绑定） |
| 可变声明 | `let mut x = 1` | `int x = 1` | `x := 1` |
| 遮蔽 | ✅ 允许，可改变类型 | ❌ 同一作用域不允许同名 | ❌ 同一作用域不允许同名 |
| 常量 | `const` / `static` | `static final` | `const` |

### 最佳实践

- **能不用 `mut` 就不用 `mut`**。只在确实需要累加、修改状态时使用。
- **遮蔽适合数据转换链**：字符串 → 清洗 → 解析 → 验证，每步用同名变量。
- **不要为了规避编译错误而加 `mut`**。先理解所有权和借用规则。

---

## 2. const 和 static 常量

### const：编译期常量

`const` 值在编译时确定，没有内存地址。每次使用时会在代码中内联替换。

```rust
const MAX_RETRIES: u32 = 3;
const PI: f64 = 3.14159265358979;

println!("最大重试: {}", MAX_RETRIES);  // 3
```

规则：
- 必须标注类型
- 只能是常量表达式（不能调用函数、不能运行时计算）
- 可以在任何作用域声明（包括函数内部）

### static：静态变量

`static` 有固定的内存地址，全局存活：

```rust
static APP_NAME: &str = "Rust 教程项目";
static VERSION: &str = "0.1.0";

println!("应用: {}", APP_NAME);
```

不可变 `static` 可以安全读取。可变 `static` 需要 `unsafe`，因为无法保证并发安全。

### const vs static 对比

| 特性 | `const` | `static` |
|------|---------|----------|
| 内存地址 | 无（内联到使用处） | 有固定地址 |
| 适用场景 | 数值常量、配置值 | 大型常量数据、全局标识 |
| 类型标注 | 必须 | 必须 |
| 可变性 | 不可变 | 可变（需 unsafe） |

### 与 Java/Go 对比

| 特性 | Rust `const` | Rust `static` | Java `static final` | Go `const` |
|------|-------------|--------------|---------------------|-----------|
| 编译期确定 | ✅ | ✅ | 部分（初始化可在运行时） | ✅ |
| 内存地址 | ❌ | ✅ | ✅ | ❌ |

### 配套代码

```rust
println!("{}", basic_syntax::const_and_static_demo());
// 输出: 应用: Rust 教程项目, 版本: 0.1.0, 最大重试: 3, PI ≈ 3.1416
```

---

## 3. 内存管理：栈与堆

### 栈（Stack）

栈存储固定大小的数据。速度快，LIFO（后进先出）结构。

```rust
let x: i32 = 42;       // 4 字节，直接放栈上
let y: bool = true;     // 1 字节
let z: char = 'A';      // 4 字节
```

特点：
- 分配和释放都是 O(1)
- 函数返回时自动弹出栈帧
- 所有标量类型（i32、f64、bool、char）都在栈上

### 堆（Heap）

堆存储动态大小的数据。`String`、`Vec<T>` 的真实数据在堆上，栈上只保存指针、长度和容量。

```rust
let s = String::from("hello");  // 栈: [ptr, len=5, cap=5]  堆: [h,e,l,l,o]
let v = vec![1, 2, 3, 4, 5];   // 栈: [ptr, len=5, cap=5]  堆: [1,2,3,4,5]
```

### 栈 vs 堆对比

| 特性 | 栈 | 堆 |
|------|-----|-----|
| 速度 | 极快（移动指针） | 较慢（需要分配器） |
| 大小 | 编译时固定 | 运行时可变 |
| 自动释放 | ✅ 函数返回时 | ✅ 所有者离开作用域时 |
| 典型类型 | `i32`、`bool`、元组 | `String`、`Vec<T>`、`Box<T>` |

### 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 内存管理 | 所有权，编译期确定释放时机 | GC 自动回收 | GC 自动回收 |
| 性能开销 | 零运行时开销 | GC 暂停 | GC 暂停 |
| 灵活性 | 需要理解所有权规则 | 无需关心，但有运行时成本 | 无需关心，但有运行时成本 |

---

## 4. String 与 &str

Rust 的字符串有两种主要类型：`String`（有所有权的堆字符串）和 `&str`（借用的字符串切片）。理解它们的区别是 Rust 入门的关键一步。

### 4.1 基础定义

#### String：可变、有所有权的堆字符串

```rust
let mut s = String::from("hello");
s.push_str(" world");
println!("{}", s);  // hello world
```

#### &str：不可变的字符串切片（借用）

```rust
let s: &str = "hello";           // 字符串字面量
let sub: &str = &s[0..3];        // 切片
```

### 4.2 内存布局差异

`String` 和 `&str` 在内存中的存储方式完全不同，这决定了它们的使用场景：

```
String（3 个字在栈上 + 真实数据在堆上）:
┌──────────────────────┐     ┌─────────────────┐
│ ptr ───────────────────────▶ h e l l o         │  堆
│ len: 5                 │     └─────────────────┘
│ cap: 5                 │
└──────────────────────┘
        栈（24 字节）

&str（2 个字在栈上，指向已有数据）:
┌──────────────────────┐
│ ptr ──── 指向某处       │     可以是：堆上的 String 数据、
│ len: 5                 │     栈上的数组、或二进制段中的字面量
└──────────────────────┘
        栈（16 字节）
```

实际演示：

```rust
fn main() {
    // String: 堆上分配，栈上保存指针/长度/容量
    let owned = String::from("hello");
    println!("String: '{}' (len={}, 栈上大小={}字节)",
        owned, owned.len(), std::mem::size_of_val(&owned));
    // 输出: String: 'hello' (len=5, 栈上大小=24字节)

    // &str 字面量: 数据嵌入二进制文件，栈上只有指针+长度
    let literal: &str = "hello";
    println!("&str: '{}' (len={}, 栈上大小={}字节)",
        literal, literal.len(), std::mem::size_of_val(&literal));
    // 输出: &str: 'hello' (len=5, 栈上大小=16字节)

    // &str 切片: 指向 String 内部的某段数据
    let slice: &str = &owned[1..4];  // "ell"
    println!("slice: '{}' (len={}, 栈上大小={}字节)",
        slice, slice.len(), std::mem::size_of_val(&slice));
    // 输出: slice: 'ell' (len=3, 栈上大小=16字节)
}
```

关键结论：

| 类型 | 栈上大小 | 数据位置 | 说明 |
|------|---------|---------|------|
| `String` | 24 字节（ptr + len + cap） | 堆 | 拥有数据，负责释放 |
| `&str` | 16 字节（ptr + len） | 借用他处 | 不拥有数据，不负责释放 |

### 4.3 函数参数：&str vs String

这是初学者最常困惑的地方。核心原则：**函数只需要读取字符串时，参数用 `&str`。**

#### 反例：用 String 作参数

```rust
// ❌ 不推荐：强制调用方转移所有权或 clone
fn greet(name: String) {
    println!("Hello, {}!", name);
}

fn main() {
    let name = String::from("Alice");
    greet(name.clone());     // 必须 clone 才能在之后继续使用 name
    // greet("Bob");         // ❌ 编译错误！字面量是 &str，不能直接转成 String
    greet("Bob".to_string()); // 必须手动转换，不方便
}
```

#### 正确做法：用 &str 作参数

```rust
// ✅ 推荐：借用，零成本，灵活
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let name = String::from("Alice");
    greet(&name);            // ✅ 传入 &String（自动解引用为 &str）
    greet("Bob");            // ✅ 直接传字面量
    greet(&name[0..3]);      // ✅ 传入切片
    println!("{}", name);    // ✅ name 仍然可用（只是借用，没有转移所有权）
}
```

#### 什么时候参数用 String？

只有当函数需要**拥有**这个字符串（存储、返回、跨线程传递）时才用 `String`：

```rust
// ✅ 正确场景：函数需要拥有数据
fn store_username(raw: &str) -> String {
    // 清洗并存储，返回拥有所有权的 String
    raw.trim().to_lowercase().replace(' ', "_")
}

// ✅ 正确场景：构建数据结构
struct Config {
    name: String,  // 结构体拥有 name 的所有权
}

impl Config {
    fn new(name: &str) -> Self {  // 参数用 &str
        Self { name: name.to_string() }  // 内部转成 String 存储
    }
}
```

#### 参数选择决策树

```
函数需要修改字符串吗？
├─ 是 → &mut String
└─ 否 → 函数需要拥有/存储/返回这个字符串吗？
        ├─ 是 → String（但建议参数仍用 &str，内部 .to_string()）
        └─ 否 → &str ← 大多数情况
```

### 4.4 转换关系详解

String、&str 和字符串字面量之间可以自由转换：

```rust
fn main() {
    // === &str → String（堆分配，有成本） ===
    let s1: String = "hello".to_string();       // 方法 1
    let s2: String = String::from("hello");     // 方法 2（等价）
    let s3: String = "hello".to_owned();        // 方法 3（等价）
    let s4: String = format!("hello {}", 42);   // format! 返回 String

    // === String → &str（零成本借用） ===
    let owned = String::from("hello world");
    let borrowed: &str = &owned;                // 自动 Deref
    let slice: &str = &owned[0..5];             // 切片也是 &str
    let explicit: &str = owned.as_str();        // 显式方法

    // === 字符串字面量 本质上是 &'static str ===
    let lit: &str = "hello";  // 类型是 &'static str
    // 数据嵌入在二进制文件中，程序运行期间始终有效

    // === String → String（克隆，有成本） ===
    let original = String::from("hello");
    let cloned = original.clone();  // 深拷贝堆数据
    // original 和 cloned 是独立的两份数据

    // === 实用转换场景 ===
    let input: &str = "  42  ";
    let trimmed: &str = input.trim();                  // &str → &str
    let parsed: u32 = trimmed.parse().unwrap();         // &str → u32
    let displayed: String = parsed.to_string();          // u32 → String
    let combined: String = format!("值: {}", parsed);   // 任意类型 → String

    println!("{}", combined);  // "值: 42"
}
```

转换成本总结：

| 转换方向 | 方法 | 成本 | 说明 |
|----------|------|------|------|
| `&str` → `String` | `.to_string()` / `String::from()` | 堆分配 | 复制全部数据到堆 |
| `String` → `&str` | `&s` / `.as_str()` | 零成本 | 只是创建引用 |
| `String` → `String` | `.clone()` | 堆分配 | 深拷贝 |
| `&str` → `&str` | `.trim()` / 切片 | 零成本 | 只移动指针 |
| 任意类型 → `String` | `format!()` / `.to_string()` | 堆分配 | 格式化输出 |

### 4.5 实际业务场景

#### 场景一：文本解析（参数用 &str，返回用 String）

```rust
/// 清洗用户名：去空白、转小写、替换空格
fn normalize_username(raw: &str) -> String {
    raw.trim().to_lowercase().replace(' ', "_")
}

// 调用方可以传任何类型的字符串
let user1 = normalize_username(" Alice Chen ");     // 字面量
let name = String::from(" Bob Smith ");
let user2 = normalize_username(&name);              // &String
let user3 = normalize_username(&name[1..8]);        // 切片
```

> 参数用 `&str`：函数只读，不需要拥有输入数据。返回用 `String`：结果是新生成的数据，调用方需要拥有它。

#### 场景二：配置读取（参数用 &str，内部存储为 String）

```rust
struct AppConfig {
    host: String,
    port: u16,
}

impl AppConfig {
    fn from_line(line: &str) -> Option<Self> {
        // line 是临时的，解析后把需要的字段转成 String 存储
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 { return None; }

        let key = parts[0].trim();
        let value = parts[1].trim();

        match key {
            "host" => Some(Self { host: value.to_string(), port: 8080 }),
            _ => None,
        }
    }
}
```

> `line` 参数是临时的（可能来自文件的某一行），解析完成后就不需要了。所以参数用 `&str`，只有真正要保存的字段才转成 `String`。

#### 场景三：API 调用中的字符串拼接

```rust
fn build_url(base: &str, path: &str, query: &str) -> String {
    if query.is_empty() {
        format!("{}{}", base, path)
    } else {
        format!("{}{}?{}", base, path, query)
    }
}

// 调用灵活
let url = build_url("https://api.example.com", "/users", "page=1&limit=10");
let url2 = build_url("https://example.com", "/health", "");
```

> 所有参数都是 `&str`（只读），返回值是 `String`（新构建的 URL）。

### 4.6 常见错误及解决方案

#### 错误 1：对 String 使用字节索引

```rust
let s = String::from("你好世界");
// let c = s[0];  // ❌ 编译错误：String 不支持索引
// let c = &s[0..1];  // ❌ 运行时 panic：1 不是字符边界（'你' 占 3 字节）

// ✅ 正确做法：用 chars() 遍历
let first_char: char = s.chars().next().unwrap();  // '你'
let char_count = s.chars().count();                 // 4

// ✅ 如果确实需要按位置取子串
let first_two: String = s.chars().take(2).collect();  // "你好"
```

**原因**：Rust 字符串是 UTF-8 编码，每个字符占 1-4 字节。索引是按字节的，很容易切到字符中间。

#### 错误 2：混淆 len() 和字符数

```rust
let s = "hello";
println!("len={}", s.len());               // 5（字节数）
println!("chars={}", s.chars().count());    // 5（字符数）

let s = "你好";
println!("len={}", s.len());               // 6（字节数！每个中文占 3 字节）
println!("chars={}", s.chars().count());    // 2（字符数）
```

**记住**：`.len()` 返回的是**字节数**，不是字符数。需要字符数时用 `.chars().count()`。

#### 错误 3：在不需要所有权的地方使用 String

```rust
// ❌ 不推荐：不必要地转移所有权
fn print_name(name: String) {
    println!("{}", name);
}
let name = String::from("Alice");
print_name(name);
// println!("{}", name);  // ❌ name 已经移动，不能再使用

// ✅ 推荐：借用
fn print_name(name: &str) {
    println!("{}", name);
}
let name = String::from("Alice");
print_name(&name);
println!("{}", name);  // ✅ name 仍然可用
```

#### 错误 4：在循环中反复分配 String

```rust
// ❌ 每次循环都创建新 String
let words = vec!["hello", "world", "rust"];
let mut result = String::new();
for word in &words {
    result = result + " " + word;  // 每次拼接都创建新 String
}

// ✅ 推荐：预分配或用 push_str
let mut result = String::with_capacity(50);  // 预分配
for word in &words {
    result.push(' ');
    result.push_str(word);  // 在原 String 上追加，不重新分配
}

// ✅ 更简洁：用 join
let result = words.join(" ");
```

#### 错误 5：忘记 String 和 &str 的比较需要显式处理

```rust
let owned = String::from("hello");
let borrowed: &str = "hello";

// ✅ 可以直接比较（Rust 自动解引用）
assert!(owned == borrowed);
assert_eq!(owned, borrowed);

// ✅ 也可以比较切片
assert!(owned.starts_with("hel"));
assert!(owned.contains("ell"));
```

### 4.7 性能差异

```rust
use std::time::Instant;

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";
    let iterations = 100_000;

    // 测试 1：&str 操作（零成本借用）
    let start = Instant::now();
    for _ in 0..iterations {
        let _trimmed = text.trim();         // 零成本
        let _first = text.split(' ').next(); // 零成本
    }
    let str_time = start.elapsed();

    // 测试 2：String 操作（每次堆分配）
    let start = Instant::now();
    for _ in 0..iterations {
        let _owned = text.to_string();          // 堆分配
        let _upper = text.to_uppercase();       // 堆分配
    }
    let string_time = start.elapsed();

    println!("&str 操作: {:?}", str_time);       // 约 1-2ms
    println!("String 操作: {:?}", string_time);   // 约 20-50ms
    // String 操作通常比 &str 操作慢 10-50 倍
}
```

**性能原则**：

| 操作 | 成本 | 说明 |
|------|------|------|
| `&str` 切片/trim/split | 零成本 | 只移动指针 |
| `.len()` / `.is_empty()` | O(1) | 直接读取元数据 |
| `.to_string()` / `.to_owned()` | O(n) | 复制全部字节到堆 |
| `.clone()` | O(n) | 深拷贝堆数据 |
| `format!()` | O(n) | 堆分配 + 格式化 |
| `.contains()` / `.find()` | O(n) | 需要遍历 |

### 4.8 配套代码

```rust
println!("{}", basic_syntax::string_operations_demo());
// trim='Hello, Rust!', lower='hello, rust!', replace='hello, world!'
// contains=true, bytes=12, chars=12

println!("{}", basic_syntax::string_conversion_demo());
// borrowed=hello, combined=hello world rust

println!("{}", basic_syntax::normalize_username(" Alice Chen "));
// alice_chen
```

### 4.9 最佳实践总结

| 场景 | 推荐类型 | 原因 |
|------|----------|------|
| 函数参数（只读） | `&str` | 灵活，零成本，调用方不需要 clone |
| 函数参数（需要存储） | `&str`，内部 `.to_string()` | 转换在函数内部完成，调用方更简洁 |
| 函数参数（需要修改） | `&mut String` | 明确表达“会修改”的意图 |
| 函数返回值 | `String` | 调用方获得所有权 |
| 结构体字段 | `String`（大多数情况） | 结构体拥有数据 |
| 结构体字段（引用外部数据） | `&'a str` | 需要生命周期标注 |
| 字符串字面量 | `&str` / `&'static str` | 数据嵌入二进制，零成本 |
| 临时拼接 | `format!()` | 返回 String，代码清晰 |
| 高性能拼接 | `String::with_capacity` + `push_str` | 避免反复分配 |

---

## 5. 元组与数组

### 元组：不同类型的固定组合

```rust
let tuple: (&str, u16, bool) = ("Rust", 2015, true);
let (name, year, stable) = tuple;  // 解构
println!("{}: {}", name, year);

// 按索引访问
let point = (3.5, 7.2);
println!("x={}, y={}", point.0, point.1);
```

### 数组：同类型、固定长度

```rust
let numbers: [i32; 5] = [10, 20, 30, 40, 50];
let zeros = [0; 3];              // [0, 0, 0]
println!("长度: {}", numbers.len());
println!("第一个: {}", numbers[0]);  // 索引访问（越界会 panic）
println!("安全访问: {:?}", numbers.get(10));  // None（不会 panic）
```

### 元组 vs 数组对比

| 特性 | 元组 `(T, U, V)` | 数组 `[T; N]` |
|------|-------------------|---------------|
| 元素类型 | 可以不同 | 必须相同 |
| 长度 | 固定 | 固定 |
| 内存位置 | 栈 | 栈 |
| 使用场景 | 临时组合少量不同值 | 同类型固定数据集 |
| 可变长度替代 | `struct` | `Vec<T>` |

### 配套代码

```rust
println!("{}", basic_syntax::describe_tuple(("Rust", 2015, true)));
// Rust 在 2015 年发布: true

println!("{}", basic_syntax::array_demo());
// numbers=[10, 20, 30, 40, 50], 长度=5, zeros=[0, 0, 0]

println!("{:?}", basic_syntax::array_slice_demo(&[3, 1, 4, 1, 5]));
// (5, 14, Some(5))
```

---

## 6. 所有权

### 三条核心规则

1. **每个值同一时间只有一个所有者。**
2. **赋值、传参、返回值可能发生移动（Move）。**
3. **所有者离开作用域时，值自动释放。**

```rust
let s1 = String::from("hello");
let s2 = s1;           // s1 的所有权移动到 s2
// println!("{}", s1);  // ❌ 编译错误：s1 已经无效
println!("{}", s2);     // ✅ s2 是新的所有者
```

### 移动语义

当 `String` 赋值给另一个变量时，不会复制堆上的数据，只复制栈上的元数据（指针、长度、容量），并让旧变量失效：

```
s1: [ptr, len=5, cap=5] ─→ [h, e, l, l, o]  （移动前）
s2: [ptr, len=5, cap=5] ─→ [h, e, l, l, o]  （移动后，s1 无效）
```

这避免了双重释放（double free）。

### 克隆：显式深拷贝

如果需要两份独立数据，用 `clone`：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("{} {}", s1, s2);  // 两者都可用
```

> ⚠️ `clone` 会复制堆上的全部数据，有性能成本。新手常见误区是遇到所有权错误就到处 `.clone()`——更好的做法是先判断函数到底需不需要取得所有权。

### 配套代码

```rust
let text = String::from("ownership");
println!("{}", basic_syntax::replace_with_length(text));
// text 已移动，不能再使用

println!("{:?}", basic_syntax::clone_then_keep_original("borrow"));
// ("borrow", 6) — 原值保留
```

### 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 赋值行为 | 移动（堆类型）/ 复制（Copy 类型） | 引用复制（共享同一对象） | 值复制 / 切片头复制 |
| 内存释放 | 编译期自动释放 | GC 运行时回收 | GC 运行时回收 |
| 双重释放 | 编译期阻止 | 不可能（GC 管理） | 不可能（GC 管理） |

---

## 7. 枚举与模式匹配

### 枚举定义

Rust 的枚举比 Java/Go 的枚举强大得多——每个变体可以携带不同类型的数据：

```rust
enum Shape {
    Circle(f64),                         // 元组变体
    Rectangle(f64, f64),                 // 元组变体
    Triangle { base: f64, height: f64 }, // 结构体变体
}
```

### match：穷举所有可能

```rust
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => 3.14159 * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }
}
```

`match` 要求覆盖所有变体，漏掉会编译报错。这是 Rust 的安全保证。

### if let：只关心一种情况

```rust
let value: Option<i32> = Some(42);
if let Some(v) = value {
    println!("找到值: {}", v);
}
```

### 配套代码

```rust
println!("{}", basic_syntax::enum_demo());
// 圆形, 半径=3: 面积=28.3; 矩形, 4×5: 面积=20.0; 三角形, 底=6, 高=3: 面积=9.0

println!("{}", basic_syntax::grade(85));  // B
println!("{}", basic_syntax::if_let_demo(Some(42)));  // 找到值: 42
```

---

## 8. 结构体

### 三种结构体

```rust
// 1. 经典结构体：每个字段有名字
struct User {
    name: String,
    age: u32,
    active: bool,
}

// 2. 元组结构体：按位置访问
struct Point(f64, f64);

// 3. 单元结构体：没有字段（通常用于 trait 实现）
struct Marker;
```

### 方法实现

```rust
impl User {
    // 关联函数（构造函数）：用 User::new(...) 调用
    fn new(name: &str, age: u32) -> Self {
        Self { name: name.to_string(), age, active: true }
    }

    // 方法：通过实例调用
    fn is_adult(&self) -> bool { self.age >= 18 }

    // 可变方法
    fn deactivate(&mut self) { self.active = false; }

    // 消费方法：取得所有权
    fn into_name(self) -> String { self.name }
}
```

### 三种 self 参数

| 参数 | 说明 | 调用后 |
|------|------|--------|
| `&self` | 只读借用 | 原值可用 |
| `&mut self` | 可变借用 | 原值已修改 |
| `self` | 取得所有权 | 原值不可用 |

### 配套代码

```rust
let mut user = basic_syntax::User::new("Alice", 25);
println!("成年? {}", user.is_adult());  // true
user.deactivate();
println!("{:?}", user);  // User { name: "Alice", age: 25, active: false }
```

---

## 9. 所有权与结构体的交互

### 结构体字段的所有权

结构体拥有其字段的所有权。当结构体被移动时，所有字段一起移动：

```rust
let user = User::new("Bob", 30);
let user2 = user;  // user 整体移动，所有字段一起转移
// println!("{}", user.name);  // ❌ user 已无效
```

### 结构体中的借用

如果结构体字段是引用，必须标注生命周期：

```rust
struct Excerpt<'a> {
    part: &'a str,  // 引用字段需要生命周期标注
}
```

这意味着 `Excerpt` 不能比它引用的字符串活得更久。

### 字段部分移动

可以只移动结构体的某个字段：

```rust
let user = User::new("Alice", 25);
let name = user.name;  // 只移动 name 字段
// println!("{}", user.name);  // ❌ name 已移动
// println!("{}", user.age);   // ✅ 其他字段仍可用
```

---

## 10. Copy/Move 语义

### 什么类型实现 Copy？

实现 `Copy` trait 的类型在赋值时是**复制**而非移动：

| 类型 | Copy? | 说明 |
|------|-------|------|
| `i32`、`u64`、`f64` 等 | ✅ | 栈上固定大小 |
| `bool`、`char` | ✅ | 栈上固定大小 |
| 元组（所有元素都 Copy） | ✅ | `(i32, bool)` ✅，`(i32, String)` ❌ |
| 数组（元素 Copy） | ✅ | `[i32; 5]` ✅ |
| `String`、`Vec<T>` | ❌ | 堆数据，移动语义 |
| `&T` | ✅ | 引用本身是 Copy |

### 判断规则

**如果一个类型的所有字段都实现了 Copy，那这个类型也可以实现 Copy。** `String` 没有实现 Copy，因为它包含堆指针，复制指针会导致双重释放。

### 配套代码

```rust
println!("{}", basic_syntax::copy_trait_demo());
// a=42, b=42, c=42 — 都能用，因为 i32 实现了 Copy
```

---

## 11. if 表达式与 match 模式匹配

### if/else 是表达式

```rust
let label = if number % 2 == 0 { "even" } else { "odd" };
```

### match 强制穷举

```rust
let grade = match score {
    90..=100 => "A",
    80..=89 => "B",
    70..=79 => "C",
    60..=69 => "D",
    _ => "F",  // 必须处理所有情况
};
```

### if/else vs match 对比

| 特性 | `if/else` | `match` |
|------|-----------|---------|
| 适用场景 | 简单条件判断 | 模式匹配、枚举变体 |
| 穷举检查 | ❌ 无 | ✅ 编译器强制 |
| 性能 | 无差异 | 无差异（编译器优化） |
| 可读性 | 条件简单时更好 | 多分支、枚举时更好 |

### 配套代码

```rust
println!("{}", basic_syntax::classify_number(7));   // positive-odd
println!("{}", basic_syntax::grade(85));             // B
println!("{}", basic_syntax::control_flow_samples(4)); // [0, 1, 2, 3, 6, 4, 2]
```

---

## 12. 函数与高阶函数

### 函数定义

```rust
fn factorial(number: u32) -> u32 {
    let mut result = 1;
    let mut current = number;
    while current > 1 {
        result *= current;
        current -= 1;
    }
    result  // 最后一行无分号 = 返回值
}
```

### 高阶函数：函数作为参数

```rust
fn apply_operation(value: i32, operation: fn(i32) -> i32) -> i32 {
    operation(value)
}

fn double(x: i32) -> i32 { x * 2 }
fn square(x: i32) -> i32 { x * x }

println!("{}", apply_operation(3, double));  // 6
println!("{}", apply_operation(5, square));  // 25
```

### 语句 vs 表达式

```rust
let x = 5;                    // 语句
let y = { let a = 3; a + 2 }; // 表达式块，y = 5
let z = { let a = 3; a + 2; };// 语句（分号），z = ()
```

### 配套代码

```rust
println!("5! = {}", basic_syntax::factorial(5));  // 120
println!("{:?}", basic_syntax::min_max(&[3, 1, 4, 1, 5]));
// (Some(1), Some(5))
```

---

## 13. 函数返回值与所有权

### 返回值转移所有权

函数返回值时，所有权从函数内部转移到调用方：

```rust
fn create_string() -> String {
    let s = String::from("hello");
    s  // 所有权移动给调用方
}
let owned = create_string();  // owned 获得所有权
```

### 返回借用需要生命周期

如果函数返回引用，编译器需要知道引用指向的数据活多久：

```rust
fn first_word(text: &str) -> &str {
    // 只有一个输入引用，生命周期自动推导
    // 返回值的生命周期 = 输入引用的生命周期
}
```

---

## 14. Result、Option 与 panic

### Option：有值或无值

```rust
fn checked_divide(left: i32, right: i32) -> Option<i32> {
    if right == 0 { None } else { Some(left / right) }
}

println!("{:?}", checked_divide(10, 2));  // Some(5)
println!("{:?}", checked_divide(10, 0));  // None
```

### Result：成功或失败

```rust
fn parse_port(raw: &str) -> Result<u16, ParseIntError> {
    let port = raw.trim();
    let port: u16 = port.parse()?;  // ? 自动处理错误
    Ok(port)
}
```

### panic：不可恢复的错误

```rust
// panic!("程序崩溃了！");  // 只在真正无法继续时使用
```

**使用原则**：

| 场景 | 使用 | 说明 |
|------|------|------|
| 可能失败的操作 | `Option` / `Result` | 调用方决定如何处理 |
| 不应该发生的错误 | `panic!` | 如数组越界（调试期） |
| 快速原型/脚本 | `unwrap()` | 简单但危险 |
| 生产代码 | `?` + 自定义错误 | 优雅处理并传播 |

---

## 15. unwrap 与 ? 操作符

### unwrap：简单但危险

```rust
let value = Some(42).unwrap();       // 42
let value = None::<i32>.unwrap();    // panic!
```

### ? 操作符：优雅的错误传播

`?` 在 `Result` 上使用时，如果出错就提前返回 `Err`，如果成功就取出 `Ok` 的值：

```rust
fn parse_and_validate(raw: &str) -> AppResult<i32> {
    if raw.trim().is_empty() {
        return Err(AppError::EmptyInput);
    }
    let value: i32 = raw.parse()?;  // 自动转换为 AppError
    if value < 0 || value > 100 {
        return Err(AppError::OutOfRange { value, min: 0, max: 100 });
    }
    Ok(value)
}
```

### Option 的链式操作

```rust
let result = "21"
    .trim().parse::<i32>().ok()    // Result → Option
    .map(|n| n * 2)                 // Some(42)
    .filter(|&n| n > 0)             // Some(42)
    .unwrap_or(0);                  // 42
```

### 配套代码

```rust
println!("{:?}", basic_syntax::parse_port_with_shadowing(" 8080 "));
// Ok(8080)

match basic_syntax::parse_and_validate("42") {
    Ok(v) => println!("验证通过: {}", v),
    Err(e) => println!("错误: {}", e),
}
// 验证通过: 42

match basic_syntax::parse_and_validate("200") {
    Ok(v) => println!("验证通过: {}", v),
    Err(e) => println!("错误: {}", e),
}
// 错误: 值 200 超出范围 [0, 100]
```

---

## 16. Error 类型设计

### 自定义错误类型

```rust
#[derive(Debug)]
enum AppError {
    EmptyInput,
    InvalidNumber(ParseIntError),
    OutOfRange { value: i32, min: i32, max: i32 },
}

// 实现 Display：用户友好的错误描述
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::EmptyInput => write!(f, "输入不能为空"),
            AppError::InvalidNumber(e) => write!(f, "数字解析失败: {}", e),
            AppError::OutOfRange { value, min, max } => {
                write!(f, "值 {} 超出范围 [{}, {}]", value, min, max)
            }
        }
    }
}

// 实现 Error trait
impl std::error::Error for AppError {}

// 实现 From：允许 ? 自动转换
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::InvalidNumber(e) }
}
```

### 错误处理策略

| 策略 | 适用场景 | 示例 |
|------|----------|------|
| `Result<T, E>` + `?` | 大多数情况 | 库函数、业务逻辑 |
| `unwrap()` / `expect()` | 快速原型、测试 | 脚本、竞赛代码 |
| `match` 手动处理 | 需要针对每种错误做不同处理 | 用户交互 |
| `anyhow` crate | 应用级错误处理 | Web 服务、CLI 工具 |
| `thiserror` crate | 库级错误类型定义 | 公共库 |

---

## 操作符重载

Rust 通过实现 `std::ops` 中的 trait 来重载运算符：

```rust
#[derive(Debug, Clone, Copy)]
struct Vector2D { x: f64, y: f64 }

impl std::ops::Add for Vector2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

let a = Vector2D { x: 1.0, y: 2.0 };
let b = Vector2D { x: 3.0, y: 4.0 };
let c = a + b;  // Vector2D { x: 4.0, y: 6.0 }
```

常用的可重载运算符：

| 运算符 | trait | 方法 |
|--------|-------|------|
| `+` | `Add` | `add` |
| `-` | `Sub` | `sub` |
| `*` | `Mul` | `mul` |
| `/` | `Div` | `div` |
| `%` | `Rem` | `rem` |
| `-`（一元） | `Neg` | `neg` |

### 配套代码

```rust
println!("{}", basic_syntax::operator_overload_demo());
// a=Vector2D { x: 1.0, y: 2.0 }, b=Vector2D { x: 3.0, y: 4.0 }
// a+b=Vector2D { x: 4.0, y: 6.0 }, |a+b|=7.21
```

---

## 常见内置 trait

| trait | 用途 | 格式符 | derive |
|-------|------|--------|--------|
| `Display` | 用户友好的字符串 | `{}` | ❌ 需手动实现 |
| `Debug` | 开发者调试字符串 | `{:?}` | ✅ |
| `Clone` | 深拷贝 | — | ✅ |
| `Copy` | 浅拷贝（栈数据） | — | ✅（需所有字段 Copy） |
| `Drop` | 离开作用域时执行清理 | — | ❌ 需手动实现 |
| `PartialEq` / `Eq` | 相等比较 | — | ✅ |
| `PartialOrd` / `Ord` | 大小比较 | — | ✅ |
| `Hash` | 可哈希（用于 HashMap key） | — | ✅ |

### 配套代码

```rust
println!("{}", basic_syntax::builtin_traits_demo());
// Debug: User { name: "Bob", age: 30, active: true }
// Display: 用户 Bob (30岁, 活跃=true)
```

---

## 迭代器

### 核心概念

迭代器是 Rust 处理序列数据的核心抽象。`Iterator` trait 只需要实现一个方法：

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

### 常用方法分类

| 类型 | 方法 | 说明 |
|------|------|------|
| **适配器** | `map()`、`filter()`、`take()`、`enumerate()` | 惰性，返回新迭代器 |
| **消费器** | `collect()`、`sum()`、`count()`、`fold()` | 触发计算，产生最终值 |
| **查找** | `find()`、`position()`、`any()`、`all()` | 短路，找到即停 |

### 链式调用

```rust
let result: Vec<i32> = (1..=100)
    .filter(|n| n % 3 == 0)   // 3 的倍数
    .map(|n| n * n)           // 平方
    .take(5)                  // 取前 5 个
    .collect();
// [9, 36, 81, 144, 225]
```

> **迭代器是惰性的**：适配器链不会执行任何计算，直到调用消费器（如 `collect()`）时才真正求值。

### 配套代码

```rust
println!("{:?}", basic_syntax::iterator_chain_demo(&[1, 2, 3, 4, 5, 6]));
// [4, 16, 36]

println!("{}", basic_syntax::iterator_methods_demo());
// sum=55, indexed=["0:Rust", "1:Go", "2:Java"], pairs=[("Alice", 95), ("Bob", 87)]

println!("{}", basic_syntax::iterator_lazy_demo());
// 100 以内 3 的倍数的前 5 个平方: [9, 36, 81, 144, 225]
```

---

## 本章配套代码汇总

运行完整示例：

```powershell
cargo run -p basic_syntax --example basic_tour
```

运行测试：

```powershell
cargo test -p basic_syntax
```

参考源码：[crates/basic_syntax/src/lib.rs](../crates/basic_syntax/src/lib.rs)

## 与 Java、Go 的基础语法对比

| 主题 | Rust | Java | Go |
|------|------|------|-----|
| 变量默认行为 | 不可变 | 可变 | 可变 |
| 类型推断 | `let` 可推断，函数参数必须写类型 | `var` 局部推断 | `:=` 推断 |
| 返回值 | 最后表达式可作为返回值 | 必须显式 `return` | 通常显式 `return` |
| 空值 | `Option<T>` | `null`（容易 NPE） | `nil` |
| 字符串 | UTF-8，`String` vs `&str` | UTF-16，`String` 不可变 | UTF-8，`string` 不可变 |
| 集合 | `Vec`、`HashMap`、`BTreeMap` | 集合生态成熟 | slice、map |
| 错误处理 | `Result` + `?` | 异常机制 | 多返回值 + error |
| 内存管理 | 所有权 | GC | GC |
