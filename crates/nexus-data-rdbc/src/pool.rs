//! Database connection pool
//! 数据库连接池
//!
//! # Overview / 概述
//!
//! Connection pool for database connections.
//! 数据库连接的连接池。

use std::time::Duration;

/// Connection pool trait
/// 连接池 trait
///
/// Generic connection pool trait.
/// 通用连接池 trait。
pub trait Pool: Send + Sync {
    /// Close the pool
    /// 关闭连接池
    fn close(&self) -> impl std::future::Future<Output = Result<(), crate::Error>> + Send;

    /// Get pool size
    /// 获取连接池大小
    fn size(&self) -> usize;

    /// Get idle connections count
    /// 获取空闲连接数
    fn idle_connections(&self) -> usize;
}

/// Pool options
/// 连接池选项
#[derive(Debug, Clone)]
pub struct PoolOptions {
    /// Maximum connections
    /// 最大连接数
    pub max_connections: u32,

    /// Minimum idle connections
    /// 最小空闲连接数
    pub min_connections: u32,

    /// Connection timeout
    /// 连接超时
    pub connect_timeout: Duration,

    /// Idle timeout
    /// 空闲超时
    pub idle_timeout: Duration,

    /// Max lifetime
    /// 最大生命周期
    pub max_lifetime: Option<Duration>,
}

impl Default for PoolOptions {
    fn default() -> Self {
        Self {
            max_connections: 10,
            min_connections: 1,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Some(Duration::from_secs(1800)),
        }
    }
}

impl PoolOptions {
    /// Create new pool options
    /// 创建新的连接池选项
    pub fn new() -> Self {
        Self::default()
    }

    /// Set max connections
    /// 设置最大连接数
    pub fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    /// Set min connections
    /// 设置最小连接数
    pub fn min_connections(mut self, min: u32) -> Self {
        self.min_connections = min;
        self
    }

    /// Set connect timeout
    /// 设置连接超时
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = timeout;
        self
    }

    /// Set idle timeout
    /// 设置空闲超时
    pub fn idle_timeout(mut self, timeout: Duration) -> Self {
        self.idle_timeout = timeout;
        self
    }

    /// Set max lifetime
    /// 设置最大生命周期
    pub fn max_lifetime(mut self, lifetime: Duration) -> Self {
        self.max_lifetime = Some(lifetime);
        self
    }
}
