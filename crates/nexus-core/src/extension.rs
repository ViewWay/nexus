//! Extension system
//! 扩展系统
//!
//! # Overview / 概述
//!
//! This module provides the extension system for storing request-scoped data.
//! 本模块提供用于存储请求范围数据的扩展系统。

use std::any::Any;
use std::collections::HashMap;

// TODO: Implement in Phase 2
// 将在第2阶段实现

/// Extensions for storing request-scoped data
/// 用于存储请求范围数据的扩展
#[derive(Default)]
pub struct Extensions {
    _inner: HashMap<String, Box<dyn Any + Send + Sync>>,
}
