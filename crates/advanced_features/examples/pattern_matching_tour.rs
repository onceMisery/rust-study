use advanced_features::{
    pattern_destructuring, pattern_guards_and_bindings, if_let_while_let,
};

fn main() {
    println!("=== 1. 解构模式 ===");
    println!("{}", pattern_destructuring());

    println!("\n=== 2. 守卫与 @ 绑定 ===");
    println!("{}", pattern_guards_and_bindings());

    println!("\n=== 3. if let / while let ===");
    println!("{}", if_let_while_let());

    println!("\n=== 4. 枚举解构 ===");
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        Color(u8, u8, u8),
    }

    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write("hello".to_string()),
        Message::Color(255, 0, 128),
    ];

    for msg in &messages {
        match msg {
            Message::Quit => println!("  Quit"),
            Message::Move { x, y } => println!("  Move to ({}, {})", x, y),
            Message::Write(text) => println!("  Write: {}", text),
            Message::Color(r, g, b) => println!("  Color({}, {}, {})", r, g, b),
        }
    }

    println!("\n=== 5. 嵌套模式 ===");
    let data = Some(vec![1, 2, 3]);
    match data {
        Some(vec) if vec.len() > 2 => println!("  长列表: {:?}", vec),
        Some(vec) => println!("  短列表: {:?}", vec),
        None => println!("  无数据"),
    }

    println!("\n=== 6. 模式匹配适用场景 ===");
    println!("✅ match: 枚举变体、多分支穷举");
    println!("✅ if let: 只关心一种匹配（Option/Result）");
    println!("✅ while let: 循环解构（栈弹出、迭代器）");
    println!("✅ 解构: 元组、结构体、嵌套模式");
}
