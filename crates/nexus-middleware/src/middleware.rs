//! Middleware module
//! 中间件模块
//!
//! # Overview / 概述
//!
//! This module provides middleware for processing requests and responses.
//! 本模块提供处理请求和响应的中间件。

use http::Request;

// TODO: Implement in Phase 3
// 将在第3阶段实现

/// Middleware trait
/// 中间件trait
pub trait Middleware<S>: Clone + Send + Sync + 'static {
    /// Output type
    /// 输出类型
    type Output;

    /// Call the middleware
    /// 调用中间件
    fn call(&self, _req: Request<()>, _next: Next<S>) -> Self::Output {
        todo!("Implement in Phase 3")
    }
}

/// Next middleware in the chain
/// 链中的下一个中间件
pub struct Next<S> {
    _phantom: std::marker::PhantomData<S>,
}
