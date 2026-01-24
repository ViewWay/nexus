//! Path parameters module
//! 路径参数模块
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - @PathVariable annotation
//! - PathVariable from URI template

#![warn(missing_docs)]
#![warn(unreachable_pub)]

use serde::Deserialize;
use std::collections::HashMap;

/// Path parameter extractor
/// 路径参数提取器
///
/// This is equivalent to Spring's `@PathVariable` annotation.
/// 这等价于Spring的`@PathVariable`注解。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_router::Path;
/// use nexus_http::FromRequest;
///
/// #[nexus_macros::get("/users/:id")]
/// async fn get_user(Path(id): Path<u64>) -> String {
///     format!("User ID: {}", id)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Path<T>(pub T);

impl<T> Path<T> {
    /// Get the inner value
    /// 获取内部值
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Get a reference to the inner value
    /// 获取内部值的引用
    pub fn get(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for Path<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

/// Path deserialization helper
/// 路径反序列化助手
pub struct PathDeserializer<'a> {
    params: &'a HashMap<String, String>,
}

impl<'a> PathDeserializer<'a> {
    /// Create a new deserializer from path parameters
    /// 从路径参数创建新反序列化器
    pub fn new(params: &'a HashMap<String, String>) -> Self {
        Self { params }
    }

    /// Get a parameter value
    /// 获取参数值
    pub fn get(&self, key: &str) -> Option<&str> {
        self.params.get(key).map(|s| s.as_str())
    }

    /// Deserialize into type T
    /// 反序列化为类型T
    pub fn deserialize<T: Deserialize<'a>>(&self) -> Result<T, String> {
        // TODO: Implement proper deserialization
        // TODO: 实现正确的反序列化
        Err("Not implemented".to_string())
    }
}

/// Query parameter extractor
/// 查询参数提取器
///
/// This is equivalent to Spring's `@RequestParam` annotation.
/// 这等价于Spring的`@RequestParam`注解。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_router::Query;
/// use nexus_http::FromRequest;
///
/// #[nexus_macros::get("/search")]
/// async fn search(Query(query): Query<String>) -> String {
///     format!("Searching for: {}", query)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Query<T>(pub T);

impl<T> Query<T> {
    /// Get the inner value
    /// 获取内部值
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Get a reference to the inner value
    /// 获取内部值的引用
    pub fn get(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for Query<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

/// Form data extractor
/// 表单数据提取器
///
/// This is equivalent to Spring's `@ModelAttribute` annotation.
/// 这等价于Spring的`@ModelAttribute`注解。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_router::Form;
/// use nexus_http::FromRequest;
///
/// #[nexus_macros::post("/login")]
/// async fn login(Form(form): Form<LoginForm>) -> String {
///     format!("Logged in as {}", form.username)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Form<T>(pub T);

impl<T> Form<T> {
    /// Get the inner value
    /// 获取内部值
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Get a reference to the inner value
    /// 获取内部值的引用
    pub fn get(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for Form<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}
