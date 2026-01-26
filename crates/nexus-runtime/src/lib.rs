//! Nexus Runtime - High-performance async runtime
//! Nexus运行时 - 高性能异步运行时
//!
//! # Overview / 概述
//!
//! `nexus-runtime` provides a high-performance async runtime based on io-uring
//! with thread-per-core architecture for maximum scalability.
//!
//! `nexus-runtime` 提供基于io-uring的高性能异步运行时，采用thread-per-core架构
//! 以实现最大可扩展性。
//!
//! # Features / 功能
//!
//! - io-uring based I/O (Linux) / 基于io-uring的I/O（Linux）
//! - epoll/kqueue fallback / epoll/kqueue回退支持
//! - Thread-per-core scheduler / Thread-per-core调度器
//! - Timer wheel for efficient timers / 高效定时器的时间轮
//! - Zero-copy I/O primitives / 零拷贝I/O原语
//!
//! # Example / 示例
//!
//! ```rust,no_run,ignore
//! use nexus_runtime::Runtime;
//!
//! fn main() -> std::io::Result<()> {
//!     let runtime = Runtime::new()?;
//!     runtime.block_on(async {
//!         println!("Hello, Nexus!");
//!     });
//!     Ok(())
//! }
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// Allow unsafe operations in unsafe functions for Rust 2024 edition compatibility
// 允许unsafe函数中的unsafe操作以兼容Rust 2024版本
#![expect(unsafe_op_in_unsafe_fn)]

// Public modules / 公共模块
pub mod channel;
pub mod driver;
pub mod io;
pub mod runtime;
pub mod scheduler;
pub mod select;
pub mod task;
pub mod time;

// Re-exports / 重新导出
pub use channel::{Receiver, RecvError, SendError, Sender, bounded, unbounded};
pub use driver::{Driver, DriverConfig, DriverConfigBuilder, DriverFactory, DriverType};
pub use runtime::{Runtime, RuntimeBuilder, RuntimeConfig};
pub use scheduler::{
    Scheduler, SchedulerConfig, SchedulerHandle, WorkStealingConfig, WorkStealingHandle,
    WorkStealingScheduler, gen_task_id,
};
pub use select::{
    SelectMultiple, SelectMultipleOutput, SelectTwo, SelectTwoOutput, select_multiple, select_two,
};
pub use task::{JoinError, JoinHandle, spawn};
pub use time::{Duration, Instant, sleep, sleep_until};
