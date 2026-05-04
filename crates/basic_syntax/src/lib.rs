//! Rust 基础语法示例。
//!
//! 文档测试示例：
//!
//! ```
//! assert_eq!(basic_syntax::factorial(4), 24);
//! assert_eq!(basic_syntax::first_word("hello rust"), "hello");
//! ```

pub fn immutable_then_shadow(input: i32) -> String {
    let value = input;
    let value = value * 2;
    format!("原始值: {input}, 遮蔽后: {value}")
}

pub fn mutable_counter(times: i32) -> i32 {
    let mut total = 0;
    for number in 1..=times {
        total += number;
    }
    total
}

pub fn scalar_summary() -> String {
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let flag: bool = true;
    let letter: char = '中';

    format!("i32={integer}, f64={float}, bool={flag}, char={letter}")
}

pub fn describe_tuple(language: (&str, u16, bool)) -> String {
    let (name, year, stable) = language;
    format!("{name} 在 {year} 年发布: {stable}")
}

pub fn factorial(number: u32) -> u32 {
    let mut result = 1;
    let mut current = number;

    while current > 1 {
        result *= current;
        current -= 1;
    }

    result
}

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

pub fn first_word(text: &str) -> &str {
    for (index, byte) in text.bytes().enumerate() {
        if byte == b' ' {
            return &text[..index];
        }
    }

    text
}
