# 专题七：集合与迭代器

Rust 标准库提供了丰富的集合类型和强大的迭代器抽象。掌握它们是写出高效 Rust 代码的基础。

配套代码：

```powershell
cargo run -p basic_syntax --example collections_tour
```

---

## 1. Vec：动态数组

### 基本操作

```rust
let mut numbers = vec![3, 1, 4];
numbers.push(1);       // 尾部追加
numbers.push(5);
numbers.insert(0, 9);  // 指定位置插入（O(n)）
numbers.sort();        // 排序
numbers.dedup();       // 去重（需先排序）

let popped = numbers.pop();           // Option<i32>，弹出最后一个
let contains = numbers.contains(&4);  // bool
let found = numbers.iter().position(|&x| x > 3);  // Option<usize>
```

### 访问方式对比

| 方式 | 语法 | 越界行为 | 返回类型 |
|------|------|----------|----------|
| 索引 | `v[i]` | panic | `T` |
| get | `v.get(i)` | 返回 None | `Option<&T>` |
| 迭代 | `for item in &v` | 不越界 | `&T` |

### 性能提示

```rust
// ✅ 预分配容量，避免多次扩容
let mut v = Vec::with_capacity(100);

// ❌ 反复扩容（每次翻倍，有复制成本）
let mut v = Vec::new();
for i in 0..100 { v.push(i); }  // 多次 realloc
```

---

## 2. HashMap：哈希映射

### 基本操作

```rust
use std::collections::HashMap;

let mut scores: HashMap<String, i32> = HashMap::new();
scores.insert("Alice".into(), 95);
scores.insert("Bob".into(), 87);

// 查找
let score = scores.get("Alice");   // Option<&i32>
let score = scores["Alice"];       // i32（不存在则 panic）

// entry API：优雅的"存在则修改，不存在则插入"
scores.entry("Charlie".into()).or_insert(80);     // 不存在时插入
*scores.entry("Alice".into()).or_insert(0) += 5;  // 存在时修改
```

### 遍历

```rust
for (name, score) in &scores {
    println!("{}: {}", name, score);
}
```

### HashMap vs BTreeMap

| 特性 | HashMap | BTreeMap |
|------|---------|----------|
| 顺序 | 无序 | 按键排序 |
| 查找 | O(1) 平均 | O(log n) |
| 范围查询 | ❌ 不支持 | ✅ 支持 |
| 内存 | 更多（哈希表） | 更少（B树） |
| 使用场景 | 快速查找 | 需要有序或范围查询 |

---

## 3. HashSet：哈希集合

### 集合运算

```rust
use std::collections::HashSet;

let frontend: HashSet<_> = ["HTML", "CSS", "JS"].iter().cloned().collect();
let backend: HashSet<_> = ["Rust", "Go", "JS"].iter().cloned().collect();

let intersection: Vec<_> = frontend.intersection(&backend).collect(); // ["JS"]
let union: Vec<_> = frontend.union(&backend).collect();     // 所有元素
let diff: Vec<_> = frontend.difference(&backend).collect(); // ["HTML", "CSS"]
```

---

## 4. 迭代器深入

### 适配器（惰性）

| 适配器 | 作用 | 示例 |
|--------|------|------|
| `map(f)` | 逐项变换 | `.map(\|x\| x * 2)` |
| `filter(p)` | 按条件过滤 | `.filter(\|&x\| x > 0)` |
| `take(n)` | 取前 n 项 | `.take(5)` |
| `skip(n)` | 跳过前 n 项 | `.skip(3)` |
| `enumerate()` | 带索引 | `.enumerate()` |
| `zip(other)` | 合并两个迭代器 | `.zip(b.iter())` |
| `chain(other)` | 拼接 | `.chain(b.iter())` |
| `chunks(n)` | 分组 | `.chunks(3)` |
| `windows(n)` | 滑动窗口 | `.windows(3)` |
| `scan(init, f)` | 带状态的 map | 累积计算 |
| `flatten()` | 展平嵌套 | 嵌套迭代器 |

### 消费器（触发计算）

| 消费器 | 作用 | 示例 |
|--------|------|------|
| `collect()` | 收集到集合 | `.collect::<Vec<_>>()` |
| `sum()` | 求和 | `.sum::<i32>()` |
| `count()` | 计数 | `.count()` |
| `fold(init, f)` | 累积 | `.fold(0, \|a, x\| a + x)` |
| `find(p)` | 查找第一个 | `.find(\|&x\| x > 10)` |
| `any(p)` | 是否存在 | `.any(\|&x\| x == 0)` |
| `all(p)` | 是否全部 | `.all(\|&x\| x > 0)` |
| `position(p)` | 第一个满足条件的位置 | `.position(\|&x\| x > 10)` |

### 链式调用示例

```rust
// 1-20 的偶数平方，取前 5 个
let result: Vec<i32> = (1..=20)
    .filter(|n| n % 2 == 0)
    .map(|n| n * n)
    .take(5)
    .collect();
// [4, 16, 36, 64, 100]
```

### 高级迭代器方法

```rust
let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// chunks: 分组
let chunks: Vec<_> = data.chunks(3).collect();
// [[1,2,3], [4,5,6], [7,8,9], [10]]

// windows: 滑动窗口
let windows: Vec<_> = data.windows(3).collect();
// [[1,2,3], [2,3,4], [3,4,5], ...]

// scan: 带状态的 map（运行总和）
let running_sum: Vec<_> = data.iter()
    .scan(0, |state, &x| { *state += x; Some(*state) })
    .collect();
// [1, 3, 6, 10, 15, 21, 28, 36, 45, 55]
```

### 性能：迭代器 vs 循环

| 方式 | 性能 | 可读性 | 推荐场景 |
|------|------|--------|----------|
| `for` 循环 | 快 | 直观 | 简单遍历 |
| 迭代器链 | 快（release 模式编译器会内联优化） | 函数式 | 复杂变换/过滤/聚合 |
| `while` 循环 | 快 | 直观 | 条件循环 |

> **最佳实践**：release 模式下迭代器链通常和手写循环一样快，甚至更快（编译器优化更好）。

---

## 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 动态数组 | `Vec<T>` | `ArrayList<T>` | slice |
| 哈希映射 | `HashMap<K,V>` | `HashMap<K,V>` | `map[K]V` |
| 有序映射 | `BTreeMap<K,V>` | `TreeMap<K,V>` | ❌ 无内置 |
| 迭代器 | trait + 适配器链 | Stream API (Java 8+) | for range |
| 惰性求值 | ✅ 默认惰性 | ✅ Stream 惰性 | ❌ |

---

## 最佳实践

1. **预分配容量**：`Vec::with_capacity(n)` 避免反复扩容
2. **用 `get()` 替代索引**：避免越界 panic
3. **用 `entry()` API**：优雅处理 HashMap 的"存在/不存在"逻辑
4. **迭代器链处理复杂变换**：比手写循环更清晰，性能相同
5. **选择合适的集合**：快速查找用 HashMap，有序用 BTreeMap

## 配套代码

```powershell
cargo run -p basic_syntax --example collections_tour
```
