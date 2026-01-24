//! Multipart error types
//! Multipart 错误类型

use std::fmt;

/// Multipart result type
/// Multipart 结果类型
pub type MultipartResult<T> = Result<T, MultipartError>;

/// Multipart error
/// Multipart 错误
///
/// Equivalent to Spring's MultipartException.
/// 等价于Spring的MultipartException。
#[derive(Debug)]
pub enum MultipartError {
    /// Invalid multipart request
    /// 无效的 multipart 请求
    InvalidRequest(String),

    /// Field not found
    /// 字段未找到
    FieldNotFound(String),

    /// File too large
    /// 文件过大
    FileTooLarge {
        /// Field name / 字段名
        field: String,
        /// File size / 文件大小
        size: usize,
        /// Max allowed size / 最大允许大小
        max_size: usize,
    },

    /// IO error
    /// IO 错误
    Io(std::io::Error),

    /// Decode error
    /// 解码错误
    Decode(String),

    /// Unknown field
    /// 未知字段
    UnknownField(String),
}

impl fmt::Display for MultipartError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(msg) => write!(f, "Invalid multipart request: {}", msg),
            Self::FieldNotFound(name) => write!(f, "Field not found: {}", name),
            Self::FileTooLarge { field, size, max_size } => {
                write!(f, "File too large: field={}, size={}, max_size={}", field, size, max_size)
            }
            Self::Io(err) => write!(f, "IO error: {}", err),
            Self::Decode(msg) => write!(f, "Decode error: {}", msg),
            Self::UnknownField(name) => write!(f, "Unknown field: {}", name),
        }
    }
}

impl std::error::Error for MultipartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<multer::Error> for MultipartError {
    fn from(err: multer::Error) -> Self {
        match err {
            multer::Error::UnknownField { field_name } => {
                Self::UnknownField(field_name.unwrap_or_else(|| "unknown".to_string()))
            }
            multer::Error::IncompleteFieldData { .. } => {
                Self::InvalidRequest("Incomplete field data".to_string())
            }
            _ => Self::InvalidRequest(err.to_string()),
        }
    }
}

impl From<std::io::Error> for MultipartError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = MultipartError::FieldNotFound("file".to_string());
        assert_eq!(err.to_string(), "Field not found: file");

        let err = MultipartError::FileTooLarge {
            field: "upload".to_string(),
            size: 20 * 1024 * 1024,
            max_size: 10 * 1024 * 1024,
        };
        assert!(err.to_string().contains("File too large"));
    }
}
