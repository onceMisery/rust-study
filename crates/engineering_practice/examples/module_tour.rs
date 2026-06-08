use engineering_practice::module_demo::{module_demo, visibility_demo, math, text};

fn main() {
    println!("=== 1. 模块系统基础 ===");
    println!("{}", module_demo());
    println!("{}", visibility_demo());

    println!("\n=== 2. 模块路径调用 ===");
    // 完整路径
    println!("math::add(5, 3) = {}", math::add(5, 3));
    println!("math::factorial(6) = {}", math::factorial(6));

    // 子模块
    println!("text::char_count(\"Rust\") = {}", text::char_count("Rust"));
    println!("text::format::to_upper(\"hello\") = {}", text::format::to_upper("hello"));
    println!("text::format::to_lower(\"HELLO\") = {}", text::format::to_lower("HELLO"));

    println!("\n=== 3. use 简化路径 ===");
    use engineering_practice::module_demo::math::add;
    println!("use 后直接调用: add(10, 20) = {}", add(10, 20));

    println!("\n=== 4. Cargo 工作空间结构 ===");
    println!("当前项目结构:");
    println!("rust-study/");
    println!("├── Cargo.toml        # workspace 根配置");
    println!("├── crates/");
    println!("│   ├── basic_syntax/    # 基础语法 crate");
    println!("│   ├── advanced_features/ # 进阶特性 crate");
    println!("│   └── engineering_practice/ # 工程实践 crate");
    println!("└── blog/                # 博客应用 crate");

    println!("\n=== 5. 可见性规则总结 ===");
    println!("pub        → 对外公开");
    println!("pub(crate) → 仅 crate 内可见");
    println!("(无 pub)   → 仅当前模块可见");
}
