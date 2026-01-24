//! HTTP Error types
//! HTTP 错误类型
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - @ResponseStatus, @ExceptionHandler, ResponseEntity

#![warn(missing_docs)]
#![warn(unreachable_pub)]

use std::fmt;

/// HTTP Error type
/// HTTP 错误类型
#[derive(Debug, Clone)]
pub enum Error {
    /// Incomplete request (needs more data) / 不完整请求（需要更多数据）
    IncompleteRequest,

    /// Invalid request / 无效请求
    InvalidRequest(String),

    /// Invalid response / 无效响应
    InvalidResponse(String),

    /// IO error / IO错误
    Io(String),

    /// Timeout error / 超时错误
    Timeout(String),

    /// Connection error / 连接错误
    Connection(String),

    /// Parse error / 解析错误
    Parse(String),

    /// Serialization error / 序列化错误
    Serialization(String),

    /// Not found error / 未找到错误
    NotFound(String),

    /// Unauthorized error / 未授权错误
    Unauthorized,

    /// Forbidden error / 禁止访问错误
    Forbidden,

    /// Internal server error / 内部服务器错误
    Internal(String),

    /// Custom error with status code / 带状态码的自定义错误
    Custom(u16, String),
}

impl Error {
    /// Create a 400 Bad Request error
    /// 创建400 Bad Request错误
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Error::InvalidRequest(msg.into())
    }

    /// Create a 401 Unauthorized error
    /// 创建401 Unauthorized错误
    pub fn unauthorized() -> Self {
        Error::Unauthorized
    }

    /// Create a 403 Forbidden error
    /// 创建403 Forbidden错误
    pub fn forbidden() -> Self {
        Error::Forbidden
    }

    /// Create a 404 Not Found error
    /// 创建404 Not Found错误
    pub fn not_found(resource: impl Into<String>) -> Self {
        Error::NotFound(resource.into())
    }

    /// Create a 500 Internal Server Error
    /// 创建500 Internal Server Error错误
    pub fn internal(msg: impl Into<String>) -> Self {
        Error::Internal(msg.into())
    }

    /// Get the HTTP status code for this error
    /// 获取此错误的HTTP状态码
    pub fn status_code(&self) -> u16 {
        match self {
            Error::InvalidRequest(_) => 400,
            Error::Unauthorized => 401,
            Error::Forbidden => 403,
            Error::NotFound(_) => 404,
            Error::Timeout(_) => 408,
            Error::Parse(_) => 422,
            Error::Internal(_) => 500,
            Error::Custom(code, _) => *code,
            _ => 500,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IncompleteRequest => write!(f, "Incomplete Request"),
            Error::InvalidRequest(msg) => write!(f, "Bad Request: {}", msg),
            Error::InvalidResponse(msg) => write!(f, "Invalid Response: {}", msg),
            Error::Io(msg) => write!(f, "IO Error: {}", msg),
            Error::Timeout(msg) => write!(f, "Timeout: {}", msg),
            Error::Connection(msg) => write!(f, "Connection Error: {}", msg),
            Error::Parse(msg) => write!(f, "Parse Error: {}", msg),
            Error::Serialization(msg) => write!(f, "Serialization Error: {}", msg),
            Error::NotFound(msg) => write!(f, "Not Found: {}", msg),
            Error::Unauthorized => write!(f, "Unauthorized"),
            Error::Forbidden => write!(f, "Forbidden"),
            Error::Internal(msg) => write!(f, "Internal Server Error: {}", msg),
            Error::Custom(code, msg) => write!(f, "Error {}: {}", code, msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

/// Result type for HTTP operations
/// HTTP操作的Result类型
pub type Result<T> = std::result::Result<T, Error>;
