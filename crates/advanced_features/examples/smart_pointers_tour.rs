use advanced_features::{box_demo, rc_demo, refcell_demo, List};

fn main() {
    println!("=== 1. Box：堆上分配 ===");
    println!("{}", box_demo());

    println!("\n=== 2. Box 递归类型 ===");
    let list = List::Cons(10, Box::new(List::Cons(20, Box::new(List::Cons(30, Box::new(List::Nil))))));
    println!("链表: {:?}", list);
    println!("转 Vec: {:?}", list.to_vec());

    println!("\n=== 3. Rc：共享所有权 ===");
    println!("{}", rc_demo());

    // Rc 的实际用途：图结构中多个节点引用同一数据
    use std::rc::Rc;
    let data = Rc::new(vec![1, 2, 3, 4, 5]);
    let view1 = Rc::clone(&data);
    let view2 = Rc::clone(&data);
    println!("view1 sum: {}", view1.iter().sum::<i32>());
    println!("view2 len: {}", view2.len());
    println!("引用计数: {}", Rc::strong_count(&data));

    println!("\n=== 4. RefCell：内部可变性 ===");
    println!("{}", refcell_demo());

    use std::cell::RefCell;
    let config = RefCell::new(std::collections::HashMap::new());
    config.borrow_mut().insert("host", "localhost");
    config.borrow_mut().insert("port", "8080");
    println!("config: {:?}", config.borrow().clone());

    println!("\n=== 5. 智能指针对比 ===");
    println!("| 类型 | 所有权 | 线程安全 | 典型场景 |");
    println!("|------|--------|----------|----------|");
    println!("| Box  | 单一 | ✅ | 递归类型、大数据转移 |");
    println!("| Rc   | 共享（计数） | ❌ | 图结构、DAG |");
    println!("| Arc  | 共享（原子计数） | ✅ | 多线程共享 |");
    println!("| RefCell | 单一（运行时检查） | ❌ | 内部可变性 |");
    println!("| Mutex | 单一（锁保护） | ✅ | 多线程可变共享 |");

    println!("\n=== 6. Deref 与 Drop ===");
    let boxed = Box::new(String::from("hello"));
    let s: &str = &*boxed; // Box<T> 自动 Deref 为 &T
    println!("Deref: {}", s);
    // boxed 离开作用域时自动 Drop（释放堆内存）
}
