//! 配置属性 / Configuration Properties
//!
//! 定义配置属性的 trait 和实现。
//! Defines traits and implementations for configuration properties.

use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;

use super::loader::ConfigurationLoader;

// ============================================================================
// ConfigurationProperties Trait / 配置属性 Trait
// ============================================================================

/// 配置属性 trait
/// Configuration properties trait
///
/// 用于将配置绑定到结构体。
/// Used to bind configuration to structs.
///
/// 等价于 Spring Boot 的 `@ConfigurationProperties`。
/// Equivalent to Spring Boot's `@ConfigurationProperties`.
///
/// # 示例 / Example
///
/// ```rust,ignore
/// #[derive(ConfigurationProperties, Deserialize)]
/// #[config(prefix = "server")]
/// pub struct ServerProperties {
///     #[config(default = "8080")]
///     pub port: u16,
///
///     #[config(default = "127.0.0.1")]
///     pub host: String,
/// }
/// ```
pub trait ConfigurationProperties: Send + Sync {
    /// 从配置加载器加载属性
    /// Load properties from configuration loader
    fn from_loader(loader: &ConfigurationLoader) -> Result<Self>
    where
        Self: Sized;
}

// ============================================================================
// PropertyResolver / 属性解析器
// ============================================================================

/// 属性解析器
/// Property resolver
///
/// 用于解析配置属性，支持占位符替换。
/// Used to resolve configuration properties with placeholder support.
#[derive(Debug, Clone)]
pub struct PropertyResolver {
    /// 配置加载器
    loader: Arc<ConfigurationLoader>,

    /// 占位符前缀
    placeholder_prefix: String,

    /// 占位符后缀
    placeholder_suffix: String,

    /// 值分隔符
    value_separator: String,
}

impl PropertyResolver {
    /// 创建新的属性解析器
    pub fn new(loader: Arc<ConfigurationLoader>) -> Self {
        Self {
            loader,
            placeholder_prefix: "${".to_string(),
            placeholder_suffix: "}".to_string(),
            value_separator: ":".to_string(),
        }
    }

    /// 解析属性值（支持占位符）
    /// Resolve property value (with placeholder support)
    ///
    /// # 示例 / Example
    ///
    /// ```text
    /// ${server.port}         -> 从配置获取 server.port
    /// ${server.port:8080}    -> 从配置获取 server.port，默认 8080
    /// ```
    pub fn resolve(&self, value: &str) -> String {
        let mut result = value.to_string();

        // 简单的占位符替换（TODO: 完整实现）
        while let Some(start) = result.find(&self.placeholder_prefix) {
            let end = match result[start..].find(&self.placeholder_suffix) {
                Some(pos) => pos + start + self.placeholder_suffix.len(),
                None => break,
            };

            let placeholder = &result[start..end];
            let inner = &placeholder[self.placeholder_prefix.len()..placeholder.len() - self.placeholder_suffix.len()];

            let resolved = if let Some(colon_pos) = inner.find(&self.value_separator) {
                // 有默认值: ${key:default}
                let key = &inner[..colon_pos];
                let default = &inner[colon_pos + 1..];
                self.loader.get_or(key, default)
            } else {
                // 无默认值: ${key}
                self.loader.get(inner).unwrap_or_else(|| placeholder.to_string())
            };

            result = format!("{}{}{}", &result[..start], resolved, &result[end..]);
        }

        result
    }

    /// 获取属性
    /// Get property
    pub fn get_property(&self, key: &str) -> Option<String> {
        self.loader.get(key)
    }

    /// 获取属性或默认值
    /// Get property or default
    pub fn get_property_or(&self, key: &str, default: &str) -> String {
        self.loader.get_or(key, default)
    }

    /// 获取必需的属性
    /// Get required property
    pub fn get_required_property(&self, key: &str) -> Result<String> {
        self.loader.get(key).ok_or_else(|| {
            anyhow::anyhow!("Required property '{}' not found", key)
        })
    }
}

// ============================================================================
// 配置属性宏 / Configuration Properties Macros
// ============================================================================

/// 配置属性字段元数据
/// Configuration property field metadata
#[derive(Debug, Clone)]
pub struct PropertyMetadata {
    /// 字段名称
    pub name: String,

    /// 配置键
    pub key: String,

    /// 是否必需
    pub required: bool,

    /// 默认值
    pub default_value: Option<String>,
}

impl PropertyMetadata {
    /// 创建新的属性元数据
    pub fn new(name: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            key: key.into(),
            required: false,
            default_value: None,
        }
    }

    /// 设置为必需
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// 设置默认值
    pub fn default_value(mut self, value: impl Into<String>) -> Self {
        self.default_value = Some(value.into());
        self
    }
}

// ============================================================================
// 测试 / Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_metadata() {
        let meta = PropertyMetadata::new("port", "server.port")
            .required()
            .default_value("8080");

        assert_eq!(meta.name, "port");
        assert_eq!(meta.key, "server.port");
        assert!(meta.required);
        assert_eq!(meta.default_value, Some("8080".to_string()));
    }

    #[test]
    fn test_property_resolver_resolve() {
        let loader = Arc::new(ConfigurationLoader::new());
        loader.set("server.port".to_string(), "9090".to_string());

        let resolver = PropertyResolver::new(loader);

        // 简单替换
        assert_eq!(resolver.resolve("${server.port}"), "9090");

        // 带默认值的替换
        assert_eq!(resolver.resolve("${missing.key:8080}"), "8080");

        // 未找到且无默认值
        assert_eq!(resolver.resolve("${missing.key}"), "${missing.key}");
    }

    #[test]
    fn test_property_resolver_get() {
        let loader = Arc::new(ConfigurationLoader::new());
        loader.set("test.key".to_string(), "test.value".to_string());

        let resolver = PropertyResolver::new(loader);
        assert_eq!(resolver.get_property("test.key"), Some("test.value".to_string()));
        assert_eq!(resolver.get_property_or("test.key", "default"), "test.value");
        assert_eq!(resolver.get_property_or("missing", "default"), "default");
    }
}
