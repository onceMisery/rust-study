//! Rust 工程实践示例。
//!
//! ```
//! assert_eq!(engineering_practice::divide(10, 2), Ok(5));
//! assert_eq!(engineering_practice::find_config("host"), Some("localhost"));
//! ```

pub mod concurrency;
pub mod config;
pub mod errors;
pub mod math;

pub use concurrency::{add_with_thread, shared_counter, sum_with_channel};
pub use config::{find_config, parse_port};
pub use errors::AppError;
pub use math::divide;
