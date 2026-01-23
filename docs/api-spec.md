# Nexus Web Framework - API Specification / API接口规范

## Version / 版本

**Version**: 0.1.0-alpha
**Date**: 2025-01-23
**Status**: Draft / 草稿

---

## Table of Contents / 目录

1. [Core APIs / 核心API](#1-core-apis-核心api)
2. [HTTP APIs / HTTP API](#2-http-apis-http-api)
3. [Middleware APIs / 中间件API](#3-middleware-apis-中间件api)
4. [Resilience APIs / 弹性API](#4-resilience-apis-弹性api)
5. [Observability APIs / 可观测性API](#5-observability-apis-可观测性api)
6. [Web3 APIs / Web3 API](#6-web3-apis-web3-api)
7. [Runtime APIs / 运行时API](#7-runtime-apis-运行时api)

---

## 1. Core APIs / 核心API

### 1.1 Application / 应用

```rust
/// Main application builder / 主应用构建器
///
/// # Example / 示例
/// ```rust,no_run
/// use nexus::prelude::*;
/// use nexus::Router;
///
/// #[tokio::main]
/// async fn main() {
///     let app = Router::new()
///         .get("/", || async { "Hello, World!" })
///         .with_state(AppState { db: Arc::new(Database::new()) });
///
///     Server::bind("0.0.0.0:3000")
///         .serve(app)
///         .await
///         .unwrap();
/// }
/// ```
pub struct Application<S = ()> {
    /// Router / 路由器
    router: Router<S>,

    /// Application state / 应用状态
    state: Arc<S>,

    /// Server configuration / 服务器配置
    config: ServerConfig,
}

impl<S> Application<S> {
    /// Create a new application / 创建新应用
    pub fn new() -> Self
    where
        S: Default;

    /// Set the application state / 设置应用状态
    pub fn with_state(mut self, state: S) -> Application<S> {
        Application {
            router: self.router.into_state(state),
            state: Arc::new(state),
            config: self.config,
        }
    }

    /// Run the server / 运行服务器
    pub async fn run(self, addr: impl Into<ServerAddr>) -> Result<(), Error>;

    /// Run until shutdown signal / 运行直到关闭信号
    pub async fn run_until_shutdown(self, addr: impl Into<ServerAddr>) -> Result<(), Error>;
}

impl Default for Application {
    fn default() -> Self {
        Self::new()
    }
}
```

### 1.2 Router / 路由器

```rust
/// HTTP router with composable middleware and extractors
/// 可组合中间件和提取器的HTTP路由器
///
/// # Route Patterns / 路由模式
///
/// - `/` - Root path / 根路径
/// - `/users` - Static path / 静态路径
/// - `/users/:id` - Path parameter / 路径参数
/// - `/users/:id/posts/:post_id` - Multiple parameters / 多个参数
/// - `/files/*path` - Wildcard match / 通配符匹配
pub struct Router<S = ()> {
    // Private fields / 私有字段
    routes: Arc<RouteTable<S>>,
    middleware: Arc<Vec<Arc<dyn Middleware>>>>,
}

impl<S> Router<S> {
    /// Create a new router / 创建新路由器
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a GET route / 添加GET路由
    ///
    /// # Arguments / 参数
    /// * `path`: Route path with optional parameters / 带可选参数的路由路径
    /// * `handler`: Request handler / 请求处理器
    pub fn get<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T>,
        T: FromRequest + 'static;

    /// Add a POST route / 添加POST路由
    pub fn post<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T>,
        T: FromRequest + 'static;

    /// Add a PUT route / 添加PUT路由
    pub fn put<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T>,
        T: FromRequest + 'static;

    /// Add a DELETE route / 添加DELETE路由
    pub fn delete<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T>,
        T: FromRequest + 'static;

    /// Add a PATCH route / 添加PATCH路由
    pub fn patch<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T>,
        T: FromRequest + 'static;

    /// Add a route with any HTTP method / 添加任意HTTP方法的路由
    pub fn route<H, T>(self, method: Method, path: &str, handler: H) -> Self
    where
        H: Handler<T>,
        T: FromRequest + 'static;

    /// Add middleware to all routes / 为所有路由添加中间件
    pub fn layer<M>(self, middleware: M) -> Self
    where
        M: Middleware<S>;

    /// Add middleware to a specific route / 为特定路由添加中间件
    pub fn route_layer<M>(self, path: &str, middleware: M) -> Self
    where
        M: Middleware<S>;

    /// Set application state / 设置应用状态
    pub fn with_state<T>(self, state: T) -> Router<T>;

    /// Merge another router / 合并另一个路由器
    ///
    /// # Example / 示例
    /// ```rust
    /// let api = Router::new()
    ///     .get("/users", list_users)
    ///     .post("/users", create_user);
    ///
    /// let app = Router::new()
    ///     .get("/", index)
    ///     .merge(api);
    /// ```
    pub fn merge(self, other: Router<S>) -> Self;

    /// Nest a router under a path / 将路由器嵌套在路径下
    ///
    /// # Example / 示例
    /// ```rust
    /// let api = Router::new()
    ///     .get("/users", list_users)
    ///     .get("/posts", list_posts);
    ///
    /// let app = Router::new()
    ///     .nest("/api", api);
    /// // Results in: /api/users, /api/posts
    /// ```
    pub fn nest(self, base: &str, router: Router<S>) -> Self;

    /// Add a fallback handler for unmatched routes / 添加未匹配路由的fallback处理器
    pub fn fallback<H, T>(self, handler: H) -> Self
    where
        H: Handler<T>,
        T: FromRequest + 'static;

    /// Add a WebSocket route / 添加WebSocket路由
    pub fn websocket<F>(self, path: &str, handler: F) -> Self
    where
        F: WebSocketHandler;
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
```

### 1.3 Handler Trait / Handler Trait

```rust
/// Trait for handling HTTP requests
/// 处理HTTP请求的trait
///
/// # Implemented For / 适用于
///
/// - Functions that match the handler signature / 匹配handler签名的函数
/// - Closures with matching signature / 匹配签名的闭包
/// - Types implementing `Handler` manually / 手动实现`Handler`的类型
pub trait Handler<T>: Clone + Send + Sync + 'static {
    /// Call the handler with extracted arguments
    /// 使用提取的参数调用handler
    fn call(&self, args: T) -> impl Future<Output = Response>;

    /// Convert to an `Arc<dyn Handler>` for type erasure
    /// 转换为`Arc<dyn Handler>`进行类型擦除
    fn into_arc_handler(self) -> Arc<dyn Handler<T>>
    where
        Self: Sized + 'static,
    {
        Arc::new(self)
    }
}

// Blanket implementation for functions / 函数的blanket实现
impl<F, Fut, Res, T> Handler<T> for F
where
    F: Fn(T) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = Res> + Send,
    Res: IntoResponse,
    T: FromRequest + 'static,
{
    fn call(&self, args: T) -> impl Future<Output = Response> {
        async move {
            self(args).await.into_response()
        }
    }
}
```

### 1.4 IntoResponse Trait / IntoResponse Trait

```rust
/// Trait for types that can be converted to HTTP responses
/// 可转换为HTTP响应的类型的trait
///
/// # Implemented For / 适用于
///
/// - `&'static str` → Plain text response / 纯文本响应
/// - `String` → Plain text response / 纯文本响应
/// - `Bytes` → Binary response / 二进制响应
/// - `Json<T>` → JSON response / JSON响应
/// - `Html<T>` → HTML response / HTML响应
/// - `StatusCode` → Status code only / 仅状态码
/// - `Result<T, E>` where `T: IntoResponse`, `E: IntoResponse`
pub trait IntoResponse {
    /// Convert self into a response / 将self转换为响应
    fn into_response(self) -> Response;
}

// Built-in implementations / 内置实现
impl IntoResponse for &'static str {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/plain; charset=utf-8")
            .body(self)
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/plain; charset=utf-8")
            .body(self)
    }
}

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        match serde_json::to_vec(&self.0) {
            Ok(body) => Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(body),
            Err(err) => {
                tracing::error!("Failed to serialize JSON: {:?}", err);
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Internal server error")
            }
        }
    }
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Ok(value) => value.into_response(),
            Err(err) => err.into_response(),
        }
    }
}
```

---

## 2. HTTP APIs / HTTP API

### 2.1 Request / 请求

```rust
/// HTTP request with context and extensions
/// 带上下文和扩展的HTTP请求
pub struct Request {
    /// HTTP method / HTTP方法
    pub method: Method,

    /// Request URI / 请求URI
    pub uri: Uri,

    /// HTTP version / HTTP版本
    pub version: Version,

    /// Request headers / 请求头
    pub headers: HeaderMap,

    /// Request body / 请求体
    pub body: Body,

    /// Extensions for custom data / 自定义数据扩展
    pub extensions: Extensions,

    /// Trace context for distributed tracing / 分布式追踪上下文
    pub trace_context: TraceContext,
}

impl Request {
    /// Get request path without query string
    /// 获取不带查询字符串的请求路径
    ///
    /// # Example / 示例
    /// ```rust
    /// // For "/users?id=123" returns "/users"
    /// let path = request.path();
    /// ```
    pub fn path(&self) -> &str;

    /// Get query string / 获取查询字符串
    pub fn query_string(&self) -> Option<&str>;

    /// Get remote address / 获取远程地址
    pub fn remote_addr(&self) -> Option<SocketAddr>;

    /// Get request ID (generated if not set)
    /// 获取请求ID（如未设置则生成）
    pub fn request_id(&self) -> &RequestId;

    /// Check if request is HTTPS / 检查请求是否为HTTPS
    pub fn is_secure(&self) -> bool;

    /// Get content type / 获取内容类型
    pub fn content_type(&self) -> Option<Mime>;

    /// Get content length / 获取内容长度
    pub fn content_length(&self) -> Option<u64>;

    /// Check if request accepts given content type
    /// 检查请求是否接受给定内容类型
    pub fn accepts(&self, content_type: &str) -> bool;

    /// Get user agent / 获取用户代理
    pub fn user_agent(&self) -> Option<&str>;
}
```

### 2.2 Response / 响应

```rust
/// HTTP response builder
/// HTTP响应构建器
pub struct Response {
    /// Status code / 状态码
    pub status: StatusCode,

    /// Response headers / 响应头
    pub headers: HeaderMap,

    /// Response body / 响应体
    pub body: Body,

    /// Response version / 响应版本
    pub version: Version,
}

impl Response {
    /// Create a new response builder / 创建新响应构建器
    pub fn builder() -> ResponseBuilder {
        ResponseBuilder::new()
    }

    /// Create a response with status code / 创建带状态码的响应
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            ..Default::default()
        }
    }

    /// Set response header / 设置响应头
    pub fn header(mut self, name: impl IntoHeaderName, value: impl IntoHeaderValue) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }

    /// Set response body / 设置响应体
    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.body = body.into();
        self
    }

    /// Set content type / 设置内容类型
    pub fn content_type(mut self, content_type: impl Into<Mime>) -> Self {
        self.headers.insert("content-type", content_type.into().to_string());
        self
    }
}

/// Response builder for fluent construction
/// 流式构建的响应构建器
pub struct ResponseBuilder {
    response: Response,
}

impl ResponseBuilder {
    /// Create a new builder / 创建新构建器
    pub fn new() -> Self {
        Self {
            response: Response::default(),
        }
    }

    /// Set status code / 设置状态码
    pub fn status(mut self, status: StatusCode) -> Self {
        self.response.status = status;
        self
    }

    /// Set header / 设置头
    pub fn header(mut self, name: impl IntoHeaderName, value: impl IntoHeaderValue) -> Self {
        self.response.headers.insert(name.into(), value.into());
        self
    }

    /// Set body / 设置体
    pub fn body(mut self, body: impl Into<Body>) -> Self {
        self.response.body = body.into();
        self
    }

    /// Build the response / 构建响应
    pub fn build(self) -> Response {
        self.response
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            status: StatusCode::OK,
            headers: HeaderMap::new(),
            body: Body::empty(),
            version: Version::HTTP_1_1,
        }
    }
}
```

### 2.3 Body / 请求体/响应体

```rust
/// Streaming or buffered body
/// 流式或缓冲体
pub struct Body {
    inner: BodyInner,
}

enum BodyInner {
    Empty,
    Bytes(Bytes),
    Stream( Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send>> ),
}

impl Body {
    /// Create an empty body / 创建空体
    pub fn empty() -> Self {
        Self { inner: BodyInner::Empty }
    }

    /// Create a body from bytes / 从字节创建体
    pub fn from_bytes(bytes: Bytes) -> Self {
        Self { inner: BodyInner::Bytes(bytes) }
    }

    /// Create a body from a string / 从字符串创建体
    pub fn from_string(s: impl Into<String>) -> Self {
        Self::from_bytes(Bytes::from(s.into()))
    }

    /// Create a streaming body / 创建流式体
    pub fn from_stream<S>(stream: S) -> Self
    where
        S: Stream<Item = Result<Bytes, Error>> + Send + 'static;

    /// Collect the entire body into bytes / 将整个体收集为字节
    pub async fn collect(self) -> Result<Bytes, Error>;

    /// Get body length if known / 如已知获取体长度
    pub fn len(&self) -> Option<usize>;

    /// Check if body is empty / 检查体是否为空
    pub fn is_empty(&self) -> bool;
}

impl From<Vec<u8>> for Body {
    fn from(bytes: Vec<u8>) -> Self {
        Self::from_bytes(Bytes::from(bytes))
    }
}

impl From<&'static str> for Body {
    fn from(s: &'static str) -> Self {
        Self::from_bytes(Bytes::from(s))
    }
}

impl From<String> for Body {
    fn from(s: String) -> Self {
        Self::from_bytes(Bytes::from(s))
    }
}
```

---

## 3. Middleware APIs / 中间件API

### 3.1 Middleware Trait / Middleware Trait

```rust
/// Middleware for processing requests and responses
/// 处理请求和响应的中间件
///
/// # Example / 示例
/// ```rust
/// use nexus::prelude::*;
///
/// struct LoggingMiddleware;
///
/// impl<S> Middleware<S> for LoggingMiddleware {
///     type Output = Response;
///
///     fn call(&self, req: Request, next: Next<S>) -> Self::Output {
///         let start = Instant::now();
///         let method = req.method().clone();
///         let path = req.path().to_string();
///
///         let response = next.run(req).await;
///
///         let duration = start.elapsed();
///         tracing::info!(
///             method = %method,
///             path = %path,
///             status = %response.status(),
///             duration_ms = duration.as_millis(),
///             "Request completed"
///         );
///
///         response
///     }
/// }
/// ```
pub trait Middleware<S>: Clone + Send + Sync + 'static {
    /// The output type / 输出类型
    type Output;

    /// Process the request / 处理请求
    fn call(&self, req: Request, next: Next<S>) -> Self::Output;
}

/// Simplified middleware for async functions
/// 异步函数的简化中间件
pub trait AsyncMiddleware<S>: Clone + Send + Sync + 'static {
    /// Process the request asynchronously / 异步处理请求
    async fn call(&self, req: Request, next: Next<S>) -> Response;
}

impl<S, M> Middleware<S> for M
where
    M: AsyncMiddleware<S>,
{
    type Output = Response;

    fn call(&self, req: Request, next: Next<S>) -> Self::Output {
        self.call(req, next)
    }
}
```

### 3.2 Next / Next

```rust
/// The remainder of the middleware chain
/// 中间件链的剩余部分
pub struct Next<S> {
    /// Next middleware or handler in chain
    /// 链中的下一个中间件或handler
    next: Arc<dyn Middleware<S>>,
}

impl<S> Next<S> {
    /// Run the next middleware/handler in the chain
    /// 运行链中的下一个中间件/handler
    pub async fn run(self, req: Request) -> Response {
        self.next.call(req, self).await
    }
}
```

### 3.3 Built-in Middleware / 内置中间件

```rust
/// CORS middleware configuration / CORS中间件配置
pub struct CorsConfig {
    /// Allowed origins / 允许的来源
    pub allowed_origins: OriginSetting,

    /// Allowed methods / 允许的方法
    pub allowed_methods: Vec<Method>,

    /// Allowed headers / 允许的头
    pub allowed_headers: Vec<String>,

    /// Exposed headers / 暴露的头
    pub exposed_headers: Vec<String>,

    /// Allow credentials / 允许凭证
    pub allow_credentials: bool,

    /// Max age for preflight / 预检最大年龄
    pub max_age: Option<Duration>,
}

pub enum OriginSetting {
    /// Allow all origins / 允许所有来源
    Any,

    /// Allow specific origins / 允许特定来源
    Specific(Vec<String>),

    /// Allow origins matching pattern / 允许匹配模式的来源
    Pattern(String),
}

/// CORS middleware / CORS中间件
pub struct CorsLayer {
    config: CorsConfig,
}

impl CorsLayer {
    /// Create a new CORS middleware / 创建新的CORS中间件
    pub fn new(config: CorsConfig) -> Self {
        Self { config }
    }

    /// Allow all origins / 允许所有来源
    pub fn any() -> Self {
        Self::new(CorsConfig {
            allowed_origins: OriginSetting::Any,
            allowed_methods: vec![Method::GET, Method::POST, Method::PUT, Method::DELETE],
            allowed_headers: vec!["*".to_string()],
            exposed_headers: vec![],
            allow_credentials: false,
            max_age: None,
        })
    }
}

/// Compression middleware / 压缩中间件
pub struct CompressionLayer {
    /// Compression level / 压缩级别
    level: CompressionLevel,

    /// Minimum size to compress / 压缩最小大小
    min_size: usize,

    /// Enabled algorithms / 启用的算法
    algorithms: Vec<CompressionAlgorithm>,
}

#[derive(Clone, Copy)]
pub enum CompressionLevel {
    Fast,
    Default,
    Best,
}

pub enum CompressionAlgorithm {
    Gzip,
    Deflate,
    Brotli,
    Zstd,
}

impl CompressionLayer {
    /// Create a new compression middleware / 创建新的压缩中间件
    pub fn new() -> Self {
        Self {
            level: CompressionLevel::Default,
            min_size: 1024,
            algorithms: vec![CompressionAlgorithm::Gzip],
        }
    }

    /// Set compression level / 设置压缩级别
    pub fn level(mut self, level: CompressionLevel) -> Self {
        self.level = level;
        self
    }

    /// Set minimum size to compress / 设置压缩最小大小
    pub fn min_size(mut self, size: usize) -> Self {
        self.min_size = size;
        self
    }
}

impl Default for CompressionLayer {
    fn default() -> Self {
        Self::new()
    }
}
```

---

## 4. Resilience APIs / 弹性API

### 4.1 Circuit Breaker / 熔断器

```rust
/// Circuit breaker for fault tolerance
/// 容错熔断器
///
/// # States / 状态
///
/// - **Closed**: Requests pass through normally / 请求正常通过
/// - **Open**: Requests fail fast / 请求快速失败
/// - **HalfOpen**: Testing if service has recovered / 测试服务是否恢复
///
/// # Example / 示例
/// ```rust
/// use nexus::prelude::*;
/// use nexus::resilience::CircuitBreaker;
///
/// let breaker = CircuitBreaker::new("api", CircuitBreakerConfig {
///     error_threshold: 0.5,      // 50% error rate / 50%错误率
///     min_requests: 10,          // After 10 requests / 10次请求后
///     open_duration: Duration::from_secs(30),
///     success_threshold: 2,      // 2 successes to close / 2次成功关闭
/// });
///
/// // Use with HTTP client / 与HTTP客户端一起使用
/// let response = breaker.call(|| {
///     http_client.get("https://api.example.com").send()
/// }).await?;
/// ```
pub struct CircuitBreaker {
    /// Circuit name for identification / 电路名称标识
    name: String,

    /// Circuit configuration / 电路配置
    config: CircuitBreakerConfig,

    /// Current state / 当前状态
    state: Arc<RwLock<CircuitBreakerState>>,

    /// Metrics / 指标
    metrics: Arc<CircuitBreakerMetrics>,
}

/// Circuit breaker configuration / 熔断器配置
#[derive(Clone, Debug)]
pub struct CircuitBreakerConfig {
    /// Error threshold (0.0 - 1.0) to trigger open state
    /// 触发开状态的错误阈值
    pub error_threshold: f64,

    /// Minimum requests before calculating error rate
    /// 计算错误率前的最小请求数
    pub min_requests: usize,

    /// Time to stay in open state before trying half-open
    /// 进入半开前保持在开状态的时间
    pub open_duration: Duration,

    /// Success count needed to close circuit
    /// 关闭电路所需的成功计数
    pub success_threshold: usize,

    /// Request timeout / 请求超时
    pub timeout: Option<Duration>,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            error_threshold: 0.5,
            min_requests: 10,
            open_duration: Duration::from_secs(30),
            success_threshold: 2,
            timeout: Some(Duration::from_secs(10)),
        }
    }
}

/// Circuit breaker state / 熔断器状态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation / 正常运行
    Closed,

    /// Circuit is open / 电路开路
    Open,

    /// Testing recovery / 测试恢复
    HalfOpen,
}

impl CircuitBreaker {
    /// Create a new circuit breaker / 创建新熔断器
    pub fn new(name: impl Into<String>, config: CircuitBreakerConfig) -> Self;

    /// Execute a operation with circuit breaker protection
    /// 使用熔断器保护执行操作
    pub async fn call<F, T, E>(&self, f: F) -> Result<T, CircuitBreakerError>
    where
        F: Future<Output = Result<T, E>> + Send,
        E: std::error::Error + Send + Sync + 'static;

    /// Get current state / 获取当前状态
    pub fn state(&self) -> CircuitState;

    /// Get metrics / 获取指标
    pub fn metrics(&self) -> &CircuitBreakerMetrics;

    /// Reset the circuit breaker to closed state
    /// 将熔断器重置为关闭状态
    pub fn reset(&self);

    /// Force open the circuit / 强制打开电路
    pub fn force_open(&self);

    /// Force close the circuit / 强制关闭电路
    pub fn force_close(&self);
}

/// Circuit breaker error / 熔断器错误
#[derive(Debug)]
pub enum CircuitBreakerError {
    /// Circuit is open / 电路开路
    Open,

    /// Operation timeout / 操作超时
    Timeout,

    /// Inner operation error / 内部操作错误
    Inner(Arc<dyn std::error::Error + Send + Sync>),
}
```

### 4.2 Rate Limiter / 限流器

```rust
/// Rate limiter for request throttling
/// 请求节流的限流器
///
/// # Example / 示例
/// ```rust
/// use nexus::prelude::*;
/// use nexus::resilience::RateLimiter;
///
/// // Token bucket: 100 requests per second / 令牌桶：每秒100请求
/// let limiter = RateLimiter::token_bucket(100, 1.0);
///
/// // In middleware / 在中间件中
/// if !limiter.check(key).await {
///     return Err(Error::too_many_requests());
/// }
/// ```
pub struct RateLimiter {
    /// Limiter type / 限流器类型
    inner: Box<dyn RateLimiterInner>,

    /// Storage backend / 存储后端
    storage: Arc<dyn RateLimitStorage>,
}

impl RateLimiter {
    /// Create a token bucket rate limiter
    /// 创建令牌桶限流器
    ///
    /// # Arguments / 参数
    /// * `capacity`: Maximum tokens / 最大令牌数
    /// * `refill_rate`: Tokens per second / 每秒令牌数
    pub fn token_bucket(capacity: u64, refill_rate: f64) -> Self;

    /// Create a leaky bucket rate limiter
    /// 创建漏桶限流器
    ///
    /// # Arguments / 参数
    /// * `capacity`: Bucket capacity / 桶容量
    /// * `leak_rate`: Leaks per second / 每秒泄漏数
    pub fn leaky_bucket(capacity: u64, leak_rate: f64) -> Self;

    /// Create a sliding window rate limiter
    /// 创建滑动窗口限流器
    ///
    /// # Arguments / 参数
    /// * `window_size`: Time window / 时间窗口
    /// * `max_requests`: Max requests in window / 窗口内最大请求数
    pub fn sliding_window(window_size: Duration, max_requests: u64) -> Self;

    /// Check if request is allowed / 检查请求是否允许
    ///
    /// # Arguments / 参数
    /// * `key`: Unique identifier (IP, user ID, etc.) / 唯一标识符
    ///
    /// # Returns / 返回
    /// * `Some(u64)` - Allowed, remaining tokens / 允许，剩余令牌
    /// * `None` - Rate limited / 限流
    pub async fn check(&self, key: &str) -> Option<u64>;

    /// Record a request (for manual rate limiting)
    /// 记录请求（用于手动限流）
    pub async fn record(&self, key: &str) -> Result<(), Error>;

    /// Reset the rate limit for a key
    /// 重置键的限流
    pub async fn reset(&self, key: &str) -> Result<(), Error>;
}

/// Storage backend for distributed rate limiting
/// 分布式限流的存储后端
#[async_trait]
pub trait RateLimitStorage: Send + Sync {
    /// Check if key is under limit / 检查键是否在限制下
    async fn check(&self, key: &str, limit: u64, window: Duration) -> Result<bool, Error>;

    /// Increment counter / 增加计数器
    async fn increment(&self, key: &str) -> Result<u64, Error>;

    /// Reset key / 重置键
    async fn reset(&self, key: &str) -> Result<(), Error>;
}
```

### 4.3 Retry / 重试

```rust
/// Retry with exponential backoff
/// 带指数退避的重试
///
/// # Example / 示例
/// ```rust
/// use nexus::resilience::{RetryPolicy, retry};
///
/// let policy = RetryPolicy::default()
///     .max_attempts(3)
///     .base_backoff(Duration::from_millis(100))
///     .max_backoff(Duration::from_secs(5))
///     .retryable(|err| err.is_transient());
///
/// let result = retry(|| async {
///     http_client.get("https://api.example.com").send()
/// }, &policy).await?;
/// ```
pub struct RetryPolicy {
    /// Maximum retry attempts / 最大重试次数
    pub max_attempts: usize,

    /// Base backoff duration / 基础退避时间
    pub base_backoff: Duration,

    /// Maximum backoff duration / 最大退避时间
    pub max_backoff: Duration,

    /// Jitter factor (0.0 - 1.0) / 抖动因子
    pub jitter_factor: f64,

    /// Predicate for retryable errors / 可重试错误判断
    pub retryable: Arc<dyn Fn(&Error) -> bool + Send + Sync>,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(5),
            jitter_factor: 0.1,
            retryable: Arc::new(|_| true),
        }
    }
}

impl RetryPolicy {
    /// Create a new retry policy / 创建新重试策略
    pub fn new() -> Self {
        Self::default()
    }

    /// Set maximum attempts / 设置最大尝试次数
    pub fn max_attempts(mut self, max: usize) -> Self {
        self.max_attempts = max;
        self
    }

    /// Set base backoff / 设置基础退避
    pub fn base_backoff(mut self, duration: Duration) -> Self {
        self.base_backoff = duration;
        self
    }

    /// Set maximum backoff / 设置最大退避
    pub fn max_backoff(mut self, duration: Duration) -> Self {
        self.max_backoff = duration;
        self
    }

    /// Set jitter factor / 设置抖动因子
    pub fn jitter(mut self, factor: f64) -> Self {
        self.jitter_factor = factor;
        self
    }

    /// Set retryable predicate / 设置可重试判断
    pub fn retryable<F>(mut self, predicate: F) -> Self
    where
        F: Fn(&Error) -> bool + Send + Sync + 'static,
    {
        self.retryable = Arc::new(predicate);
        self
    }

    /// Calculate next retry delay / 计算下次重试延迟
    pub fn next_delay(&self, attempt: usize) -> Duration {
        let exponential = self.base_backoff * 2_u32.pow(attempt as u32);
        let backoff = exponential.min(self.max_backoff);

        // Add jitter / 添加抖动
        let jitter_ms = (backoff.as_millis() as f64 * self.jitter_factor) as u64;
        let jitter = Duration::from_millis(
            rand::random::<u64>() % jitter_ms.max(1)
        );

        backoff.saturating_add(jitter)
    }
}

/// Retry an operation with the given policy
/// 使用给定策略重试操作
pub async fn retry<F, Fut, T, E>(op: F, policy: &RetryPolicy) -> Result<T, E>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
    E: std::error::Error,
{
    let mut last_error = None;

    for attempt in 0..=policy.max_attempts {
        match op().await {
            Ok(result) => return Ok(result),
            Err(err) => {
                last_error = Some(err);

                // Don't delay after last attempt / 最后一次尝试后不延迟
                if attempt < policy.max_attempts {
                    let delay = policy.next_delay(attempt);
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    Err(last_error.unwrap())
}
```

### 4.4 Timeout / 超时

```rust
/// Timeout middleware for request handlers
/// 请求处理器的超时中间件
///
/// # Example / 示例
/// ```rust
/// use nexus::prelude::*;
/// use std::time::Duration;
///
/// let app = Router::new()
///     .get("/api/search", search_handler)
///     .layer(TimeoutLayer::new(Duration::from_secs(5)));
/// ```
pub struct TimeoutLayer {
    /// Timeout duration / 超时时长
    timeout: Duration,
}

impl TimeoutLayer {
    /// Create a new timeout middleware / 创建新超时中间件
    pub fn new(timeout: Duration) -> Self {
        Self { timeout }
    }

    /// Timeout for slow requests / 对慢请求超时
    pub fn slow(timeout: Duration) -> Self {
        Self::new(timeout)
    }

    /// Timeout for fast requests / 对快请求超时
    pub fn fast(timeout: Duration) -> Self {
        Self::new(timeout)
    }
}

impl<S> Middleware<S> for TimeoutLayer {
    type Output = Response;

    fn call(&self, req: Request, next: Next<S>) -> Self::Output {
        async move {
            let timeout = tokio::time::timeout(self.timeout, next.run(req));

            match timeout.await {
                Ok(response) => response,
                Err(_) => {
                    tracing::warn!("Request timed out after {:?}", self.timeout);
                    Response::builder()
                        .status(StatusCode::REQUEST_TIMEOUT)
                        .body("Request timeout")
                }
            }
        }
        .boxed()
    }
}
```

---

## 5. Observability APIs / 可观测性API

### 5.1 Tracing / 追踪

```rust
/// Distributed tracing context
/// 分布式追踪上下文
///
/// # Example / 示例
/// ```rust
/// use nexus::observability::{TraceContext, Tracer};
///
/// // Create a new span / 创建新span
/// let tracer = Tracer::global();
/// let mut span = tracer.start_span("http_request");
/// span.set_attribute("http.method", "GET");
/// span.set_attribute("http.url", "/api/users");
///
/// // Execute code in span context / 在span上下文中执行代码
/// span.in_scope(|| async {
///     // Your code here / 你的代码
/// }).await;
/// ```
#[derive(Clone, Debug)]
pub struct TraceContext {
    /// Trace ID / 追踪ID
    pub trace_id: TraceId,

    /// Span ID / Span ID
    pub span_id: SpanId,

    /// Parent span ID / 父Span ID
    pub parent_span_id: Option<SpanId>,

    /// Sampling decision / 采样决策
    pub sampled: bool,

    /// Baggage for trace propagation / 追踪传播的额外数据
    pub baggage: HashMap<String, String>,
}

/// Trace ID (128-bit) / 追踪ID（128位）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TraceId([u8; 16]);

/// Span ID (64-bit) / Span ID（64位）
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SpanId([u8; 8]);

/// Span represents a single operation in a trace
/// Span表示追踪中的单个操作
pub struct Span {
    /// Span context / Span上下文
    context: TraceContext,

    /// Operation name / 操作名称
    pub name: String,

    /// Start time / 开始时间
    start_time: Instant,

    /// End time / 结束时间
    end_time: Option<Instant>,

    /// Status / 状态
    pub status: SpanStatus,

    /// Span attributes / Span属性
    pub attributes: HashMap<String, AttributeValue>,

    /// Span events / Span事件
    pub events: Vec<SpanEvent>,

    /// Links to other spans / 到其他span的链接
    pub links: Vec<SpanLink>,
}

/// Span status / Span状态
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpanStatus {
    /// Span is still running / Span仍在运行
    Unset,

    /// Span completed successfully / Span成功完成
    Ok,

    /// Span completed with error / Span完成但出错
    Error { description: String },
}

/// Span event / Span事件
#[derive(Clone, Debug)]
pub struct SpanEvent {
    /// Event name / 事件名称
    pub name: String,

    /// Event timestamp / 事件时间戳
    pub timestamp: Instant,

    /// Event attributes / 事件属性
    pub attributes: HashMap<String, AttributeValue>,
}

/// Attribute value / 属性值
#[derive(Clone, Debug)]
pub enum AttributeValue {
    String(String),
    Int(i64),
    Double(f64),
    Bool(bool),
    Array(Vec<AttributeValue>),
}

/// Tracer for creating spans / 创建span的追踪器
pub struct Tracer {
    exporter: Box<dyn SpanExporter>,
    sampler: Box<dyn Sampler>,
}

impl Tracer {
    /// Get the global tracer / 获取全局追踪器
    pub fn global() -> &'static Tracer;

    /// Start a new root span / 启动新根span
    pub fn start_span(&self, name: &str) -> Span;

    /// Start a child span / 启动子span
    pub fn start_child(&self, name: &str, parent: &TraceContext) -> Span;

    /// Export spans / 导出spans
    pub async fn export(&self, spans: Vec<Span>) -> Result<(), ExportError>;
}

impl Span {
    /// Set an attribute / 设置属性
    pub fn set_attribute(&mut self, key: impl Into<String>, value: impl Into<AttributeValue>) {
        self.attributes.insert(key.into(), value.into());
    }

    /// Record an event / 记录事件
    pub fn add_event(&mut self, name: impl Into<String>, attributes: HashMap<String, AttributeValue>) {
        self.events.push(SpanEvent {
            name: name.into(),
            timestamp: Instant::now(),
            attributes,
        });
    }

    /// Mark span as completed / 标记span完成
    pub fn finish(mut self) {
        self.end_time = Some(Instant::now());
        // Export to backend / 导出到后端
    }

    /// Execute a future within this span's context / 在此span上下文中执行future
    pub async fn in_scope<F, R>(self, f: F) -> R
    where
        F: Future<Output = R>,
    {
        // Set span as current / 设置span为当前
        f.await
    }
}
```

### 5.2 Metrics / 指标

```rust
/// Metrics registry
/// 指标注册表
///
/// # Example / 示例
/// ```rust
/// use nexus::observability::{MetricsRegistry, Counter, Histogram};
///
/// let registry = MetricsRegistry::global();
///
/// // Record a counter / 记录计数器
/// registry.counter("http_requests_total")
///     .with_label("method", "GET")
///     .with_label("path", "/api/users")
///     .increment();
///
/// // Record a histogram / 记录直方图
/// registry.histogram("http_request_duration_ms")
///     .with_label("method", "GET")
///     .observe(42.5);
/// ```
pub struct MetricsRegistry {
    metrics: DashMap<String, Metric>,
    exporter: Option<Box<dyn MetricsExporter>>,
}

impl MetricsRegistry {
    /// Get the global registry / 获取全局注册表
    pub fn global() -> &'static MetricsRegistry;

    /// Get or create a counter / 获取或创建计数器
    pub fn counter(&self, name: &str) -> Counter {
        // ...
    }

    /// Get or create a gauge / 获取或创建仪表
    pub fn gauge(&self, name: &str) -> Gauge {
        // ...
    }

    /// Get or create a histogram / 获取或创建直方图
    pub fn histogram(&self, name: &str) -> Histogram {
        // ...
    }

    /// Export metrics / 导出指标
    pub async fn export(&self) -> Result<(), ExportError>;
}

/// Counter metric (monotonically increasing)
/// 计数器指标（单调递增）
pub struct Counter {
    name: String,
    value: Arc<AtomicU64>,
    labels: LabelSet,
}

impl Counter {
    /// Increment by 1 / 增加1
    pub fn increment(&self) {
        self.increment_by(1);
    }

    /// Increment by value / 增加指定值
    pub fn increment_by(&self, value: u64) {
        self.value.fetch_add(value, Ordering::Relaxed);
    }

    /// Get current value / 获取当前值
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    /// Add labels / 添加标签
    pub fn with_label(&self, key: impl Into<String>, value: impl Into<String>) -> Self {
        // Return new counter with labels / 返回带标签的新计数器
        // ...
    }
}

/// Gauge metric (arbitrary value that can go up or down)
/// 仪表指标（可升可降的任意值）
pub struct Gauge {
    name: String,
    value: Arc<AtomicF64>,
    labels: LabelSet,
}

impl Gauge {
    /// Set value / 设置值
    pub fn set(&self, value: f64) {
        self.value.store(value, Ordering::Relaxed);
    }

    /// Increment by 1 / 增加1
    pub fn increment(&self) {
        self.fetch_add(1.0);
    }

    /// Decrement by 1 / 减少1
    pub fn decrement(&self) {
        self.fetch_add(-1.0);
    }

    /// Add value / 添加值
    pub fn add(&self, value: f64) {
        self.fetch_add(value);
    }

    /// Get current value / 获取当前值
    pub fn get(&self) -> f64 {
        self.value.load(Ordering::Relaxed)
    }
}

/// Histogram metric (counts observations into buckets)
/// 直方图指标（将观测值计数到桶中）
pub struct Histogram {
    name: String,
    bounds: Vec<f64>,
    buckets: Arc<Vec<AtomicU64>>,
    sum: Arc<AtomicU64>,
    count: Arc<AtomicU64>,
    labels: LabelSet,
}

impl Histogram {
    /// Observe a value / 观测值
    pub fn observe(&self, value: f64) {
        self.count.fetch_add(1, Ordering::Relaxed);

        // Find appropriate bucket / 找到合适的桶
        for (i, bound) in self.bounds.iter().enumerate() {
            if value <= *bound {
                self.buckets[i].fetch_add(1, Ordering::Relaxed);
                return;
            }
        }

        // Value exceeds all bounds / 值超过所有边界
        self.buckets.last().unwrap().fetch_add(1, Ordering::Relaxed);
    }

    /// Get count of observations / 获取观测计数
    pub fn count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    /// Get sum of observations / 获取观测总和
    pub fn sum(&self) -> u64 {
        self.sum.load(Ordering::Relaxed)
    }
}
```

### 5.3 Logging / 日志

```rust
/// Structured logging
/// 结构化日志
///
/// # Example / 示例
/// ```rust
/// use nexus::observability::info;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct User {
///     id: u64,
///     name: String,
/// }
///
/// let user = User { id: 123, name: "Alice".to_string() };
///
/// info!(
///     user_id = user.id,
///     user_name = user.name,
///     action = "login",
///     "User logged in"
/// );
/// ```
///
/// # Log Levels / 日志级别
///
/// - `trace!()` - Very detailed information / 非常详细的信息
/// - `debug!()` - Debugging information / 调试信息
/// - `info!()` - General informational messages / 一般信息消息
/// - `warn!()` - Warning messages / 警告消息
/// - `error!()` - Error messages / 错误消息

/// Log a trace message / 记录trace消息
#[macro_export]
macro_rules! trace {
    ( $($tt:tt)* ) => { /* ... */ }
}

/// Log a debug message / 记录debug消息
#[macro_export]
macro_rules! debug {
    ( $($tt:tt)* ) => { /* ... */ }
}

/// Log an info message / 记录info消息
#[macro_export]
macro_rules! info {
    ( $($tt:tt)* ) => { /* ... */ }
}

/// Log a warning message / 记录警告消息
#[macro_export]
macro_rules! warn {
    ( $($tt:tt)* ) => { /* ... */ }
}

/// Log an error message / 记录错误消息
#[macro_export]
macro_rules! error {
    ( $($tt:tt)* ) => { /* ... */ }
}
```

---

## 6. Web3 APIs / Web3 API

### 6.1 Chain Trait / Chain Trait

```rust
/// Blockchain interface
/// 区块链接口
///
/// # Example / 示例
/// ```rust
/// use nexus::web3::{Chain, ChainId, Address};
///
/// let chain = Chain::ethereum(ChainId::Mainnet);
///
/// // Get balance / 获取余额
/// let balance = chain.get_balance(address).await?;
///
/// // Send transaction / 发送交易
/// let tx_hash = chain.send_transaction(tx).await?;
/// ```
#[async_trait]
pub trait Chain: Send + Sync {
    /// Get chain identifier / 获取链标识符
    fn chain_id(&self) -> ChainId;

    /// Get current block number / 获取当前区块号
    async fn block_number(&self) -> Result<u64, ChainError>;

    /// Get balance for address / 获取地址余额
    async fn get_balance(&self, address: &Address) -> Result<U256, ChainError>;

    /// Get transaction count / 获取交易计数
    async fn get_transaction_count(&self, address: &Address) -> Result<u64, ChainError>;

    /// Get transaction by hash / 按哈希获取交易
    async fn get_transaction(&self, hash: &TxHash) -> Result<Option<Transaction>, ChainError>;

    /// Get transaction receipt / 获取交易回执
    async fn get_transaction_receipt(&self, hash: &TxHash) -> Result<Option<TransactionReceipt>, ChainError>;

    /// Send transaction / 发送交易
    async fn send_transaction(&self, tx: Transaction) -> Result<TxHash, ChainError>;

    /// Call contract (read-only) / 调用合约（只读）
    async fn call_contract(&self, request: &CallRequest) -> Result<Bytes, ChainError>;

    /// Estimate gas / 估算gas
    async fn estimate_gas(&self, tx: &Transaction) -> Result<u64, ChainError>;

    /// Get gas price / 获取gas价格
    async fn get_gas_price(&self) -> Result<U256, ChainError>;

    /// Subscribe to new blocks / 订阅新区块
    async fn subscribe_blocks(&self) -> Result<mpsc::Receiver<Block>, ChainError>;

    /// Subscribe to logs / 订阅日志
    async fn subscribe_logs(&self, filter: &LogFilter) -> Result<mpsc::Receiver<Log>, ChainError>;
}

/// Create an Ethereum chain / 创建以太坊链
pub fn ethereum(chain_id: ChainId) -> Arc<dyn Chain> {
    // ...
}
```

### 6.2 Contract / 合约

```rust
/// Smart contract interface
/// 智能合约接口
///
/// # Example / 示例
/// ```rust
/// use nexus::web3::{Contract, Address};
///
/// let contract = Contract::new(address, abi, chain);
///
/// // Call read-only method / 调用只读方法
/// let balance: U256 = contract
///     .method("balanceOf", user_address)
///     .call()
///     .await?;
///
/// // Send transaction / 发送交易
/// let tx_hash = contract
///     .method("transfer", (recipient, amount))
///     .send()
///     .await?;
/// ```
pub struct Contract<C> {
    /// Chain reference / 链引用
    chain: Arc<C>,

    /// Contract address / 合约地址
    address: Address,

    /// Contract ABI / 合约ABI
    abi: Abi,

    /// Default sender for transactions / 交易的默认发送者
    sender: Option<Address>,
}

impl<C: Chain> Contract<C> {
    /// Create a new contract interface / 创建新合约接口
    pub fn new(address: Address, abi: Abi, chain: Arc<C>) -> Self {
        Self {
            chain,
            address,
            abi,
            sender: None,
        }
    }

    /// Create a contract method call / 创建合约方法调用
    pub fn method<T: Tokenize>(&self, name: &str, args: T) -> ContractMethod<C> {
        ContractMethod {
            contract: self,
            name: name.to_string(),
            args: args.into_tokens(),
            value: U256::zero(),
            gas_limit: None,
            gas_price: None,
        }
    }

    /// Subscribe to contract events / 订阅合约事件
    pub async fn subscribe_events(&self, event_name: &str) -> Result<mpsc::Receiver<Event>, ContractError> {
        // ...
    }
}

/// Contract method call builder
/// 合约方法调用构建器
pub struct ContractMethod<'a, C> {
    contract: &'a Contract<C>,
    name: String,
    args: Vec<Token>,
    value: U256,
    gas_limit: Option<u64>,
    gas_price: Option<U256>,
}

impl<'a, C: Chain> ContractMethod<'a, C> {
    /// Set value to send / 设置发送金额
    pub fn value(mut self, value: U256) -> Self {
        self.value = value;
        self
    }

    /// Set gas limit / 设置gas限制
    pub fn gas_limit(mut self, limit: u64) -> Self {
        self.gas_limit = Some(limit);
        self
    }

    /// Call contract method (read-only) / 调用合约方法（只读）
    pub async fn call<R: Detokenize>(self) -> Result<R, ContractError> {
        // ...
    }

    /// Send transaction / 发送交易
    pub async fn send(self) -> Result<TxHash, ContractError> {
        // ...
    }
}
```

### 6.3 Wallet / 钱包

```rust
/// Wallet for signing transactions
/// 用于签名交易的钱包
///
/// # Example / 示例
/// ```rust
/// use nexus::web3::{Wallet, LocalWallet};
///
/// // Create new wallet / 创建新钱包
/// let wallet = LocalWallet::new(&mut rand::thread_rng());
///
/// // Or from mnemonic / 或从助记词创建
/// let wallet = LocalWallet::from_phrase(
///     "confirm heart later craft cross still either afford wage usual impose ghost",
///     &mut rand::thread_rng()
/// )?;
///
/// let address = wallet.address();
/// ```
pub trait Wallet: Send + Sync {
    /// Get wallet address / 获取钱包地址
    fn address(&self) -> Address;

    /// Sign a transaction / 签名交易
    fn sign_transaction(&self, tx: &mut Transaction) -> Result<Signature, WalletError>;

    /// Sign arbitrary data / 签名任意数据
    fn sign(&self, data: &[u8]) -> Result<Signature, WalletError>;

    /// Get chain ID / 获取链ID
    fn chain_id(&self) -> ChainId;
}

/// Local wallet with private key
/// 带私钥的本地钱包
pub struct LocalWallet {
    /// Private key / 私钥
    private_key: PrivateKey,

    /// Chain ID / 链ID
    chain_id: ChainId,
}

impl LocalWallet {
    /// Create a new random wallet / 创建新的随机钱包
    pub fn new<R: Rng>(rng: &mut R) -> Self;

    /// Create wallet from phrase / 从助记词创建钱包
    pub fn from_phrase<R: Rng>(phrase: &str, rng: &mut R) -> Result<Self, MnemonicError>;

    /// Create wallet from private key / 从私钥创建钱包
    pub fn from_private_key(key: PrivateKey) -> Self;

    /// Get wallet address / 获取钱包地址
    pub fn address(&self) -> Address;
}

impl Wallet for LocalWallet {
    fn address(&self) -> Address {
        // ...
    }

    fn sign_transaction(&self, tx: &mut Transaction) -> Result<Signature, WalletError> {
        // ...
    }

    fn sign(&self, data: &[u8]) -> Result<Signature, WalletError> {
        // ...
    }

    fn chain_id(&self) -> ChainId {
        self.chain_id
    }
}
```

---

## 7. Runtime APIs / 运行时API

### 7.1 Runtime / 运行时

```rust
/// Nexus async runtime
/// Nexus异步运行时
///
/// # Example / 示例
/// ```rust
/// use nexus::Runtime;
///
/// #[tokio::main]
/// async fn main() {
///     let runtime = Runtime::new()?;
///     runtime.block_on(async {
///         // Your async code / 你的异步代码
///     });
/// }
/// ```
pub struct Runtime<D> {
    /// Runtime driver / 运行时驱动
    driver: D,

    /// Runtime configuration / 运行时配置
    config: RuntimeConfig,
}

/// Runtime configuration / 运行时配置
#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    /// Number of worker threads / 工作线程数
    pub worker_threads: Option<usize>,

    /// Thread affinity (CPU pinning) / 线程亲和性（CPU绑定）
    pub thread_affinity: bool,

    /// Enable I/O driver / 启用I/O驱动
    pub enable_io: bool,

    /// Enable time driver / 启用时间驱动
    pub enable_time: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            worker_threads: None, // Use CPU count / 使用CPU数
            thread_affinity: false,
            enable_io: true,
            enable_time: true,
        }
    }
}

impl Runtime<IoUringDriver> {
    /// Create a new runtime with io-uring driver
    /// 使用io-uring驱动创建新运行时
    pub fn new() -> Result<Self, RuntimeError> {
        Self::with_config(RuntimeConfig::default())
    }

    /// Create runtime with custom configuration
    /// 使用自定义配置创建运行时
    pub fn with_config(config: RuntimeConfig) -> Result<Self, RuntimeError> {
        // ...
    }

    /// Run a future to completion on this runtime
    /// 在此运行时上运行future直到完成
    pub fn block_on<F>(&mut self, future: F) -> F::Output
    where
        F: Future;

    /// Spawn a task / 生成任务
    pub fn spawn<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
}

impl Runtime<LegacyDriver> {
    /// Create a new runtime with legacy driver (epoll/kqueue)
    /// 使用legacy驱动（epoll/kqueue）创建新运行时
    pub fn new_legacy() -> Result<Self, RuntimeError> {
        Self::with_config(RuntimeConfig::default())
    }
}
```

### 7.2 Task / 任务

```rust
/// Join handle for a spawned task
/// 生成任务的join句柄
///
/// # Example / 示例
/// ```rust
/// use nexus::Runtime;
///
/// let runtime = Runtime::new()?;
/// runtime.block_on(async {
///     let handle = nexus::spawn(async {
///         // Background task / 后台任务
///         42
///     });
///
///     let result = handle.await.unwrap();
///     assert_eq!(result, 42);
/// });
/// ```
pub struct JoinHandle<T> {
    /// Inner receiver / 内部接收器
    receiver: flume::Receiver<T>,
}

impl<T> JoinHandle<T> {
    /// Wait for the task to complete
    /// 等待任务完成
    pub async fn await(self) -> Result<T, JoinError> {
        // ...
    }

    /// Abort the task / 中止任务
    pub fn abort(&self) {
        // ...
    }

    /// Check if task is finished / 检查任务是否完成
    pub fn is_finished(&self) -> bool {
        // ...
    }
}

/// Spawn a new async task / 生成新的异步任务
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    // ...
}

/// Spawn a blocking task in a thread pool
/// 在线程池中生成阻塞任务
pub fn spawn_blocking<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    // ...
}
```

---

## Appendix A: Type Aliases / 类型别名

```rust
// Common type aliases for convenience / 便捷的常用类型别名
pub type Result<T> = std::result::Result<T, Error>;
pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
pub type Json<T> = nexus_response::Json<T>;
pub type Html<T> = nexus_response::Html<T>;

// Web3 type aliases / Web3类型别名
pub type Address = nexus_web3::Address;
pub type TxHash = nexus_web3::TxHash;
pub type U256 = nexus_web3::U256;
```

---

## Appendix B: Prelude / 预导入

```rust
//! The nexus prelude
//! nexus预导入
//!
//! # Example / 示例
//! ```rust
//! use nexus::prelude::*;
//! ```
//!
//! The prelude re-exports common types and traits for convenience.
//! 预导入重新导出常用类型和trait以方便使用。

pub use crate::Application;
pub use crate::Router;
pub use crate::Server;

pub use crate::extractor::{Path, Query, Json, State, Extension};
pub use crate::response::{Response, IntoResponse, Html, Json};

pub use crate::error::{Error, ErrorKind, Result};

pub use crate::middleware::{Middleware, Next};

pub use crate::observability::{info, warn, error, debug};

#[cfg(feature = "web3")]
pub use crate::web3::{Chain, Contract, Wallet, Address};

pub use http::{Method, StatusCode, Uri, Version};
pub use http::header::{HeaderMap, HeaderName, HeaderValue};
```

---

**This document is a living document and will be updated as the API evolves.**
/ **本文档是动态文档，将随API发展更新。**
