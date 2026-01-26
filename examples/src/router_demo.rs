//! Router Demo Example for Nexus framework
//! Nexus框架的路由器演示示例
//!
//! Run with: cargo run --bin router_demo
//! 运行: cargo run --bin router_demo
//!
//! Demonstrates:
//! - HTTP method routing (GET, POST, PUT, DELETE)
//! - Path parameter extraction
//! - Stateful handlers with shared state
//! - Static responses
//! - Async handlers

use nexus_http::{Body, Response, Server, StatusCode};
use nexus_router::{Router, Stateful};
use nexus_runtime::task::block_on;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Application state shared across all handlers
/// 应用状态，在所有处理程序之间共享
#[derive(Clone)]
struct AppState {
    /// Request counter
    /// 请求计数器
    request_count: Arc<AtomicU64>,
}

impl AppState {
    /// Create a new application state
    /// 创建新的应用状态
    fn new() -> Self {
        Self {
            request_count: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Increment and get the request count
    /// 增加并获取请求计数
    fn increment_request_count(&self) -> u64 {
        self.request_count.fetch_add(1, Ordering::SeqCst)
    }

    /// Get the current request count
    /// 获取当前请求计数
    fn request_count(&self) -> u64 {
        self.request_count.load(Ordering::SeqCst)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    // 初始化追踪
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create shared application state
    // 创建共享应用状态
    let state = AppState::new();

    tracing::info!("Starting Nexus HTTP server on http://127.0.0.1:8080");
    tracing::info!("Available endpoints:");
    tracing::info!("  GET  /              - Hello World");
    tracing::info!("  GET  /health        - Health check");
    tracing::info!("  GET  /stats         - Request statistics (stateful)");
    tracing::info!("  GET  /users/:id     - Get user by ID (path param)");
    tracing::info!("  GET  /users         - List all users");
    tracing::info!("  POST /users         - Create user");
    tracing::info!("  PUT  /users/:id     - Update user");
    tracing::info!("  DELETE /users/:id   - Delete user");

    // Build the router with state
    // 使用状态构建路由器
    let app = Router::with_state(state.clone())
        // Root endpoint - static response
        // 根端点 - 静态响应
        .get("/", "Hello, Nexus! Welcome to the Router Demo!")

        // Health check
        // 健康检查
        .get("/health", |req: nexus_http::Request| async move {
            tracing::info!("Health check from {}", req.path());
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"status": "healthy"}"#))
                .unwrap())
        })

        // Stats endpoint - stateful handler accessing shared state
        // 统计端点 - 有状态处理程序访问共享状态
        .get("/stats", Stateful::new(|req, state: Arc<AppState>| async move {
            let count = state.request_count();
            tracing::info!("Stats request #{}", count);
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(format!(r#"{{"total_requests": {}}}"#, count)))
                .unwrap())
        }))

        // Get user by ID - path parameter extraction
        // 通过ID获取用户 - 路径参数提取
        .get("/users/:id", |req: nexus_http::Request| async move {
            // Extract path parameter
            // 提取路径参数
            let user_id = req.path_var("id").unwrap_or("unknown");

            tracing::info!("Getting user: {}", user_id);

            // In a real app, you would fetch from database
            // 在实际应用中，您将从数据库获取
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{"id": "{}", "name": "User {}", "email": "user{}@example.com"}}"#,
                    user_id, user_id, user_id
                )))
                .unwrap())
        })

        // List users
        // 列出用户
        .get("/users", |req| async move {
            tracing::info!("Listing all users");

            let users = r#"[
                {"id": "1", "name": "Alice", "email": "alice@example.com"},
                {"id": "2", "name": "Bob", "email": "bob@example.com"},
                {"id": "3", "name": "Charlie", "email": "charlie@example.com"}
            ]"#;

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(users))
                .unwrap())
        })

        // Create user
        // 创建用户
        .post("/users", |req| async move {
            tracing::info!("Creating user");

            // In a real app, you would parse request body and save to database
            // 在实际应用中，您将解析请求体并保存到数据库

            Ok(Response::builder()
                .status(StatusCode::CREATED)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"id": "4", "name": "New User", "email": "new@example.com", "created": true}"#))
                .unwrap())
        })

        // Update user
        // 更新用户
        .put("/users/:id", |req: nexus_http::Request| async move {
            let user_id = req.path_var("id").unwrap_or("unknown");
            tracing::info!("Updating user: {}", user_id);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{"id": "{}", "updated": true}}"#,
                    user_id
                )))
                .unwrap())
        })

        // Delete user
        // 删除用户
        .delete("/users/:id", |req: nexus_http::Request| async move {
            let user_id = req.path_var("id").unwrap_or("unknown");
            tracing::info!("Deleting user: {}", user_id);

            Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(Body::empty())
                .unwrap())
        });

    // Run the async server using the runtime
    // 使用运行时运行异步服务器
    block_on(async {
        let _server = Server::bind("127.0.0.1:8080").run(app).await?;

        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    })
}
