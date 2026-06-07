# 阶段二：进阶特性

这一章是 Rust 的核心。基础语法只是"会写 Rust"，进阶特性决定你是否真正理解 Rust 为什么能做到无 GC、内存安全和高性能。

配套代码：

```powershell
cargo run -p advanced_features --example advanced_tour
```

---

## 17. 借用检查器（Borrow Checker）

### 工作原理

借用检查器是 Rust 编译器的核心组件，它在编译期验证所有引用的合法性，而不需要运行时检查。

### 两条核心规则

1. **任何时刻，要么只有一个可变引用 `&mut T`，要么有多个不可变引用 `&T`。不能同时存在。**
2. **引用必须始终有效——不能比它指向的数据活得更久。**

### 规则 1：互斥借用

```rust
let mut s = String::from("hello");

// ✅ 多个不可变借用可以同时存在
let r1 = &s;
let r2 = &s;
println!("{} {}", r1, r2);

// ✅ 不可变借用全部结束后，才能创建可变借用
let r3 = &mut s;
r3.push_str(" world");
```

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &mut s;  // ❌ 编译错误：不能同时有不可变和可变借用
println!("{}", r1);
```

### 规则 2：引用必须有效

```rust
let r;
{
    let s = String::from("hello");
    r = &s;
}  // s 在这里被释放
// println!("{}", r);  // ❌ 编译错误：r 指向已释放的数据
```

### NLL：Non-Lexical Lifetimes

Rust 2018+ 使用 NLL（非词法作用域生命周期），借用持续到**最后一次使用**而非作用域结束：

```rust
let mut data = vec![1, 2, 3];
let r = &data[0];
println!("读取: {}", r);
// r 的最后使用在上面，此后 r 不再有效

data.push(4);  // ✅ 可以在 r 失效后修改
println!("{:?}", data);  // [1, 2, 3, 4]
```

### 常见错误及修复

| 错误 | 原因 | 修复方式 |
|------|------|----------|
| `cannot borrow as mutable` | 已有不可变借用 | 确保不可变借用不再使用后，再创建可变借用 |
| `does not live long enough` | 引用比数据活得更久 | 调整变量声明顺序，或改用所有权类型 |
| `cannot move out of borrowed content` | 试图从借用中移动 | 使用 `clone()` 或重构为所有权传递 |

### 配套代码

```rust
println!("{}", advanced_features::borrow_checker_demo());
// hello + hello
// 修改后: hello world

println!("{}", advanced_features::nll_demo());
// 修改后: [1, 2, 3, 4]
```

### 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 并发安全检查 | 编译期（借用检查器） | 运行时（依赖开发者） | 运行时（依赖开发者） |
| 数据竞争 | 编译期阻止 | 可能运行时发生 | 可能运行时发生 |
| 性能开销 | 零（编译期检查） | 锁、volatile 等运行时开销 | channel、锁等运行时开销 |

---

## 18. 生命周期（Lifetimes）

### 为什么需要生命周期

当函数返回引用时，编译器需要确认返回的引用不会指向已释放的数据。大多数情况下编译器能自动推导，只有复杂场景需要手动标注。

### 函数中的生命周期标注

```rust
fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() { left } else { right }
}
```

`'a` 的含义：**返回引用的有效期不超过 `left` 和 `right` 的共同有效期**。

```rust
let s1 = String::from("long string");
let result;
{
    let s2 = String::from("xyz");
    result = longest(&s1, &s2);
    println!("{}", result);  // ✅ s2 还活着
}
// println!("{}", result);  // ❌ s2 已释放，result 可能无效
```

### 生命周期省略规则

编译器有三条自动推导规则，满足时不需要手动标注：

1. **每个引用参数获得独立的生命周期**：`fn foo(a: &str, b: &str)` → `fn foo<'a, 'b>(a: &'a str, b: &'b str)`
2. **只有一个输入生命周期时**，它被赋给所有输出：`fn first(s: &str) -> &str` 自动合法
3. **有 `&self` 时**，self 的生命周期被赋给所有输出：方法返回引用时自动合法

### 结构体中的生命周期

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,  // 引用字段必须标注生命周期
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { 3 }
    fn announce_and_return(&self, announcement: &str) -> &str {
        // 规则 3 生效：返回 self 的生命周期
        self.part
    }
}
```

### 'static 生命周期

`'static` 表示引用在整个程序运行期间都有效。字符串字面量就是 `'static`：

```rust
let s: &'static str = "hello";  // 嵌入在二进制文件中
```

> ⚠️ 不要随便写 `'static`。它意味着"永远存活"，大多数场景不需要。

### 配套代码

```rust
println!("{}", advanced_features::lifetime_elision_demo());
// first='hello', longest='much longer text'

println!("{}", advanced_features::struct_lifetime_demo());
// 摘录: 'Call me Ishmael', level=3

println!("{}", advanced_features::static_lifetime_demo());
// 这个字符串活在程序的全部生命周期里
```

### 最佳实践

- **不要把生命周期当语法补丁**。如果标注越来越复杂，说明数据所有权边界需要重新设计。
- **大多数代码不需要手动标注**。只有返回引用且多个输入引用时才需要。
- **不要随便写 `'static`**。

---

## 19. 泛型（Generics）

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

println!("{:?}", largest(&[3, 1, 4, 1, 5]));  // Some(5)
println!("{:?}", largest(&["a", "z", "m"]));  // Some("z")
```

### 类型参数与约束

泛型不是"什么都能做"。如果需要比较大小，需要 `PartialOrd`；如果需要打印，需要 `Display`：

```rust
fn display_pair<T: Display, U: Display>(a: T, b: U) -> String {
    format!("({}, {})", a, b)
}
```

### 约束越小越好

```rust
// ✅ 只需要排序就用 Ord
fn sort_and_get<T: Ord>(items: &mut [T]) { items.sort(); }

// ❌ 不要加一堆不需要的约束
fn print_and_sort<T: Display + Ord + Clone + Hash>(items: &mut [T]) { ... }
```

### 配套代码

```rust
println!("{}", advanced_features::display_pair("Rust", 2015));
// (Rust, 2015)

println!("{:?}", advanced_features::largest(&[3, 1, 4, 1, 5, 9]));
// Some(9)
```

---

## 20. 泛型结构体

### 定义泛型结构体

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

可以为同一个结构体写多个 `impl` 块，每个块有不同的约束：

```rust
// 所有 T 都能用
impl<T> Container<T> {
    fn len(&self) -> usize { self.items.len() }
}

// 只有 T: Ord + Copy 时才有 max_item
impl<T> Container<T> where T: Ord + Copy {
    fn max_item(&self) -> Option<T> {
        self.items.iter().copied().max()
    }
}
```

### 性能：单态化

Rust 使用**单态化**（monomorphization）实现泛型——编译时为每种具体类型生成独立代码。这意味着泛型函数和手写具体类型函数**性能相同**，没有运行时开销。

```rust
let int_c = Container::new(vec![1, 2, 3]);      // 编译为 Container<i32>
let str_c = Container::new(vec!["a", "b", "c"]); // 编译为 Container<&str>
// 编译器生成两份独立的代码，各自优化
```

### 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 泛型实现 | 单态化（编译时） | 类型擦除（运行时） | 类型参数（1.18+） |
| 性能开销 | 零 | 装箱/拆箱开销 | 较小 |
| 约束语法 | `T: Trait` / `where` | `<T extends Interface>` | `[T any]` / `[T constraints]` |

### 配套代码

```rust
println!("{}", advanced_features::generics_demo());
// int: len=5, max=Some(5); str: len=2, first=Some("hello")
```

---

## 21. Trait（特征）

### 定义 trait

```rust
trait Summary {
    fn summary(&self) -> String;        // 必须实现的方法

    fn category(&self) -> &'static str { // 默认方法（可选覆盖）
        "可摘要对象"
    }
}
```

### 实现 trait

```rust
impl Summary for Point {
    fn summary(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
    // category() 使用默认实现
}

impl Summary for Article {
    fn summary(&self) -> String {
        format!("{}, by {} - ...", self.title, self.author)
    }
    // 也可以覆盖 category()
}
```

### Trait Bound：约束泛型

```rust
// 简写形式
fn notify(item: &impl Summary) -> String {
    format!("通知: {}", item.summary())
}

// 完整形式（适合多参数、多约束）
fn notify<T: Summary + Display>(item: &T) -> String { ... }

// where 子句（约束很长时更清晰）
fn process<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Display + Clone,
{ ... }
```

### 配套代码

```rust
println!("{}", advanced_features::trait_demo());
// Point: Point(1, 2) [可摘要对象]
// Article: Rust 入门, by 张三 - 这是一篇关于 Rust 语言的入门... [可摘要对象]
```

---

## 22. Trait Object 与 Box

### 静态分发 vs 动态分发

```rust
// 静态分发：编译时确定类型，零开销
fn notify_static(item: &impl Summary) -> String { ... }

// 动态分发：运行时通过 vtable 查找方法
fn notify_dynamic(item: &dyn Summary) -> String { ... }
```

### Trait Object：不同类型放入同一集合

```rust
let items: Vec<Box<dyn Summary>> = vec![
    Box::new(Point { x: 1, y: 2 }),
    Box::new(Article { title: "新闻".into(), ... }),
];

for item in &items {
    println!("{}", item.summary());
}
```

`Box<dyn Summary>` 是一个**胖指针**：包含数据指针 + vtable 指针。

### Box：堆上分配

`Box<T>` 把数据放在堆上，栈上只保存指针。常用于：
- 递归类型（如链表、树）
- 大量数据需要转移所有权时避免复制
- trait object（动态分发）

### 静态 vs 动态分发选择

| 场景 | 推荐 | 原因 |
|------|------|------|
| 类型在编译时确定 | 静态分发 `impl Trait` | 零开销，编译器可内联优化 |
| 需要异构集合 | 动态分发 `dyn Trait` | 不同类型放入同一 Vec |
| 性能敏感 | 静态分发 | 避免 vtable 查找 |
| 二进制大小敏感 | 动态分发 | 避免单态化代码膨胀 |

### 配套代码

```rust
println!("{}", advanced_features::trait_object_demo());
// Point(1, 2)
// 新闻, by 记者 - 今天发生了一件重要的事情...

println!("{}", advanced_features::dispatch_comparison());
// [静态] 通知: Point(5, 10)
// [动态] 通知: 技术, by 工程师 - Rust 的所有权系统...
```

---

## 23. Trait + 泛型：多态与继承

### Rust 没有类继承

Rust 没有 Java 那样的类层次结构。取而代之的是 **trait 组合**——通过实现多个 trait 来组合行为。

### 组合替代继承

```rust
trait Drawable {
    fn draw(&self) -> String;
}

trait Resizable {
    fn resize(&mut self, factor: f64);
}

struct Circle { radius: f64 }

impl Drawable for Circle {
    fn draw(&self) -> String { format!("绘制圆形(半径={})", self.radius) }
}

impl Resizable for Circle {
    fn resize(&mut self, factor: f64) { self.radius *= factor; }
}
```

### 多态：接收任何实现了 trait 的类型

```rust
fn draw_all(shapes: &[&dyn Drawable]) -> Vec<String> {
    shapes.iter().map(|s| s.draw()).collect()
}
```

### 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 多态方式 | trait 组合 | 类继承 + 接口 | 接口组合 |
| 继承 | ❌ 无类继承 | ✅ 单继承 + 接口 | ❌ 无继承 |
| 代码复用 | trait 默认方法 | 基类方法 | 组合/嵌入 |
| 灵活性 | 高（任意组合 trait） | 受限于类层次 | 高（隐式实现） |

### 配套代码

```rust
let mut circle = Circle { radius: 5.0 };
println!("{}", circle.draw());      // 绘制圆形(半径=5)
circle.resize(2.0);
println!("{}", circle.draw());      // 绘制圆形(半径=10)
```

---

## 24. 操作符重载

Rust 通过实现 `std::ops` 中的 trait 来重载运算符（详见基础语法章节的操作符重载部分）。

核心可重载运算符：`Add`(+)、`Sub`(-)、`Mul`(*)、`Div`(/)、`Rem`(%)、`Neg`(一元-)、`Index`([])、`Deref`(*)。

### 配套代码

```rust
println!("{}", basic_syntax::operator_overload_demo());
// a=Vector2D { x: 1.0, y: 2.0 }, b=Vector2D { x: 3.0, y: 4.0 }
// a+b=Vector2D { x: 4.0, y: 6.0 }, |a+b|=7.21
```

---

## 25. 常见的内置 trait

### Debug 和 Display

```rust
// Debug：开发者视图，用 {:?} 打印
#[derive(Debug)]
struct Point { x: i32, y: i32 }
println!("{:?}", Point { x: 1, y: 2 });  // Point { x: 1, y: 2 }

// Display：用户视图，用 {} 打印，需要手动实现
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
println!("{}", Point { x: 1, y: 2 });  // (1, 2)
```

### Clone 和 Copy

- **Clone**：显式深拷贝，`let b = a.clone()`，有运行时成本
- **Copy**：隐式浅拷贝（栈数据），`let b = a`，零成本

```rust
let a = 42;
let b = a;  // Copy：a 仍然可用
let c = a;  // 可以多次复制

let s1 = String::from("hello");
let s2 = s1;       // Move：s1 不再可用
// let s3 = s1;    // ❌ 编译错误
```

### Drop

值离开作用域时自动调用。适合资源清理（文件关闭、网络连接等）：

```rust
struct FileHandle { name: String }

impl Drop for FileHandle {
    fn drop(&mut self) {
        println!("关闭文件: {}", self.name);
    }
}

{
    let f = FileHandle { name: "data.txt".into() };
    // 离开作用域时自动打印 "关闭文件: data.txt"
}
```

### 配套代码

```rust
println!("{}", advanced_features::display_and_debug(42));
// Display: 42, Debug: 42

println!("{}", advanced_features::complex_bounds("hello", 123));
// original: hello | 123, cloned: hello | 123
```

---

## 26. 迭代器与循环

### Iterator trait

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

### 适配器方法（惰性）

| 方法 | 作用 | 示例 |
|------|------|------|
| `map(f)` | 逐项变换 | `iter.map(\|x\| x * 2)` |
| `filter(p)` | 按条件过滤 | `iter.filter(\|&x\| x > 0)` |
| `take(n)` | 取前 n 项 | `iter.take(5)` |
| `skip(n)` | 跳过前 n 项 | `iter.skip(3)` |
| `enumerate()` | 带索引 | `iter.enumerate()` |
| `zip(other)` | 合并两个迭代器 | `a.iter().zip(b.iter())` |
| `chain(other)` | 拼接 | `a.iter().chain(b.iter())` |
| `flatten()` | 展平嵌套 | `vecs.iter().flatten()` |

### 消费器方法（触发计算）

| 方法 | 作用 | 示例 |
|------|------|------|
| `collect()` | 收集到集合 | `iter.collect::<Vec<_>>()` |
| `sum()` | 求和 | `iter.sum::<i32>()` |
| `count()` | 计数 | `iter.count()` |
| `fold(init, f)` | 累积 | `iter.fold(0, \|acc, x\| acc + x)` |
| `find(p)` | 查找 | `iter.find(\|&x\| x > 10)` |
| `any(p)` | 是否存在 | `iter.any(\|&x\| x == 0)` |
| `all(p)` | 是否全部 | `iter.all(\|&x\| x > 0)` |

### for 循环 vs 迭代器

```rust
// for 循环：简洁直观
for item in &vec { println!("{}", item); }

// 迭代器链：函数式风格，编译器优化更好
let result: Vec<_> = vec.iter()
    .filter(|&&x| x > 0)
    .map(|&x| x * 2)
    .collect();
```

> **最佳实践**：简单的遍历用 `for`，复杂的变换/过滤/聚合用迭代器链。迭代器链在 release 模式下通常比手写循环更快（编译器会内联优化）。

### 配套代码

```rust
// 基础语法 crate
println!("{}", basic_syntax::iterator_methods_demo());
// sum=55, indexed=["0:Rust", "1:Go", "2:Java"], pairs=[("Alice", 95), ("Bob", 87)]

println!("{}", basic_syntax::iterator_lazy_demo());
// 100 以内 3 的倍数的前 5 个平方: [9, 36, 81, 144, 225]
```

---

## 27. 闭包

### 语法

闭包是能捕获外部环境变量的匿名函数：

```rust
let add = |a, b| a + b;                    // 简写
let add_typed = |a: i32, b: i32| -> i32 { a + b };  // 完整类型标注
let no_args = || println!("hello");         // 无参数
```

### 环境捕获：三种方式

Rust 根据闭包如何使用捕获的变量，自动推导它实现哪个 trait：

| trait | 捕获方式 | 可调用次数 | 场景 |
|-------|----------|-----------|------|
| `Fn` | 共享借用 `&T` | 多次 | 只读过滤、格式化 |
| `FnMut` | 可变借用 `&mut T` | 多次 | 累加计数、修改状态 |
| `FnOnce` | 取得所有权 `T` | 一次 | 消费资源、线程 move |

### Fn：只读捕获

```rust
fn apply_twice<F: Fn(i32) -> i32>(value: i32, operation: F) -> i32 {
    operation(operation(value))
}

let doubled = apply_twice(3, |x| x * 2);  // 12
```

### FnMut：可变捕获

```rust
let mut count = 0;
let mut increment = || {
    count += 1;
    count
};
println!("{}", increment());  // 1
println!("{}", increment());  // 2
println!("{}", increment());  // 3
```

### FnOnce：消费所有权

```rust
let name = String::from("Rust");
let greet = move |greeting: String| -> String {
    format!("{}, {}!", greeting, name)
    // name 的所有权被移动进闭包（move 关键字强制）
};
println!("{}", greet("Hello".into()));  // Hello, Rust!
// name 不能再使用
```

### 闭包作为返回值

```rust
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

let add5 = make_adder(5);
let add10 = make_adder(10);
println!("{}", add5(3));   // 8
println!("{}", add10(3));  // 13
```

### 闭包与迭代器的配合

```rust
let threshold = 100;
let filtered: Vec<_> = amounts
    .iter()
    .copied()
    .filter(|amount| *amount >= threshold)  // 闭包捕获 threshold
    .collect();
```

### 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 语法 | `\|args\| expr` | `(args) -> expr` | `func(args) type { ... }` |
| 捕获方式 | 自动推导 Fn/FnMut/FnOnce | effectively final | 值捕获（引用类型共享） |
| 性能 | 零开销抽象 | 可能装箱 | 函数值有分配开销 |
| 所有权 | 受所有权系统约束 | GC 管理 | GC 管理 |

### 配套代码

```rust
println!("{}", advanced_features::closure_demo());
// doubled=12, filtered=[120, 200, 150], add5(3)=8, add10(3)=13

println!("{}", advanced_features::closure_fn_mut_demo());
// FnMut 计数器: a=1, b=2, c=3, 最终=3

println!("{}", advanced_features::closure_fn_once_demo());
// Hello, Rust!
```

---

## 本章注意事项

- **所有权错误先想"谁拥有数据"**，不要急着 `clone`。
- **参数优先用借用**，只有确实要接管资源时才用拥有类型。
- **生命周期标注描述关系**，不负责延长引用寿命。
- **trait 是 Rust 抽象能力的核心**，优先学会 trait bound。
- **闭包不是简单语法糖**，它和所有权捕获方式绑定。
- **迭代器是惰性的**，只有消费器才触发计算。

## 与 Java、Go 的进阶特性对比

| 主题 | Rust | Java | Go |
|------|------|------|-----|
| 内存管理 | 所有权、借用、生命周期 | GC | GC |
| 接口抽象 | trait 显式实现 | interface/class | interface 隐式实现 |
| 泛型实现 | 单态化，零开销 | 类型擦除 | 类型参数 |
| 闭包捕获 | 受所有权约束 | effectively final | 值/引用捕获 |
| 并发安全 | 类型系统阻止数据竞争 | 库 + 锁 + 运行时约定 | channel + 锁 + 约定 |
| 多态 | trait 组合 | 类继承 + 接口 | 接口组合 |

## 本章配套代码汇总

运行完整示例：

```powershell
cargo run -p advanced_features --example advanced_tour
```

运行测试：

```powershell
cargo test -p advanced_features
```

参考源码：[crates/advanced_features/src/lib.rs](../crates/advanced_features/src/lib.rs)
