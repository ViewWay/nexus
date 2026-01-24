//! Compression middleware module
//! 压缩中间件模块
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - server.compression.enabled
//! - GzipFilter, DeflateFilter
//! - Content-Encoding

#![warn(missing_docs)]
#![warn(unreachable_pub)]

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::{Request, Response, Result, Middleware, Next};

/// Compression type
/// 压缩类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    /// Gzip compression
    /// Gzip压缩
    Gzip,

    /// Deflate compression
    /// Deflate压缩
    Deflate,

    /// Brotli compression
    /// Brotli压缩
    Brotli,

    /// No compression
    /// 不压缩
    None,
}

/// Compression middleware
/// 压缩中间件
///
/// Equivalent to Spring Boot's:
/// - `server.compression.enabled=true`
/// - `GzipFilter`, `DeflateFilter`
/// - `ContentEncodingFilter`
///
/// 这等价于Spring Boot的：
/// - `server.compression.enabled=true`
/// - `GzipFilter`, `DeflateFilter`
/// - `ContentEncodingFilter`
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_router::Router;
/// use nexus_middleware::CompressionMiddleware;
/// use std::sync::Arc;
///
/// let compression = Arc::new(CompressionMiddleware::new());
/// let router = Router::new()
///     .middleware(compression)
///     .get("/", handler);
/// ```
#[derive(Clone)]
pub struct CompressionMiddleware {
    /// Minimum response size to compress (bytes)
    /// 压缩的最小响应大小（字节）
    pub min_response_size: usize,

    /// Compression types to offer
    /// 提供的压缩类型
    pub compression_types: Vec<CompressionType>,

    /// MIME types to compress
    /// 要压缩的MIME类型
    pub mime_types: Vec<String>,

    /// Excluded user agents
    /// 排除的用户代理
    pub excluded_agents: Vec<String>,
}

impl CompressionMiddleware {
    /// Create a new compression middleware
    /// 创建新的压缩中间件
    pub fn new() -> Self {
        Self {
            min_response_size: 1024,
            compression_types: vec![CompressionType::Gzip, CompressionType::Deflate],
            mime_types: vec![
                "text/html".to_string(),
                "text/plain".to_string(),
                "text/css".to_string(),
                "text/javascript".to_string(),
                "application/javascript".to_string(),
                "application/json".to_string(),
                "application/xml".to_string(),
            ],
            excluded_agents: Vec::new(),
        }
    }

    /// Set minimum response size
    /// 设置最小响应大小
    pub fn min_size(mut self, size: usize) -> Self {
        self.min_response_size = size;
        self
    }

    /// Add a compression type
    /// 添加压缩类型
    pub fn add_compression(mut self, compression: CompressionType) -> Self {
        self.compression_types.push(compression);
        self
    }

    /// Add MIME types to compress
    /// 添加要压缩的MIME类型
    pub fn mime_type(mut self, mime: impl Into<String>) -> Self {
        self.mime_types.push(mime.into());
        self
    }

    /// Add excluded user agent
    /// 添加排除的用户代理
    pub fn exclude_agent(mut self, agent: impl Into<String>) -> Self {
        self.excluded_agents.push(agent.into());
        self
    }
}

impl Default for CompressionMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Middleware<S> for CompressionMiddleware
where
    S: Send + Sync + 'static,
{
    fn call(
        &self,
        req: Request,
        state: Arc<S>,
        next: Next<S>,
    ) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>> {
        let _min_size = self.min_response_size;
        let _compression_types = self.compression_types.clone();
        let _mime_types = self.mime_types.clone();

        Box::pin(async move {
            // Check Accept-Encoding header
            // 检查 Accept-Encoding header
            let accept_encoding = req.header("Accept-Encoding").unwrap_or("").to_string();

            // TODO: Check if client supports compression
            // TODO: 检查客户端是否支持压缩
            if accept_encoding.is_empty() {
                return next.call(req, state).await;
            }

            // Call next middleware/handler
            // 调用下一个中间件/处理程序
            let response = next.call(req, state).await?;

            // TODO: Compress response if applicable
            // TODO: 如果适用，压缩响应
            tracing::debug!("Compression middleware applied (encoding: {})", accept_encoding);

            Ok(response)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_creation() {
        let compression = CompressionMiddleware::new();
        assert_eq!(compression.min_response_size, 1024);
        assert_eq!(compression.compression_types.len(), 2);
    }

    #[test]
    fn test_compression_builder() {
        let compression = CompressionMiddleware::new()
            .min_size(2048)
            .mime_type("text/html");

        assert_eq!(compression.min_response_size, 2048);
        assert!(compression.mime_types.contains(&"text/html".to_string()));
    }
}
