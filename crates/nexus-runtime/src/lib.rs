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
pub use channel::{bounded, unbounded, Receiver, Sender, RecvError, SendError};
pub use driver::{Driver, DriverConfig, DriverConfigBuilder, DriverFactory, DriverType};
pub use runtime::{Runtime, RuntimeBuilder, RuntimeConfig};
pub use scheduler::{
    gen_task_id, Scheduler, SchedulerConfig, SchedulerHandle,
    WorkStealingScheduler, WorkStealingConfig, WorkStealingHandle,
};
pub use select::{select_multiple, select_two, SelectMultiple, SelectMultipleOutput, SelectTwo, SelectTwoOutput};
pub use task::{spawn, JoinHandle, JoinError};
pub use time::{sleep, sleep_until, Duration, Instant};
