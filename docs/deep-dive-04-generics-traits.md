# 专题四：泛型与 Trait

泛型和 Trait 是 Rust 实现代码复用和抽象的核心工具。它们让你在编译期获得最大的类型安全，同时保持零运行时开销。

配套代码：

```powershell
cargo run -p advanced_features --example generics_tour
```

---

## 1. 泛型基础

### 泛型函数

```rust
fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() { return None; }
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest { largest = item; }
    }
    Some(largest)
}

println!("{:?}", largest(&[3, 1, 4, 1, 5]));     // Some(5)
println!("{:?}", largest(&["c", "a", "b"]));      // Some("c")
```

### 类型约束

```rust
// 简写：T 必须实现 Display
fn print<T: Display>(item: T) { println!("{}", item); }

// where 子句（约束多时更清晰）
fn process<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Display + Clone,
{
    format!("{} | {}", t, u)
}
```

### 约束越小越好

```rust
// ✅ 只需要比较就用 PartialOrd
fn compare<T: PartialOrd>(a: &T, b: &T) -> bool { a > b }

// ❌ 不要加不必要的约束
fn compare<T: Display + Hash + Clone + PartialOrd>(a: &T, b: &T) -> bool { a > b }
```

---

## 2. 泛型结构体

### 定义

```rust
struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T> {
    fn new(items: Vec<T>) -> Self { Self { items } }
    fn len(&self) -> usize { self.items.len() }
    fn first(&self) -> Option<&T> { self.items.first() }
}
```

### 带约束的 impl 块

```rust
// 只有 T: Ord + Copy 时才提供 max_item
impl<T> Container<T>
where
    T: Ord + Copy,
{
    fn max_item(&self) -> Option<T> {
        self.items.iter().copied().max()
    }
}
```

---

## 3. 性能：单态化

Rust 使用**单态化**实现泛型——编译时为每种具体类型生成独立代码：

```rust
let int_c = Container::new(vec![1, 2, 3]);      // 编译为 Container<i32>
let str_c = Container::new(vec!["a", "b"]);      // 编译为 Container<&str>
```

这意味着：

| 特性 | 说明 |
|------|------|
| 性能 | 与手写具体类型**完全相同**，零运行时开销 |
| 二进制大小 | 每种类型生成独立代码，可能增大二进制 |
| 编译时间 | 类型多时编译时间增长 |

### 与 Java 对比

| 特性 | Rust 单态化 | Java 类型擦除 |
|------|------------|--------------|
| 运行时开销 | 零 | 装箱/拆箱开销 |
| 二进制大小 | 较大 | 较小 |
| 类型信息 | 编译时确定 | 运行时擦除 |

---

## 4. Trait 定义与实现

### 基本语法

```rust
trait Summary {
    fn summary(&self) -> String;        // 必须实现

    fn category(&self) -> &'static str { // 默认方法（可选覆盖）
        "可摘要对象"
    }
}

impl Summary for Point {
    fn summary(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
    // category() 使用默认实现
}
```

### Trait Bound

```rust
// 简写
fn notify(item: &impl Summary) -> String { ... }

// 完整形式
fn notify<T: Summary>(item: &T) -> String { ... }

// 多约束
fn notify<T: Summary + Display>(item: &T) -> String { ... }

// where 子句
fn notify<T>(item: &T) -> String
where
    T: Summary + Display + Clone,
{ ... }
```

---

## 5. Trait Object 与动态分发

### 静态分发 vs 动态分发

```rust
// 静态分发：编译时确定类型，零开销
fn print_summary(item: &impl Summary) {
    println!("{}", item.summary());
}

// 动态分发：运行时通过 vtable 查找
fn print_dynamic(item: &dyn Summary) {
    println!("{}", item.summary());
}
```

### 异构集合

```rust
let items: Vec<Box<dyn Summary>> = vec![
    Box::new(Point { x: 1, y: 2 }),
    Box::new(Article { ... }),
];
for item in &items {
    println!("{}", item.summary());
}
```

### 选择指南

| 场景 | 推荐 | 原因 |
|------|------|------|
| 类型在编译时确定 | `impl Trait`（静态） | 零开销，可内联 |
| 异构集合 | `dyn Trait`（动态） | 不同类型放入同一 Vec |
| 性能敏感 | 静态 | 避免 vtable 查找 |
| 二进制大小敏感 | 动态 | 避免单态化代码膨胀 |

---

## 6. Trait 组合替代继承

Rust 没有类继承，用 trait 组合实现多态：

```rust
trait Drawable { fn draw(&self) -> String; }
trait Resizable { fn resize(&mut self, factor: f64); }

struct Circle { radius: f64 }
impl Drawable for Circle { ... }
impl Resizable for Circle { ... }

fn draw_all(shapes: &[&dyn Drawable]) -> Vec<String> {
    shapes.iter().map(|s| s.draw()).collect()
}
```

### 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 多态方式 | trait 组合 | 类继承 + 接口 | 接口组合 |
| 继承 | ❌ 无 | ✅ 单继承 | ❌ 无 |
| 代码复用 | trait 默认方法 | 基类方法 | 组合/嵌入 |
| 泛型性能 | 单态化（零开销） | 类型擦除（有开销） | 类型参数 |

---

## 最佳实践

1. **约束越小越好**——只约束你实际需要的 trait
2. **优先用 `impl Trait` 参数**——简单且零开销
3. **需要异构集合时用 `Box<dyn Trait>`**
4. **用 trait 组合替代继承层次**
5. **泛型结构体用多个 impl 块**，每个块有不同的约束

## 配套代码

```powershell
cargo run -p advanced_features --example generics_tour
```
