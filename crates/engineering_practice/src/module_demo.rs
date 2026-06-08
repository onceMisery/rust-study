//! 模块组织示例。
//!
//! 演示 Rust 的模块系统、可见性控制和代码组织方式。

/// 公开模块：math 运算
pub mod math {
    /// 公开函数：加法
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// 公开函数：使用内部辅助函数
    pub fn factorial(n: u32) -> u64 {
        internal_factorial(n)
    }

    /// 私有函数：只在模块内部可用
    fn internal_factorial(n: u32) -> u64 {
        if n <= 1 { 1 } else { n as u64 * internal_factorial(n - 1) }
    }
}

/// 公开模块：文本处理
pub mod text {
    /// 公开函数：统计字符数
    pub fn char_count(s: &str) -> usize {
        s.chars().count()
    }

    /// 公开子模块
    pub mod format {
        /// 转大写
        pub fn to_upper(s: &str) -> String {
            s.to_uppercase()
        }

        /// 转小写
        pub fn to_lower(s: &str) -> String {
            s.to_lowercase()
        }
    }
}

/// 演示模块的使用方式。
pub fn module_demo() -> String {
    // 通过完整路径调用
    let sum = math::add(3, 7);
    let fact = math::factorial(5);
    let chars = text::char_count("你好世界");
    let upper = text::format::to_upper("hello");

    format!(
        "add(3,7)={}, 5!={}, 字符数={}, upper={}",
        sum, fact, chars, upper
    )
}

/// 演示可见性规则。
pub fn visibility_demo() -> String {
    // ✅ 公开函数可以直接调用
    let result = math::add(10, 20);

    // ❌ 私有函数无法从外部调用
    // let internal = math::internal_factorial(5); // 编译错误

    format!("公开函数结果: {}", result)
}
