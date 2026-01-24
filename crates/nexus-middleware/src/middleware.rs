//! Middleware module
//! 中间件模块
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - Filter, OncePerRequestFilter
//! - HandlerInterceptor
//! - WebMvcConfigurer

#![warn(missing_docs)]
#![warn(unreachable_pub)]

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Request type (will be defined in nexus-http)
/// 请求类型（将在nexus-http中定义）
pub struct Request<B = ()> {
    // TODO: Define Request structure
    // TODO: 定义Request结构
    _phantom: std::marker::PhantomData<B>,
}

/// Response type
/// 响应类型
pub struct Response;

/// Error type
/// 错误类型
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// Result type
/// Result类型
pub type Result<T> = std::result::Result<T, Error>;

/// Middleware trait
/// 中间件trait
///
/// This is equivalent to Spring's:
/// - `Filter`
/// - `HandlerInterceptor`
/// - `OncePerRequestFilter`
///
/// 这等价于Spring的：
/// - `Filter`
/// - `HandlerInterceptor`
/// - `OncePerRequestFilter`
pub trait Middleware<S>: Clone + Send + Sync + 'static {
    /// Output type
    /// 输出类型
    type Output: Future<Output = Result<Response>> + Send;

    /// Call the middleware
    /// 调用中间件
    fn call(&self, req: Request, next: Next<S>) -> Self::Output;
}

/// Next middleware in the chain
/// 链中的下一个中间件
///
/// This represents the next filter/handler in the chain.
/// 这表示链中的下一个filter/handler。
pub struct Next<S> {
    inner: Arc<dyn Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>> + Send + Sync>,
    _phantom: std::marker::PhantomData<S>,
}

impl<S> Next<S> {
    /// Create a new Next
    /// 创建新的Next
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>> + Send + Sync + 'static,
    {
        Self {
            inner: Arc::new(f),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Call the next middleware
    /// 调用下一个中间件
    pub async fn call(self, req: Request) -> Result<Response> {
        (self.inner)(req).await
    }
}

/// Middleware stack
/// 中间件栈
///
/// This manages a chain of middleware that will be executed in order.
/// 这管理将按顺序执行的中间件链。
///
/// Equivalent to Spring's `FilterChain` or `HandlerExecutionChain`.
/// 等价于Spring的`FilterChain`或`HandlerExecutionChain`。
#[derive(Clone)]
pub struct MiddlewareStack<S> {
    middleware: Vec<Arc<dyn MiddlewareFn<S>>>,
    _phantom: std::marker::PhantomData<S>,
}

/// Middleware function trait
/// 中间件函数trait
trait MiddlewareFn<S>: Send + Sync {
    /// Call the middleware function
    fn call_boxed(
        &self,
        req: Request,
        next: Next<S>,
    ) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>>;
}

impl<F, S, Fut> MiddlewareFn<S> for F
where
    F: Fn(Request, Next<S>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<Response>> + Send + 'static,
{
    fn call_boxed(
        &self,
        req: Request,
        next: Next<S>,
    ) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>> {
        Box::pin(self(req, next))
    }
}

impl<S> MiddlewareStack<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// Create a new middleware stack
    /// 创建新的中间件栈
    pub fn new() -> Self {
        Self {
            middleware: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Add a middleware to the stack
    /// 向栈中添加中间件
    pub fn add<M>(mut self, middleware: M) -> Self
    where
        M: Middleware<S> + 'static,
        M::Output: Future<Output = Result<Response>> + Send,
    {
        let mw = Arc::new(move |req: Request, next: Next<S>| {
            Box::pin(middleware.call(req, next)) as Pin<Box<dyn Future<Output = Result<Response>> + Send>>
        });
        self.middleware.push(mw);
        self
    }

    /// Add a function-based middleware
    /// 添加基于函数的中间件
    pub fn add_fn<F, Fut>(mut self, f: F) -> Self
    where
        F: Fn(Request, Next<S>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Response>> + Send + 'static,
    {
        let mw = Arc::new(move |req: Request, next: Next<S>| {
            Box::pin(f(req, next)) as Pin<Box<dyn Future<Output = Result<Response>> + Send>>
        });
        self.middleware.push(mw);
        self
    }

    /// Execute the middleware stack
    /// 执行中间件栈
    pub async fn execute(&self, req: Request) -> Result<Response> {
        self.execute_from(0, req).await
    }

    /// Execute from a specific middleware index
    /// 从特定中间件索引执行
    fn execute_from(&self, index: usize, req: Request) -> Pin<Box<dyn Future<Output = Result<Response>> + Send>> {
        if index >= self.middleware.len() {
            // End of chain - return error
            // 链结束 - 返回错误
            Box::pin(async move { Err("No handler".into()) })
        } else {
            let middleware = self.middleware[index].clone();
            let next_index = index + 1;
            let stack = self.clone();

            let next = Next::new(move |req: Request| {
                stack.execute_from(next_index, req)
            });

            middleware.call_boxed(req, next)
        }
    }
}

impl<S> Default for MiddlewareStack<S>
where
    S: Clone + Send + Sync + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
