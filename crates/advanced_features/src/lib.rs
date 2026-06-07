//! Rust 进阶特性示例。
//!
//! ```
//! use advanced_features::{longest, Point, Summary};
//!
//! assert_eq!(longest("abc", "abcdef"), "abcdef");
//! assert_eq!(Point { x: 1, y: 2 }.summary(), "Point(1, 2)");
//! ```

use std::fmt::Display;

// ============================================================
// 主题 1：所有权 — 移动语义与克隆
// ============================================================

/// 消费 `String`，演示所有权移动。
///
/// 调用方把 `String` 的所有权移动进函数后，就不能再使用原变量。
/// 这类 API 适合"函数需要接管资源并最终释放资源"的场景。
pub fn replace_with_length(text: String) -> usize {
    text.len()
}

/// 克隆一份字符串，演示"保留原值 + 生成拥有所有权的新值"。
///
/// `clone` 有堆内存复制成本；真实项目中只在确实需要两份独立数据时使用。
pub fn clone_then_keep_original(text: &str) -> (String, usize) {
    let owned = text.to_owned();
    let len = owned.len();
    (owned, len)
}

/// 演示所有权的转移规则：赋值、传参、返回值。
pub fn ownership_move_demo() -> String {
    let s1 = String::from("hello");

    // 赋值移动：s1 的所有权转移到 s2
    let s2 = s1;
    // 此时 s1 不能再使用

    // 函数传参移动：s2 的所有权移动进函数
    let len = replace_with_length(s2);
    // 此时 s2 不能再使用

    // 返回值转移所有权
    let s3 = format!("长度是 {}，现在创建新字符串", len);
    s3 // 所有权移动给调用方
}

/// 演示 Copy trait：栈上类型赋值不会移动。
pub fn copy_vs_move_demo() -> String {
    // i32 实现 Copy：赋值是复制，不是移动
    let a = 42;
    let b = a;
    let _c = a; // a 仍然可用

    // String 没有 Copy：赋值是移动
    let s1 = String::from("hello");
    let s2 = s1;
    // let _s3 = s1; // 编译错误！s1 已经移动

    format!("a={}, b={} (Copy); s2={} (Move)", a, b, s2)
}

// ============================================================
// 主题 2：借用与引用
// ============================================================

/// 共享借用切片，只读访问集合。
pub fn shared_borrow_sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

/// 可变借用字符串，原地追加后缀。
///
/// 同一时间只能有一个 `&mut String`，这能在编译期避免多个写入者互相覆盖。
pub fn append_suffix(text: &mut String, suffix: &str) {
    text.push_str(suffix);
}

/// 演示借用规则：多个不可变借用 vs 单个可变借用。
pub fn borrow_rules_demo() -> String {
    let mut data = String::from("Hello");

    // 规则 1：可以同时存在多个不可变借用
    let r1 = &data;
    let r2 = &data;
    let result1 = format!("r1={}, r2={}", r1, r2);

    // 规则 2：不可变借用全部结束后，才能创建可变借用
    let r3 = &mut data;
    append_suffix(r3, " World");

    // 规则 3：可变借用期间不能有不可变借用
    let r4 = &data;
    let result2 = format!("修改后: {}", r4);

    format!("{}\n{}", result1, result2)
}

/// 演示借用不能比数据活得更久（悬垂引用）。
pub fn no_dangling_reference() -> String {
    let result;
    let s = String::from("hello world");
    result = first_longer_word(&s);
    // s 还活着，所以 result 是安全的
    format!("第一个长单词: {}", result)
}

fn first_longer_word(text: &str) -> &str {
    for word in text.split_whitespace() {
        if word.len() > 4 {
            return word;
        }
    }
    ""
}

// ============================================================
// 主题 3：生命周期
// ============================================================

/// 显式生命周期示例：返回两个输入引用中更长的一个。
///
/// `'a` 表示返回引用的有效期不能超过 `left` 和 `right` 的共同有效期。
pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

/// 演示生命周期省略规则。
///
/// 编译器有三条省略规则：
/// 1. 每个引用参数获得独立的生命周期
/// 2. 如果只有一个输入生命周期，它被赋给所有输出
/// 3. 如果有 &self 或 &mut self，self 的生命周期被赋给所有输出
pub fn lifetime_elision_demo() -> String {
    // 无需标注：只有一个输入引用
    let first = first_word_in("hello world");

    // 需要标注：两个输入引用，返回引用和哪个相关？
    let longer = longest("short", "much longer text");

    format!("first='{}', longest='{}'", first, longer)
}

fn first_word_in(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

/// 带生命周期的结构体：引用字段必须标注生命周期。
///
/// 这意味着 `ImportantExcerpt` 不能比它引用的 `part` 活得更久。
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    pub fn level(&self) -> i32 {
        3
    }

    pub fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

/// 演示结构体中的生命周期。
pub fn struct_lifetime_demo() -> String {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    format!("摘录: '{}', level={}", excerpt.part, excerpt.level())
}

/// 演示 `'static` 生命周期：整个程序运行期间都有效。
pub fn static_lifetime_demo() -> &'static str {
    // 字符串字面量具有 'static 生命周期
    "这个字符串活在程序的全部生命周期里"
}

// ============================================================
// 主题 4：Trait 定义与实现
// ============================================================

/// trait 类似接口，但可以带默认方法，也能作为泛型约束。
pub trait Summary {
    fn summary(&self) -> String;

    fn category(&self) -> &'static str {
        "可摘要对象"
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Summary for Point {
    fn summary(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
}

/// 演示 trait 默认方法的复用。
pub struct Article {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Article {
    fn summary(&self) -> String {
        let end = self.content.char_indices().nth(20).map(|(i, _)| i).unwrap_or(self.content.len());
        format!("{}, by {} - {}...", self.title, self.author, &self.content[..end])
    }

    // category() 使用默认实现
}

/// `impl Summary` 是参数位置的简写，适合单个 trait 约束。
pub fn notify(item: &impl Summary) -> String {
    format!("通知: {}", item.summary())
}

/// 演示 trait 的使用和默认方法。
pub fn trait_demo() -> String {
    let point = Point { x: 1, y: 2 };
    let article = Article {
        title: "Rust 入门".to_string(),
        author: "张三".to_string(),
        content: "这是一篇关于 Rust 语言的入门教程，主要讲解基础语法和核心概念。".to_string(),
    };

    format!(
        "Point: {} [{}]\nArticle: {} [{}]",
        point.summary(),
        point.category(),
        article.summary(),
        article.category()
    )
}

// ============================================================
// 主题 5：泛型
// ============================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn first(&self) -> Option<&T> {
        self.items.first()
    }
}

impl<T> Container<T>
where
    T: Ord + Copy,
{
    /// `T: Ord + Copy` 说明元素既能比较，也能被复制出来。
    pub fn max_item(&self) -> Option<T> {
        self.items.iter().copied().max()
    }
}

/// 泛型函数：处理任何实现 Display 的类型。
pub fn display_pair<T: Display, U: Display>(a: T, b: U) -> String {
    format!("({}, {})", a, b)
}

/// `where` 子句适合多个泛型参数或较长约束。
pub fn describe_pair<T, U>(left: T, right: U) -> String
where
    T: Display,
    U: Display,
{
    format!("{left} | {right}")
}

/// 演示泛型的使用。
pub fn generics_demo() -> String {
    let int_container = Container::new(vec![3, 1, 4, 1, 5]);
    let str_container = Container::new(vec!["hello", "world"]);

    format!(
        "int: len={}, max={:?}; str: len={}, first={:?}",
        int_container.len(),
        int_container.max_item(),
        str_container.len(),
        str_container.first()
    )
}

// ============================================================
// 主题 6：Trait Object 与动态分发
// ============================================================

/// 使用 trait object 实现动态分发。
///
/// `&dyn Summary` 是一个胖指针，包含数据指针和 vtable 指针。
/// 运行时通过 vtable 查找实际方法，有少量性能开销，但更灵活。
pub fn notify_dynamic(item: &dyn Summary) -> String {
    format!("[动态] 通知: {}", item.summary())
}

/// 使用泛型实现静态分发（编译时确定类型）。
///
/// 编译器为每种类型生成独立代码，无运行时开销，但二进制更大。
pub fn notify_static(item: &impl Summary) -> String {
    format!("[静态] 通知: {}", item.summary())
}

/// 演示 trait object 的集合用法：不同类型放入同一个 Vec。
pub fn trait_object_demo() -> String {
    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(Point { x: 1, y: 2 }),
        Box::new(Article {
            title: "新闻".to_string(),
            author: "记者".to_string(),
            content: "今天发生了一件重要的事情，引起了广泛关注。".to_string(),
        }),
    ];

    let results: Vec<String> = items.iter().map(|item| item.summary()).collect();
    results.join("\n")
}

/// 演示静态分发 vs 动态分发的选择。
pub fn dispatch_comparison() -> String {
    let point = Point { x: 5, y: 10 };
    let article = Article {
        title: "技术".to_string(),
        author: "工程师".to_string(),
        content: "Rust 的所有权系统是它最独特的特性之一。".to_string(),
    };

    // 静态分发：编译时确定类型，零开销
    let s1 = notify_static(&point);
    let s2 = notify_static(&article);

    // 动态分发：运行时查找 vtable，少量开销
    let d1 = notify_dynamic(&point);
    let d2 = notify_dynamic(&article);

    format!("{}\n{}\n{}\n{}", s1, s2, d1, d2)
}

// ============================================================
// 主题 7：Trait + 泛型 + 多态
// ============================================================

/// trait bound 约束：函数只接受实现了特定 trait 的类型。
pub fn largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }
    let mut largest = &list[0];
    for item in &list[1..] {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

/// 多 trait bound：类型必须同时实现多个 trait。
pub fn display_and_debug<T: Display + std::fmt::Debug>(item: T) -> String {
    format!("Display: {}, Debug: {:?}", item, item)
}

/// 使用 `where` 子句简化多约束写法。
pub fn complex_bounds<T, U>(t: T, u: U) -> String
where
    T: Display + Clone,
    U: Display + Clone,
{
    let t_clone = t.clone();
    let u_clone = u.clone();
    format!("original: {} | {}, cloned: {} | {}", t, u, t_clone, u_clone)
}

/// 演示 Rust 的组合替代继承。
///
/// Rust 没有类继承，但通过 trait 组合实现多态。
/// 类似 Go 的接口组合，比 Java 的类层次更灵活。
pub trait Drawable {
    fn draw(&self) -> String;
}

pub trait Resizable {
    fn resize(&mut self, factor: f64);
}

/// 同时实现 Drawable 和 Resizable。
#[derive(Debug)]
pub struct Circle {
    pub radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) -> String {
        format!("绘制圆形(半径={})", self.radius)
    }
}

impl Resizable for Circle {
    fn resize(&mut self, factor: f64) {
        self.radius *= factor;
    }
}

/// 演示多态：接收任何实现了 Drawable 的类型。
pub fn draw_all(shapes: &[&dyn Drawable]) -> Vec<String> {
    shapes.iter().map(|s| s.draw()).collect()
}

// ============================================================
// 主题 8：闭包
// ============================================================

/// 闭包作为参数：调用同一个只读闭包两次。
pub fn apply_twice<F>(value: i32, operation: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operation(operation(value))
}

/// 闭包捕获外部变量的业务场景：按阈值过滤订单金额。
pub fn filter_amounts(amounts: &[i32], min_amount: i32) -> Vec<i32> {
    amounts
        .iter()
        .copied()
        .filter(|amount| *amount >= min_amount)
        .collect()
}

/// 演示 FnMut：闭包修改捕获的外部变量。
pub fn closure_fn_mut_demo() -> String {
    let mut count = 0;

    // FnMut: 闭包可变借用 count
    let mut increment = || {
        count += 1;
        count
    };

    let a = increment();
    let b = increment();
    let c = increment();

    format!("FnMut 计数器: a={}, b={}, c={}, 最终={}", a, b, c, count)
}

/// 演示 FnOnce：闭包消费所有权。
pub fn closure_fn_once_demo() -> String {
    let name = String::from("Rust");

    // FnOnce: 闭包取得 name 的所有权
    let greet = |greeting: String| -> String {
        format!("{}, {}!", greeting, name)
        // name 的所有权被移动进闭包
    };

    let result = greet(String::from("Hello"));
    // name 在这里不能再使用

    result
}

/// 演示闭包作为返回值的场景。
pub fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

/// 综合演示闭包的各种用法。
pub fn closure_demo() -> String {
    // Fn: 只读捕获
    let doubled = apply_twice(3, |x| x * 2);

    // 捕获外部变量
    let threshold = 100;
    let filtered = filter_amounts(&[50, 120, 200, 80, 150], threshold);

    // 工厂函数
    let add5 = make_adder(5);
    let add10 = make_adder(10);

    format!(
        "doubled={}, filtered={:?}, add5(3)={}, add10(3)={}",
        doubled,
        filtered,
        add5(3),
        add10(3)
    )
}

// ============================================================
// 主题 9：借用检查器常见场景
// ============================================================

/// 演示借用检查器的核心规则：
/// 1. 任何时刻，要么只有一个可变引用，要么有多个不可变引用
/// 2. 引用必须始终有效
pub fn borrow_checker_demo() -> String {
    let mut s = String::from("hello");

    // 多个不可变借用可以同时存在
    let r1 = &s;
    let r2 = &s;
    let combined = format!("{} + {}", r1, r2);
    // r1, r2 的最后使用在这里（NLL: Non-Lexical Lifetimes）

    // 因为 r1, r2 已经不再使用，可以创建可变借用
    s.push_str(" world");

    let r3 = &s;
    format!("{}\n修改后: {}", combined, r3)
}

/// 演示 NLL（Non-Lexical Lifetimes）：借用持续到"最后一次使用"而非"作用域结束"。
pub fn nll_demo() -> String {
    let mut data = vec![1, 2, 3];

    let r = &data[0];
    println!("读取: {}", r);
    // r 的最后一次使用在上面，此后 r 不再有效

    // 可以在 r 失效后修改 data
    data.push(4);

    format!("修改后: {:?}", data)
}
