# 阶段二：进阶特性

这一章是 Rust 的核心。基础语法只是“会写 Rust”，进阶特性决定你是否真正理解 Rust 为什么能做到无 GC、内存安全和高性能。

配套代码：

```powershell
cargo run -p advanced_features --example advanced_tour
```

## 1. 所有权：谁负责释放资源

Rust 的每个值都有一个所有者。所有者离开作用域时，值会被自动释放。

```rust
let text = String::from("ownership");
let len = advanced_features::replace_with_length(text);
// text 在这里不能再使用，因为所有权已经移动进函数
```

这和 Java / Go 最大的不同是：Rust 没有 GC 后台回收对象，而是在编译期确定资源何时释放。

常见规则：

1. 一个值同一时间只有一个所有者。
2. 赋值、传参、返回值可能发生移动。
3. 所有者离开作用域时自动释放值。

如果你想保留原值，可以显式克隆：

```rust
let (owned, len) = advanced_features::clone_then_keep_original("borrow");
```

注意：`clone` 对堆数据有复制成本。新手常见误区是遇到所有权错误就到处 `.clone()`；更好的做法是先判断函数到底需不需要取得所有权。

## 2. 借用与引用：只看一眼，不拿走

如果函数只需要读取数据，应该借用：

```rust
let values = vec![1, 2, 3, 4];
let sum = advanced_features::shared_borrow_sum(&values);
```

`&T` 是共享引用，可以同时存在多个。`&mut T` 是可变引用，同一时间只能存在一个：

```rust
let mut title = String::from("Rust");
advanced_features::append_suffix(&mut title, " 入门");
```

这条规则直接服务于并发安全：如果同一时间不允许多个写入者，也不允许读写混杂，很多数据竞争就不会出现。

## 3. 栈、堆与移动

一般可以这样理解：

| 数据 | 常见位置 | 行为 |
| --- | --- | --- |
| `i32`、`bool`、`char` | 栈 | 通常实现 `Copy`，赋值后原变量仍可用 |
| `String`、`Vec<T>` | 栈上保存指针、长度、容量，真实数据在堆 | 默认移动所有权 |
| `&T` | 栈上的引用值 | 借用，不拥有资源 |

`String` 移动时，不会复制堆上的全部字符，只复制栈上的指针、长度、容量，并让旧变量失效。这避免了双重释放。

## 4. 生命周期：引用不能比数据活得更久

大多数生命周期由编译器自动推导。只有当返回引用和多个输入引用相关时，才需要显式标注：

```rust
pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}
```

`'a` 的意思不是“让变量活得更久”，而是说明返回值和输入值之间的借用关系：返回引用的有效期不能超过两个输入引用的共同有效期。

新手注意：

- 不要把生命周期当成语法补丁。
- 不要随便写 `'static`。
- 如果生命周期标注越来越复杂，通常说明数据所有权边界需要重新设计。

## 5. Trait：Rust 的接口抽象

trait 类似 Java 的 interface，也类似 Go 的 interface，但 Rust 的 trait 更常和泛型一起使用：

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

默认方法适合放通用逻辑，具体类型只实现必要方法即可。

使用 trait 作为参数：

```rust
pub fn notify(item: &impl Summary) -> String {
    format!("通知: {}", item.summary())
}
```

实际应用场景：

- 日志格式化：不同事件实现同一个 `Summary`。
- 持久化抽象：不同存储后端实现同一个 `Repository`。
- 业务规则扩展：不同策略实现同一个 trait。

## 6. 泛型与约束

泛型让容器或函数能处理多种类型：

```rust
pub struct Container<T> {
    items: Vec<T>,
}
```

但泛型不是“什么都能做”。如果你要比较大小，需要约束：

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

`T: Ord + Copy` 表示：元素必须能排序，并且能被复制出来。

最佳实践：约束越小越好。只需要打印就用 `Display`，只需要排序就用 `Ord`，不要为了省事加一堆不必要约束。

## 7. 闭包：能捕获环境的匿名函数

闭包可以捕获外部变量：

```rust
let min_amount = 100;
let filtered: Vec<_> = amounts
    .iter()
    .copied()
    .filter(|amount| *amount >= min_amount)
    .collect();
```

Rust 根据闭包如何使用捕获变量，自动推导它实现哪个 trait：

| trait | 捕获方式 | 调用次数 | 场景 |
| --- | --- | --- | --- |
| `Fn` | 共享借用 | 可多次 | 只读过滤、格式化 |
| `FnMut` | 可变借用 | 可多次 | 累加计数、修改外部状态 |
| `FnOnce` | 取得所有权 | 通常一次 | 消费资源、线程 `move` |

示例代码：

```rust
let doubled = advanced_features::apply_twice(3, |x| x * 2);
let amounts = advanced_features::filter_amounts(&[99, 120, 300], 100);
```

闭包在 Rust 中很常见，尤其是迭代器、线程、异步任务和回调式 API。

## 本章注意事项

- 所有权错误先想“谁拥有数据”，不要急着 `clone`。
- 参数优先用借用，只有确实要接管资源时才用拥有类型。
- 生命周期标注描述关系，不负责延长引用寿命。
- trait 是 Rust 抽象能力的核心，优先学会 trait bound。
- 闭包不是简单语法糖，它和所有权捕获方式绑定。

## 与 Java、Go 的进阶特性对比

| 主题 | Rust | Java | Go |
| --- | --- | --- | --- |
| 内存管理 | 所有权、借用、生命周期，编译期检查 | GC 管理对象生命周期 | GC 管理对象生命周期 |
| 接口抽象 | trait 显式实现，可做泛型约束 | interface/class 体系，OOP 风格强 | interface 隐式实现，组合简单 |
| 泛型实现 | 单态化，通常无运行时泛型开销 | 主要是类型擦除 | 编译器支持类型参数，语法较轻 |
| 闭包捕获 | 捕获方式受所有权约束，区分 `Fn/FnMut/FnOnce` | lambda 捕获 effectively final 变量 | 闭包捕获变量，常和 goroutine 使用 |
| 并发安全基础 | 类型系统阻止数据竞争 | 依靠库、锁和运行时约定 | 依靠 channel、锁和开发者约定 |

Java 和 Go 更强调降低日常业务开发复杂度；Rust 更强调把资源管理和并发安全放到类型系统里。代价是学习曲线更高，收益是在系统级、性能敏感和高可靠场景中减少运行时不确定性。
