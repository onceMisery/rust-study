use advanced_features::{
    from_into_demo, as_ref_demo, try_from_demo, deref_demo, conversion_patterns,
};

fn main() {
    println!("=== 1. From 与 Into ===");
    println!("{}", from_into_demo());

    println!("\n=== 2. AsRef 与 AsMut ===");
    println!("{}", as_ref_demo());

    println!("\n=== 3. TryFrom 与 TryInto ===");
    println!("{}", try_from_demo());

    println!("\n=== 4. Deref 自动解引用 ===");
    println!("{}", deref_demo());

    println!("\n=== 5. 常见转换模式 ===");
    println!("{}", conversion_patterns());

    println!("\n=== 6. 类型转换 trait 选择指南 ===");
    println!("| Trait | 方向 | 是否可能失败 | 示例 |");
    println!("|-------|------|-------------|------|");
    println!("| From  | T -> U | ❌ 不会 | String::from(\"hi\") |");
    println!("| Into  | T -> U | ❌ 不会 | let s: String = \"hi\".into() |");
    println!("| TryFrom | T -> U | ✅ 可能 | i32::try_from(big_i64) |");
    println!("| AsRef | &T -> &U | ❌ 不会 | s.as_ref() -> &str |");
    println!("| Deref | &T -> &U | ❌ 不会 | Box<String> -> &str |");

    println!("\n=== 7. 最佳实践 ===");
    println!("✅ 函数参数用 AsRef<str> 接受 &str/String/&String");
    println!("✅ 类型转换用 From/Into（不会失败时）");
    println!("✅ 可能失败的转换用 TryFrom/TryInto");
    println!("❌ 避免滥用 as 关键字（可能静默截断）");
}
