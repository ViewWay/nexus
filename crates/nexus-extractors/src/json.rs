//! JSON extractor module
//! JSON提取器模块
//!
//! # Overview / 概述
//!
//! This module provides JSON body extraction.
//! 本模块提供JSON body提取。

use serde::Deserialize;

// TODO: Implement in Phase 2
// 将在第2阶段实现

/// JSON extractor
/// JSON提取器
pub struct Json<T>(pub T);

impl<T> Json<T> {
    /// Consume the JSON and get the inner value
    /// 消耗JSON并获取内部值
    pub fn into_inner(self) -> T {
        self.0
    }
}
