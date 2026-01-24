//! HTTP Response type
//! HTTP 响应类型
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - ResponseEntity, @ResponseBody, @ResponseStatus

#![warn(missing_docs)]
#![warn(unreachable_pub)]

use super::{
    body::Body,
    status::StatusCode,
    error::{Error, Result},
};
use std::collections::HashMap;

/// HTTP Response
/// HTTP 响应
#[derive(Debug, Clone)]
pub struct Response {
    status: StatusCode,
    headers: HashMap<String, String>,
    body: Body,
}

impl Response {
    /// Create a new response
    /// 创建新响应
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: HashMap::new(),
            body: Body::empty(),
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

    /// Get a header value
    /// 获取header值
    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).map(|s| s.as_str())
    }

    /// Get all headers
    /// 获取所有headers
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    /// Get the response body
    /// 获取响应body
    pub fn body(&self) -> &Body {
        &self.body
    }

    /// Convert self into the body
    /// 将self转换为body
    pub fn into_body(self) -> Body {
        self.body
    }
}

/// Response builder
/// 响应构建器
#[derive(Debug, Default)]
pub struct ResponseBuilder {
    status: Option<StatusCode>,
    headers: HashMap<String, String>,
    body: Option<Body>,
}

impl ResponseBuilder {
    /// Create a new builder
    /// 创建新构建器
    pub fn new() -> Self {
        Self {
            status: None,
            headers: HashMap::new(),
            body: None,
        }
    }

    /// Set the status code
    /// 设置状态码
    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = Some(status);
        self
    }

    /// Add a header
    /// 添加header
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Set the body and build the response
    /// 设置body并构建响应
    pub fn body(mut self, body: Body) -> Result<Response> {
        self.body = Some(body);
        Ok(Response {
            status: self.status.unwrap_or_default(),
            headers: self.headers,
            body: self.body.unwrap_or_default(),
        })
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new(StatusCode::OK)
    }
}
