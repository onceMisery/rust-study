use advanced_features::{
    describe_pair, longest, notify, shared_borrow_sum, Container, Point, Summary,
};

fn main() {
    let text = String::from("ownership");
    println!(
        "字符串长度: {}",
        advanced_features::replace_with_length(text)
    );

    let values = vec![1, 2, 3, 4];
    println!("借用求和: {}", shared_borrow_sum(&values));

    println!("更长的字符串: {}", longest("short", "much longer"));

    let point = Point { x: 3, y: 4 };
    println!("{}", point.summary());
    println!("{}", notify(&point));

    let numbers = Container::new(vec![3, 1, 2]);
    println!("最大值: {:?}", numbers.max_item());
    println!("{}", describe_pair("Rust", 2015));
}
