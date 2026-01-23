//! Nexus Middleware - Request/response middleware
//! Nexus中间件 - 请求/响应中间件
//!
//! # Overview / 概述
//!
//! `nexus-middleware` provides middleware for processing requests and responses.
//!
//! `nexus-middleware` 提供处理请求和响应的中间件。

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod middleware;
pub mod cors;
pub mod compression;
pub mod timeout;
pub mod logger;

pub use middleware::{Middleware, Next};
