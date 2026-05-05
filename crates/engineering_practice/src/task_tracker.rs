use std::collections::BTreeMap;
use std::fmt;

/// 任务状态。
///
/// 使用 enum 的好处是：状态值被限定在三个合法分支内，
/// 调用方不能传入 `"doingg"` 这类拼写错误的非法状态。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Todo,
    Doing,
    Done,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            TaskStatus::Todo => "待办",
            TaskStatus::Doing => "进行中",
            TaskStatus::Done => "已完成",
        };
        formatter.write_str(label)
    }
}

/// 一条任务记录。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    pub id: u64,
    pub title: String,
    pub status: TaskStatus,
}

/// 任务看板错误。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskError {
    EmptyTitle,
    InvalidCommand,
    InvalidTaskId,
    TaskNotFound(u64),
}

/// 命令解析结果。
///
/// 文档中的 CLI 示例会把用户输入解析成这个 enum，然后交给 `TaskBoard` 执行。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    Add(String),
    Start(u64),
    Done(u64),
    Remove(u64),
    List,
}

/// 一个内存版任务看板。
///
/// 为了让新手先理解工程组织，这里暂时不引入数据库和文件持久化。
/// `BTreeMap` 会按任务 id 排序，输出稳定，便于测试和阅读。
#[derive(Debug, Default)]
pub struct TaskBoard {
    next_id: u64,
    tasks: BTreeMap<u64, Task>,
}

impl TaskBoard {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            tasks: BTreeMap::new(),
        }
    }

    /// 新增任务，返回新任务 id。
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

    pub fn start_task(&mut self, id: u64) -> Result<(), TaskError> {
        self.update_status(id, TaskStatus::Doing)
    }

    pub fn finish_task(&mut self, id: u64) -> Result<(), TaskError> {
        self.update_status(id, TaskStatus::Done)
    }

    pub fn remove_task(&mut self, id: u64) -> Result<Task, TaskError> {
        self.tasks.remove(&id).ok_or(TaskError::TaskNotFound(id))
    }

    pub fn list(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    pub fn list_by_status(&self, status: TaskStatus) -> Vec<&Task> {
        self.tasks
            .values()
            .filter(|task| task.status == status)
            .collect()
    }

    /// 生成适合 CLI 打印的文本。
    pub fn render(&self) -> String {
        if self.tasks.is_empty() {
            return "暂无任务".to_string();
        }

        self.tasks
            .values()
            .map(|task| format!("#{} [{}] {}", task.id, task.status, task.title))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// 执行解析后的命令。
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

    fn update_status(&mut self, id: u64, status: TaskStatus) -> Result<(), TaskError> {
        let task = self.tasks.get_mut(&id).ok_or(TaskError::TaskNotFound(id))?;
        task.status = status;
        Ok(())
    }
}

/// 把用户输入解析成命令。
///
/// 支持的格式：
/// - `add 学习 Rust 所有权`
/// - `start 1`
/// - `done 1`
/// - `remove 1`
/// - `list`
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

fn parse_task_id(raw: &str) -> Result<u64, TaskError> {
    raw.parse::<u64>().map_err(|_| TaskError::InvalidTaskId)
}
