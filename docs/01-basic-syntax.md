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

> mut 是 mutable 的缩写，意思是可变的。反义词：immutable（不可变的）

```rust
let x = 5;
x = 6;  // ❌ 编译错误！不能修改不可变变量

let mut y = 5;
y = 6;  // ✅ 正确，y 是可变的
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

以下是 Rust 的主要基础类型（标量类型）汇总表：

| 类别       | 类型名称    | 说明                     | 字面量示例                  |
|----------|---------|------------------------|------------------------|
| **整数**   | `i8`    | 8位有符号整数                | `-10`                  |
|          | `u8`    | 8位无符号整数                | `10`                   |
|          | `i16`   | 16位有符号整数               | `-1000`                |
|          | `u16`   | 16位无符号整数               | `1000`                 |
|          | `i32`   | 32位有符号整数（默认推断类型）       | `-42000`               |
|          | `u32`   | 32位无符号整数               | `42000`                |
|          | `i64`   | 64位有符号整数               | `-123456789`           |
|          | `u64`   | 64位无符号整数               | `123456789`            |
|          | `i128`  | 128位有符号整数              | `-12345678901234`      |
|          | `u128`  | 128位无符号整数              | `12345678901234`       |
|          | `isize` | 指针宽度有符号整数（32/64位取决于架构） | 常用于索引                  |
|          | `usize` | 指针宽度无符号整数（32/64位取决于架构） | 常用于数组索引                |
| **浮点数**  | `f32`   | 32位单精度浮点数              | `3.14f32`              |
|          | `f64`   | 64位双精度浮点数（默认推断类型）      | `3.14` / `2.0`         |
| **布尔**   | `bool`  | 真或假                    | `true` / `false`       |
| **字符**   | `char`  | 单个 Unicode 字符（4字节）     | `'A'` / `'中'` / `'😀'` |
| **单元类型** | `()`    | 空元组，表示无值               | `()` （用于函数无返回值）        |

> **补充说明：**
> - 整数默认推导为 `i32`，浮点数默认推导为 `f64`
> - 数字字面量可加后缀指定类型，如 `42u8`、`3.14f32`
> - 数字支持可读性分隔符：`1_000_000`（等价于 1000000）
> - 字符支持 Unicode，因此可以存储中文、emoji 等

这些是 Rust 的**标量类型**（scalar types），代表单个值。此外还有**复合类型**（compound types）如元组 `(i32, f64, char)` 和数组
`[i32; 5]`。

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
fn 函数名(参数名: 参数类型, ...) -> 返回值类型 {
    // 函数体
    // 最后一条表达式（不加分号）作为返回值
}
```

```rust
fn factorial(number: u32) -> u32 {
    let mut result = 1;
    let mut current = number;

    while current > 1 {
        result *= current;
        current -= 1;
    }

    result  // 等价于 return result; 但一般不用，return一般用于提前返回
}
```

最后一行没有分号时，它是表达式，会作为返回值。也可以显式使用 `return`，但 Rust 代码通常偏好表达式返回。

注意事项：语句以分号结尾，表达式不以分号结尾。误加分号会把返回值变成 `()`。

## 控制流

`if/else` 是表达式，可以直接赋值：

```rust
let label = if number % 2 == 0 { "even" } else { "odd" };
```

```rust
fn factorial_with_early_return(n: u32) -> u32 {
    if n == 0 {
        return 1;  // 提前返回
    }
    // ... 其他逻辑
}
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

**示例**

```rust
fn main() {
    // 1. if-else 表达式
    let age = 18;
    let status = if age >= 18 {
        "成年人"
    } else {
        "未成年人"
    };
    println!("状态: {}", status);

    // 2. while 循环
    let mut count = 3;
    while count > 0 {
        println!("while 倒计时: {}", count);
        count -= 1;
    }

    // 3. loop 无限循环（手动 break）
    let mut attempts = 0;
    let result = loop {
        attempts += 1;
        if attempts >= 3 {
            break attempts;  // 跳出循环并返回值
        }
        println!("第 {} 次尝试", attempts);
    };
    println!("loop 共尝试 {} 次", result);

    // 4. for 循环（遍历范围）
    println!("for 循环 1 到 3:");
    for i in 1..=3 {  // 包含 3
        println!("数字: {}", i);
    }

    // 5. for 循环（遍历数组）
    let numbers = [10, 20, 30, 40];
    for &num in numbers.iter() {
        println!("数组元素: {}", num);
    }

    // 6. match 模式匹配（类似 switch）
    let score = 85;
    let grade = match score {
        90..=100 => "A",
        80..=89  => "B",
        70..=79  => "C",
        60..=69  => "D",
        _        => "F",  // 下划线代表其他所有情况
    };
    println!("得分: {}, 等级: {}", score, grade);

    // 7. if-else if-else 链
    let temperature = 25;
    if temperature > 30 {
        println!("天气很热");
    } else if temperature > 20 {
        println!("天气温暖");
    } else {
        println!("天气凉爽");
    }

    // 8. 嵌套循环 + 标签（跳出外层循环）
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if i == 2 && j == 2 {
                println!("遇到 i=2,j=2，跳出外层循环");
                break 'outer;
            }
            println!("i={}, j={}", i, j);
        }
    }
}
```

## 字符串

### 字符串切片

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
Rust 的字符串主要有两种类型：`String`（可变的堆分配字符串）和 `&str`（不可变的字符串切片）。以下是常用操作分类整理：

## 创建字符串

```rust
// 1. 创建空 String
let mut s = String::new();

// 2. 从字面量创建
let s = String::from("hello");
let s = "hello".to_string();
let s = "hello".to_owned();

// 3. 字符串字面量 (&str)
let lit: &str = "hello";
```

## 常用操作（String 类型）

| 操作       | 方法/语法                                       | 示例                        |
|----------|---------------------------------------------|---------------------------|
| **获取长度** | `.len()`                                    | `"hello".len()` → 5       |
| **判断为空** | `.is_empty()`                               | `"".is_empty()` → true    |
| **拼接**   | `push()` / `push_str()` / `+` / `format!`   | 见下方示例                     |
| **索引**   | ❌ 不支持直接索引                                   | 需用 `chars()` 或 `bytes()`  |
| **切片**   | `&s[start..end]`                            | `&s[0..4]`（需字节边界）         |
| **遍历**   | `.chars()` / `.bytes()`                     | `for c in s.chars()`      |
| **查找**   | `.find()` / `.contains()`                   | `s.find("world")`         |
| **替换**   | `.replace()`                                | `s.replace("a", "b")`     |
| **修剪**   | `.trim()` / `.trim_start()` / `.trim_end()` | `"  hi  ".trim()` → "hi"  |
| **分割**   | `.split()` / `.split_whitespace()`          | `s.split(",").collect()`  |
| **大小写**  | `.to_uppercase()` / `.to_lowercase()`       | `"hello".to_uppercase()`  |
| **追加**   | `.push()` / `.push_str()`                   | 见下方示例                     |
| **插入**   | `.insert()` / `.insert_str()`               | `s.insert(0, 'H')`        |
| **删除**   | `.pop()` / `.remove()` / `.clear()`         | `s.pop()` 删除最后字符          |
| **连接多个** | `.join()` / `.concat()`                     | `vec!["a","b"].join(",")` |

## 代码示例

```rust
fn main() {
    let mut s = String::from("hello");
    
    // 1. 追加字符/字符串
    s.push(' ');           // hello 
    s.push_str("world");   // hello world
    println!("{}", s);
    
    // 2. 拼接（推荐 format!）
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let result = format!("{}-{}-{}", s1, s2, s3);  // tic-tac-toe
    
    // 3. 切片（注意边界）
    let hello = "Здравствуйте";
    let s = &hello[0..4];  // "Зд" (每个字符占2字节)
    
    // 4. 遍历字符
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    
    // 5. 字符串查找与替换
    let story = "Rust is great, Rust is powerful";
    println!("contains 'great': {}", story.contains("great"));
    println!("find 'great': {:?}", story.find("great"));  // Some(8)
    println!("replace: {}", story.replace("Rust", "Go"));
    
    // 6. 分割处理
    let csv = "apple,banana,orange";
    let fruits: Vec<&str> = csv.split(',').collect();
    println!("{:?}", fruits);  // ["apple", "banana", "orange"]
    
    // 7. 修剪空白
    let messy = "  hello world  \n";
    println!("trimmed: '{}'", messy.trim());
    
    // 8. 大小写转换
    let mixed = "Hello World";
    println!("uppercase: {}", mixed.to_uppercase());
    println!("lowercase: {}", mixed.to_lowercase());
    
    // 9. 删除操作
    let mut s = String::from("hello world");
    s.pop();  // 删除最后一个字符，返回 Option<char>
    println!("after pop: {}", s);  // hello worl
    s.remove(0);  // 删除索引0处的字符
    println!("after remove: {}", s);  // ello worl
    s.clear();  // 清空字符串
    println!("is empty: {}", s.is_empty());
    
    // 10. 字符串连接
    let words = vec!["hello", "world", "from", "rust"];
    let sentence = words.join(" ");
    println!("{}", sentence);  // hello world from rust
}
```

## String vs &str 对比

| 特性   | `String`                  | `&str`        |
|------|---------------------------|---------------|
| 可变性  | ✅ 可变                      | ❌ 不可变         |
| 所有权  | 拥有的                       | 借用的           |
| 内存位置 | 堆上                        | 可位于堆、栈或二进制数据段 |
| 修改操作 | 支持（push, insert, remove等） | 不支持           |
| 性能   | 有运行时成本                    | 极轻量           |
| 使用场景 | 需要所有权或修改时                 | 字符串字面量、函数参数   |

## 常见陷阱与注意事项

```rust
// ❌ 错误：不能通过索引访问
let s = String::from("hello");
// let c = s[0];  // 编译错误！

// ✅ 正确方式
let c = &s[0..1];  // 切片
let chars: Vec<char> = s.chars().collect();
let first_char = chars[0];

// ⚠️ 注意 UTF-8 边界
let chinese = "中";
println!("len: {}", chinese.len());  // 3（字节数）
println!("chars: {}", chinese.chars().count());  // 1（字符数）

// ✅ 函数参数建议用 &str
fn greet(name: &str) {
    println!("Hello, {}", name);
}
greet(&String::from("Alice"));  // 可传入 &String 自动解引用
greet("Bob");  // 也可直接传字面量
```

这些是 Rust 字符串最常用的操作。Rust 的字符串处理强调正确处理 UTF-8，这也是它为何不支持直接用索引的原因。

## 集合

Rust 的标准集合类型主要分布在 `std::collections` 中，对应 Java 的概念如下：

| Java         | Rust                                   | 说明          |
|--------------|----------------------------------------|-------------|
| `ArrayList`  | `Vec<T>`                               | 可变数组，最常用    |
| `HashSet`    | `HashSet<T>`                           | 基于哈希的无序集合   |
| `TreeSet`    | `BTreeSet<T>`                          | 基于 B 树的有序集合 |
| `HashMap`    | `HashMap<K,V>`                         | 基于哈希的无序映射   |
| `TreeMap`    | `BTreeMap<K,V>`                        | 基于 B 树的有序映射 |
| `LinkedList` | `VecDeque<T>` (双端队列) 或 `LinkedList<T>` | 双向链表（很少用）   |

以下是完整的示例代码：

```rust
use std::collections::{HashMap, HashSet, BTreeMap, BTreeSet, VecDeque};

fn main() {
    // ========== 1. Vec<T> - 类似 ArrayList ==========
    println!("=== Vec (动态数组) ===");
    
    // 创建
    let mut vec1 = Vec::new();
    vec1.push(10);
    vec1.push(20);
    vec1.push(30);
    
    let vec2 = vec![1, 2, 3, 4, 5];  // 宏创建
    
    // 访问
    println!("第一个元素: {}", vec2[0]);           // 索引访问（可能 panic）
    println!("第一个元素: {:?}", vec2.get(0));     // 安全访问返回 Option
    
    // 遍历
    for item in &vec2 {
        print!("{} ", item);
    }
    println!();
    
    // 修改
    vec1[1] = 25;
    vec1.push(40);
    vec1.pop();  // 删除最后一个
    
    // 删除指定位置
    vec1.remove(0);
    
    // 包含判断
    println!("vec1 是否包含 25? {}", vec1.contains(&25));
    
    // 排序
    let mut unsorted = vec![3, 1, 4, 1, 5];
    unsorted.sort();
    println!("排序后: {:?}", unsorted);
    
    // ========== 2. HashSet<T> - 类似 Java HashSet ==========
    println!("\n=== HashSet (无序集合) ===");
    
    let mut set = HashSet::new();
    set.insert("apple");
    set.insert("banana");
    set.insert("apple");  // 重复插入无效
    set.insert("orange");
    
    println!("集合: {:?}", set);
    println!("是否包含 'banana'? {}", set.contains("banana"));
    println!("元素个数: {}", set.len());
    
    // 遍历
    for fruit in &set {
        println!("  {}", fruit);
    }
    
    // 删除
    set.remove("orange");
    
    // 集合运算
    let set1: HashSet<_> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<_> = [3, 4, 5, 6].iter().cloned().collect();
    
    println!("交集: {:?}", set1.intersection(&set2).collect::<Vec<_>>());
    println!("并集: {:?}", set1.union(&set2).collect::<Vec<_>>());
    println!("差集 (set1 - set2): {:?}", set1.difference(&set2).collect::<Vec<_>>());
    
    // ========== 3. BTreeSet<T> - 类似 Java TreeSet (有序) ==========
    println!("\n=== BTreeSet (有序集合) ===");
    
    let mut btree_set = BTreeSet::new();
    btree_set.insert(3);
    btree_set.insert(1);
    btree_set.insert(4);
    btree_set.insert(1);  // 重复无效
    
    println!("BTreeSet (自动排序): {:?}", btree_set);  // [1, 3, 4]
    
    // 范围查询
    for &num in btree_set.range(2..=4) {
        println!("  范围内: {}", num);
    }
    
    // ========== 4. HashMap<K,V> - 类似 Java HashMap ==========
    println!("\n=== HashMap (键值对) ===");
    
    let mut scores = HashMap::new();
    
    // 插入
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);
    scores.insert(String::from("Blue"), 25);  // 覆盖旧值
    
    // 访问
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Blue 得分: {:?}", score);
    
    // 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // entry API (类似 Java computeIfAbsent)
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(100);  // 不会覆盖已有值
    
    // 修改值
    let blue_score = scores.entry(String::from("Blue")).or_insert(0);
    *blue_score += 10;  // 解引用并修改
    
    println!("修改后: {:?}", scores);
    
    // 删除
    scores.remove("Red");
    
    // ========== 5. BTreeMap<K,V> - 类似 Java TreeMap (有序) ==========
    println!("\n=== BTreeMap (有序键值对) ===");
    
    let mut btree_map = BTreeMap::new();
    btree_map.insert(3, "three");
    btree_map.insert(1, "one");
    btree_map.insert(4, "four");
    btree_map.insert(2, "two");
    
    println!("BTreeMap (按键排序): {:?}", btree_map);
    // 输出: {1: "one", 2: "two", 3: "three", 4: "four"}
    
    // 范围查询
    for (key, value) in btree_map.range(2..=3) {
        println!("  {}: {}", key, value);
    }
    
    // ========== 6. VecDeque<T> - 双端队列 ==========
    println!("\n=== VecDeque (双端队列) ===");
    
    let mut deque = VecDeque::new();
    deque.push_back(1);   // 尾部添加
    deque.push_front(0);  // 头部添加
    deque.push_back(2);
    
    println!("队列: {:?}", deque);  // [0, 1, 2]
    
    println!("弹出头部: {:?}", deque.pop_front());  // Some(0)
    println!("弹出尾部: {:?}", deque.pop_back());   // Some(2)
    
    // ========== 7. 常用集合操作综合示例 ==========
    println!("\n=== 综合示例：单词统计 ===");
    
    let text = "hello world hello rust rust world";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("单词统计: {:?}", word_count);
    
    // 找出出现最多的单词
    let max_word = word_count.iter()
        .max_by_key(|&(_, &count)| count)
        .map(|(word, _)| word);
    
    println!("出现最多的单词: {:?}", max_word);
}
```

## 关键特性总结

| 操作  | Vec                     | HashSet            | HashMap             | 说明         |
|-----|-------------------------|--------------------|---------------------|------------|
| 创建  | `vec![]` 或 `Vec::new()` | `HashSet::new()`   | `HashMap::new()`    | -          |
| 添加  | `push()`                | `insert()`         | `insert()`          | Vec 只能尾部添加 |
| 访问  | `[index]` 或 `get()`     | `contains()`       | `get()`             | -          |
| 删除  | `remove()` / `pop()`    | `remove()`         | `remove()`          | -          |
| 遍历  | `for &item in &vec`     | `for item in &set` | `for (k,v) in &map` | -          |
| 长度  | `len()`                 | `len()`            | `len()`             | -          |
| 是否空 | `is_empty()`            | `is_empty()`       | `is_empty()`        | -          |

## 性能对比

| 集合         | 插入       | 删除       | 访问       | 适用场景       |
|------------|----------|----------|----------|------------|
| `Vec`      | O(1)*    | O(n)     | O(1)     | 顺序存储，频繁索引  |
| `HashSet`  | O(1)平均   | O(1)平均   | O(1)平均   | 快速去重、成员检测  |
| `BTreeSet` | O(log n) | O(log n) | O(log n) | 需要有序集合     |
| `HashMap`  | O(1)平均   | O(1)平均   | O(1)平均   | KV 存储，快速查找 |
| `BTreeMap` | O(log n) | O(log n) | O(log n) | 需要有序 KV    |

> 注：Vec 的末尾插入是 O(1)+ 偶尔扩容，中间插入是 O(n)

