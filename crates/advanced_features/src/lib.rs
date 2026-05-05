//! Rust 进阶特性示例。
//!
//! ```
//! use advanced_features::{longest, Point, Summary};
//!
//! assert_eq!(longest("abc", "abcdef"), "abcdef");
//! assert_eq!(Point { x: 1, y: 2 }.summary(), "Point(1, 2)");
//! ```

use std::fmt::Display;

/// 消费 `String`，演示所有权移动。
///
/// 调用方把 `String` 的所有权移动进函数后，就不能再使用原变量。
/// 这类 API 适合“函数需要接管资源并最终释放资源”的场景。
pub fn replace_with_length(text: String) -> usize {
    text.len()
}

/// 克隆一份字符串，演示“保留原值 + 生成拥有所有权的新值”。
///
/// `clone` 有堆内存复制成本；真实项目中只在确实需要两份独立数据时使用。
pub fn clone_then_keep_original(text: &str) -> (String, usize) {
    let owned = text.to_owned();
    let len = owned.len();
    (owned, len)
}

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

/// `impl Summary` 是参数位置的简写，适合单个 trait 约束。
pub fn notify(item: &impl Summary) -> String {
    format!("通知: {}", item.summary())
}

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

/// `where` 子句适合多个泛型参数或较长约束。
pub fn describe_pair<T, U>(left: T, right: U) -> String
where
    T: Display,
    U: Display,
{
    format!("{left} | {right}")
}

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
