//! Nexus Router - HTTP request router
//! Nexus路由器 - HTTP请求路由器
//!
//! # Overview / 概述
//!
//! `nexus-router` provides efficient HTTP request routing with path parameters
//! and middleware support.
//!
//! `nexus-router` 提供高效的HTTP请求路由，支持路径参数和中间件。

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod router;
pub mod params;

pub use router::Router;
pub use params::Path;
