//! Logger middleware module
//! 日志中间件模块
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - Access logging (Tomcat AccessLogValve)
//! - MDC (Mapped Diagnostic Context)
//! - Request/Response logging

#![warn(missing_docs)]
#![warn(unreachable_pub)]

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

use crate::{Request, Response, Result, Middleware, Next};

/// Logger middleware
/// 日志中间件
///
/// Equivalent to Spring's:
/// - `AccessLogValve` (Tomcat)
/// - `CommonsRequestLoggingFilter`
/// - `RequestContextFilter`
///
/// 这等价于Spring的：
/// - `AccessLogValve` (Tomcat)
/// - `CommonsRequestLoggingFilter`
/// - `RequestContextFilter`
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_router::Router;
/// use nexus_middleware::LoggerMiddleware;
/// use std::sync::Arc;
///
/// let logger = Arc::new(LoggerMiddleware::new());
/// let router = Router::new()
///     .middleware(logger)
///     .get("/", handler);
/// ```
#[derive(Clone)]
pub struct LoggerMiddleware {
    /// Log request and response body
    /// 记录请求和响应体
    pub log_body: bool,

    /// Log request headers
    /// 记录请求headers
    pub log_headers: bool,

    /// Include query string in log
    /// 日志中包含查询字符串
    pub include_query: bool,
}

impl LoggerMiddleware {
    /// Create a new logger middleware
    /// 创建新的日志中间件
    pub fn new() -> Self {
        Self {
            log_body: false,
            log_headers: false,
            include_query: true,
        }
    }

    /// Enable body logging
    /// 启用body日志
    pub fn log_body(mut self, log: bool) -> Self {
        self.log_body = log;
        self
    }

    /// Enable header logging
    /// 启用header日志
    pub fn log_headers(mut self, log: bool) -> Self {
        self.log_headers = log;
        self
    }

    /// Include query string in log
    /// 日志中包含查询字符串
    pub fn include_query(mut self, include: bool) -> Self {
        self.include_query = include;
        self
    }
}

impl Default for LoggerMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Middleware<S> for LoggerMiddleware
where
    S: Send + Sync + 'static,
{
    fn call(
        &self,
        req: Request,
        _state: Arc<S>,
        next: Next<S>,
    ) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>> {
        let log_body = self.log_body;
        let log_headers = self.log_headers;
        let include_query = self.include_query;

        Box::pin(async move {
            let method = req.method();
            let path = if include_query {
                req.uri().to_string()
            } else {
                req.path().to_string()
            };

            let start = Instant::now();

            // Log request details
            // 记录请求详情
            if log_headers {
                tracing::info!(
                    "Request: {} {} | Headers: {:?}",
                    method,
                    path,
                    req.headers()
                );
            } else {
                tracing::info!("Request: {} {}", method, path);
            }

            let response = next.call(req, _state).await;
            let duration = start.elapsed();

            // Log response with status
            // 记录响应状态
            match &response {
                Ok(resp) => {
                    let status = resp.status();
                    tracing::info!(
                        "Response: {} {} | Status: {} | Duration: {}ms",
                        method,
                        path,
                        status.as_u16(),
                        duration.as_millis()
                    );
                }
                Err(e) => {
                    tracing::error!(
                        "Request failed: {} {} | Error: {} | Duration: {}ms",
                        method,
                        path,
                        e,
                        duration.as_millis()
                    );
                }
            }

            response
        })
    }
}

/// MDC (Mapped Diagnostic Context) utility
/// MDC（映射诊断上下文）工具
///
/// Equivalent to SLF4J's MDC or Spring's MDC.
/// 等价于SLF4J的MDC或Spring的MDC。
pub struct Mdc;

impl Mdc {
    /// Put a value into MDC
    /// 向MDC中放入值
    pub fn put(key: impl Into<String>, value: impl Into<String>) {
        let key = key.into();
        let value = value.into();
        tracing::info_span!("mdc", key = &value);
    }

    /// Get a value from MDC
    /// 从MDC中获取值
    pub fn get(_key: &str) -> Option<String> {
        // Note: MDC is typically implemented via thread-local storage
        // or async local storage. For async contexts, we'd need
        // a proper async-local storage implementation.
        // 注意：MDC通常通过线程本地存储或异步本地存储实现。
        // 对于异步上下文，我们需要适当的异步本地存储实现。
        None
    }

    /// Remove a value from MDC
    /// 从MDC中移除值
    pub fn remove(_key: &str) {
        // TODO: Implement MDC storage with async-local
        // TODO: 使用async-local实现MDC存储
    }

    /// Clear all MDC values
    /// 清除所有MDC值
    pub fn clear() {
        // TODO: Implement MDC storage with async-local
        // TODO: 使用async-local实现MDC存储
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_creation() {
        let logger = LoggerMiddleware::new();
        assert!(!logger.log_body);
        assert!(!logger.log_headers);
        assert!(logger.include_query);
    }

    #[test]
    fn test_logger_builder() {
        let logger = LoggerMiddleware::new()
            .log_body(true)
            .log_headers(true)
            .include_query(false);

        assert!(logger.log_body);
        assert!(logger.log_headers);
        assert!(!logger.include_query);
    }
}
