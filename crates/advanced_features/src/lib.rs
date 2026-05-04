//! Rust 进阶特性示例。
//!
//! ```
//! use advanced_features::{longest, Point, Summary};
//!
//! assert_eq!(longest("abc", "abcdef"), "abcdef");
//! assert_eq!(Point { x: 1, y: 2 }.summary(), "Point(1, 2)");
//! ```

use std::fmt::Display;

pub fn replace_with_length(text: String) -> usize {
    text.len()
}

pub fn shared_borrow_sum(values: &[i32]) -> i32 {
    values.iter().sum()
}

pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

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
    pub fn max_item(&self) -> Option<T> {
        self.items.iter().copied().max()
    }
}

pub fn describe_pair<T, U>(left: T, right: U) -> String
where
    T: Display,
    U: Display,
{
    format!("{left} | {right}")
}
