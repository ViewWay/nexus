//! HTML response module
//! HTML响应模块
//!
//! # Overview / 概述
//!
//! This module provides HTML response types.
//! 本模块提供HTML响应类型。

use crate::response::IntoResponse;

// TODO: Implement in Phase 2
// 将在第2阶段实现

/// HTML response wrapper
/// HTML响应包装器
pub struct Html<T>(pub T);

impl<T: AsRef<str>> IntoResponse for Html<T> {
    fn into_response(self) -> crate::response::Response {
        todo!("Implement in Phase 2")
    }
}
