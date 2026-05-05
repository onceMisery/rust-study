fn main() {
    println!("=== 变量、类型与函数 ===");
    println!("{}", basic_syntax::immutable_then_shadow(5));
    println!("{}", basic_syntax::scalar_summary());
    println!("{}", basic_syntax::describe_tuple(("Rust", 2015, true)));
    println!(
        "数字统计: {:?}",
        basic_syntax::summarize_numbers(&[3, 1, 4, 1, 5])
    );
    println!(
        "端口解析: {:?}",
        basic_syntax::parse_port_with_shadowing(" 8080 ")
    );
    println!("安全除法: {:?}", basic_syntax::checked_divide(10, 2));

    println!("\n=== 控制流 ===");
    println!("5! = {}", basic_syntax::factorial(5));
    println!("8 是 {}", basic_syntax::classify_number(8));
    println!("成绩等级: {}", basic_syntax::grade(85));
    println!("{:?}", basic_syntax::control_flow_samples(4));

    println!("\n=== 字符串与集合 ===");
    println!("{}", basic_syntax::first_word("hello rust world"));
    println!("{}", basic_syntax::normalize_username(" Alice Chen "));
    println!("{:?}", basic_syntax::word_count("rust rust java go rust"));
}
