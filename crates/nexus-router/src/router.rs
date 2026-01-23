//! Router module
//! 路由器模块
//!
//! # Overview / 概述
//!
//! This module provides HTTP request routing.
//! 本模块提供HTTP请求路由。

// TODO: Implement in Phase 2
// 将在第2阶段实现

/// HTTP router
/// HTTP路由器
pub struct Router<S = ()> {
    _phantom: std::marker::PhantomData<S>,
}

impl<S> Router<S> {
    /// Create a new router / 创建新路由器
    pub fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }

    /// Add a GET route / 添加GET路由
    pub fn get(&self, _path: &str) -> &Self {
        self
    }
}

impl<S> Default for Router<S> {
    fn default() -> Self {
        Self::new()
    }
}
