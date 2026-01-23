//! Response module
//! 响应模块
//!
//! # Overview / 概述
//!
//! This module provides HTTP response types.
//! 本模块提供HTTP响应类型。

use http::StatusCode;

// TODO: Implement in Phase 2
// 将在第2阶段实现

/// HTTP response
/// HTTP响应
pub struct Response {
    _status: StatusCode,
    _body: Vec<u8>,
}

impl Response {
    /// Create a new response
    /// 创建新响应
    pub fn new() -> Self {
        Self {
            _status: StatusCode::OK,
            _body: Vec::new(),
        }
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for types that can be converted to HTTP responses
/// 可转换为HTTP响应的类型的trait
pub trait IntoResponse {
    /// Convert self into a response
    /// 将self转换为响应
    fn into_response(self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}
