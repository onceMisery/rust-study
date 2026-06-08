# 专题二：模块系统与代码组织

Rust 的模块系统让你能把代码组织成清晰的层次结构，控制可见性，管理依赖关系。

配套代码：

```powershell
cargo run -p engineering_practice --example module_tour
```

---

## 1. 模块基础

### 模块定义

```rust
pub mod math {
    pub fn add(a: i32, b: i32) -> i32 { a + b }

    fn internal_helper() -> i32 { 42 }  // 私有，仅模块内可见
}

pub mod text {
    pub fn char_count(s: &str) -> usize { s.chars().count() }

    pub mod format {  // 子模块
        pub fn to_upper(s: &str) -> String { s.to_uppercase() }
    }
}
```

### 调用路径

```rust
math::add(3, 7);              // 完整路径
text::char_count("Rust");     // 模块路径
text::format::to_upper("hi"); // 子模块路径
```

---

## 2. 可见性控制

### pub 关键字

| 修饰符 | 可见范围 | 使用场景 |
|--------|----------|----------|
| （无） | 当前模块 | 内部辅助函数 |
| `pub` | 所有外部代码 | 公开 API |
| `pub(crate)` | 当前 crate 内 | crate 内部共享 |
| `pub(super)` | 父模块 | 有限暴露 |
| `pub(in path)` | 指定路径 | 精确控制 |

```rust
pub mod api {
    pub fn handle_request() { ... }        // 对外公开
    pub(crate) fn internal_cache() { ... } // 仅 crate 内
    fn validate_input() { ... }            // 仅模块内
}
```

---

## 3. use 关键字简化路径

```rust
// 完整路径调用
engineering_practice::module_demo::math::add(1, 2);

// use 引入后直接调用
use engineering_practice::module_demo::math::add;
add(1, 2);

// 引入模块（调用时带模块名）
use engineering_practice::module_demo::math;
math::add(1, 2);
```

### use 的最佳实践

```rust
// ✅ 引入类型（不是函数）
use std::collections::HashMap;

// ✅ 引入 trait（才能调用 trait 方法）
use std::fmt::Display;

// ✅ 用 as 解决命名冲突
use std::io::Result as IoResult;

// ❌ 避免：引入整个模块的所有内容
use std::collections::*;  // 不推荐
```

---

## 4. 文件组织

### 单文件模块

```rust
// src/lib.rs
pub mod math;      // 对应 src/math.rs
pub mod text;      // 对应 src/text.rs
```

### 目录模块

```
src/
├── lib.rs          // 入口
├── math.rs         // math 模块
├── text/
│   ├── mod.rs      // text 模块入口
│   └── format.rs   // text::format 子模块
```

---

## 5. Cargo 工作空间

### workspace 配置

```toml
# 根目录 Cargo.toml
[workspace]
members = [
    "crates/basic_syntax",
    "crates/advanced_features",
    "crates/engineering_practice",
    "blog",
]
resolver = "2"
```

### workspace 的好处

| 特性 | 说明 |
|------|------|
| 共享 `target/` | 所有 crate 编译产物放在同一目录 |
| 共享 `Cargo.lock` | 依赖版本全局一致 |
| 统一命令 | `cargo test` 运行所有 crate 的测试 |
| 跨 crate 依赖 | crate 之间可以直接引用 |

### 常用命令

```powershell
cargo build -p basic_syntax              # 构建指定 crate
cargo test                               # 测试所有 crate
cargo run -p advanced_features --example advanced_tour
cargo clippy --workspace                 # 检查所有 crate
```

---

## 6. 实际项目结构

### 库项目结构

```
my_lib/
├── Cargo.toml
├── src/
│   ├── lib.rs          # 库入口
│   ├── models/
│   │   ├── mod.rs      # 模型模块入口
│   │   ├── user.rs     # User 模型
│   │   └── post.rs     # Post 模型
│   ├── services/
│   │   ├── mod.rs
│   │   └── auth.rs
│   └── errors.rs       # 错误类型定义
├── tests/
│   └── integration.rs  # 集成测试
└── examples/
    └── demo.rs         # 示例代码
```

### 应用程序结构

```
my_app/
├── Cargo.toml
├── src/
│   ├── main.rs         # 程序入口
│   ├── config.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   └── api.rs
│   └── db.rs
└── .env                # 环境配置
```

---

## 与 Java/Go 对比

| 特性 | Rust | Java | Go |
|------|------|------|-----|
| 模块定义 | `mod` + 文件/目录 | `package` + 目录 | `package` + 目录 |
| 可见性 | `pub` / `pub(crate)` | `public` / `private` / `protected` | 首字母大写=公开 |
| 包管理 | Cargo | Maven / Gradle | go modules |
| 工作空间 | `[workspace]` | 多模块项目 | go work |

---

## 最佳实践

1. **模块按功能划分**，而非按类型（不要把所有 struct 放一个模块）
2. **用 `pub(crate)` 限制内部 API**，减少公开表面积
3. **`use` 引入类型和 trait**，函数用完整路径调用
4. **子模块用目录组织**，超过 3 个文件时拆目录
5. **workspace 共享依赖版本**，避免版本冲突

## 配套代码

```powershell
cargo run -p engineering_practice --example module_tour
```
