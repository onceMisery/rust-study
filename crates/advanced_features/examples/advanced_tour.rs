use advanced_features::{
    describe_pair, longest, notify, shared_borrow_sum, Container, Point, Summary,
};

fn main() {
    println!("=== 所有权与借用 ===");
    let text = String::from("ownership");
    println!(
        "字符串长度: {}",
        advanced_features::replace_with_length(text)
    );
    println!(
        "克隆并保留原始输入: {:?}",
        advanced_features::clone_then_keep_original("borrow")
    );

    let values = vec![1, 2, 3, 4];
    println!("借用求和: {}", shared_borrow_sum(&values));

    let mut title = String::from("Rust");
    advanced_features::append_suffix(&mut title, " 入门");
    println!("可变借用修改后: {title}");

    println!("\n=== 生命周期、trait 与泛型 ===");
    println!("更长的字符串: {}", longest("short", "much longer"));

    let point = Point { x: 3, y: 4 };
    println!("{}", point.summary());
    println!("{}", notify(&point));

    let numbers = Container::new(vec![3, 1, 2]);
    println!("最大值: {:?}", numbers.max_item());
    println!("{}", describe_pair("Rust", 2015));

    println!("\n=== 闭包 ===");
    println!("连续翻倍: {}", advanced_features::apply_twice(3, |x| x * 2));
    println!(
        "过滤金额: {:?}",
        advanced_features::filter_amounts(&[99, 120, 300], 100)
    );
}
