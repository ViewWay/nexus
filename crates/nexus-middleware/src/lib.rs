//! Nexus Middleware - Request/response middleware
//! Nexus中间件 - 请求/响应中间件
//!
//! # Overview / 概述
//!
//! `nexus-middleware` provides middleware for processing requests and responses.
//!
//! `nexus-middleware` 提供处理请求和响应的中间件。
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - Filter, HandlerInterceptor
//! - @CrossOrigin
//! - OncePerRequestFilter
//! - CorsConfiguration, CORS filter
//! - Request logging / MDC

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod middleware;
pub mod cors;
pub mod compression;
pub mod timeout;
pub mod logger;

// Re-export from nexus-http
// 从nexus-http重新导出
pub use nexus_http::{Request, Response};

// Re-export from nexus-router for middleware compatibility
// 从nexus-router重新导出以兼容中间件
pub use nexus_router::{Middleware, Next};

// Re-export error/result types
// 重新导出错误/结果类型
pub type Error = nexus_http::Error;
pub type Result<T> = nexus_http::Result<T>;

// Re-export middleware types
// 重新导出中间件类型
pub use cors::{CorsMiddleware, CorsConfig};
pub use compression::CompressionMiddleware;
pub use timeout::TimeoutMiddleware;
pub use logger::LoggerMiddleware;
