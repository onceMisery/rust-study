# 阶段四：Rust、Java 与 Go 对比

## 总览

| 维度 | Rust | Java | Go |
| --- | --- | --- | --- |
| 内存管理 | 所有权 + 借用检查，无 GC | JVM GC | 运行时 GC |
| 性能模型 | 接近 C/C++，零成本抽象 | JIT 优化，运行时成本较高 | 编译快，运行时较轻但有 GC |
| 并发 | 线程、channel、async，类型系统防数据竞争 | 线程、线程池、虚拟线程、并发库 | goroutine + channel |
| 错误处理 | `Result` / `Option` 显式建模 | 异常 + Optional | 多返回值 `error` |
| 泛型 | 单态化，trait bound 强约束 | 类型擦除为主 | 支持类型参数，约束较简洁 |
| 生态 | 系统、CLI、嵌入式、区块链、性能服务增长快 | 企业服务生态非常成熟 | 云原生、网络服务、DevOps 工具体系强 |

## 内存安全

Java 和 Go 通过 GC 避免大多数手动内存错误。优点是开发体验简单，缺点是运行时需要追踪对象并在合适时机回收，可能带来暂停、额外内存和运行时开销。

Rust 通过所有权系统在编译期检查资源生命周期：

```rust
let text = String::from("hello");
let len = text.len();
```

值离开作用域时自动释放。没有 GC，也不需要手动释放。代价是初学者需要理解移动、借用和生命周期。

## 错误处理

Java 常见写法：

```java
try {
    int port = Integer.parseInt(raw);
} catch (NumberFormatException e) {
    // handle
}
```

Go 常见写法：

```go
port, err := strconv.Atoi(raw)
if err != nil {
    return err
}
```

Rust 常见写法：

```rust
let port: u16 = raw.parse().map_err(|_| AppError::InvalidPort)?;
```

Rust 的优势是错误类型进入函数签名，调用方不能无视 `Result`。这让错误路径更显式，也更适合写可靠系统。

## 接口、trait 与泛型

Java interface 通常依靠对象和动态分发：

```java
interface Summary {
    String summary();
}
```

Go interface 是隐式实现：

```go
type Summary interface {
    Summary() string
}
```

Rust trait 需要显式实现：

```rust
trait Summary {
    fn summary(&self) -> String;
}
```

Rust 的 trait 既可以做接口，也可以做泛型约束。它更严格，但也让 API 契约更清楚。

## 并发模型

Go 的并发体验最直接，goroutine 很轻量，channel 是语言级常用模式。Java 传统线程模型成熟，近年的虚拟线程改善了高并发服务的编程体验。

Rust 的并发学习曲线更高，但优势是许多并发错误在编译期暴露。例如共享可变状态通常需要 `Arc<Mutex<T>>`，这会迫使开发者显式面对锁和所有权。

## 适用场景建议

Rust 更适合：

- 系统软件、数据库、网络代理、运行时组件。
- 对延迟、内存占用、稳定性要求高的服务。
- CLI 工具、WebAssembly、嵌入式开发。

Java 更适合：

- 大型企业业务系统。
- 需要成熟框架、团队招聘和长期维护稳定性的项目。
- 强依赖 JVM 生态的系统。

Go 更适合：

- 云原生基础设施、微服务、网关、运维平台。
- 团队希望语言简单、编译部署快。
- 并发 IO 多、业务模型相对直接的服务。

## 理解 Rust 设计理念

Rust 的核心取舍是：把一部分运行时问题提前到编译期处理。它让程序员写代码时多付出一些类型和所有权成本，换取运行时更少的不确定性。理解这一点后，借用检查器就不是阻碍，而是把内存安全和并发安全前移的工具。
