//! Response module
//! 响应模块
//!
//! # Overview / 概述
//!
//! This module provides HTTP response types and utilities.
//! 本模块提供HTTP响应类型和工具。

use http::{StatusCode, HeaderMap};
use bytes::Bytes;

/// HTTP response body
/// HTTP响应体
pub type Body = Bytes;

/// HTTP response
/// HTTP响应
///
/// Represents an HTTP response with status, headers, and body.
/// 表示包含状态、头部和主体的HTTP响应。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_response::{Response, ResponseBuilder};
///
/// let response = Response::builder()
///     .status(200)
///     .header("Content-Type", "application/json")
///     .body(r#"{"message": "Hello"}"#)
///     .unwrap();
/// ```
pub struct Response {
    /// Status code / 状态码
    status: StatusCode,
    /// Response headers / 响应头
    headers: HeaderMap,
    /// Response body / 响应体
    body: Body,
}

impl Response {
    /// Create a new response with default values
    /// 使用默认值创建新响应
    ///
    /// Default: 200 OK, empty headers, empty body
    /// 默认：200 OK，空头部，空主体
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Body::new(),
        }
    }

    /// Create a response builder
    /// 创建响应构建器
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::new()
    }

    /// Get the status code
    /// 获取状态码
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get the response headers
    /// 获取响应头
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    /// Get the response body
    /// 获取响应体
    pub fn body(&self) -> &Body {
        &self.body
    }

    /// Get a mutable reference to the headers
    /// 获取头的可变引用
    pub fn headers_mut(&mut self) -> &mut HeaderMap {
        &mut self.headers
    }

    /// Set the status code
    /// 设置状态码
    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }

    /// Set the body
    /// 设置主体
    pub fn set_body(&mut self, body: impl Into<Body>) {
        self.body = body.into();
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new()
    }
}

impl From<http::Response<Body>> for Response {
    fn from(resp: http::Response<Body>) -> Self {
        let (parts, body) = resp.into_parts();
        Self {
            status: parts.status,
            headers: parts.headers,
            body,
        }
    }
}

impl From<Response> for http::Response<Body> {
    fn from(resp: Response) -> Self {
        let mut builder = http::Response::builder()
            .status(resp.status);

        // Swap headers
        // 交换头部
        for (name, value) in resp.headers.into_iter() {
            builder = builder.header(name, value);
        }

        builder.body(resp.body).unwrap()
    }
}

/// Response builder
/// 响应构建器
///
/// Provides a fluent API for constructing HTTP responses.
/// 提供用于构建HTTP响应的流畅API。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_response::{Response, ResponseBuilder};
///
/// let response = Response::builder()
///     .status(StatusCode::CREATED)
///     .header("Location", "/resource/123")
///     .body("Resource created")
///     .unwrap();
/// ```
pub struct ResponseBuilder {
    status: Option<StatusCode>,
    headers: HeaderMap,
}

impl ResponseBuilder {
    /// Create a new response builder
    /// 创建新的响应构建器
    pub fn new() -> Self {
        Self {
            status: None,
            headers: HeaderMap::new(),
        }
    }

    /// Set the status code
    /// 设置状态码
    pub fn status(self, status: impl Into<StatusCode>) -> Self {
        Self {
            status: Some(status.into()),
            ..self
        }
    }

    /// Set the status code using StatusCode enum
    /// 使用StatusCode枚举设置状态码
    pub fn status_code(self, status: StatusCode) -> Self {
        Self {
            status: Some(status),
            ..self
        }
    }

    /// Add a header
    /// 添加头部
    pub fn header(self, name: impl Into<http::HeaderName>, value: impl Into<http::HeaderValue>) -> Self {
        let mut headers = self.headers;
        if let Err(_) = headers.append(name, value) {
            // Ignore invalid headers
            // 忽略无效头部
        }
        Self { headers, ..self }
    }

    /// Set the body
    /// 设置主体
    pub fn body(self, body: impl Into<Body>) -> Result<Response, http::Error> {
        let status = self.status.unwrap_or(StatusCode::OK);
        Ok(Response {
            status,
            headers: self.headers,
            body: body.into(),
        })
    }

    /// Build with empty body
    /// 使用空主体构建
    pub fn finish(self) -> Result<Response, http::Error> {
        self.body(Body::new())
    }
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for types that can be converted to HTTP responses
/// 可转换为HTTP响应的类型的trait
///
/// Types implementing this trait can be used as return types from handlers.
/// 实现此trait的类型可以用作处理器的返回类型。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_response::IntoResponse;
///
/// struct User {
///     name: String,
/// }
///
/// impl IntoResponse for User {
///     fn into_response(self) -> Response {
///         Response::builder()
///             .body(format!(r#"{{"name":"{}"}}"#, self.name))
///             .unwrap()
///     }
/// }
/// ```
pub trait IntoResponse {
    /// Convert self into a response
    /// 将self转换为响应
    fn into_response(self) -> Response;
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response {
        Response::builder()
            .header("content-type", "text/plain; charset=utf-8")
            .body(self)
            .unwrap()
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response::builder()
            .header("content-type", "text/plain; charset=utf-8")
            .body(self)
            .unwrap()
    }
}

impl IntoResponse for &'static [u8] {
    fn into_response(self) -> Response {
        Response::builder()
            .header("content-type", "application/octet-stream")
            .body(self)
            .unwrap()
    }
}

impl IntoResponse for Vec<u8> {
    fn into_response(self) -> Response {
        Response::builder()
            .body(self)
            .unwrap()
    }
}

impl IntoResponse for Bytes {
    fn into_response(self) -> Response {
        Response::builder()
            .body(self)
            .unwrap()
    }
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response {
        Response::builder()
            .status(self)
            .finish()
            .unwrap()
    }
}

/// Create a response with the given status code
/// 使用给定状态码创建响应
pub fn status(status: StatusCode) -> Response {
    Response::builder()
        .status(status)
        .finish()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_new() {
        let resp = Response::new();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.headers().is_empty());
        assert!(resp.body().is_empty());
    }

    #[test]
    fn test_response_builder() {
        let resp = Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header("x-custom", "test")
            .body("Not found")
            .unwrap();

        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
        assert_eq!(resp.headers().get("x-custom").unwrap(), "test");
        assert_eq!(resp.body(), "Not found");
    }

    #[test]
    fn test_into_response_str() {
        let resp = "Hello, World!".into_response();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.body(), "Hello, World!");
    }

    #[test]
    fn test_status_function() {
        let resp = status(StatusCode::CREATED);
        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}
