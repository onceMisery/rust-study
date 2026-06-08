use advanced_features::{
    borrow_checker_demo, borrow_rules_demo, copy_vs_move_demo, longest, nll_demo,
    ownership_move_demo, struct_lifetime_demo, List,
};

fn main() {
    println!("=== 1. 所有权基础规则 ===");
    println!("{}", ownership_move_demo());
    println!("{}", copy_vs_move_demo());

    println!("\n=== 2. 借用规则 ===");
    println!("{}", borrow_rules_demo());

    println!("\n=== 3. 借用检查器与 NLL ===");
    println!("{}", borrow_checker_demo());
    println!("{}", nll_demo());

    println!("\n=== 4. 生命周期 ===");
    println!("longest: {}", longest("short", "much longer"));
    println!("{}", struct_lifetime_demo());

    println!("\n=== 5. 所有权链式传递 ===");
    let s = String::from("hello");
    let len = advanced_features::replace_with_length(s);
    // s 已经移动，不能再使用
    println!("字符串长度: {}", len);

    let (owned, len) = advanced_features::clone_then_keep_original("world");
    println!("克隆保留: owned='{}', len={}", owned, len);

    println!("\n=== 6. 递归类型（Box） ===");
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("链表: {:?}", list);
    println!("转为 Vec: {:?}", list.to_vec());
}
