fn main() {
    println!("host = {:?}", engineering_practice::find_config("host"));
    println!("port = {:?}", engineering_practice::parse_port("8080"));
    println!("12 / 3 = {:?}", engineering_practice::divide(12, 3));
    println!(
        "线程计算: {}",
        engineering_practice::add_with_thread(20, 22)
    );
    println!(
        "channel 求和: {}",
        engineering_practice::sum_with_channel(vec![1, 2, 3, 4])
    );
    println!("共享计数: {}", engineering_practice::shared_counter(8));
}
