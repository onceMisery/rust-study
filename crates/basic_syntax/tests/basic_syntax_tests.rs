use basic_syntax::{
    classify_number, control_flow_samples, describe_tuple, factorial, first_word,
    immutable_then_shadow, mutable_counter, scalar_summary,
};

#[test]
fn basic_syntax_examples_are_runnable() {
    assert_eq!(immutable_then_shadow(5), "原始值: 5, 遮蔽后: 10");
    assert_eq!(mutable_counter(3), 6);
    assert_eq!(scalar_summary(), "i32=42, f64=3.14, bool=true, char=中");
    assert_eq!(
        describe_tuple(("Rust", 2015, true)),
        "Rust 在 2015 年发布: true"
    );
    assert_eq!(factorial(5), 120);
    assert_eq!(classify_number(-1), "negative");
    assert_eq!(classify_number(8), "positive-even");
    assert_eq!(control_flow_samples(4), vec![0, 1, 2, 3, 6, 4, 2]);
    assert_eq!(first_word("hello rust world"), "hello");
    assert_eq!(basic_syntax::summarize_numbers(&[3, 1, 4]), (3, 8, Some(4)));
    assert_eq!(basic_syntax::parse_port_with_shadowing(" 8080 "), Ok(8080));
    assert_eq!(basic_syntax::checked_divide(10, 0), None);
    assert_eq!(basic_syntax::grade(85), "B");
    assert_eq!(
        basic_syntax::normalize_username(" Alice Chen "),
        "alice_chen"
    );
    assert_eq!(
        basic_syntax::word_count("rust rust go").get("rust"),
        Some(&2)
    );
}
