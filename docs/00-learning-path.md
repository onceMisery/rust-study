# Rust 系统学习路线

本项目按“先语法、再模型、后工程”的顺序组织。Rust 的难点不是关键字数量，而是它把内存安全、性能和并发安全放进了类型系统。学习时不要急着背规则，应该先写小程序，再观察编译器如何提示你修正所有权、借用和生命周期问题。

## 阶段一：基础语法

目标是能读懂并写出普通 Rust 函数。重点包括变量声明、可变性、遮蔽、标量类型、复合类型、函数、表达式、控制流和切片。

配套代码：

```powershell
cargo run -p basic_syntax --example basic_tour
```

## 阶段二：进阶特性

目标是理解 Rust 和 Java / Go 最大的不同：Rust 没有垃圾回收器，却能在编译期保证内存安全。重点包括所有权、移动、复制、借用、可变引用、生命周期、trait、默认方法、泛型和 trait bound。

配套代码：

```powershell
cargo run -p advanced_features --example advanced_tour
```

## 阶段三：工程实践

目标是能写一个结构清晰、可维护的 Rust 小项目。重点包括模块系统、Cargo、错误处理、panic 使用边界、线程、消息传递、共享状态、单元测试、集成测试和文档测试。

配套代码：

```powershell
cargo run -p engineering_practice --example engineering_tour
```

## 阶段四：和 Java / Go 对比

目标是建立语言设计层面的判断力。Rust 适合对性能、资源控制、可靠性要求很高的系统；Java 适合成熟企业生态和大型业务系统；Go 适合网络服务、云原生工具和简单并发服务。

## 建议节奏

1. 每读完一节，运行对应 example。
2. 修改示例代码，故意触发编译错误，阅读错误信息。
3. 不要绕过编译器。Rust 编译器的报错本身就是学习材料。
4. 每个阶段结束后，执行 `cargo test`，确认示例仍可运行。
