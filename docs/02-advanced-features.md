# 阶段二：进阶特性

## 所有权系统

Rust 的每个值都有唯一所有者。所有者离开作用域时，值会被自动释放。这个规则让 Rust 不需要垃圾回收器，也不需要程序员手动 `free`。

```rust
let text = String::from("ownership");
let len = advanced_features::replace_with_length(text);
```

上面代码中，`text` 被移动进函数。移动之后，调用方不能再使用 `text`。这避免了悬垂指针和重复释放。

设计原理：栈上的简单值可以快速复制，堆上的资源需要明确所有权。Rust 把资源释放绑定到作用域结束，这种模式也叫 RAII。

注意事项：如果函数只需要读取数据，不要取得所有权，使用引用。

## 借用与引用

引用允许函数临时访问值，而不取得所有权：

```rust
let values = vec![1, 2, 3, 4];
let sum = advanced_features::shared_borrow_sum(&values);
```

共享引用 `&T` 可以有多个；可变引用 `&mut T` 在同一时间只能有一个。这个规则从类型层面避免数据竞争。

实际应用场景：读取集合、格式化数据、校验输入时通常使用共享引用；需要原地修改数据时才使用可变引用。

## 生命周期

生命周期描述引用有效多久。多数情况下编译器能自动推导；当一个函数返回的引用可能来自多个输入引用时，需要显式标注：

```rust
pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}
```

这里的 `'a` 表示返回值不会比 `left` 和 `right` 中较短的那个活得更久。生命周期不是延长变量寿命，而是告诉编译器引用之间的关系。

注意事项：不要一遇到生命周期报错就到处加 `'static`。`'static` 表示引用可存活整个程序周期，通常不是业务数据真正需要的语义。

## Trait：接口、默认方法与动态能力

trait 类似 Java 的 interface 或 Go 的 interface，但它和泛型、静态分发结合更紧密：

```rust
pub trait Summary {
    fn summary(&self) -> String;

    fn category(&self) -> &'static str {
        "可摘要对象"
    }
}
```

实现 trait：

```rust
impl Summary for Point {
    fn summary(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
}
```

默认方法适合提供通用行为，具体类型只需要实现最核心的方法。

## 泛型与类型参数约束

泛型让代码可以处理多种类型，trait bound 则说明这些类型必须具备哪些能力：

```rust
impl<T> Container<T>
where
    T: Ord + Copy,
{
    pub fn max_item(&self) -> Option<T> {
        self.items.iter().copied().max()
    }
}
```

这里 `T: Ord + Copy` 表示元素既能比较大小，也能按位复制。Rust 泛型默认使用单态化：编译器会为具体类型生成专门代码，因此通常没有运行时泛型开销。

最佳实践：函数参数优先使用最小必要约束。例如只需要打印就约束 `Display`，不要要求更强的 trait。
