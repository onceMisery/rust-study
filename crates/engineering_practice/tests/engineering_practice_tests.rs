use engineering_practice::{
    add_with_thread, divide, find_config, parse_port, shared_counter, sum_with_channel, AppError,
};

#[test]
fn engineering_practice_examples_are_runnable() {
    assert_eq!(find_config("host"), Some("localhost"));
    assert_eq!(find_config("missing"), None);
    assert_eq!(parse_port("8080"), Ok(8080));
    assert_eq!(parse_port("abc"), Err(AppError::InvalidPort));
    assert_eq!(divide(12, 3), Ok(4));
    assert_eq!(divide(12, 0), Err(AppError::DivideByZero));

    assert_eq!(add_with_thread(20, 22), 42);
    assert_eq!(sum_with_channel(vec![1, 2, 3, 4]), 10);
    assert_eq!(shared_counter(8), 8);
}
