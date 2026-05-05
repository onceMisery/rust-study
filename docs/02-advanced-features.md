# 阶段二：进阶特性

## Rust 闭包与所有权

闭包（closure）是**可以捕获其环境中变量的匿名函数**。Rust 的闭包与所有权系统紧密结合，理解它们的关系非常重要。

## 一、闭包基础语法

```rust
fn main() {
    // 闭包定义：|参数| -> 返回值 { 函数体 }
    let add_one = |x: i32| -> i32 { x + 1 };
    
    // 类型可省略（编译器推导）
    let add_one = |x| x + 1;
    
    // 调用闭包
    let result = add_one(5);
    println!("{}", result);  // 6
    
    // 多参数闭包
    let multiply = |a, b| a * b;
    println!("{}", multiply(3, 4));  // 12
    
    // 无参数闭包
    let greet = || println!("Hello!");
    greet();
}
```

## 二、闭包捕获环境（核心特性）

**闭包可以自动捕获定义时所在作用域的变量：**

```rust
fn main() {
    let x = 10;
    let y = 20;
    
    // 闭包捕获了 x 和 y
    let add = || {
        println!("x + y = {}", x + y);
    };
    
    add();  // 输出: x + y = 30
}
```

### 三种捕获方式（对应三个 Fn 系列 trait [类似接口]）

Rust 根据闭包如何使用捕获的变量，自动选择三种方式之一：

| 方式         | 对应的 Trait | 转移所有权          | 使用场景          |
|------------|-----------|----------------|---------------|
| **Fn**     | `Fn`      | 不可变借用（`&T`）    | 只读访问，可多次调用    |
| **FnMut**  | `FnMut`   | 可变借用（`&mut T`） | 修改捕获的变量，可多次调用 |
| **FnOnce** | `FnOnce`  | 转移所有权（`T`）     | 只能调用一次        |

```rust
fn main() {
    let s = String::from("hello");
    
    // 1. Fn: 不可变借用（只读）
    let read_only = || {
        println!("{}", s);  // 只是读取，不修改
    };
    read_only();
    read_only();  // 可以多次调用
    println!("调用后仍可使用 s: {}", s);  // ✅ s 仍有效
    
    let mut count = 0;
    
    // 2. FnMut: 可变借用（修改）
    let mut mutable = || {
        count += 1;  // 修改捕获的变量
        println!("count: {}", count);
    };
    mutable();
    mutable();  // 可以多次调用
    // println!("{}", count);  // ❌ 闭包还在借用 count
    
    // 3. FnOnce: 转移所有权
    let s2 = String::from("world");
    let takes_ownership = || {
        drop(s2);  // 转移所有权给 drop 函数
    };
    takes_ownership();
    // takes_ownership();  // ❌ 不能再次调用（s2 已被移动）
    // println!("{}", s2);  // ❌ s2 所有权已转移
}
```

## 三、闭包与所有权的详细示例

### 示例 1：不可变借用（Fn）

```rust
fn main() {
    let data = vec![1, 2, 3];
    
    let print_data = || {
        println!("{:?}", data);  // 只读借用
    };
    
    print_data();
    print_data();  // 可以多次调用
    println!("仍然可以访问 data: {:?}", data);  // ✅ 所有权未转移
}
```

### 示例 2：可变借用（FnMut）

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];
    
    let mut push_number = || {
        numbers.push(4);  // 修改借用
        println!("{:?}", numbers);
    };
    
    push_number();  // [1, 2, 3, 4]
    push_number();  // [1, 2, 3, 4, 4]
    
    // numbers.push(5);  // ❌ 闭包还在借用 numbers
    drop(push_number);  // 手动释放闭包
    numbers.push(5);    // ✅ 闭包结束后可以访问
}
```

### 示例 3：转移所有权（FnOnce）

```rust
fn main() {
    let s = String::from("Hello");
    
    let consume = || {
        let owned = s;  // 转移所有权到闭包内
        println!("{}", owned);
        // s 在这里被 drop
    };
    
    consume();
    // consume();  // ❌ 不能调用两次
    // println!("{}", s);  // ❌ s 所有权已转移
}
```

### 示例 4：强制所有权转移（`move` 关键字）

```rust
fn main() {
    let s = String::from("Hello");
    
    // 使用 move 强制将所有权转移到闭包中
    let consume = move || {
        println!("{}", s);
        // s 被移动到闭包中
    };
    
    consume();
    // println!("{}", s);  // ❌ s 已被移动到闭包
}
```

**move 闭包的典型用途：多线程**

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];
    
    // move 将 data 的所有权转移到新线程
    let handle = thread::spawn(move || {
        println!("子线程: {:?}", data);
    });
    
    handle.join().unwrap();
    // println!("{:?}", data);  // ❌ data 已被移动到子线程
}
```

## 四、闭包作为函数参数

```rust
// 三种泛型约束写法

// 1. 接受不可变闭包（最常见）
fn call_twice<F>(closure: F) 
where
    F: Fn(),
{
    closure();
    closure();
}

// 2. 接受可变闭包
fn modify<F>(mut closure: F)
where
    F: FnMut(),
{
    closure();
    closure();
}

// 3. 接受一次性闭包
fn run_once<F>(closure: F)
where
    F: FnOnce(),
{
    closure();
}

fn main() {
    let x = 10;
    let print = || println!("x = {}", x);
    call_twice(print);  // ✅ Fn 可以传给所有类型
    
    let mut count = 0;
    let increment = || count += 1;
    modify(increment);  // ✅ FnMut
    
    let s = String::from("hello");
    let consume = || drop(s);
    run_once(consume);  // ✅ FnOnce
}
```

## 五、返回闭包

闭包大小未知，必须放在 `Box` 或使用 `impl Trait`：

```rust
// 方法1：使用 Box（动态分发）
fn factory_one() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// 方法2：使用 impl Trait（静态分发，Rust 2021+）
fn factory_two() -> impl Fn(i32) -> i32 {
    |x| x * 2
}

// 方法3：带 move 的返回
fn factory_three() -> impl FnOnce() -> String {
    let msg = String::from("Hello");
    move || msg  // 返回所有权
}

fn main() {
    let f1 = factory_one();
    println!("{}", f1(5));  // 6
    
    let f2 = factory_two();
    println!("{}", f2(5));  // 10
    
    let f3 = factory_three();
    println!("{}", f3());   // Hello
}
```

## 六、实战示例：自定义排序

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let mut people = vec![
        Person { name: String::from("Alice"), age: 30 },
        Person { name: String::from("Bob"), age: 25 },
        Person { name: String::from("Charlie"), age: 35 },
    ];
    
    // 按年龄排序（使用闭包）
    people.sort_by(|a, b| a.age.cmp(&b.age));
    println!("按年龄排序: {:?}", people);
    
    // 按名字长度排序
    people.sort_by(|a, b| a.name.len().cmp(&b.name.len()));
    println!("按名字长度排序: {:?}", people);
    
    // 使用外部参数进行过滤
    let min_age = 30;
    let adults: Vec<_> = people
        .iter()
        .filter(|p| p.age >= min_age)  // 捕获 min_age
        .collect();
    println!("成年人: {:?}", adults);
}
```

## 七、所有权规则总结

| 捕获方式          | 闭包类型     | 所有权变化 | 调用次数 | 外部变量状态  |
|---------------|----------|-------|------|---------|
| 只读借用          | `Fn`     | 无     | 多次   | 仍可用     |
| 可变借用          | `FnMut`  | 临时借用  | 多次   | 闭包释放后可用 |
| 转移所有权         | `FnOnce` | 转移    | 一次   | 不可再用    |
| `move` + 转移   | `FnOnce` | 强制转移  | 一次   | 不可再用    |
| `move` + 复制类型 | `Fn`     | 复制    | 多次   | 仍可用     |

## 关键要点

1. **闭包自动推断捕获方式**：根据使用情况决定 `Fn`/`FnMut`/`FnOnce`
2. **默认不可变借用**：尽量不转移所有权
3. **用 `move` 强制转移**：需要所有权时（如多线程）
4. **函数参数用泛型约束**：`F: Fn()` 而不是直接使用 `impl`
5. **返回闭包用 `Box` 或 `impl Trait`**
6. **闭包大小未知**：不能直接返回裸闭包

Rust 的闭包设计既提供了类似动态语言的便利性，又保持了所有权系统的安全性，是 Rust 表达力的重要体现。

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
