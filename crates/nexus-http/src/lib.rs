//! Nexus HTTP - HTTP server and client
//! Nexus HTTP - HTTP服务器和客户端
//!
//! # Overview / 概述
//!
//! `nexus-http` provides HTTP server and client implementations for the
//! Nexus framework.
//!
//! `nexus-http` 为Nexus框架提供HTTP服务器和客户端实现。

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod body;
pub mod server;
pub mod conn;
pub mod service;

pub use body::Body;
pub use server::Server;
