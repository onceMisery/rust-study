use advanced_features::{
    describe_pair, longest, notify, replace_with_length, shared_borrow_sum, Container, Point,
    Summary,
};

#[test]
fn advanced_feature_examples_are_runnable() {
    let text = String::from("ownership");
    assert_eq!(replace_with_length(text), 9);
    assert_eq!(
        advanced_features::clone_then_keep_original("borrow"),
        ("borrow".to_string(), 6)
    );

    let values = vec![1, 2, 3, 4];
    assert_eq!(shared_borrow_sum(&values), 10);
    let mut title = String::from("Rust");
    advanced_features::append_suffix(&mut title, " 入门");
    assert_eq!(title, "Rust 入门");

    let left = String::from("short");
    let right = String::from("much longer");
    assert_eq!(longest(&left, &right), "much longer");

    let point = Point { x: 3, y: 4 };
    assert_eq!(point.summary(), "Point(3, 4)");
    assert_eq!(point.category(), "可摘要对象");
    assert_eq!(notify(&point), "通知: Point(3, 4)");

    let numbers = Container::new(vec![3, 1, 2]);
    assert_eq!(numbers.max_item(), Some(3));
    assert_eq!(describe_pair("Rust", 2015), "Rust | 2015");
    assert_eq!(advanced_features::apply_twice(3, |x| x * 2), 12);
    assert_eq!(
        advanced_features::filter_amounts(&[99, 120, 300], 100),
        vec![120, 300]
    );
}
