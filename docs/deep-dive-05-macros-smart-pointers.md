# 专题五：宏与智能指针

宏是 Rust 的元编程工具，智能指针是 Rust 扩展普通引用能力的核心抽象。两者都是 Rust 高级特性的重要组成部分。

配套代码：

```powershell
cargo run -p advanced_features --example smart_pointers_tour
```

---

## 1. 声明式宏（macro_rules!）

### 基本语法

```rust
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

let v = my_vec![1, 2, 3];  // 展开为 Vec 创建代码
```

### 宏 vs 函数

| 特性 | 宏 | 函数 |
|------|-----|------|
| 展开时机 | 编译时 | 运行时 |
| 参数类型 | 任意语法片段 | 必须指定类型 |
| 返回值 | 可以是任意表达式 | 固定返回类型 |
| 性能 | 零运行时开销 | 函数调用开销 |
| 可读性 | 较难理解 | 清晰直观 |

### 常见内置宏

| 宏 | 用途 | 示例 |
|----|------|------|
| `println!` | 格式化打印 | `println!("{} = {}", "x", 42)` |
| `format!` | 格式化到 String | `format!("Hello, {}!", name)` |
| `vec!` | 创建 Vec | `vec![1, 2, 3]` |
| `assert_eq!` | 断言相等 | `assert_eq!(2 + 2, 4)` |
| `panic!` | 程序崩溃 | `panic!("不应该到这里")` |
| `todo!` | 标记未完成 | `todo!("待实现")` |

### 何时使用宏

- 需要接受可变数量参数（`println!`、`vec!`）
- 需要操作语法（而非值）
- 需要编译时计算
- 大多数情况下优先用函数，只有函数无法满足时才用宏

---

## 2. 过程宏（简介）

过程宏在编译时处理代码，生成新代码。三种类型：

| 类型 | 用途 | 示例 |
|------|------|------|
| `#[derive]` 宏 | 自动实现 trait | `#[derive(Debug, Clone)]` |
| 属性宏 | 修改函数/结构体 | `#[tokio::main]` |
| 函数式宏 | 像函数调用的宏 | `html! { <div>...</div> }` |

```rust
// derive 宏：自动实现 Debug、Clone、PartialEq
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}
```

---

## 3. Box：堆上分配

### 基本用法

```rust
let boxed = Box::new(42);         // i32 放在堆上
println!("{}", boxed);            // 42
println!("大小: {} 字节", std::mem::size_of_val(&boxed));  // 8（只是指针）
```

### 递归类型

编译器无法确定递归类型的大小，必须用 Box：

```rust
enum List {
    Cons(i32, Box<List>),  // Box 让 Cons 的大小固定
    Nil,
}

let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
```

### 何时使用 Box

- 递归类型（链表、树）
- 大量数据需要转移所有权时（避免复制）
- 只关心值的类型（trait object：`Box<dyn Trait>`）

---

## 4. Rc：引用计数共享所有权

### 基本用法

```rust
use std::rc::Rc;

let shared = Rc::new(String::from("共享数据"));
let owner1 = Rc::clone(&shared);   // 引用计数 +1
let owner2 = Rc::clone(&shared);   // 引用计数 +1

println!("引用计数: {}", Rc::strong_count(&shared));  // 3
// shared、owner1、owner2 都指向同一份数据
// 最后一个 Rc 被 drop 时，数据才被释放
```

### Rc 的限制

- ❌ **只能单线程使用**。多线程场景用 `Arc`（Atomic Rc）
- ❌ **数据不可变**。需要可变共享时用 `Rc<RefCell<T>>`

---

## 5. RefCell：内部可变性

### 基本用法

```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);
data.borrow_mut().push(4);       // 可变借用
data.borrow_mut().push(5);
let snapshot = data.borrow().clone();  // 不可变借用
println!("{:?}", snapshot);      // [1, 2, 3, 4, 5]
```

### RefCell vs 普通借用

| 特性 | 普通借用（编译期） | RefCell（运行时） |
|------|-------------------|------------------|
| 检查时机 | 编译期 | 运行时 |
| 违反规则 | 编译错误 | panic |
| 适用场景 | 大多数情况 | 需要内部可变性时 |
| 性能 | 零开销 | 少量运行时检查开销 |

### Rc + RefCell 组合

```rust
use std::rc::Rc;
use std::cell::RefCell;

let shared_list = Rc::new(RefCell::new(vec![1, 2, 3]));

let view1 = Rc::clone(&shared_list);
let view2 = Rc::clone(&shared_list);

view1.borrow_mut().push(4);     // view1 修改
println!("{:?}", view2.borrow());  // view2 看到修改：[1, 2, 3, 4]
```

---

## 6. 智能指针选择指南

| 类型 | 所有权 | 线程安全 | 可变性 | 典型场景 |
|------|--------|----------|--------|----------|
| `Box<T>` | 单一 | ✅ | 外部可变 | 递归类型、大数据转移 |
| `Rc<T>` | 共享（计数） | ❌ | ❌ 不可变 | 图结构、DAG |
| `Arc<T>` | 共享（原子计数） | ✅ | ❌ 不可变 | 多线程共享 |
| `RefCell<T>` | 单一 | ❌ | ✅ 运行时可变 | 内部可变性 |
| `Mutex<T>` | 单一 | ✅ | ✅ 锁保护 | 多线程可变共享 |
| `Rc<RefCell<T>>` | 共享 | ❌ | ✅ | 单线程图结构 |
| `Arc<Mutex<T>>` | 共享 | ✅ | ✅ | 多线程可变共享 |

---

## 7. Deref 与 Drop

### Deref：自动解引用

```rust
let boxed = Box::new(String::from("hello"));
let s: &str = &*boxed;   // Box<String> → &String → &str
// 自动 Deref 链：Box<String> → String → str
```

### Drop：自动清理

```rust
struct FileHandle { name: String }

impl Drop for FileHandle {
    fn drop(&mut self) {
        println!("关闭文件: {}", self.name);
    }
}

{
    let f = FileHandle { name: "data.txt".into() };
    // 离开作用域时自动调用 drop
}
```

---

## 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 堆分配 | 显式 `Box::new()` | 隐式（new 关键字） | 隐式（new 或字面量） |
| 引用计数 | `Rc`/`Arc` | GC 追踪 | GC 追踪 |
| 内部可变性 | `RefCell` | 所有对象默认可变 | 所有变量默认可变 |
| 宏 | `macro_rules!` + 过程宏 | 注解处理器 | `go generate` |

---

## 最佳实践

1. **优先用普通引用**，只在必要时用智能指针
2. **递归类型必须用 Box**
3. **单线程共享用 `Rc`，多线程用 `Arc`**
4. **需要可变共享时用 `Rc<RefCell<T>>` 或 `Arc<Mutex<T>>`**
5. **优先用函数，只有函数无法满足时才用宏**
6. **`derive` 宏是日常最常用的宏**（Debug、Clone、PartialEq 等）

## 配套代码

```powershell
cargo run -p advanced_features --example smart_pointers_tour
```
