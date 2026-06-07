# 从零开始：Rust 开发环境搭建与第一个程序

本文面向零基础开发者，手把手带你完成从安装 Rust 到运行第一个程序的全过程。

## 第一部分：安装 Rust 工具链

### 什么是 Rust 工具链

Rust 工具链包含三个核心组件：

| 组件 | 说明 | 类比 |
|------|------|------|
| `rustc` | Rust 编译器，把源码编译成可执行文件 | Java 的 `javac`、Go 的 `go build` |
| `cargo` | 包管理器 + 构建工具，管理依赖、编译、测试 | Node.js 的 `npm`、Java 的 `Maven` |
| `rustup` | 工具链管理器，用来安装和更新 Rust | 类似 SDK 版本管理器 |

### Windows 安装步骤

#### 1. 下载 rustup 安装程序

打开浏览器，访问 Rust 官方安装页面：

```
https://www.rust-lang.org/tools/install
```

页面上会显示一个下载按钮。Windows 系统会自动下载 `rustup-init.exe` 安装程序。

> 如果官网访问较慢，可以先配置国内镜像源（详见下方 [配置国内镜像源](#配置国内镜像源推荐-rsproxy) 章节），然后再运行安装程序。

#### 2. 运行安装程序

双击运行 `rustup-init.exe`，会看到一个文本界面，类似如下内容：

```
Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  C:\Users\YourName\.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory is located at:

  C:\Users\YourName\.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  C:\Users\YourName\.cargo\bin

This path will then be added to your PATH environment variable by
modifying the HKEY_CURRENT_USER/Environment/PATH registry key.

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with standard installation (default - just press enter)
2) Customize installation
3) Cancel installation
```

**直接按回车键**选择默认安装即可。安装过程会下载约 300MB 的工具链文件。

> 如果你已经安装了 Visual Studio Build Tools（含 MSVC），安装程序会自动检测并使用 `msvc` 工具链。如果没有，Rust 会自动下载所需的构建工具。

#### 3. 验证安装

安装完成后，**关闭并重新打开** PowerShell（重要！需要刷新环境变量），然后运行：

```powershell
rustc --version
cargo --version
rustup --version
```

如果看到类似输出，说明安装成功：

```
rustc 1.80.0 (05146edc3 2024-07-21)
cargo 1.80.0 (376290515 2024-07-16)
rustup 1.27.1 (54dd3d00f 2024-04-24)
```

#### 4. 更新 Rust

Rust 每 6 周发布一个新版本。定期更新工具链：

```powershell
rustup update
```

### macOS / Linux 安装（参考）

在终端中运行：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

按提示操作即可。安装完成后运行 `source $HOME/.cargo/env` 或重新打开终端。

### 配置国内镜像源（推荐 RsProxy）

国内直接访问 crates.io 和 Rust 官方服务器速度较慢，推荐使用字节跳动维护的 [RsProxy](https://rsproxy.cn) 镜像，包含两部分配置：**Rustup 工具链镜像** 和 **crates.io 依赖包镜像**。

#### 步骤一：配置 Rustup 镜像（安装前设置）

在运行 `rustup-init.exe` **之前**，先在 PowerShell 中设置环境变量：

```powershell
$env:RUSTUP_DIST_SERVER="https://rsproxy.cn"
$env:RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
```

> 如果想永久生效，可以将这两行添加到 PowerShell 配置文件中。运行以下命令打开配置文件：
> ```powershell
> notepad $PROFILE
> ```
> 将上面两行 `export` 换成 `$env:... = ...` 的形式粘贴进去保存即可。

设置完成后再运行 `rustup-init.exe`，安装过程会从国内镜像下载，速度显著提升。

#### 步骤二：配置 crates.io 镜像（安装后设置）

安装完成后，创建或编辑 Cargo 全局配置文件：

```powershell
# 打开配置文件（如果不存在会自动创建）
notepad "$env:USERPROFILE\.cargo\config.toml"
```

将以下内容粘贴进去并保存：

```toml
[source.crates-io]
replace-with = 'rsproxy-sparse'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[registries.rsproxy-sparse]
index = "sparse+https://rsproxy.cn/index/"

[net]
git-fetch-with-cli = true
```

配置说明：

| 配置项 | 说明 |
|--------|------|
| `replace-with = 'rsproxy-sparse'` | 将 crates.io 默认源替换为 Sparse 协议镜像，速度最快 |
| `sparse+https://...` | Sparse 协议只下载需要的索引片段，比 Git 协议（拉取整个索引仓库）快很多 |
| `git-fetch-with-cli = true` | 使用系统 Git 拉取依赖，兼容性更好 |

> **Sparse vs Git 协议**：Git 协议需要下载上百兆的完整索引仓库，即使走国内镜像也很慢。Sparse 协议（Rust 1.68+ 稳定）只按需拉取索引片段，是目前社区主流方案。

#### 全局配置 vs 项目级配置

上面介绍的是**全局配置**（`~/.cargo/config.toml`），对所有项目生效。如果你只想给某个项目单独配置镜像源，可以在**项目根目录**下创建 `.cargo/config.toml`：

```
my_project/
├── .cargo/
│   └── config.toml    # 仅对本项目生效的镜像配置
├── Cargo.toml
└── src/
    └── main.rs
```

**操作步骤**：

```powershell
# 1. 进入项目根目录
cd C:\user-data\code\my-infra\rust-study

# 2. 创建 .cargo 目录（如果不存在）
mkdir .cargo

# 3. 创建配置文件
notepad .\.cargo\config.toml
```

将以下内容粘贴到 `.cargo/config.toml` 中并保存：

```toml
[source.crates-io]
replace-with = 'rsproxy-sparse'

[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[net]
git-fetch-with-cli = true
```

项目级配置与全局配置内容基本相同，但优先级更高——如果两者同时存在，Cargo 会优先使用项目级配置。

**选择建议**：

| 场景 | 推荐方式 | 说明 |
|------|----------|------|
| 个人开发、所有项目都用镜像 | 全局配置 | 一劳永逸，只配一次 |
| 团队项目、需要统一依赖源 | 项目级配置 | 提交到 Git 仓库，团队成员自动生效 |
| 某些依赖走官方源、某些走镜像 | 项目级配置 | 可以精细控制每个 source 的替换规则 |

> **提示**：项目级 `.cargo/config.toml` 建议提交到 Git 仓库，这样团队其他人拉取代码后不需要手动配置，直接就能用镜像下载依赖。

#### 验证镜像配置

配置完成后，添加一个依赖来测试镜像是否生效：

```powershell
# 创建测试项目
cargo new mirror_test
cd mirror_test

# 添加一个依赖
cargo add serde

# 编译项目，观察下载源是否显示 rsproxy.cn
cargo build
```

如果编译输出中看到类似 `Downloading from sparse+https://rsproxy.cn/index/...` 的字样，说明镜像配置成功。

#### 其他备选镜像源

如果 RsProxy 出现不可用的情况，也可以考虑以下备选：

| 镜像源 | Rustup 环境变量 | crates.io 配置 |
|--------|----------------|---------------|
| **中科大（USTC）** | `RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static` | `replace-with = 'ustc'`，`registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"` |
| **清华（TUNA）** | `RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup` | `replace-with = 'tuna'`，`registry = "sparse+https://mirrors.tuna.tsinghua.edu.cn/crates.io-index/"` |

---

## 第二部分：搭建 VSCode 开发环境

### 安装 VSCode

从官网下载并安装 Visual Studio Code：

```
https://code.visualstudio.com/
```

### 必装插件

打开 VSCode，点击左侧活动栏的**扩展**图标（四个方块组成的图标），或使用快捷键 `Ctrl+Shift+X`，搜索并安装以下插件：

#### 核心插件（必装）

| 插件名 | 作者 | 功能说明 |
|--------|------|----------|
| **rust-analyzer** | The Rust Programming Language | Rust 官方语言服务器，提供代码补全、类型推断、错误提示、代码格式化等核心功能 |

> **重要**：不要安装名为 "Rust" 的旧插件（rust-lang.rust），它已被弃用。请认准 **rust-analyzer**。

安装 rust-analyzer 后，打开任意 `.rs` 文件，右下角会显示加载进度。首次加载会稍慢，完成后你将获得：

- 实时编译错误提示（红色波浪线）
- 类型提示（变量名右侧灰色文字显示推断类型）
- 代码补全（输入时自动弹出建议）
- 快速跳转定义（`F12` 或 `Ctrl+Click`）
- 内联类型标注（`Ctrl+Shift+P` → "Toggle Inlay Hints"）

#### 推荐插件（可选但有用）

| 插件名 | 功能说明 |
|--------|----------|
| **Even Better TOML** | TOML 文件语法高亮和补全（用于编辑 `Cargo.toml`） |
| **CodeLLDB** | 基于 LLDB 的调试器，支持断点调试 Rust 程序 |
| **Error Lens** | 将错误/警告信息直接显示在代码行尾，不用悬停查看 |
| **crates** | 在 `Cargo.toml` 中显示依赖的最新版本，方便升级 |
| **Todo Tree** | 高亮并汇总代码中的 TODO/FIXME 注释 |

### VSCode 配置优化

打开 VSCode 设置（`Ctrl+,`），点击右上角的文件图标打开 `settings.json`，添加以下配置：

```json
{
    // Rust 相关配置
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.inlayHints.typeHints.enable": true,
    "rust-analyzer.inlayHints.parameterHints.enable": true,
    "rust-analyzer.cargo.features": "all",

    // 保存时自动格式化
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer",
        "editor.formatOnSave": true,
        "editor.tabSize": 4
    },

    // 文件排除（隐藏编译产物）
    "files.exclude": {
        "**/target": true
    },
    "files.watcherExclude": {
        "**/target/**": true
    }
}
```

配置说明：

- `rust-analyzer.check.command: "clippy"` — 保存时自动运行 Clippy（Rust 官方代码检查工具），比默认 `cargo check` 能发现更多问题
- `inlayHints` — 在编辑器中直接显示变量类型和参数名，对初学者理解代码非常有帮助
- `files.exclude` — 隐藏 `target` 目录（编译产物），避免文件树过于拥挤
- `files.watcherExclude` — 不监控 `target` 目录变化，提升 VSCode 性能

### 调试配置

VSCode 调试 Rust 需要配合 CodeLLDB 插件。安装插件后，在项目中创建调试配置：

在项目根目录创建 `.vscode/launch.json`：

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug executable 'hello_rust'",
            "cargo": {
                "args": [
                    "build",
                    "--bin=hello_rust",
                    "--package=hello_rust"
                ],
                "filter": {
                    "name": "hello_rust",
                    "kind": "bin"
                }
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        },
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug unit tests",
            "cargo": {
                "args": [
                    "test",
                    "--no-run"
                ]
            },
            "args": [],
            "cwd": "${workspaceFolder}"
        }
    ]
}
```

使用方法：

1. 在代码行号左侧点击，添加断点（出现红色圆点）
2. 按 `F5` 启动调试
3. 程序会在断点处暂停，你可以查看变量值、调用栈、逐步执行

---

## 第三部分：创建第一个 Rust 项目

### 使用 Cargo 创建项目

在 PowerShell 中执行：

```powershell
cd C:\user-data\code
cargo new hello_rust
cd hello_rust
```

`cargo new` 会自动生成如下项目结构：

```
hello_rust/
├── Cargo.toml      # 项目配置文件（类似 package.json / pom.xml）
├── Cargo.lock      # 依赖版本锁定文件（自动生成，不要手动编辑）
├── src/
│   └── main.rs     # 主程序入口
└── .git/            # Git 仓库（自动生成）
    └── .gitignore   # Git 忽略规则（已包含 /target）
```

### 理解 Cargo.toml

打开 `Cargo.toml`，默认内容如下：

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
```

各字段含义：

| 字段 | 说明 |
|------|------|
| `name` | 项目名称，也是编译后生成的可执行文件名 |
| `version` | 版本号，遵循语义化版本（主版本.次版本.补丁） |
| `edition` | Rust 版本纪年，目前常用 `2021`，最新为 `2024` |
| `[dependencies]` | 第三方依赖声明区域，目前为空 |

### 编写第一个程序

打开 `src/main.rs`，里面已经有一个 Hello World 程序：

```rust
fn main() {
    println!("Hello, world!");
}
```

我们来改一下，写一个稍微丰富点的版本：

```rust
fn main() {
    // 变量与类型
    let name = "Rust 初学者";
    let mut count = 0;

    println!("你好，{}！欢迎来到 Rust 的世界。", name);

    // 循环
    for i in 1..=5 {
        count += i;
        println!("第 {} 次循环，累计值: {}", i, count);
    }

    // 条件判断
    if count > 10 {
        println!("累计值 {} 超过了 10，Rust 真有趣！", count);
    } else {
        println!("累计值: {}", count);
    }

    // 函数调用
    let result = add(3, 7);
    println!("3 + 7 = {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // 最后一行没有分号，作为返回值
}
```

### 编译和运行

```powershell
# 方式一：直接运行（自动编译）
cargo run

# 方式二：先编译，再运行
cargo build
.\target\debug\hello_rust.exe

# 方式三：发布模式编译（优化过的二进制文件）
cargo build --release
.\target\release\hello_rust.exe
```

运行 `cargo run` 后，你应该看到：

```
   Compiling hello_rust v0.1.0 (C:\user-data\code\hello_rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
     Running `target\debug\hello_rust.exe`
你好，Rust 初学者！欢迎来到 Rust 的世界。
第 1 次循环，累计值: 1
第 2 次循环，累计值: 3
第 3 次循环，累计值: 6
第 4 次循环，累计值: 10
第 5 次循环，累计值: 15
累计值 15 超过了 10，Rust 真有趣！
3 + 7 = 10
```

### 常用 Cargo 命令速查

```powershell
cargo new <项目名>         # 创建新的二进制项目
cargo new <项目名> --lib   # 创建新的库项目
cargo build                # 编译项目（debug 模式）
cargo build --release      # 编译项目（release 模式，带优化）
cargo run                  # 编译并运行
cargo check                # 只检查代码是否有编译错误（不生成二进制，速度快）
cargo test                 # 运行测试
cargo clippy               # 运行 Clippy 代码检查（需先安装：rustup component add clippy）
cargo fmt                  # 自动格式化代码（需先安装：rustup component add rustfmt）
cargo clean                # 清除编译产物（删除 target 目录）
cargo add <库名>           # 添加依赖
cargo update               # 更新依赖到最新兼容版本
cargo doc --open           # 生成并打开本地文档
```

---

## 第四部分：与本项目整合

本项目 `rust-study` 是一个 Cargo workspace（工作区），包含多个子 crate。根目录的 `Cargo.toml` 配置如下：

```toml
[workspace]
members = [
    "crates/basic_syntax",
    "crates/advanced_features",
    "crates/engineering_practice",
    "blog",
]
resolver = "2"
```

### 理解 workspace

| 概念 | 说明 |
|------|------|
| workspace | 多个 Rust 项目共享同一个 `Cargo.lock` 和 `target` 目录 |
| member | workspace 中的每个子项目称为一个 member |
| `resolver = "2"` | 使用第二版依赖解析器，处理特性（features）时更精确 |

### 运行本项目的示例代码

```powershell
# 进入项目根目录
cd C:\user-data\code\my-infra\rust-study

# 运行基础语法示例
cargo run -p basic_syntax --example basic_tour

# 运行进阶特性示例
cargo run -p advanced_features --example advanced_tour

# 运行工程实践示例
cargo run -p engineering_practice --example engineering_tour

# 运行所有测试
cargo test

# 只运行某个 crate 的测试
cargo test -p basic_syntax
```

参数说明：

- `-p <crate名>`：指定要操作的 crate
- `--example <名称>`：运行 `examples/` 目录下的指定示例

### 在 workspace 中添加自己的练习 crate

如果你想在本项目中添加自己的练习代码：

```powershell
# 在项目根目录下创建新的 crate
cd C:\user-data\code\my-infra\rust-study\crates
cargo new my_practice --lib
```

然后编辑根目录的 `Cargo.toml`，把新 crate 加入 workspace：

```toml
[workspace]
members = [
    "crates/basic_syntax",
    "crates/advanced_features",
    "crates/engineering_practice",
    "crates/my_practice",
    "blog",
]
resolver = "2"
```

现在你可以用以下命令运行你的练习代码：

```powershell
cargo run -p my_practice
cargo test -p my_practice
```

### 推荐学习顺序

完成环境搭建后，按照以下顺序学习本项目：

1. 阅读 [学习路线](00-learning-path.md)，了解整体规划
2. 学习 [基础语法](01-basic-syntax.md)，运行 `cargo run -p basic_syntax --example basic_tour`
3. 学习 [进阶特性](02-advanced-features.md)，运行 `cargo run -p advanced_features --example advanced_tour`
4. 学习 [工程实践](03-engineering-practice.md)，运行 `cargo run -p engineering_practice --example engineering_tour`
5. 阅读 [语言对比](04-rust-java-go-comparison.md)，加深理解
6. 动手实践 [任务看板](05-build-task-tracker-from-zero.md)
7. 动手实践 [本地博客](06-build-local-blog-with-axum.md)

---

## 常见问题解答（FAQ）

### Q1：安装时提示缺少 Visual Studio Build Tools

**现象**：运行 `rustup-init.exe` 时提示需要 MSVC 构建工具。

**解决方案**：

1. 下载并安装 [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. 安装时勾选 **"使用 C++ 的桌面开发"** 工作负载
3. 确保勾选了 **"Windows 10 SDK"**（或更新版本）
4. 安装完成后重新运行 `rustup-init.exe`

### Q2：PowerShell 中运行 cargo 提示 "无法识别的命令"

**现象**：`cargo --version` 报错 `无法将"cargo"项识别为 cmdlet、函数、脚本文件或可运行程序的名称`。

**解决方案**：

1. 确认安装完成后**重新打开**了 PowerShell 窗口
2. 如果仍然不行，手动将 `C:\Users\你的用户名\.cargo\bin` 添加到系统 PATH 环境变量
3. 或者运行 `refreshenv`（需要安装 Chocolatey）

### Q3：编译速度很慢

**现象**：首次编译项目时间很长。

**说明**：这是正常的。Rust 编译器会做大量的静态分析和优化，首次编译（尤其是包含依赖的项目）可能需要几分钟。后续增量编译会快很多。

**优化建议**：

- 开发时使用 `cargo check` 代替 `cargo build`（只做类型检查，不生成二进制文件，速度快很多）
- 使用 `cargo clippy` 获取代码改进建议
- 在 VSCode 中 rust-analyzer 会自动在后台运行检查

### Q4：rust-analyzer 不工作 / 没有代码提示

**排查步骤**：

1. 确认安装了 **rust-analyzer** 插件（不是旧的 "Rust" 插件）
2. 确认项目根目录有 `Cargo.toml` 文件
3. 查看 VSCode 底部状态栏是否显示 rust-analyzer 加载状态
4. 按 `Ctrl+Shift+P`，输入 `rust-analyzer: Restart Server` 重启语言服务器
5. 如果仍然不行，检查 VSCode 输出面板（`Ctrl+Shift+U`）中 rust-analyzer 的日志

### Q5：如何查看标准库文档

Rust 的标准库文档非常优秀，有以下几种方式查看：

```powershell
# 方式一：本地生成并打开（推荐，离线可用）
cargo doc --open

# 方式二：在线查看
# 浏览器访问 https://doc.rust-lang.org/std/

# 方式三：在 VSCode 中，按住 Ctrl 点击类型名可以直接跳转到文档
```

### Q6：如何卸载 Rust

```powershell
rustup self uninstall
```

这会删除所有 Rust 工具链、Cargo 缓存和相关配置。

### Q7：`cargo run` 和 `cargo build` 的区别

| 命令 | 行为 | 适用场景 |
|------|------|----------|
| `cargo build` | 编译项目，生成 `target/debug/` 下的可执行文件 | 只需要编译，不需要立即运行 |
| `cargo run` | 编译并立即运行程序 | 开发时最常用 |
| `cargo check` | 只做类型检查，不生成可执行文件 | 快速验证代码是否正确 |
| `cargo build --release` | 编译并优化，生成 `target/release/` 下的可执行文件 | 发布或性能测试时使用 |

### Q8：Windows 上需要使用 GNU 工具链而不是 MSVC

如果你需要使用 GNU 工具链（例如某些 C 库的兼容性问题）：

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

> 大多数情况下推荐使用默认的 MSVC 工具链，兼容性更好。

---

## 速查卡片

### 一天的 Rust 开发流程

```powershell
# 1. 打开项目
code C:\user-data\code\my-infra\rust-study

# 2. 快速检查代码是否正确
cargo check

# 3. 运行示例，观察输出
cargo run -p basic_syntax --example basic_tour

# 4. 修改代码，保存，VSCode 自动显示错误

# 5. 运行测试，确保没有破坏已有功能
cargo test

# 6. 格式化代码
cargo fmt

# 7. 检查代码质量
cargo clippy
```

### Rust 核心概念预览

作为初学者，你将在后续学习中逐步遇到这些核心概念：

| 概念 | 一句话说明 | 何时学习 |
|------|-----------|----------|
| 所有权（Ownership） | 每个值只有一个所有者，离开作用域自动释放 | 阶段二 |
| 借用（Borrowing） | 通过引用临时使用值，不转移所有权 | 阶段二 |
| 生命周期（Lifetime） | 编译器追踪引用的有效范围 | 阶段二 |
| Trait | 定义类型的行为契约，类似接口 | 阶段二 |
| 枚举 + match | 安全的状态表达和模式匹配 | 阶段一/二 |
| 错误处理（Result/Option） | 没有异常机制，用类型表达成功/失败/有/无 | 阶段三 |
| 并发安全 | 编译器保证数据竞争在编译期被发现 | 阶段三 |

> 不需要现在就理解这些概念，按照学习路线一步步来就好。Rust 编译器是你最好的老师——它的错误信息非常详细，会告诉你哪里出了问题以及怎么修复。
