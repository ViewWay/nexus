//! Validation error types
//! 验证错误类型

use std::collections::HashMap;
use std::fmt;

/// Validation result type
/// 验证结果类型
pub type ValidationResult<T> = Result<T, ValidationError>;

/// Validation error
/// 验证错误
///
/// Equivalent to Spring's MethodArgumentNotValidException.
/// 等价于Spring的MethodArgumentNotValidException。
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Field errors
    /// 字段错误
    pub field_errors: HashMap<String, Vec<String>>,

    /// Global errors
    /// 全局错误
    pub global_errors: Vec<String>,
}

impl ValidationError {
    /// Create a new validation error
    /// 创建新的验证错误
    pub fn new() -> Self {
        Self {
            field_errors: HashMap::new(),
            global_errors: Vec::new(),
        }
    }

    /// Add a field error
    /// 添加字段错误
    pub fn add_field_error(&mut self, field: impl Into<String>, message: impl Into<String>) {
        let field = field.into();
        let message = message.into();
        self.field_errors
            .entry(field)
            .or_insert_with(Vec::new)
            .push(message);
    }

    /// Add a global error
    /// 添加全局错误
    pub fn add_global_error(&mut self, message: impl Into<String>) {
        self.global_errors.push(message.into());
    }

    /// Check if there are any errors
    /// 检查是否有任何错误
    pub fn has_errors(&self) -> bool {
        !self.field_errors.is_empty() || !self.global_errors.is_empty()
    }

    /// Get the total error count
    /// 获取总错误数
    pub fn error_count(&self) -> usize {
        self.field_errors.values().map(|v| v.len()).sum::<usize>()
            + self.global_errors.len()
    }

    /// Create from validator errors
    /// 从验证器错误创建
    pub fn from_validator_errors(errors: validator::ValidationErrors) -> Self {
        let mut validation_error = Self::new();

        for (field, field_errors_kind) in errors.errors() {
            let field_name = field.to_string();
            let mut messages = Vec::new();

            // ValidationErrorsKind is an enum - match on its variants
            match field_errors_kind {
                validator::ValidationErrorsKind::Field(errs) => {
                    // Field-level validation errors
                    for err in errs {
                        let message = if let Some(ref msg) = err.message {
                            msg.to_string()
                        } else {
                            err.code.to_string()
                        };
                        messages.push(message);
                    }
                }
                validator::ValidationErrorsKind::List(list_errors) => {
                    // List/index-level validation errors (e.g., for Vec items)
                    for (_index, nested_errors) in list_errors {
                        // nested_errors is &Box<ValidationErrors>, need to extract errors from it
                        for (_nested_field, nested_kind) in nested_errors.errors() {
                            if let validator::ValidationErrorsKind::Field(errs) = nested_kind {
                                for err in errs {
                                    let message = if let Some(ref msg) = err.message {
                                        msg.to_string()
                                    } else {
                                        err.code.to_string()
                                    };
                                    messages.push(message);
                                }
                            }
                        }
                    }
                }
                validator::ValidationErrorsKind::Struct(nested_errors) => {
                    // Nested struct validation errors
                    for (_nested_field, nested_kind) in nested_errors.errors() {
                        if let validator::ValidationErrorsKind::Field(errs) = nested_kind {
                            for err in errs {
                                let message = if let Some(ref msg) = err.message {
                                    msg.to_string()
                                } else {
                                    err.code.to_string()
                                };
                                messages.push(message);
                            }
                        }
                    }
                }
            }

            if !messages.is_empty() {
                validation_error.field_errors.insert(field_name, messages);
            }
        }

        validation_error
    }
}

impl Default for ValidationError {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Validation failed with {} errors", self.error_count())
    }
}

impl std::error::Error for ValidationError {}

/// Convert from validator::ValidationErrors
impl From<validator::ValidationErrors> for ValidationError {
    fn from(errors: validator::ValidationErrors) -> Self {
        Self::from_validator_errors(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error() {
        let mut err = ValidationError::new();
        err.add_field_error("username", "Username is required");
        err.add_field_error("email", "Invalid email format");
        err.add_global_error("Validation failed");

        assert!(err.has_errors());
        assert_eq!(err.error_count(), 3);
        assert_eq!(err.field_errors.len(), 2);
        assert_eq!(err.global_errors.len(), 1);
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::new();
        assert_eq!(err.to_string(), "Validation failed with 0 errors");

        let mut err = ValidationError::new();
        err.add_field_error("test", "error");
        assert!(err.to_string().contains("1 error"));
    }
}
