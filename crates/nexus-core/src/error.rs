//! Error types
//! 错误类型
//!
//! # Overview / 概述
//!
//! This module provides core error types used throughout the framework.
//! 本模块提供框架中使用的核心错误类型。

// TODO: Implement error types in Phase 1
// 将在第1阶段实现错误类型

/// Framework error type
/// 框架错误类型
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
}

/// Error kind
/// 错误类型
#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    /// Bad request / 错误请求
    BadRequest,
    /// Not found / 未找到
    NotFound,
    /// Internal error / 内部错误
    Internal,
}

/// Result type alias
/// Result类型别名
pub type Result<T> = std::result::Result<T, Error>;
