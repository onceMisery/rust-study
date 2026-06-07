fn main() {
    println!("=== 1. 变量声明与不可变性 ===");
    println!("{}", basic_syntax::immutable_then_shadow(5));
    println!("可变计数器: sum(1..=10) = {}", basic_syntax::mutable_counter(10));
    println!(
        "遮蔽改变类型: {:?}",
        basic_syntax::shadowing_type_change(" 42 ")
    );

    println!("\n=== 2. const 和 static ===");
    println!("{}", basic_syntax::const_and_static_demo());
    println!("MAX_RETRIES = {}", basic_syntax::MAX_RETRIES);
    println!("PI = {}", basic_syntax::PI);

    println!("\n=== 3. 数据类型：标量、元组、数组 ===");
    println!("{}", basic_syntax::scalar_summary());
    println!(
        "元组: {}",
        basic_syntax::describe_tuple(("Rust", 2015, true))
    );
    println!("元组索引: {}", basic_syntax::tuple_index_access());
    println!("数组: {}", basic_syntax::array_demo());
    println!(
        "数组切片统计: {:?}",
        basic_syntax::array_slice_demo(&[3, 1, 4, 1, 5, 9])
    );

    println!("\n=== 4. 结构体 ===");
    println!("{}", basic_syntax::struct_demo());
    let mut user = basic_syntax::User::new("Bob", 17);
    println!("用户: {}", user);
    println!("成年? {}", user.is_adult());
    user.deactivate();
    println!("停用后: {:?}", user);

    let point = basic_syntax::Point(3.0, 4.0);
    println!("点到原点距离: {:.2}", point.distance_to_origin());

    println!("\n=== 5. 枚举与模式匹配 ===");
    println!("{}", basic_syntax::enum_demo());
    println!("成绩等级: 85 -> {}", basic_syntax::grade(85));
    println!("数字分类: 7 -> {}", basic_syntax::classify_number(7));
    println!(
        "if let: {:?}",
        basic_syntax::if_let_demo(Some(42))
    );
    println!(
        "if let None: {:?}",
        basic_syntax::if_let_demo(None)
    );

    println!("\n=== 6. 控制流 ===");
    println!("5! = {}", basic_syntax::factorial(5));
    println!("{:?}", basic_syntax::control_flow_samples(4));

    println!("\n=== 7. 函数与高阶函数 ===");
    println!("apply double(3): {}", basic_syntax::apply_operation(3, basic_syntax::double));
    println!("apply square(5): {}", basic_syntax::apply_operation(5, basic_syntax::square));
    println!("min_max: {:?}", basic_syntax::min_max(&[3, 1, 4, 1, 5, 9]));

    println!("\n=== 8. 字符串 String 与 &str ===");
    println!("第一个单词: '{}'", basic_syntax::first_word("hello rust world"));
    println!("{}", basic_syntax::string_conversion_demo());
    println!("{}", basic_syntax::string_operations_demo());
    println!("用户名规范化: {}", basic_syntax::normalize_username(" Alice Chen "));

    println!("\n=== 9. Result、Option 与错误处理 ===");
    println!(
        "端口解析: {:?}",
        basic_syntax::parse_port_with_shadowing(" 8080 ")
    );
    println!("安全除法: {:?}", basic_syntax::checked_divide(10, 2));
    println!("除零: {:?}", basic_syntax::checked_divide(10, 0));
    println!("Option 链式: {}", basic_syntax::option_chain_demo(" 21 "));

    println!("\n=== 10. 自定义错误类型 ===");
    match basic_syntax::parse_and_validate("42") {
        Ok(v) => println!("验证通过: {}", v),
        Err(e) => println!("错误: {}", e),
    }
    match basic_syntax::parse_and_validate("") {
        Ok(v) => println!("验证通过: {}", v),
        Err(e) => println!("错误: {}", e),
    }
    match basic_syntax::parse_and_validate("200") {
        Ok(v) => println!("验证通过: {}", v),
        Err(e) => println!("错误: {}", e),
    }
    match basic_syntax::parse_and_validate("abc") {
        Ok(v) => println!("验证通过: {}", v),
        Err(e) => println!("错误: {}", e),
    }

    println!("\n=== 11. 操作符重载 ===");
    println!("{}", basic_syntax::operator_overload_demo());

    println!("\n=== 12. 常见内置 trait ===");
    println!("{}", basic_syntax::builtin_traits_demo());
    println!("{}", basic_syntax::copy_trait_demo());

    println!("\n=== 13. 迭代器 ===");
    println!("{:?}", basic_syntax::word_count("rust rust java go rust"));
    println!(
        "偶数平方: {:?}",
        basic_syntax::iterator_chain_demo(&[1, 2, 3, 4, 5, 6])
    );
    println!("{}", basic_syntax::iterator_methods_demo());
    println!("{}", basic_syntax::iterator_lazy_demo());
}
