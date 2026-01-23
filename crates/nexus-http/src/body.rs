//! Body type
//! Body类型
//!
//! # Overview / 概述
//!
//! This module provides HTTP body types.
//! 本模块提供HTTP body类型。

// TODO: Implement in Phase 2
// 将在第2阶段实现

/// HTTP body type
/// HTTP body类型
pub struct Body {
    _inner: Vec<u8>,
}

impl Body {
    /// Create an empty body / 创建空body
    pub fn empty() -> Self {
        Self { _inner: Vec::new() }
    }
}
