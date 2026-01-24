//! Nexus Extractors - Request data extractors
//! Nexus提取器 - 请求数据提取器
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - `Path<T>` - `@PathVariable`
//! - `Query<T>` - `@RequestParam` / `@RequestParam`
//! - `Json<T>` - `@RequestBody`
//! - `Form<T>` - `@ModelAttribute`
//! - `State<T>` - `@Autowired` / Application state
//! - `Header<T>` - `@RequestHeader`
//! - `Cookie<T>` - `@CookieValue`

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod path;
pub mod query;
pub mod json;
pub mod form;
pub mod state;
pub mod header;
pub mod cookie;

pub use path::Path;
pub use query::Query;
pub use json::Json;
pub use form::Form;
pub use state::State;
pub use header::{Header, HeaderOption, NamedHeader};
pub use cookie::{Cookie, CookieOption, NamedCookie};

use std::future::Future;
use std::pin::Pin;

// Re-export Request from nexus-http
// 从 nexus-http 重新导出 Request
pub use nexus_http::Request;

/// Extractor trait
/// 提取器trait
///
/// Equivalent to Spring's method parameter resolution for:
/// - `@PathVariable`
/// - `@RequestParam`
/// - `@RequestBody`
/// - `@RequestHeader`
/// - `@CookieValue`
/// - `@ModelAttribute`
///
/// 这等价于Spring的以下方法参数解析：
/// - `@PathVariable`
/// - `@RequestParam`
/// - `@RequestBody`
/// - `@RequestHeader`
/// - `@CookieValue`
/// - `@ModelAttribute`
pub trait FromRequest: Sized {
    /// Extract from request
    /// 从请求提取
    fn from_request(req: &Request) -> ExtractorFuture<Self>;
}

/// Future type returned by FromRequest
/// FromRequest 返回的 Future 类型
pub type ExtractorFuture<T> = Pin<Box<dyn Future<Output = Result<T, ExtractorError>> + Send>>;

/// Extractor error
/// 提取器错误
#[derive(Debug, thiserror::Error)]
pub enum ExtractorError {
    /// Missing parameter
    /// 缺少参数
    #[error("Missing parameter: {0}")]
    Missing(String),

    /// Invalid parameter format
    /// 无效参数格式
    #[error("Invalid parameter format: {0}")]
    Invalid(String),

    /// IO error
    /// IO错误
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    /// JSON错误
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Other error
    /// 其他错误
    #[error("Error: {0}")]
    Other(String),
}
