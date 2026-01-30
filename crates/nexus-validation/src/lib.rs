//! Nexus Validation - 验证模块 / Validation Module
//!
//! 提供请求参数校验功能 / Provides request parameter validation
//!
//! # 基本使用 / Basic Usage
//!
//! ```rust,ignore
//! use nexus_validation::{Validate, ValidationErrors};
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Debug, Deserialize, Serialize, Validate)]
//! struct CreateUserRequest {
//!     #[validate(length(min = 3, max = 20))]
//!     username: String,
//!
//!     #[validate(email)]
//!     email: String,
//!
//!     #[validate(range(min = 18, max = 120))]
//!     age: u32,
//! }
//!
//! #[nexus_macros::post("/users")]
//! async fn create_user(
//!     #[validated] request: CreateUserRequest,
//! ) -> Result<Json<User>, Error> {
//!     // request is validated
//!     Ok(Json(user))
//! }
//! ```

pub mod error;
pub mod extractor;
pub mod traits;
pub mod validators;

// Re-exports commonly used types
pub use error::{ValidationError, ValidationErrors};
pub use extractor::Valid;
pub use traits::Validate;
pub use validators::*;

use std::fmt;

/// 验证结果 / Validation result
pub type ValidationResult<T> = Result<T, ValidationErrors>;

/// 验证上下文 / Validation context
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// 字段名 / Field name
    pub field: String,
    /// 字段值 / Field value
    pub value: String,
    /// 自定义消息 / Custom message
    pub message: Option<String>,
    /// 代码 / Code
    pub code: String,
}

impl ValidationContext {
    /// Create a new validation context
    /// 创建新的验证上下文
    pub fn new(field: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            value: value.into(),
            message: None,
            code: "validation_failed".to_string(),
        }
    }

    /// Set custom message
    /// 设置自定义消息
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Set error code
    /// 设置错误代码
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = code.into();
        self
    }
}

/// 验证规则 / Validation rules
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationRule {
    /// 非空 / Not empty
    NotEmpty,
    /// 长度范围 / Length range
    Length {
        /// Minimum length
        /// 最小长度
        min: Option<usize>,
        /// Maximum length
        /// 最大长度
        max: Option<usize>,
    },
    /// 数值范围 / Range
    Range {
        /// Minimum value
        /// 最小值
        min: Option<i64>,
        /// Maximum value
        /// 最大值
        max: Option<i64>
    },
    /// 邮箱 / Email
    Email,
    /// URL
    Url,
    /// 正则表达式 / Regex
    Regex(&'static str),
    /// 自定义 / Custom
    Custom(&'static str),
}

impl fmt::Display for ValidationRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationRule::NotEmpty => write!(f, "not_empty"),
            ValidationRule::Length { min, max } => {
                write!(f, "length(min={:?}, max={:?})", min, max)
            },
            ValidationRule::Range { min, max } => {
                write!(f, "range(min={:?}, max={:?})", min, max)
            },
            ValidationRule::Email => write!(f, "email"),
            ValidationRule::Url => write!(f, "url"),
            ValidationRule::Regex(pattern) => write!(f, "regex({})", pattern),
            ValidationRule::Custom(name) => write!(f, "custom({})", name),
        }
    }
}
