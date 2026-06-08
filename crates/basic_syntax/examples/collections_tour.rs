fn main() {
    println!("=== 1. Vec 操作 ===");
    println!("{}", basic_syntax::vec_operations());

    println!("\n=== 2. HashMap 操作 ===");
    println!("{}", basic_syntax::hashmap_operations());

    println!("\n=== 3. HashSet 集合运算 ===");
    println!("{}", basic_syntax::hashset_operations());

    println!("\n=== 4. 迭代器高级用法 ===");
    println!("{}", basic_syntax::iterator_advanced());

    println!("\n=== 5. 迭代器适配器链 ===");
    let result: Vec<i32> = (1..=20)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .take(5)
        .collect();
    println!("1-20 偶数的平方前5: {:?}", result);

    println!("\n=== 6. 集合转换 ===");
    let words = "hello world hello rust world";
    let word_count = basic_syntax::word_count(words);
    println!("词频统计: {:?}", word_count);

    println!("\n=== 7. BTreeMap vs HashMap ===");
    println!("BTreeMap: 有序，O(log n)，范围查询");
    println!("HashMap:  无序，O(1)平均，快速查找");
}
