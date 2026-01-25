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
pub mod static_files;
pub mod jwt_auth;

// Re-export core types from nexus-http and nexus-router
// 从nexus-http和nexus-router重新导出核心类型
pub use nexus_http::{Request, Response, Error};
pub use nexus_router::{Middleware, Next};

// Re-export result type
// 重新导出结果类型
pub type Result<T> = nexus_http::Result<T>;

// Re-export middleware types
// 重新导出中间件类型
pub use middleware::MiddlewareStack;
pub use cors::{CorsMiddleware, CorsConfig};
pub use compression::CompressionMiddleware;
pub use timeout::TimeoutMiddleware;
pub use logger::LoggerMiddleware;
pub use static_files::StaticFiles;
pub use jwt_auth::{JwtAuthenticationMiddleware, JwtRequestExt};
