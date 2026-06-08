use advanced_features::{
    Container, Point, Summary, display_pair, describe_pair, largest,
    display_and_debug, complex_bounds, draw_all, Circle, Drawable, Resizable,
};

fn main() {
    println!("=== 1. 泛型函数 ===");
    println!("largest(&[3,1,4,1,5]): {:?}", largest(&[3, 1, 4, 1, 5]));
    println!("largest(&[\"c\",\"a\",\"b\"]): {:?}", largest(&["c", "a", "b"]));
    println!("{}", display_pair("Rust", 2015));
    println!("{}", describe_pair("hello", 42));

    println!("\n=== 2. 泛型结构体 ===");
    let int_c = Container::new(vec![3, 1, 4, 1, 5]);
    println!("int container: len={}, max={:?}", int_c.len(), int_c.max_item());

    let str_c = Container::new(vec!["hello", "world", "rust"]);
    println!("str container: len={}, first={:?}", str_c.len(), str_c.first());

    println!("\n=== 3. Trait 定义与实现 ===");
    let point = Point { x: 3, y: 4 };
    println!("Point summary: {}", point.summary());
    println!("Point category: {}", point.category());

    let article = advanced_features::Article {
        title: "Rust 入门".to_string(),
        author: "张三".to_string(),
        content: "这是一篇关于 Rust 语言核心概念的详细教程文章。".to_string(),
    };
    println!("Article summary: {}", article.summary());

    println!("\n=== 4. Trait Bound 约束 ===");
    println!("{}", display_and_debug(42));
    println!("{}", complex_bounds("hello", 123));

    println!("\n=== 5. Trait Object 动态分发 ===");
    let items: Vec<Box<dyn Summary>> = vec![
        Box::new(Point { x: 1, y: 2 }),
        Box::new(article),
    ];
    for item in &items {
        println!("  {}", item.summary());
    }

    println!("\n=== 6. 多态：Trait 组合 ===");
    let mut circle = Circle { radius: 5.0 };
    println!("{}", circle.draw());
    circle.resize(2.0);
    println!("放大后: {}", circle.draw());

    let shapes: Vec<&dyn Drawable> = vec![&circle];
    println!("绘制所有: {:?}", draw_all(&shapes));

    println!("\n=== 7. 单态化 vs 动态分发 ===");
    println!("单态化（静态分发）：编译时为每种类型生成独立代码，零运行时开销");
    println!("动态分发（dyn Trait）：通过 vtable 查找方法，有少量开销但更灵活");
}
