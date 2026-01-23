//! Nexus Response - Response builders
//! Nexus响应 - 响应构建器
//!
//! # Overview / 概述
//!
//! `nexus-response` provides response builders and types for HTTP responses.
//!
//! `nexus-response` 提供HTTP响应的构建器和类型。

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod response;
pub mod json;
pub mod html;

pub use response::{IntoResponse, Response};
pub use json::Json;
pub use html::Html;
