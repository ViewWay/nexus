//! Nexus Core - Core types and traits
//! Nexus核心 - 核心类型和trait
//!
//! # Overview / 概述
//!
//! `nexus-core` provides the foundational types and traits used throughout
//! the Nexus framework.
//!
//! `nexus-core` 提供Nexus框架中使用的基础类型和trait。
//!
//! # Features / 功能
//!
//! - Application state management / 应用状态管理
//! - Error types / 错误类型
//! - Extension system / 扩展系统
//! - Request/Response context / 请求/响应上下文

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod error;
pub mod context;
pub mod extension;

// Re-exports / 重新导出
pub use error::{Error, ErrorKind, Result};
pub use extension::Extensions;
