use advanced_features::{
    describe_pair, longest, notify, shared_borrow_sum, Circle, Container, Drawable, Point,
    Resizable, Summary,
};

fn main() {
    println!("=== 1. 所有权：移动与克隆 ===");
    let text = String::from("ownership");
    println!(
        "字符串长度: {}",
        advanced_features::replace_with_length(text)
    );
    // text 已经移动，不能再使用
    println!(
        "克隆并保留原始输入: {:?}",
        advanced_features::clone_then_keep_original("borrow")
    );
    println!("{}", advanced_features::ownership_move_demo());
    println!("{}", advanced_features::copy_vs_move_demo());

    println!("\n=== 2. 借用与引用 ===");
    let values = vec![1, 2, 3, 4];
    println!("借用求和: {}", shared_borrow_sum(&values));

    let mut title = String::from("Rust");
    advanced_features::append_suffix(&mut title, " 入门");
    println!("可变借用修改后: {title}");

    println!("{}", advanced_features::borrow_rules_demo());
    println!("{}", advanced_features::no_dangling_reference());

    println!("\n=== 3. 生命周期 ===");
    println!("更长的字符串: {}", longest("short", "much longer"));
    println!("{}", advanced_features::lifetime_elision_demo());
    println!("{}", advanced_features::struct_lifetime_demo());
    println!("static: {}", advanced_features::static_lifetime_demo());

    println!("\n=== 4. Trait 定义与实现 ===");
    let point = Point { x: 3, y: 4 };
    println!("{}", point.summary());
    println!("{}", notify(&point));
    println!("{}", advanced_features::trait_demo());

    println!("\n=== 5. 泛型 ===");
    let numbers = Container::new(vec![3, 1, 2]);
    println!("最大值: {:?}", numbers.max_item());
    println!("{}", describe_pair("Rust", 2015));
    println!("{}", advanced_features::generics_demo());
    println!(
        "泛型 largest: {:?}",
        advanced_features::largest(&[3, 1, 4, 1, 5, 9])
    );

    println!("\n=== 6. Trait Object 与动态分发 ===");
    println!("{}", advanced_features::trait_object_demo());
    println!("{}", advanced_features::dispatch_comparison());

    println!("\n=== 7. Trait + 泛型 + 多态 ===");
    println!(
        "Display+Debug: {}",
        advanced_features::display_and_debug(42)
    );
    println!(
        "复杂约束: {}",
        advanced_features::complex_bounds("hello", 123)
    );

    let mut circle = Circle { radius: 5.0 };
    println!("{}", circle.draw());
    circle.resize(2.0);
    println!("放大后: {}", circle.draw());

    let shapes: Vec<&dyn Drawable> = vec![&circle];
    println!("绘制所有: {:?}", advanced_features::draw_all(&shapes));

    println!("\n=== 8. 闭包 ===");
    println!(
        "连续翻倍: {}",
        advanced_features::apply_twice(3, |x| x * 2)
    );
    println!(
        "过滤金额: {:?}",
        advanced_features::filter_amounts(&[99, 120, 300], 100)
    );
    println!("{}", advanced_features::closure_fn_mut_demo());
    println!("{}", advanced_features::closure_fn_once_demo());
    println!("{}", advanced_features::closure_demo());

    println!("\n=== 9. 借用检查器 ===");
    println!("{}", advanced_features::borrow_checker_demo());
    println!("{}", advanced_features::nll_demo());
}
