# Middleware
# 中间件

Middleware in Nexus provides a way to process requests and responses in a composable manner, similar to Spring's Filter/Interceptor pattern.

Nexus 中的中间件提供了一种以可组合方式处理请求和响应的方法，类似于 Spring 的 Filter/Interceptor 模式。

## Overview / 概述

```
Request  →  Middleware 1  →  Middleware 2  →  Handler
                ↓                 ↓              ↓
Response ←  Middleware 1  ←  Middleware 2  ←  Result
```

## Middleware Trait / 中间件 Trait

```rust
use nexus_router::{Middleware, Next};
use nexus_http::{Request, Response};

pub trait Middleware: Clone + Send + Sync + 'static {
    /// Process the request and call next middleware/handler
    /// 处理请求并调用下一个中间件/处理器
    async fn call(&self, req: Request, next: Next) -> Response;
}
```

## Built-in Middleware / 内置中间件

### Logger Middleware / 日志中间件

```rust
use nexus_middleware::LoggerMiddleware;

let router = Router::new()
    .get("/", handler)
    .layer(LoggerMiddleware::new());

// Output: 
// INFO  GET /api/users 200 OK 15ms
```

### CORS Middleware / CORS 中间件

```rust
use nexus_middleware::{CorsMiddleware, CorsConfig};

// Allow all origins / 允许所有来源
let cors = CorsMiddleware::any();

// Custom configuration / 自定义配置
let cors = CorsMiddleware::new(CorsConfig {
    allowed_origins: vec!["https://example.com".into()],
    allowed_methods: vec![Method::GET, Method::POST],
    allowed_headers: vec!["Content-Type".into(), "Authorization".into()],
    allow_credentials: true,
    max_age: Some(Duration::from_secs(3600)),
});

let router = Router::new()
    .get("/api/data", handler)
    .layer(cors);
```

### Timeout Middleware / 超时中间件

```rust
use nexus_middleware::TimeoutMiddleware;
use std::time::Duration;

let router = Router::new()
    .get("/api/slow", slow_handler)
    .layer(TimeoutMiddleware::new(Duration::from_secs(30)));
```

### Compression Middleware / 压缩中间件

```rust
use nexus_middleware::CompressionMiddleware;

let router = Router::new()
    .get("/api/data", handler)
    .layer(CompressionMiddleware::new());

// Supports: gzip, deflate, br (brotli)
```

## Creating Custom Middleware / 创建自定义中间件

### Function-based Middleware / 函数式中间件

```rust
use nexus_router::{Middleware, Next};
use nexus_http::{Request, Response};

async fn auth_middleware(req: Request, next: Next) -> Response {
    // Check authorization header / 检查授权头
    match req.header("authorization") {
        Some(token) if is_valid_token(token) => {
            // Continue to next middleware/handler
            // 继续到下一个中间件/处理器
            next.run(req).await
        }
        _ => {
            Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from("Unauthorized"))
                .unwrap()
        }
    }
}
```

### Struct-based Middleware / 结构体中间件

```rust
use std::time::Instant;

#[derive(Clone)]
struct TimingMiddleware;

impl Middleware for TimingMiddleware {
    async fn call(&self, req: Request, next: Next) -> Response {
        let start = Instant::now();
        let method = req.method().to_string();
        let path = req.path().to_string();
        
        // Call next middleware/handler / 调用下一个中间件/处理器
        let response = next.run(req).await;
        
        let duration = start.elapsed();
        tracing::info!(
            method = %method,
            path = %path,
            status = %response.status(),
            duration_ms = %duration.as_millis(),
            "Request completed"
        );
        
        response
    }
}
```

### Middleware with State / 带状态的中间件

```rust
#[derive(Clone)]
struct RateLimitMiddleware {
    requests_per_second: u32,
    limiter: Arc<RwLock<HashMap<String, u32>>>,
}

impl RateLimitMiddleware {
    fn new(rps: u32) -> Self {
        Self {
            requests_per_second: rps,
            limiter: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Middleware for RateLimitMiddleware {
    async fn call(&self, req: Request, next: Next) -> Response {
        let ip = req.remote_addr()
            .map(|a| a.to_string())
            .unwrap_or_default();
        
        // Check rate limit / 检查速率限制
        {
            let mut limiter = self.limiter.write().await;
            let count = limiter.entry(ip.clone()).or_insert(0);
            
            if *count >= self.requests_per_second {
                return Response::builder()
                    .status(StatusCode::TOO_MANY_REQUESTS)
                    .body(Body::from("Rate limit exceeded"))
                    .unwrap();
            }
            
            *count += 1;
        }
        
        next.run(req).await
    }
}
```

## Middleware Ordering / 中间件顺序

Middleware is applied in the order it's added, but executed in reverse order for responses:

中间件按添加顺序应用，但响应按相反顺序执行：

```rust
let router = Router::new()
    .get("/", handler)
    .layer(LoggerMiddleware::new())    // 1st added, outermost
    .layer(CorsMiddleware::any())      // 2nd added
    .layer(TimeoutMiddleware::new(30s)); // 3rd added, innermost

// Request flow:  Logger → CORS → Timeout → Handler
// Response flow: Handler → Timeout → CORS → Logger
```

## Spring Boot Comparison / Spring Boot 对比

| Spring Boot | Nexus | Description |
|-------------|-------|-------------|
| `Filter` | `Middleware` trait | Request/response processing |
| `HandlerInterceptor` | `Middleware` trait | Handler interception |
| `@CrossOrigin` | `CorsMiddleware` | CORS configuration |
| `OncePerRequestFilter` | - | Single execution per request |
| Filter chain | `.layer()` chaining | Middleware composition |

## Best Practices / 最佳实践

1. **Keep middleware lightweight** / 保持中间件轻量
   - Avoid heavy computation in middleware
   - Use async for I/O operations

2. **Order matters** / 顺序很重要
   - Put authentication before authorization
   - Put logging first to capture all requests

3. **Use appropriate scope** / 使用适当的作用域
   - Global middleware: Add to root router
   - Route-specific: Add to nested router

4. **Handle errors gracefully** / 优雅处理错误
   - Return proper error responses
   - Don't panic in middleware

## Complete Example / 完整示例

```rust
use nexus_router::Router;
use nexus_middleware::{
    LoggerMiddleware, 
    CorsMiddleware, 
    TimeoutMiddleware,
    CompressionMiddleware,
};
use std::time::Duration;

fn build_router() -> Router {
    // Public routes / 公共路由
    let public = Router::new()
        .get("/health", health_check)
        .get("/version", version);
    
    // Protected API routes / 受保护的 API 路由
    let api = Router::new()
        .get("/users", list_users)
        .post("/users", create_user)
        .layer(auth_middleware);  // Auth only for API
    
    // Build main router / 构建主路由
    Router::new()
        .merge(public)
        .nest("/api", api)
        // Global middleware / 全局中间件
        .layer(LoggerMiddleware::new())
        .layer(CorsMiddleware::any())
        .layer(CompressionMiddleware::new())
        .layer(TimeoutMiddleware::new(Duration::from_secs(30)))
}
```

---

*← [Previous / 上一页](./router.md) | [Next / 下一页](./extractors.md) →*
