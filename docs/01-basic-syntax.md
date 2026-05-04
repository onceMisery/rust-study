# 阶段一：基础语法

## 变量声明：不可变、可变与遮蔽

Rust 默认变量不可变：

```rust
let x = 5;
```

如果需要修改变量绑定的值，必须使用 `mut`：

```rust
let mut total = 0;
total += 1;
```

Rust 还支持遮蔽。遮蔽不是修改原变量，而是用同名的新绑定覆盖旧绑定：

```rust
let value = 5;
let value = value * 2;
```

实际应用场景：解析配置、逐步转换数据时，遮蔽可以让变量名保持语义稳定。例如先拿到字符串，再解析成数字：

```rust
let port = "8080";
let port: u16 = port.parse().unwrap();
```

注意事项：能不用 `mut` 就不用 `mut`。不可变默认值能减少状态变化，让代码更容易推理。

参考代码：[crates/basic_syntax/src/lib.rs](../crates/basic_syntax/src/lib.rs)

## 数据类型

标量类型包括整数、浮点数、布尔值和字符：

```rust
let integer: i32 = 42;
let float: f64 = 3.14;
let flag: bool = true;
let letter: char = '中';
```

复合类型常见的是元组和数组：

```rust
let language = ("Rust", 2015, true);
let numbers = [1, 2, 3, 4];
```

元组适合临时组合少量不同类型的值；数组适合同类型、固定长度的数据。可变长度集合通常使用 `Vec<T>`。

## 函数、参数与返回值

Rust 函数使用 `fn` 定义，参数必须声明类型：

```rust
fn factorial(number: u32) -> u32 {
    let mut result = 1;
    let mut current = number;

    while current > 1 {
        result *= current;
        current -= 1;
    }

    result
}
```

最后一行没有分号时，它是表达式，会作为返回值。也可以显式使用 `return`，但 Rust 代码通常偏好表达式返回。

注意事项：语句以分号结尾，表达式不以分号结尾。误加分号会把返回值变成 `()`。

## 控制流

`if/else` 是表达式，可以直接赋值：

```rust
let label = if number % 2 == 0 { "even" } else { "odd" };
```

`loop` 是无限循环，常配合 `break`：

```rust
let mut current = 6;
loop {
    if current <= 0 {
        break;
    }
    current -= 2;
}
```

`while` 适合条件循环，`for` 适合遍历迭代器：

```rust
for number in 0..4 {
    println!("{number}");
}
```

最佳实践：遍历集合时优先使用 `for item in collection` 或迭代器方法，少写手动索引循环。

## 字符串切片

`&str` 是字符串切片，常用于只读字符串参数：

```rust
fn first_word(text: &str) -> &str {
    for (index, byte) in text.bytes().enumerate() {
        if byte == b' ' {
            return &text[..index];
        }
    }
    text
}
```

实际应用场景：函数只需要读取字符串时，参数优先写成 `&str`，这样既能接收字符串字面量，也能接收 `String` 的引用。
