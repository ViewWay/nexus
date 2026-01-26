# Nexus API Quick Reference
# Nexus API 快速参考

## Core Types / 核心类型

### Application / 应用

```rust,ignore
use nexus::prelude::*;

// Create and run application
#[tokio::main]
async fn main() {
    let app = Router::new()
        .get("/", || async { "Hello" })
        .get("/users/:id", get_user);

    Server::bind("0.0.0.0:8080")
        .serve(app)
        .await
        .unwrap();
}
```

### Router / 路由器

```rust,ignore
let app = Router::new()
    // HTTP methods
    .get("/", handler)
    .post("/", handler)
    .put("/", handler)
    .delete("/", handler)
    .patch("/", handler)
    // Path parameters
    .get("/users/:id", get_user)
    .get("/posts/:post_id/comments/:comment_id", get_comment)
    // Wildcard
    .get("/files/*path", serve_file)
    // Middleware
    .middleware(logger)
    .nest("/api", api_routes)
    .with_state(state);
```

### Request / 请求

```rust,ignore
use nexus::Request;

pub async fn handler(req: Request) -> Response {
    let method = req.method();
    let uri = req.uri();
    let headers = req.headers();
    let body = req.body();
}
```

### Response / 响应

```rust,ignore
use nexus::Response;
use nexus::response::Json;

// JSON response
Json(user).into_response()

// Status with body
(StatusCode::OK, "Hello").into_response()

// Builder
Response::builder()
    .status(200)
    .header("Content-Type", "application/json")
    .body(body.into())
    .unwrap()
```

---

## Extractors / 提取器

### Path / 路径参数

```rust,ignore
#[get("/users/:id")]
async fn get_user(id: String) -> Json<User> {
    Json(user_service.find_by_id(&id).await)
}

// Typed parameter
#[get("/posts/:id")]
async fn get_post(id: u64) -> Json<Post> {
    Json(post_service.find(id).await)
}
```

### Query / 查询参数

```rust,ignore
#[get("/search")]
async fn search(
    #[query] q: String,
    #[query] page: Option<u32>,
    #[query(default = 10)] limit: u32,
) -> Json<Results> {
    Json(search(q, page.unwrap_or(1), limit).await)
}
```

### JSON Body / JSON 请求体

```rust,ignore
#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

#[post("/users")]
async fn create_user(#[request_body] user: CreateUser) -> Json<User> {
    Json(user_service.create(user).await)
}
```

### Headers / 请求头

```rust,ignore
#[get("/info")]
async fn info(#[request_header] user_agent: String) -> String {
    format!("User-Agent: {}", user_agent)
}

// Optional header
#[get("/auth")]
async fn auth(#[request_header] auth: Option<String>) -> Status {
    if auth.is_some() {
        Status::OK
    } else {
        Status::UNAUTHORIZED
    }
}
```

### State / 状态

```rust,ignore
#[derive(Clone)]
struct AppState {
    db: Arc<Database>,
}

#[get("/users")]
async fn list_users(#[state] state: Arc<AppState>) -> Json<Vec<User>> {
    Json(state.db.find_users().await)
}
```

---

## Middleware / 中间件

### Creating Middleware / 创建中间件

```rust,ignore
use nexus_middleware::{Middleware, Next};

struct MyMiddleware;

impl Middleware for MyMiddleware {
    async fn call(
        &self,
        req: Request,
        next: Next,
    ) -> Result<Response, Error> {
        // Before handler
        println!("Request: {:?}", req.uri());
        
        // Call next
        let response = next.run(req).await?;
        
        // After handler
        println!("Response: {:?}", response.status());
        
        Ok(response)
    }
}
```

### Built-in Middleware / 内置中间件

```rust,ignore
use nexus_middleware::*;

// Logger
.middleware(Arc::new(LoggerMiddleware::new()))

// CORS
.middleware(Arc::new(CorsMiddleware::new(
    CorsConfig::new().allow_all()
)))

// Compression
.middleware(Arc::new(CompressionMiddleware::new()))

// Timeout
.middleware(Arc::new(TimeoutMiddleware::new(
    Duration::from_secs(30)
)))
```

---

## Error Handling / 错误处理

### Custom Error / 自定义错误

```rust,ignore
#[derive(Debug)]
enum AppError {
    NotFound(String),
    Unauthorized,
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(id) => (StatusCode::NOT_FOUND, format!("{} not found", id)),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = Json(json!({ "error": message }));
        (status, body).into_response()
    }
}
```

### Result Response / 结果响应

```rust,ignore
#[get("/users/:id")]
async fn get_user(id: String) -> Result<Json<User>, AppError> {
    let user = user_service.find_by_id(&id).await
        .ok_or_else(|| AppError::NotFound(id))?;
    Ok(Json(user))
}
```

---

## Configuration / 配置

### Config Struct / 配置结构

```rust,ignore
use nexus_macros::config;

#[config(prefix = "app")]
struct AppConfig {
    name: String,
    port: u16,
    debug: bool,
}

// Load from application.toml or environment
let config = AppConfig::load()?;
```

### Environment Variables / 环境变量

```rust,ignore
use nexus_macros::value;

#[value("${SERVER_PORT:8080}")]
static SERVER_PORT: u16 = 8080;

#[value("${DATABASE_URL}")]
static DATABASE_URL: &str = "postgresql://localhost/mydb";
```

---

## Annotations / 注解

### Controller / 控制器

```rust,ignore
use nexus_macros::{main, controller, get, post};

#[main]
struct Application;

#[controller]
struct UserController;

#[get("/users")]
async fn list_users() -> Json<Vec<User>> {
    Json(vec![])
}

#[post("/users")]
async fn create_user(#[request_body] user: CreateUser) -> Json<User> {
    Json(user)
}
```

### Service / 服务

```rust,ignore
use nexus_macros::{service, autowired};

#[service]
struct UserService {
    #[autowired]
    repository: Arc<UserRepository>,
}

impl UserService {
    fn find_by_id(&self, id: &str) -> Result<User, Error> {
        self.repository.find_by_id(id)
    }
}
```

### Transactional / 事务

```rust,ignore
use nexus_macros::transactional;

#[transactional]
async fn transfer(from: &str, to: &str, amount: f64) -> Result<(), Error> {
    // Runs in transaction
    db.execute(&format!("UPDATE accounts SET balance = balance - {} WHERE id = {}", amount, from)).await?;
    db.execute(&format!("UPDATE accounts SET balance = balance + {} WHERE id = {}", amount, to)).await?;
    Ok(())
}
```

### Cacheable / 缓存

```rust,ignore
use nexus_macros::{cacheable, cache_evict};

#[cacheable("users")]
async fn get_user(id: &str) -> Result<User, Error> {
    db.query_user(id).await
}

#[cache_evict("users")]
async fn update_user(user: User) -> Result<User, Error> {
    db.update_user(&user).await
}
```

### Scheduled / 定时

```rust,ignore
use nexus_macros::{scheduled, enable_scheduling};

#[enable_scheduling]
struct Scheduler;

#[scheduled(cron = "0 0 * * * *")]  // Daily at midnight
async fn cleanup() {
    // Cleanup logic
}

#[scheduled(fixed_rate = 5000)]  // Every 5 seconds
async fn refresh_cache() {
    // Refresh logic
}
```

---

## Resilience / 弹性

### Circuit Breaker / 熔断器

```rust,ignore
use nexus_resilience::circuit::{CircuitBreaker, CircuitConfig};

let breaker = CircuitBreaker::new(
    CircuitConfig::new()
        .failure_threshold(5)
        .timeout(Duration::from_secs(60))
);

let result = breaker.call("api", || async {
    api_call().await
}).await?;
```

### Retry / 重试

```rust,ignore
use nexus_resilience::retry::{Retry, RetryConfig};

let retry = Retry::new(
    RetryConfig::exponential()
        .max_attempts(3)
        .initial_delay(Duration::from_millis(100))
);

let result = retry.call(|| async {
    api_call().await
}).await?;
```

---

## Observability / 可观测性

### Tracing / 追踪

```rust,ignore
use nexus_observability::trace::{Tracer, span};

#[span(name = "get_user")]
async fn get_user(id: &str) -> User {
    // Automatically traced
    db.find_user(id).await
}
```

### Logging / 日志

```rust,ignore
use nexus_observability::log::Logger;

let logger = LoggerFactory::get("my_service");

logger.info()
    .field("user_id", "123")
    .field("action", "login")
    .message("User logged in")
    .log();
```

### Metrics / 指标

```rust,ignore
use nexus_observability::metrics::{Counter, Histogram};

let counter = Counter::new("requests_total", "Total requests");
counter.inc();

let histogram = Histogram::new("request_duration_seconds", "Request duration");
histogram.observe(0.042);
```

---

## Quick Imports / 快速导入

```rust,ignore
// Prelude - most common types
use nexus::prelude::*;

// All macros
use nexus_macros::*;

// Extractors
use nexus::extractors::*;

// Middleware
use nexus_middleware::*;

// Error handling
use nexus::error::*;

// Response types
use nexus::response::*;

// Runtime
use nexus_runtime::*;

// Observability
use nexus_observability::*;

// Resilience
use nexus_resilience::*;

// Web3
use nexus_web3::*;
```

---

## Common Patterns / 常见模式

### REST CRUD

```rust,ignore
#[controller]
struct UserController;

#[get("/users")]
async fn list(#[query] page: Option<u32>) -> Json<Vec<User>> { /* ... */ }

#[get("/users/:id")]
async fn get(id: String) -> Result<Json<User>, Error> { /* ... */ }

#[post("/users")]
async fn create(#[request_body] user: CreateUser) -> Result<Json<User>, Error> { /* ... */ }

#[put("/users/:id")]
async fn update(id: String, #[request_body] user: UpdateUser) -> Result<Json<User>, Error> { /* ... */ }

#[delete("/users/:id")]
async fn delete(id: String) -> Result<Status, Error> { /* ... */ }
```

### With Authentication / 带认证

```rust,ignore
#[get("/profile")]
async fn profile(
    #[request_header] auth: String,
) -> Result<Json<User>, Error> {
    let claims = auth_service.validate(&auth)?;
    let user = user_service.find_by_id(&claims.sub).await?;
    Ok(Json(user))
}
```

### With Validation / 带校验

```rust,ignore
#[derive(Deserialize, Validate)]
struct CreateUser {
    #[validate(length(min = 3))]
    username: String,
    #[validate(email)]
    email: String,
}

#[post("/users")]
async fn create_user(#[validated] user: CreateUser) -> Result<Json<User>, Error> {
    user.validate()?;
    Ok(Json(user_service.create(user).await?))
}
```

---

**Full API Documentation: [api-spec.md](api-spec.md)**
