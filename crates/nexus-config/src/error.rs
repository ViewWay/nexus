//! Configuration error types
//! 配置错误类型

use std::path::PathBuf;
use thiserror::Error;

/// Configuration error type
/// 配置错误类型
///
/// Equivalent to Spring's `ConfigurationPropertiesException`.
/// 等价于Spring的`ConfigurationPropertiesException`。
#[derive(Error, Debug)]
pub enum ConfigError {
    /// I/O error
    /// I/O错误
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Parse error
    /// 解析错误
    #[error("Parse error: {0}")]
    Parse(String),

    /// Validation error
    /// 验证错误
    #[error("Validation error: {0}")]
    Validation(String),

    /// Missing required property
    /// 缺少必需属性
    #[error("Missing required property: {0}")]
    MissingProperty(String),

    /// Type conversion error
    /// 类型转换错误
    #[error("Type conversion error for '{key}': expected {expected}, got {value}")]
    TypeConversion {
        key: String,
        expected: String,
        value: String,
    },

    /// File not found
    /// 文件未找到
    #[error("Configuration file not found: {0}")]
    FileNotFound(PathBuf),

    /// Invalid format
    /// 无效格式
    #[error("Invalid configuration format: {0}")]
    InvalidFormat(String),

    /// Cycle detected in configuration
    /// 配置中检测到循环
    #[error("Cycle detected in configuration: {0}")]
    CycleDetected(String),

    /// Override not allowed
    /// 不允许覆盖
    #[error("Override not allowed for property: {0}")]
    OverrideNotAllowed(String),

    /// Unknown profile
    /// 未知配置文件
    #[error("Unknown profile: {0}")]
    UnknownProfile(String),

    /// Deserialize error
    /// 反序列化错误
    #[error("Deserialize error: {0}")]
    Deserialize(String),
}

/// Configuration result type
/// 配置结果类型
pub type ConfigResult<T> = Result<T, ConfigError>;

impl From<config::ConfigError> for ConfigError {
    fn from(err: config::ConfigError) -> Self {
        ConfigError::Parse(err.to_string())
    }
}

impl From<serde_json::Error> for ConfigError {
    fn from(err: serde_json::Error) -> Self {
        ConfigError::Deserialize(err.to_string())
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::Parse(err.to_string())
    }
}

// Note: yaml_rust2 error type is not directly exported
// If yaml parsing fails, it will be caught as serde_yaml error
// 注：yaml_rust2 错误类型未直接导出
// 如果 yaml 解析失败，将被捕获为 serde_yaml 错误
