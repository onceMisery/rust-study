# 专题九：类型转换与常见 Trait

Rust 的类型系统提供了多种安全的类型转换方式。理解这些转换 trait 是写出 idiomatic Rust 代码的关键。

配套代码：

```powershell
cargo run -p advanced_features --example type_conversions_tour
```

---

## 1. From 与 Into：不会失败的转换

### From trait

```rust
// 标准库已实现：&str -> String
let s: String = String::from("hello");

// 自定义 From
struct Celsius(f64);
impl From<f64> for Celsius {
    fn from(f: f64) -> Self {
        Celsius((f - 32.0) * 5.0 / 9.0)
    }
}

let boiling = Celsius::from(212.0);  // 100°C
```

### Into trait

`Into` 是 `From` 的反向——实现了 `From<T>` 就自动获得 `Into<U>`：

```rust
let s: String = "hello".into();   // &str -> String
let big: u64 = 42u32.into();      // u32 -> u64（小类型到大类型）
```

### 函数参数中的 Into

```rust
fn greet(name: impl Into<String>) {
    let name: String = name.into();
    println!("Hello, {}!", name);
}

greet("Alice");              // &str
greet(String::from("Bob")); // String
```

---

## 2. TryFrom 与 TryInto：可能失败的转换

当转换可能失败（如溢出、格式错误）时使用：

```rust
use std::convert::TryFrom;

let big: i64 = 42;
let small = i32::try_from(big);     // Ok(42)

let too_big: i64 = i64::MAX;
let overflow = i32::try_from(too_big);  // Err(TryFromIntError)
```

### 与 From 的区别

| 特性 | `From` / `Into` | `TryFrom` / `TryInto` |
|------|-----------------|----------------------|
| 是否可能失败 | ❌ 永远不会 | ✅ 可能失败 |
| 返回类型 | `U` | `Result<U, E>` |
| 使用场景 | 无损转换 | 可能有损转换 |
| 示例 | `u32 -> u64` | `i64 -> i32`（可能溢出） |

---

## 3. AsRef 与 AsMut：借用转换

`AsRef` 允许函数接受多种输入类型，都转换为引用：

```rust
fn print_len<T: AsRef<str>>(s: T) -> usize {
    s.as_ref().len()
}

print_len("hello");              // &str
print_len(String::from("world"));  // String
print_len(&String::from("rust")); // &String
```

### AsRef vs Borrow

| 特性 | `AsRef` | `Borrow` |
|------|---------|----------|
| 用途 | 类型转换 | 哈希/比较一致性 |
| 典型场景 | 函数参数泛化 | HashMap 键查找 |
| 实现 | 多数类型 | 与 `Eq`/`Hash`/`Ord` 配合 |

---

## 4. Deref：自动解引用

`Deref` trait 让 Rust 自动在引用类型之间转换：

```rust
let boxed = Box::new(String::from("hello world"));

// 自动 Deref 链：
// Box<String> -> &String -> &str
let s: &str = &boxed;

// 方法调用时自动 Deref：
// Box<String> -> String -> str::len()
let len = boxed.len();
```

### Deref 链示例

```
Box<String>  →  String  →  str
Rc<String>   →  String  →  str
&String      →             str
```

---

## 5. as 关键字

`as` 是简单但不安全的类型转换：

```rust
let big: i64 = 42;
let small = big as i32;      // 可能截断

let float: f64 = 3.14;
let integer = float as i32;  // 3（截断小数）

let byte: u8 = 255;
let overflow = (byte + 1) as u8;  // 0（溢出回绕）
```

> ⚠️ `as` 可能静默截断或溢出。优先用 `TryFrom` / `TryInto`。

### as vs TryFrom

| 特性 | `as` | `TryFrom` |
|------|------|-----------|
| 安全性 | ❌ 可能静默截断 | ✅ 返回 Result |
| 性能 | 零开销 | 可能有检查开销 |
| 适用场景 | 确定不会溢出的简单转换 | 不确定是否安全的转换 |

---

## 6. 常见转换模式

### 数字转换

```rust
let n: u32 = 42;
let big: u64 = n.into();          // 无损：小 → 大
let small: u8 = 42;               // 直接赋值
let back: u32 = small as u32;     // as 关键字
```

### 字符串转换

```rust
let num_str = "123";
let num: i32 = num_str.parse().unwrap_or(0);  // 字符串 → 数字
let back_str = num.to_string();                 // 数字 → 字符串

// Display trait 提供 to_string()
#[derive(Debug)]
struct Point { x: i32, y: i32 }
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
let p = Point { x: 1, y: 2 };
let s = p.to_string();  // "(1, 2)"
```

### 集合转换

```rust
// Vec → HashSet（去重）
let vec = vec![1, 2, 3, 2, 1];
let set: std::collections::HashSet<_> = vec.into_iter().collect();
// {1, 2, 3}

// Vec → BTreeMap
let pairs = vec![("a", 1), ("b", 2)];
let map: std::collections::BTreeMap<_, _> = pairs.into_iter().collect();
```

---

## 7. 类型转换 Trait 选择指南

| Trait | 方向 | 是否可能失败 | 示例 |
|-------|------|-------------|------|
| `From` | `T → U` | ❌ | `String::from("hi")` |
| `Into` | `T → U` | ❌ | `let s: String = "hi".into()` |
| `TryFrom` | `T → U` | ✅ | `i32::try_from(big_i64)` |
| `TryInto` | `T → U` | ✅ | `let s: i32 = big.try_into()` |
| `AsRef` | `&T → &U` | ❌ | `s.as_ref() → &str` |
| `AsMut` | `&mut T → &mut U` | ❌ | 可变借用转换 |
| `Deref` | `&T → &U` | ❌ | `Box<String> → &str` |
| `Display` | `T → String` | ❌ | `x.to_string()` |
| `FromStr` | `&str → T` | ✅ | `"42".parse::<i32>()` |

---

## 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 隐式转换 | 极少（只有 Deref） | 自动装箱/拆箱 | 隐式接口转换 |
| 显式转换 | `From`/`Into`/`as` | 强制类型转换 | `T(v)` |
| 安全转换 | `TryFrom`/`TryInto` | try-catch | 多返回值 |
| 字符串转换 | `Display` + `FromStr` | `toString()` + `parseInt()` | `fmt.Sprintf()` + `strconv` |

---

## 最佳实践

1. **函数参数用 `AsRef<str>`** 接受 `&str` / `String` / `&String`
2. **类型转换用 `From` / `Into`**（不会失败时）
3. **可能失败的转换用 `TryFrom` / `TryInto`**
4. **实现 `From` 自动获得 `Into`**（只需实现一个）
5. **避免滥用 `as` 关键字**——它可能静默截断，用 `TryFrom` 更安全
6. **实现 `Display` 自动获得 `to_string()`**

## 配套代码

```powershell
cargo run -p advanced_features --example type_conversions_tour
```
