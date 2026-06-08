//! Rust 基础语法示例。
//!
//! 文档测试示例：
//!
//! ```
//! assert_eq!(basic_syntax::factorial(4), 24);
//! assert_eq!(basic_syntax::first_word("hello rust"), "hello");
//! ```

use std::collections::BTreeMap;
use std::fmt;
use std::num::ParseIntError;
use std::ops::Add;

// ============================================================
// 主题 1：变量声明与不可变性
// ============================================================

/// 演示"不可变绑定 + 遮蔽"。
///
/// `input` 本身没有被修改；第二个 `let value = ...` 创建了一个新的同名绑定。
/// 这在"先拿到原始字符串，再解析成强类型值"的场景里很常见。
pub fn immutable_then_shadow(input: i32) -> String {
    let value = input;
    let value = value * 2;
    format!("原始值: {input}, 遮蔽后: {value}")
}

/// 演示 `mut` 可变变量。
///
/// 这里使用可变变量累加 `1..=times`。真实项目中，如果可以用迭代器表达，
/// 通常会优先选择 `sum()`，但初学阶段手写循环更容易看清状态变化。
pub fn mutable_counter(times: i32) -> i32 {
    let mut total = 0;
    for number in 1..=times {
        total += number;
    }
    total
}

/// 演示变量遮蔽改变类型。
///
/// 遮蔽允许新变量使用不同类型，这在解析场景中很实用：
/// 先是字符串，解析后变成数字。
pub fn shadowing_type_change(raw: &str) -> Result<u32, ParseIntError> {
    let value = raw.trim();
    let value: u32 = value.parse()?;
    let value = value * 10;
    Ok(value)
}

// ============================================================
// 主题 2：const 和 static
// ============================================================

/// 编译期常量：值在编译时确定，没有内存地址。
/// 每次使用时会在代码中内联替换，适合数学常数、配置上限等。
pub const MAX_RETRIES: u32 = 3;

/// 编译期常量：圆周率近似值。
pub const PI: f64 = 3.14159265358979;

/// 静态变量：有固定内存地址，全局存活。
/// 不可变 static 可以安全读取，但可变 static 需要 unsafe。
pub static APP_NAME: &str = "Rust 教程项目";

/// 静态变量：应用版本号。
pub static VERSION: &str = "0.1.0";

/// 演示 const 和 static 的使用。
pub fn const_and_static_demo() -> String {
    format!(
        "应用: {}, 版本: {}, 最大重试: {}, PI ≈ {:.4}",
        APP_NAME, VERSION, MAX_RETRIES, PI
    )
}

// ============================================================
// 主题 3：数据类型（标量、元组、数组）
// ============================================================

/// 演示标量类型：整数、浮点数、布尔值和 Unicode 字符。
pub fn scalar_summary() -> String {
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let flag: bool = true;
    let letter: char = '中';

    format!("i32={integer}, f64={float}, bool={flag}, char={letter}")
}

/// 演示元组：创建、解构和按索引访问。
///
/// 元组适合临时组合少量不同类型的值。
pub fn describe_tuple(language: (&str, u16, bool)) -> String {
    let (name, year, stable) = language;
    format!("{name} 在 {year} 年发布: {stable}")
}

/// 演示元组的按索引访问和嵌套。
pub fn tuple_index_access() -> String {
    let point = (3.5, 7.2);
    let nested = ("坐标", point);
    format!(
        "x={}, y={}, 标签={}",
        nested.1 .0, nested.1 .1, nested.0
    )
}

/// 演示数组：固定长度、同类型、栈上分配。
pub fn array_demo() -> String {
    let numbers: [i32; 5] = [10, 20, 30, 40, 50];
    let zeros = [0; 3];
    format!(
        "numbers={:?}, 长度={}, zeros={:?}",
        numbers,
        numbers.len(),
        zeros
    )
}

/// 演示数组切片和边界检查。
pub fn array_slice_demo(data: &[i32]) -> (usize, i32, Option<i32>) {
    let len = data.len();
    let sum: i32 = data.iter().sum();
    let max = data.iter().copied().max();
    (len, sum, max)
}

/// 演示数组和切片的常见的统计场景。
///
/// 返回值依次是：元素个数、总和、最大值。
pub fn summarize_numbers(numbers: &[i32]) -> (usize, i32, Option<i32>) {
    let len = numbers.len();
    let sum = numbers.iter().sum();
    let max = numbers.iter().copied().max();
    (len, sum, max)
}

// ============================================================
// 主题 4：结构体
// ============================================================

/// 经典结构体：每个字段有名字。
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub active: bool,
}

impl User {
    /// 关联函数（类似构造函数）：使用 `User::new(...)` 调用。
    pub fn new(name: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            age,
            active: true,
        }
    }

    /// 方法：第一个参数是 `&self`，通过实例调用。
    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }

    /// 可变方法：需要 `&mut self`。
    pub fn deactivate(&mut self) {
        self.active = false;
    }

    /// 消费方法：取得所有权，调用后原变量不可再用。
    pub fn into_name(self) -> String {
        self.name
    }
}

/// 元组结构体：字段没有名字，按位置访问。
/// 适合只有 1-3 个字段且语义简单的场景。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance_to_origin(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
}

/// 演示结构体操作。
pub fn struct_demo() -> String {
    let mut user = User::new("Alice", 25);
    let adult = user.is_adult();
    user.deactivate();
    format!(
        "{:?}, 成年={}, 活跃={}",
        user,
        adult,
        user.active
    )
}

// ============================================================
// 主题 5：枚举与模式匹配
// ============================================================

/// 枚举：每个变体可以携带不同类型的数据。
/// Rust 的枚举比 Java/Go 的枚举强大得多，变体可以携带数据。
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => PI * r * r,
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    pub fn describe(&self) -> String {
        match self {
            Shape::Circle(r) => format!("圆形, 半径={}", r),
            Shape::Rectangle(w, h) => format!("矩形, {}×{}", w, h),
            Shape::Triangle { base, height } => {
                format!("三角形, 底={}, 高={}", base, height)
            }
        }
    }
}

/// 用枚举模拟 Option（标准库的 Option 也是这样实现的）。
#[derive(Debug, PartialEq)]
pub enum MyOption<T> {
    Some(T),
    None,
}

/// 演示枚举和模式匹配的综合使用。
pub fn enum_demo() -> String {
    let shapes = vec![
        Shape::Circle(3.0),
        Shape::Rectangle(4.0, 5.0),
        Shape::Triangle {
            base: 6.0,
            height: 3.0,
        },
    ];

    let areas: Vec<String> = shapes
        .iter()
        .map(|s| format!("{}: 面积={:.1}", s.describe(), s.area()))
        .collect();

    areas.join("; ")
}

// ============================================================
// 主题 6：控制流与 match
// ============================================================

/// 演示 `if / else if / else` 链。
pub fn classify_number(number: i32) -> &'static str {
    if number < 0 {
        "negative"
    } else if number == 0 {
        "zero"
    } else if number % 2 == 0 {
        "positive-even"
    } else {
        "positive-odd"
    }
}

/// 演示 `match` 模式匹配。
///
/// 这个例子模拟考试成绩转换。`90..=100` 是闭区间模式。
pub fn grade(score: u8) -> &'static str {
    match score {
        90..=100 => "A",
        80..=89 => "B",
        70..=79 => "C",
        60..=69 => "D",
        _ => "F",
    }
}

/// 演示 `if let` 和 `while let`：只关心一种匹配模式时使用。
pub fn if_let_demo(value: Option<i32>) -> String {
    if let Some(v) = value {
        format!("找到值: {}", v)
    } else {
        "没有值".to_string()
    }
}

/// 同时演示 `for`、`loop` 和 `break`。
pub fn control_flow_samples(limit: i32) -> Vec<i32> {
    let mut values = Vec::new();

    for number in 0..limit {
        values.push(number);
    }

    let mut current = limit + 2;
    loop {
        if current <= 0 {
            break;
        }

        values.push(current);
        current -= 2;
    }

    values
}

// ============================================================
// 主题 7：函数与返回值
// ============================================================

/// 演示函数参数和表达式返回。
///
/// 最后一行 `result` 没有分号，它是表达式的值，作为返回值。
/// `return` 关键字一般只用于提前返回。
pub fn factorial(number: u32) -> u32 {
    let mut result = 1;
    let mut current = number;

    while current > 1 {
        result *= current;
        current -= 1;
    }

    result
}

/// 演示高阶函数：函数作为参数。
///
/// `operation` 是一个闭包/函数，接收 i32 返回 i32。
pub fn apply_operation(value: i32, operation: fn(i32) -> i32) -> i32 {
    operation(value)
}

/// 用于演示高阶函数的辅助函数。
pub fn double(x: i32) -> i32 {
    x * 2
}

/// 用于演示高阶函数的辅助函数。
pub fn square(x: i32) -> i32 {
    x * x
}

/// 演示返回多个值（通过元组）。
pub fn min_max(numbers: &[i32]) -> (Option<i32>, Option<i32>) {
    let min = numbers.iter().copied().min();
    let max = numbers.iter().copied().max();
    (min, max)
}

// ============================================================
// 主题 8：字符串 String 与 &str
// ============================================================

/// 返回字符串中的第一个单词。
///
/// 参数使用 `&str`，因此既可以接收字符串字面量，也可以接收 `String` 的引用。
pub fn first_word(text: &str) -> &str {
    for (index, byte) in text.bytes().enumerate() {
        if byte == b' ' {
            return &text[..index];
        }
    }

    text
}

/// 演示常见字符串清洗流程：去空白、转小写、替换分隔符。
pub fn normalize_username(raw: &str) -> String {
    raw.trim().to_lowercase().replace(' ', "_")
}

/// 演示 String 和 &str 之间的转换。
pub fn string_conversion_demo() -> String {
    // &str → String（堆分配）
    let owned: String = "hello".to_string();
    let owned2: String = String::from("world");
    let owned3: String = "rust".to_owned();

    // String → &str（借用，零成本）
    let borrowed: &str = &owned;

    // 拼接
    let combined = format!("{} {} {}", owned, owned2, owned3);

    format!("borrowed={}, combined={}", borrowed, combined)
}

/// 演示字符串常见操作的输出。
pub fn string_operations_demo() -> String {
    let s = String::from("  Hello, Rust!  ");
    let trimmed = s.trim();
    let lower = trimmed.to_lowercase();
    let replaced = lower.replace("rust", "world");
    let contains = lower.contains("rust");
    let len_bytes = trimmed.len();
    let len_chars = trimmed.chars().count();

    format!(
        "trim='{}', lower='{}', replace='{}', contains={}, bytes={}, chars={}",
        trimmed, lower, replaced, contains, len_bytes, len_chars
    )
}

// ============================================================
// 主题 9：Result、Option 与错误处理
// ============================================================

/// 演示函数返回 `Result` 以及变量遮蔽的组合。
pub fn parse_port_with_shadowing(raw: &str) -> Result<u16, ParseIntError> {
    let port = raw.trim();
    let port: u16 = port.parse()?;
    Ok(port)
}

/// 演示 `Option`：除数为 0 时没有有效结果。
pub fn checked_divide(left: i32, right: i32) -> Option<i32> {
    if right == 0 {
        None
    } else {
        Some(left / right)
    }
}

/// 演示 Option 的链式操作。
pub fn option_chain_demo(raw: &str) -> String {
    let result = raw
        .trim()
        .parse::<i32>()
        .ok()
        .map(|n| n * 2)
        .filter(|&n| n > 0)
        .unwrap_or(0);
    format!("链式处理结果: {}", result)
}

/// 自定义错误类型：演示 Error 类型设计。
///
/// 实际项目中常用 `thiserror` 或 `anyhow` 来简化错误类型定义。
#[derive(Debug)]
pub enum AppError {
    /// 输入为空。
    EmptyInput,
    /// 解析失败，包裹原始错误。
    InvalidNumber(ParseIntError),
    /// 值超出预期范围。
    OutOfRange { value: i32, min: i32, max: i32 },
}

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

impl std::error::Error for AppError {}

/// From trait 允许 `?` 自动转换错误类型。
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::InvalidNumber(e)
    }
}

/// 便捷类型别名：项目中常见写法。
pub type AppResult<T> = Result<T, AppError>;

/// 演示 `?` 操作符和自定义错误的组合。
///
/// `?` 会在出错时自动将错误类型转换（通过 `From` trait）并提前返回。
pub fn parse_and_validate(raw: &str) -> AppResult<i32> {
    if raw.trim().is_empty() {
        return Err(AppError::EmptyInput);
    }
    let value: i32 = raw.parse()?;
    if value < 0 || value > 100 {
        return Err(AppError::OutOfRange {
            value,
            min: 0,
            max: 100,
        });
    }
    Ok(value)
}

/// 演示 Option 与 Result 的互相转换。
pub fn option_result_interop(raw: &str) -> Result<i32, String> {
    raw.trim()
        .parse::<i32>()
        .ok()
        .ok_or_else(|| format!("无法解析 '{}' 为整数", raw))
}

// ============================================================
// 主题 10：操作符重载
// ============================================================

/// 二维向量结构体，用于演示操作符重载。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2D {
    pub x: f64,
    pub y: f64,
}

impl Vector2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

/// 实现加法操作符 `+`。
impl Add for Vector2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// 演示操作符重载的使用。
pub fn operator_overload_demo() -> String {
    let a = Vector2D::new(1.0, 2.0);
    let b = Vector2D::new(3.0, 4.0);
    let c = a + b;
    format!(
        "a={:?}, b={:?}, a+b={:?}, |a+b|={:.2}",
        a,
        b,
        c,
        c.length()
    )
}

// ============================================================
// 主题 11：常见内置 trait
// ============================================================

/// 演示 Display trait 实现：用户友好的字符串表示。
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "用户 {} ({}岁, 活跃={})", self.name, self.age, self.active)
    }
}

/// 演示 Display trait 实现。
impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Shape::Circle(r) => write!(f, "圆形(r={})", r),
            Shape::Rectangle(w, h) => write!(f, "矩形({}×{})", w, h),
            Shape::Triangle { base, height } => {
                write!(f, "三角形(底={}, 高={})", base, height)
            }
        }
    }
}

/// 演示常见内置 trait 的用途。
pub fn builtin_traits_demo() -> String {
    let user = User::new("Bob", 30);

    // Debug: 用 {:?} 打印，面向开发者
    let debug = format!("{:?}", user);

    // Display: 用 {} 打印，面向用户
    let display = format!("{}", user);

    // Clone: 深拷贝
    let cloned = user.clone();
    assert_eq!(user, cloned);

    format!("Debug: {}\nDisplay: {}", debug, display)
}

/// 演示 Drop trait（值离开作用域时自动调用）。
pub struct DropLogger {
    pub name: String,
}

impl Drop for DropLogger {
    fn drop(&mut self) {
        // 实际项目中 drop 里不应该 panic
        println!("[Drop] {} 被释放", self.name);
    }
}

/// 演示 Copy trait：栈上数据赋值后原变量仍可用。
pub fn copy_trait_demo() -> String {
    let a: i32 = 42;
    let b = a; // i32 实现 Copy，这里是复制而非移动
    let c = a; // 可以多次复制
    format!("a={}, b={}, c={} — 都能用，因为 i32 实现了 Copy", a, b, c)
}

// ============================================================
// 主题 12：迭代器
// ============================================================

/// 演示 BTreeMap 词频统计。
///
/// 这里选择 `BTreeMap` 而不是 `HashMap`，是因为它按 key 排序，
/// 示例输出更稳定，适合作为教程代码。
pub fn word_count(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();

    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    counts
}

/// 演示迭代器适配器链：map → filter → collect。
pub fn iterator_chain_demo(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .copied()
        .filter(|&n| n % 2 == 0)
        .map(|n| n * n)
        .collect()
}

/// 演示迭代器常用方法：fold、enumerate、zip。
pub fn iterator_methods_demo() -> String {
    // fold: 累积计算
    let sum = (1..=10).fold(0, |acc, x| acc + x);

    // enumerate: 带索引遍历
    let indexed: Vec<String> = ["Rust", "Go", "Java"]
        .iter()
        .enumerate()
        .map(|(i, lang)| format!("{}:{}", i, lang))
        .collect();

    // zip: 合并两个迭代器
    let names = vec!["Alice", "Bob"];
    let scores = vec![95, 87];
    let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();

    format!("sum={}, indexed={:?}, pairs={:?}", sum, indexed, pairs)
}

/// 演示迭代器的惰性求值特性。
pub fn iterator_lazy_demo() -> String {
    // take + filter + map 链式调用，只在 collect 时才真正计算
    let result: Vec<i32> = (1..=100)
        .filter(|n| n % 3 == 0)
        .map(|n| n * n)
        .take(5)
        .collect();

    format!("100 以内 3 的倍数的前 5 个平方: {:?}", result)
}

// ============================================================
// 主题 13：集合深入
// ============================================================

/// 演示 Vec 的常用操作。
pub fn vec_operations() -> String {
    let mut numbers = vec![3, 1, 4];
    numbers.push(1);
    numbers.push(5);
    numbers.insert(0, 9);  // 头部插入
    numbers.sort();
    numbers.dedup();  // 去重（需先排序）

    let popped = numbers.pop();  // Option<i32>
    let contains = numbers.contains(&4);
    let found = numbers.iter().position(|&x| x > 3);

    format!(
        "sorted_dedup={:?}, popped={:?}, contains_4={}, first_gt_3={:?}",
        numbers, popped, contains, found
    )
}

/// 演示 HashMap 的常用操作。
pub fn hashmap_operations() -> String {
    use std::collections::HashMap;
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Alice".into(), 95);
    scores.insert("Bob".into(), 87);
    scores.insert("Charlie".into(), 92);

    // entry API：存在则不插入，不存在则插入默认值
    scores.entry("Diana".into()).or_insert(80);
    // entry API：存在则修改
    *scores.entry(String::from("Alice")).or_insert(0) += 5;

    // 查找
    let bob_score = scores.get("Bob");  // Option<&i32>
    let missing = scores.get("Eve");    // None

    // 过滤
    let high_scorers: Vec<_> = scores
        .iter()
        .filter(|(_, &score)| score >= 90)
        .map(|(name, _)| name.as_str())
        .collect();

    format!(
        "scores={:?}, bob={:?}, eve={:?}, high={:?}",
        scores, bob_score, missing, high_scorers
    )
}

/// 演示 HashSet 的集合运算。
pub fn hashset_operations() -> String {
    use std::collections::HashSet;
    let frontend: HashSet<_> = ["HTML", "CSS", "JavaScript"].iter().cloned().collect();
    let backend: HashSet<_> = ["Rust", "Go", "JavaScript"].iter().cloned().collect();

    let intersection: Vec<_> = frontend.intersection(&backend).cloned().collect();
    let union: Vec<_> = frontend.union(&backend).cloned().collect();
    let diff: Vec<_> = frontend.difference(&backend).cloned().collect();

    format!(
        "交集={:?}, 并集={:?}, 差集(frontend-backend)={:?}",
        intersection, union, diff
    )
}

/// 演示迭代器的高级用法。
pub fn iterator_advanced() -> String {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // chunks: 分组
    let chunks: Vec<_> = data.chunks(3).collect();

    // windows: 滑动窗口
    let windows: Vec<_> = data.windows(3).collect();

    // fold: 累积
    let product: i32 = data.iter().fold(1, |acc, &x| acc * x);

    // scan: 带状态的 map
    let running_sum: Vec<_> = data
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();

    format!(
        "chunks={:?}, windows前3={:?}, product={}, running_sum={:?}",
        chunks.len(),
        &windows[..3.min(windows.len())],
        product,
        running_sum
    )
}
