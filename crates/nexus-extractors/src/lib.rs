//! Nexus Extractors - Request data extractors
//! Nexus提取器 - 请求数据提取器
//!
//! # Overview / 概述
//!
//! `nexus-extractors` provides extractors for pulling data from HTTP requests.
//!
//! `nexus-extractors` 提供从HTTP请求中提取数据的提取器。

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod path;
pub mod query;
pub mod json;
pub mod form;
pub mod state;

pub use path::Path;
pub use query::Query;
pub use json::Json;
pub use form::Form;
pub use state::State;
