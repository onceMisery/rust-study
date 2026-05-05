//! Rust 基础语法示例。
//!
//! 文档测试示例：
//!
//! ```
//! assert_eq!(basic_syntax::factorial(4), 24);
//! assert_eq!(basic_syntax::first_word("hello rust"), "hello");
//! ```

use std::collections::BTreeMap;
use std::num::ParseIntError;

/// 演示“不可变绑定 + 遮蔽”。
///
/// `input` 本身没有被修改；第二个 `let value = ...` 创建了一个新的同名绑定。
/// 这在“先拿到原始字符串，再解析成强类型值”的场景里很常见。
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

/// 演示标量类型：整数、浮点数、布尔值和 Unicode 字符。
pub fn scalar_summary() -> String {
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let flag: bool = true;
    let letter: char = '中';

    format!("i32={integer}, f64={float}, bool={flag}, char={letter}")
}

/// 演示元组解构。
///
/// 元组适合临时组合少量不同类型的字段；如果字段语义长期存在，
/// 更推荐定义 `struct`。
pub fn describe_tuple(language: (&str, u16, bool)) -> String {
    let (name, year, stable) = language;
    format!("{name} 在 {year} 年发布: {stable}")
}

/// 演示数组和切片的常见统计场景。
///
/// 返回值依次是：元素个数、总和、最大值。
pub fn summarize_numbers(numbers: &[i32]) -> (usize, i32, Option<i32>) {
    let len = numbers.len();
    let sum = numbers.iter().sum();
    let max = numbers.iter().copied().max();
    (len, sum, max)
}

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

/// 演示 `while` 循环和表达式返回。
pub fn factorial(number: u32) -> u32 {
    let mut result = 1;
    let mut current = number;

    while current > 1 {
        result *= current;
        current -= 1;
    }

    result
}

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

/// 演示 `BTreeMap` 词频统计。
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
