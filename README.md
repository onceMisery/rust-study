# rust-study

这是一个面向 Rust 系统学习的中文项目，采用“文档 + 可运行示例代码”的形式组织。

## 学习顺序

1. 阅读 [docs/00-learning-path.md](docs/00-learning-path.md)，了解整体路线。
2. 学习 [docs/01-basic-syntax.md](docs/01-basic-syntax.md)，运行基础语法示例。
3. 学习 [docs/02-advanced-features.md](docs/02-advanced-features.md)，理解所有权、借用、生命周期、trait 与泛型。
4. 学习 [docs/03-engineering-practice.md](docs/03-engineering-practice.md)，掌握模块、Cargo、错误处理、并发与测试。
5. 阅读 [docs/04-rust-java-go-comparison.md](docs/04-rust-java-go-comparison.md)，从 Java / Go 对比中理解 Rust 的设计取舍。

## 运行命令

```powershell
cargo test
cargo run -p basic_syntax --example basic_tour
cargo run -p advanced_features --example advanced_tour
cargo run -p engineering_practice --example engineering_tour
```

## 项目结构

```text
crates/
  basic_syntax/          # 基础语法示例
  advanced_features/     # 所有权、生命周期、trait、泛型
  engineering_practice/  # 模块、错误处理、并发、测试
docs/                    # 中文学习文档
```
