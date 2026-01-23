//! JSON response module
//! JSON响应模块
//!
//! # Overview / 概述
//!
//! This module provides JSON response types.
//! 本模块提供JSON响应类型.

use crate::response::IntoResponse;
use serde::Serialize;

// TODO: Implement in Phase 2
// 将在第2阶段实现

/// JSON response wrapper
/// JSON响应包装器
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> crate::response::Response {
        todo!("Implement in Phase 2")
    }
}
