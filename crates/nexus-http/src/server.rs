//! Server module
//! 服务器模块
//!
//! # Overview / 概述
//!
//! This module provides HTTP server functionality.
//! 本模块提供HTTP服务器功能。

// TODO: Implement in Phase 2
// 将在第2阶段实现

/// HTTP server
/// HTTP服务器
pub struct Server;

impl Server {
    /// Create a new server / 创建新服务器
    pub fn new() -> Self {
        Self
    }

    /// Bind to an address / 绑定地址
    pub fn bind(&self, _addr: &str) -> &Self {
        self
    }
}
