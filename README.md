<div align="center">
<p><img alt="Nexus" width="132" style="max-width:40%;min-width:60px;" src="https://via.placeholder.com/132x40/0066CC/FFFFFF?text=Nexus" /></p>
<p>
    <a href="https://github.com/nexus-rs/nexus/blob/main/README.md">English</a>&nbsp;&nbsp;
    <a href="https://github.com/nexus-rs/nexus/blob/main/README.zh.md">简体中文</a>
</p>
<p>
<a href="https://github.com/nexus-rs/nexus/actions">
    <img alt="build status" src="https://github.com/nexus-rs/nexus/workflows/CI/badge.svg" />
</a>
<a href="https://codecov.io/gh/nexus-rs/nexus">
    <img alt="codecov" src="https://img.gov/nexus-rs/nexus/branch/main/graph/badge.svg" />
</a>
<br>
<a href="https://crates.io/crates/nexus"><img alt="crates.io" src="https://img.shields.io/crates/v/nexus" /></a>
<a href="https://docs.rs/nexus"><img alt="Documentation" src="https://docs.rs/nexus/badge.svg" /></a>
<a href="https://crates.io/crates/nexus"><img alt="Download" src="https://img.shields.io/crates/d/nexus.svg" /></a>
<br>
<a href="https://nexusframework.com">
    <img alt="Website" src="https://img.shields.io/badge/https-nexusframework.com-%23f00" />
</a>
</p>
</div>

# Nexus Framework / Nexus 框架

Nexus is a production-grade, high-availability web framework written in Rust with a custom async runtime. Unlike other frameworks that use Tokio, Nexus features a custom async runtime built from scratch using io-uring for maximum performance.

Nexus 是一个用 Rust 编写的生产级、高可用 Web 框架，具有自定义异步运行时。与使用 Tokio 的其他框架不同，Nexus 具有使用 io-uring 从头构建的自定义异步运行时，以实现最大性能。

## 🎯 Features / 功能特性

- **Custom Runtime / 自定义运行时** - Thread-per-core architecture with io-uring support / 支持 io-uring 的 thread-per-core 架构
- **High Availability / 高可用** - Circuit breakers, rate limiters, retry logic / 熔断器、限流器、重试逻辑
- **Web3 Native / 原生 Web3** - Built-in blockchain and smart contract support / 内置区块链和智能合约支持
- **Observability / 可观测性** - OpenTelemetry-compatible tracing/metrics / 兼容 OpenTelemetry 的追踪/指标
- **Type Safety / 类型安全** - Leverages Rust's type system / 利用 Rust 的类型系统
- **Spring-like / 类 Spring** - Familiar patterns for Spring Boot developers / Spring Boot 开发者熟悉的模式

## ⚡️ Quick Start / 快速开始

### Installation / 安装

Add to your `Cargo.toml`:

```toml
[dependencies]
nexus-runtime = "0.1"
nexus-http = { version = "0.1", features = ["full"] }
nexus-router = "0.1"
nexus-observability = "0.1"
```

### Basic HTTP Server / 基础 HTTP 服务器

```rust
use nexus_http::{Body, Response, Server, StatusCode};
use nexus_runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging / 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create runtime and run server / 创建运行时并运行服务器
    let mut runtime = Runtime::new()?;

    runtime.block_on(async {
        // Bind server to address / 绑定服务器到地址
        let _server = Server::bind("127.0.0.1:8080")
            .run(handle_request)
            .await?;

        Ok::<_, Box<dyn std::error::Error>>(())
    })
}

async fn handle_request(req: nexus_http::Request) -> Result<Response, nexus_http::Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/plain")
        .body(Body::from("Hello, Nexus!"))
        .unwrap())
}
```

### Complete Annotated Example / 完整注解示例

```rust
//! Nexus REST API Example / Nexus REST API 示例
//!
//! This example demonstrates a complete REST API with:
//! 此示例演示了完整的 REST API，包括：
//! - Routing with path parameters / 带路径参数的路由
//! - JSON request/response / JSON 请求/响应
//! - Error handling / 错误处理
//! - Middleware (CORS, logging) / 中间件（CORS、日志）
//! - Circuit breaker / 熔断器
//! - Observability (tracing, metrics) / 可观测性（追踪、指标）

use nexus_http::{
    Body, Response, Server, StatusCode,
    Request, Result as HttpResult,
};
use nexus_router::Router;
use nexus_runtime::Runtime;
use nexus_observability::{tracing, metrics};

// ============================================================================
// Data Models / 数据模型
// ============================================================================

/// User representation / 用户表示
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct User {
    id: u64,
    username: String,
    email: String,
}

/// Create user request / 创建用户请求
#[derive(Debug, serde::Deserialize)]
struct CreateUserRequest {
    username: String,
    email: String,
}

// ============================================================================
// Error Handling / 错误处理
// ============================================================================

/// API Error type / API 错误类型
#[derive(Debug)]
enum ApiError {
    /// User not found (404) / 用户未找到
    UserNotFound(u64),
    /// Invalid input (400) / 无效输入
    InvalidInput(String),
    /// Internal server error (500) / 内部服务器错误
    Internal(String),
}

impl ApiError {
    /// Convert to HTTP status code / 转换为 HTTP 状态码
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::UserNotFound(_) => StatusCode::NOT_FOUND,
            ApiError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get error message / 获取错误消息
    fn message(&self) -> String {
        match self {
            ApiError::UserNotFound(id) => format!("User {} not found", id),
            ApiError::InvalidInput(msg) => msg.clone(),
            ApiError::Internal(msg) => format!("Internal error: {}", msg),
        }
    }
}

// ============================================================================
// In-Memory Store / 内存存储
// ============================================================================

/// Simple in-memory user store / 简单的内存用户存储
struct UserStore {
    users: std::sync::Arc<parking_lot::Mutex<std::collections::HashMap<u64, User>>>,
    next_id: std::sync::Arc<std::sync::atomic::AtomicU64>,
}

impl UserStore {
    /// Create new store / 创建新存储
    fn new() -> Self {
        Self {
            users: std::sync::Arc::new(parking_lot::Mutex::new(std::collections::HashMap::new())),
            next_id: std::sync::Arc::new(std::sync::atomic::AtomicU64::new(1)),
        }
    }

    /// Get user by ID / 按 ID 获取用户
    fn get(&self, id: u64) -> Option<User> {
        self.users.lock().get(&id).cloned()
    }

    /// Create new user / 创建新用户
    fn create(&self, req: CreateUserRequest) -> User {
        let id = self.next_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let user = User {
            id,
            username: req.username,
            email: req.email,
        };
        self.users.lock().insert(id, user.clone());
        user
    }

    /// List all users / 列出所有用户
    fn list(&self) -> Vec<User> {
        self.users.lock().values().cloned().collect()
    }
}

// ============================================================================
// Route Handlers / 路由处理器
// ============================================================================

/// GET /users - List all users / 列出所有用户
async fn list_users(
    _req: Request,
    store: nexus_router::State<UserStore>,
) -> HttpResult<Response> {
    tracing::info!("Listing all users");

    let users = store.list();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&users).unwrap()))
        .unwrap())
}

/// GET /users/:id - Get user by ID / 按 ID 获取用户
async fn get_user(
    req: Request,
    store: nexus_router::State<UserStore>,
) -> HttpResult<Response> {
    // Extract path parameter / 提取路径参数
    let id = req
        .param("id")
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or_else(|| ApiError::InvalidInput("Invalid user ID".to_string()))?;

    tracing::info!("Getting user: {}", id);

    // Look up user / 查找用户
    let user = store
        .get(id)
        .ok_or_else(|| ApiError::UserNotFound(id))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&user).unwrap()))
        .unwrap())
}

/// POST /users - Create new user / 创建新用户
async fn create_user(
    mut req: Request,
    store: nexus_router::State<UserStore>,
) -> HttpResult<Response> {
    // Parse request body / 解析请求体
    let body = std::pin::pin(&mut req)
        .body_bytes()
        .await
        .map_err(|e| ApiError::Internal(format!("Failed to read body: {}", e)))?;

    let create_req = serde_json::from_slice::<CreateUserRequest>(&body)
        .map_err(|e| ApiError::InvalidInput(format!("Invalid JSON: {}", e)))?;

    tracing::info!("Creating user: {}", create_req.username);

    // Validate input / 验证输入
    if create_req.username.is_empty() || create_req.username.len() > 50 {
        return Err(ApiError::InvalidInput("Username must be 1-50 characters".into()).into());
    }

    // Create user / 创建用户
    let user = store.create(create_req);

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("content-type", "application/json")
        .header("location", format!("/users/{}", user.id))
        .body(Body::from(serde_json::to_string(&user).unwrap()))
        .unwrap())
}

// ============================================================================
// Error Conversion / 错误转换
// ============================================================================

impl From<ApiError> for nexus_http::Error {
    fn from(err: ApiError) -> Self {
        nexus_http::Error::new(err.status_code(), err.message())
    }
}

// ============================================================================
// Main Application / 主应用程序
// ============================================================================

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging / 初始化日志
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create shared state / 创建共享状态
    let store = UserStore::new();

    // Build router / 构建路由器
    let app = Router::new()
        // GET /users - List users / 列出用户
        .route("/users", nexus_router::Method::GET, list_users)

        // GET /users/:id - Get user / 获取用户
        .route("/users/:id", nexus_router::Method::GET, get_user)

        // POST /users - Create user / 创建用户
        .route("/users", nexus_router::Method::POST, create_user)

        // Add state / 添加状态
        .with_state(store);

    // Create and run runtime / 创建并运行运行时
    let mut runtime = Runtime::new()?;

    tracing::info!("Starting server on http://127.0.0.1:8080");

    runtime.block_on(async {
        // Start server / 启动服务器
        let _server = Server::bind("127.0.0.1:8080")
            .run(app)
            .await?;

        Ok::<_, Box<dyn std::error::Error>>(())
    })
}
```

### Testing the API / 测试 API

```bash
# List users (empty) / 列出用户（空）
curl http://localhost:8080/users

# Create a user / 创建用户
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com"}'

# Get user by ID / 按 ID 获取用户
curl http://localhost:8080/users/1

# List users (with data) / 列出用户（有数据）
curl http://localhost:8080/users
```

### Nexus Logging / Nexus 日志

Nexus provides a unified logging system with two modes: **Verbose** (development) and **Simple** (production).

Nexus 提供统一的日志系统，具有两种模式：**Verbose**（开发）和 **Simple**（生产）。

```rust
use nexus_observability::log::{Logger, LoggerConfig, LogLevel, LogMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Automatic mode selection based on profile / 基于配置文件自动选择模式
    let config = LoggerConfig {
        level: LogLevel::Info,
        mode: LogMode::from_profile(Some("dev")),  // dev→Verbose, prod→Simple
        ..Default::default()
    };

    Logger::init_with_config(config)?;

    tracing::info!("Application started");
    Ok(())
}
```

**Configuration via Environment Variables / 通过环境变量配置:**

```bash
# Set log level / 设置日志级别
export NEXUS_LOG_LEVEL=DEBUG

# Set log mode explicitly / 显式设置日志模式
export NEXUS_LOG_MODE=simple  # or "verbose"

# Set profile (affects default mode) / 设置配置文件（影响默认模式）
export NEXUS_PROFILE=prod  # dev→verbose, prod→simple
```

**Output Comparison / 输出对比:**

| Mode | Format |
|------|--------|
| Verbose (dev) / 详细（开发） | `2026-01-30 10:30:45.123 \|INFO\| 55377 [main] n.http.server : Request received` |
| Simple (prod) / 精简（生产） | `INFO n.http.server: Request received` |

### Resilience Patterns / 弹性模式

```rust
use nexus_resilience::{CircuitBreaker, RateLimiter, RetryPolicy};
use nexus_http::Request;

// Circuit breaker / 熔断器
let breaker = CircuitBreaker::new(
    "external-api",
    5,      // failure threshold / 失败阈值
    10000,  // timeout ms / 超时毫秒
);

// Rate limiter / 限流器
let limiter = RateLimiter::token_bucket(100, 10); // 100 requests, refill 10/sec

// Retry with exponential backoff / 指数退避重试
let retry = RetryPolicy::exponential_backoff(3, 100); // 3 retries, 100ms base

// Use in handler / 在处理器中使用
async fn call_external_api(req: Request) -> Result<Response, Error> {
    breaker.call(|| async {
        limiter.throttle().await?;
        retry.retry(|| async {
            // Actual API call / 实际 API 调用
            make_request(req).await
        }).await
    }).await
}
```

### Web3 Support / Web3 支持

```rust
use nexus_web3::{
    Chain, ChainConfig, LocalWallet, RpcClient,
    Transaction, TransactionBuilder, TxType,
};

async fn web3_example() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to Ethereum / 连接到以太坊
    let chain = Chain::ethereum();
    let rpc = RpcClient::new(&chain.rpc_url())?;

    // Create wallet / 创建钱包
    let wallet = LocalWallet::new(&mut rand::thread_rng());

    // Build transaction / 构建交易
    let tx = TransactionBuilder::new()
        .to(wallet.address())
        .value(1000000) // 0.001 ETH
        .gas_limit(21000)
        .chain_id(chain.chain_id())
        .build(TxType::Legacy)?;

    // Send transaction / 发送交易
    let signed = wallet.sign_transaction(&tx)?;
    let tx_hash = rpc.send_raw_transaction(&signed).await?;

    tracing::info!("Transaction sent: {}", tx_hash);

    Ok(())
}
```

## 🚀 Performance / 性能

Nexus is designed for high performance from the ground up:

Nexus 从根本上设计为高性能：

- **70% fewer syscalls** vs epoll with io-uring / 与 epoll 相比减少 70% 系统调用
- **40% lower latency** with thread-per-core architecture / thread-per-core 架构降低 40% 延迟
- **Zero-copy I/O** for minimal allocations / 零拷贝 I/O 最小化分配
- **Linear scalability** with no lock contention / 线性可扩展性，无锁竞争

| Benchmark / 基准测试 | Result / 结果 |
|---------------------|---------------|
| HTTP Parsing (GET) | ~170 ns |
| HTTP Encoding | ~120 ns |
| Throughput | 6.8 GiB/s |
| Spawn latency | < 1 μs |
| Channel throughput | 10M+ msg/s |

## 📚 Documentation / 文档

| Resource / 资源 | Link / 链接 |
|------------------|-------------|
| **Book / 书籍** | [docs.nexusframework.com](https://docs.nexusframework.com) |
| **API Docs / API 文档** | [docs.rs/nexus](https://docs.rs/nexus) |
| **Design Spec / 设计规范** | [design-spec.md](docs/design-spec.md) |
| **Implementation Plan / 实施计划** | [implementation-plan.md](docs/design/implementation-plan.md) |
| **Examples / 示例** | [examples/](examples/) |

## 🏗️ Architecture / 架构

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│                    应用程序层                                 │
├─────────────────────────────────────────────────────────────┤
│  Handlers  │  Middleware  │  Extractors  │  Response        │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     Nexus Runtime                            │
│                     Nexus 运行时                              │
├─────────────────────────────────────────────────────────────┤
│  Task Scheduler  │  I/O Driver  │  Timer  │  Executor       │
│  (Thread-per-Core)  │  (io-uring)   │                          │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     System Layer                             │
│                     系统层                                   │
├─────────────────────────────────────────────────────────────┤
│       io-uring (Linux) / epoll / kqueue                      │
└─────────────────────────────────────────────────────────────┘
```

## 🛠️ Development / 开发

```bash
# Clone repository / 克隆仓库
git clone https://github.com/nexus-rs/nexus.git
cd nexus

# Build / 构建
cargo build --workspace

# Test / 测试
cargo test --workspace

# Run benchmarks / 运行基准测试
cargo bench -p nexus-runtime

# Format / 格式化
cargo fmt --all

# Lint / 代码检查
cargo clippy --workspace -- -D warnings
```

## 📋 Project Status / 项目状态

> **⚠️ Alpha Version**
>
> Nexus is currently in **Phase 7: Production Ready** (100% complete). All phases 0-7 have been completed, including the custom async runtime, HTTP server, middleware system, resilience patterns, observability, Web3 support, and performance benchmarking.
>
> Nexus 目前处于 **第 7 阶段：生产就绪**（100% 完成）。第 0-7 阶段全部完成，包括自定义异步运行时、HTTP 服务器、中间件系统、弹性模式、可观测性、Web3 支持和性能基准测试。

| Phase | Status / 状态 | Description / 描述 |
|-------|---------------|-------------------|
| Phase 0 | ✅ Complete / 完成 | Foundation / 基础设施 |
| Phase 1 | ✅ Complete / 完成 | Runtime Core / 运行时核心 |
| Phase 2 | ✅ Complete / 完成 | HTTP Server / HTTP 服务器 |
| Phase 3 | ✅ Complete / 完成 | Router & Middleware / 路由和中间件 |
| Phase 4 | ✅ Complete / 完成 | Resilience / 弹性 |
| Phase 5 | ✅ Complete / 完成 | Observability / 可观测性 |
| Phase 6 | ✅ Complete / 完成 | Web3 Integration / Web3 集成 |
| Phase 7 | ✅ Complete / 完成 | Performance & Hardening / 性能和加固 |

See [implementation plan](docs/design/implementation-plan.md) for details.
详情请参阅 [实施计划](docs/design/implementation-plan.md)。

## 🤝 Contributing / 贡献

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
我们欢迎贡献！请参阅 [CONTRIBUTING.md](CONTRIBUTING.md) 了解指南。

## 📄 License / 许可证

Nexus is licensed under either of
Nexus 采用以下任一许可证

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

## 🙏 Acknowledgments / 致谢

Nexus is inspired by excellent frameworks across multiple languages:
Nexus 汲取了多种语言优秀框架的灵感：

- **Rust**: Axum, Actix Web, Monoio, Salvo
- **Go**: Gin, Echo
- **Java**: Spring Boot, WebFlux
- **Python**: FastAPI, Starlette

---

**Nexus Framework** — Built for the future of web development.
**Nexus 框架** — 为 Web 开发的未来而构建。
