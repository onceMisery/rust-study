fn main() {
    println!("{}", basic_syntax::immutable_then_shadow(5));
    println!("{}", basic_syntax::scalar_summary());
    println!("{}", basic_syntax::describe_tuple(("Rust", 2015, true)));
    println!("5! = {}", basic_syntax::factorial(5));
    println!("8 是 {}", basic_syntax::classify_number(8));
    println!("{:?}", basic_syntax::control_flow_samples(4));
    println!("{}", basic_syntax::first_word("hello rust world"));
}
