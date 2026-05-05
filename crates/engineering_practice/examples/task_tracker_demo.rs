use engineering_practice::task_tracker::{parse_command, TaskBoard};

fn main() {
    let mut board = TaskBoard::new();

    let script = [
        "add 阅读 Rust 基础语法",
        "add 完成所有权练习",
        "start 2",
        "done 1",
        "list",
    ];

    for line in script {
        let command = parse_command(line).expect("示例命令应该合法");
        let output = board.execute(command).expect("示例命令应该执行成功");
        println!("> {line}\n{output}\n");
    }
}
