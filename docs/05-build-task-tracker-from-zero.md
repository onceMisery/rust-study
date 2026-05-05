# 从零构建任务看板案例

这一节带你从 0 到 1 构建一个内存版任务看板。它不是玩具片段，而是一个有模型、有错误、有命令解析、有业务操作、有运行入口的小项目。

对应代码：

- `crates/engineering_practice/src/task_tracker.rs`
- `crates/engineering_practice/examples/task_tracker_demo.rs`

运行：

```powershell
cargo run -p engineering_practice --example task_tracker_demo
```

## 1. 需求定义

我们先做一个命令式任务看板，支持：

```text
add 阅读 Rust 基础语法
start 1
done 1
remove 1
list
```

暂不做文件持久化、交互式终端输入、多用户和数据库。原因是新手阶段应该先把 Rust 工程骨架、类型建模和错误处理跑通，再引入 IO
和外部依赖。

## 2. 建模任务状态

任务状态适合用 enum：

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}
```

不要用字符串表示状态。字符串容易拼错，编译器无法帮你检查。enum 的好处是状态集合固定，`match` 时还能提醒你是否漏掉分支。

## 3. 建模任务

任务使用 struct：

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub status: TaskStatus,
}
```

这里 `title` 用 `String`，因为任务需要拥有标题内容；如果只是临时读取标题，函数参数可以用 `&str`。

## 4. 建模错误

错误使用 enum：

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskError {
    EmptyTitle,
    InvalidCommand,
    InvalidTaskId,
    TaskNotFound(u64),
}
```

这种写法比返回字符串更适合工程代码：调用方可以精确匹配错误类型，测试可以直接比较错误分支，后续也可以为错误实现 `Display`
或接入错误库。

## 5. 设计命令类型

用户输入是字符串，但业务层不应该直接处理字符串。先解析成强类型命令：

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Add(String),
    Start(u64),
    Done(u64),
    Remove(u64),
    List,
}
```

这样业务层只关心“执行什么动作”，不关心原始输入怎么切分。

## 6. 实现任务看板

核心结构：

```rust
#[derive(Debug, Default)]
pub struct TaskBoard {
    next_id: u64,
    tasks: BTreeMap<u64, Task>,
}
```

为什么用 `BTreeMap`？

- key 是任务 id。
- 输出按 id 排序，稳定可读。
- 对教程更友好。

新增任务：

```rust
pub fn add_task(&mut self, title: impl Into<String>) -> Result<u64, TaskError> {
    let title = title.into().trim().to_owned();
    if title.is_empty() {
        return Err(TaskError::EmptyTitle);
    }

    let id = self.next_id;
    self.next_id += 1;
    self.tasks.insert(
        id,
        Task {
            id,
            title,
            status: TaskStatus::Todo,
        },
    );
    Ok(id)
}
```

这里有几个 Rust 工程习惯：

- `impl Into<String>` 让调用方既能传 `String`，也能传 `&str`。
- `trim()` 清理输入。
- 空标题返回 `Result::Err`，不使用 `panic!`。
- 插入成功后返回任务 id。

## 7. 更新任务状态

开始任务和完成任务都可以复用内部函数：

```rust
pub fn start_task(&mut self, id: u64) -> Result<(), TaskError> {
    self.update_status(id, TaskStatus::Doing)
}

pub fn finish_task(&mut self, id: u64) -> Result<(), TaskError> {
    self.update_status(id, TaskStatus::Done)
}
```

内部实现：

```rust
fn update_status(&mut self, id: u64, status: TaskStatus) -> Result<(), TaskError> {
    let task = self.tasks.get_mut(&id).ok_or(TaskError::TaskNotFound(id))?;
    task.status = status;
    Ok(())
}
```

重点是 `get_mut`：它返回可变引用，让我们能原地修改任务状态。

## 8. 解析命令

命令解析函数：

```rust
pub fn parse_command(input: &str) -> Result<Command, TaskError> {
    let input = input.trim();
    if input == "list" {
        return Ok(Command::List);
    }

    let (name, rest) = input.split_once(' ').ok_or(TaskError::InvalidCommand)?;
    let rest = rest.trim();

    match name {
        "add" if !rest.is_empty() => Ok(Command::Add(rest.to_owned())),
        "start" => Ok(Command::Start(parse_task_id(rest)?)),
        "done" => Ok(Command::Done(parse_task_id(rest)?)),
        "remove" => Ok(Command::Remove(parse_task_id(rest)?)),
        _ => Err(TaskError::InvalidCommand),
    }
}
```

这里用到了 `trim()`、`split_once`、`ok_or`、`?` 和 `match`。这些都是 Rust 工程代码中非常高频的组合。

## 9. 执行业务命令

`TaskBoard::execute` 把命令映射到业务操作：

```rust
pub fn execute(&mut self, command: Command) -> Result<String, TaskError> {
    match command {
        Command::Add(title) => {
            let id = self.add_task(title)?;
            Ok(format!("已新增任务 #{id}"))
        }
        Command::Start(id) => {
            self.start_task(id)?;
            Ok(format!("任务 #{id} 已开始"))
        }
        Command::Done(id) => {
            self.finish_task(id)?;
            Ok(format!("任务 #{id} 已完成"))
        }
        Command::Remove(id) => {
            self.remove_task(id)?;
            Ok(format!("任务 #{id} 已删除"))
        }
        Command::List => Ok(self.render()),
    }
}
```

这个函数是业务入口，未来如果做 CLI、HTTP API 或 GUI，都可以复用它。

## 10. 运行示例

example 中模拟了一组用户命令：

```rust
let script = [
    "add 阅读 Rust 基础语法",
    "add 完成所有权练习",
    "start 2",
    "done 1",
    "list",
];
```

运行后你会看到类似输出：

```text
> add 阅读 Rust 基础语法
已新增任务 #1

> list
#1 [已完成] 阅读 Rust 基础语法
#2 [进行中] 完成所有权练习
```

## 11. 你可以继续扩展什么

完成当前案例后，可以按这个顺序扩展：

1. 增加 `priority` 字段，支持高、中、低优先级。
2. 增加 `created_at` 字段，学习时间处理库。
3. 把任务保存到文件，学习 `std::fs`。
4. 引入 `serde`，学习 JSON 序列化。
5. 做真正的 CLI 参数解析，学习 `clap`。
6. 把业务层改成 HTTP API，学习 Web 框架。

## 与 Java、Go 的项目构建对比

| 步骤   | Rust                             | Java                      | Go                    |
|------|----------------------------------|---------------------------|-----------------------|
| 建模状态 | enum 强约束，`match` 穷尽检查            | enum 或 class hierarchy    | iota 常量或自定义类型         |
| 错误返回 | `Result<T, E>`                   | exception 或返回对象           | `(T, error)`          |
| 集合选择 | `BTreeMap<u64, Task>` 所有权明确      | `Map<Long, Task>`，GC 管理引用 | `map[uint64]Task` 或指针 |
| 构建运行 | `cargo run -p ... --example ...` | Maven/Gradle run 或 IDE    | `go run ./...`        |
| 扩展方向 | 类型约束强，重构安全                       | 框架生态强，结构偏重                | 简洁快速，约定较少             |

如果用 Java 写这个项目，代码可能会更偏类和对象；如果用 Go 写，代码会更轻量直接；Rust
的特点是你需要更早设计所有权、错误类型和数据边界，但一旦编译通过，很多资源生命周期问题已经被排除。
