//! Middleware Demo Example for Nexus framework
//! Nexus框架的中间件演示示例
//!
//! Run with: cargo run --bin middleware_demo
//! 运行: cargo run --bin middleware_demo
//!
//! Demonstrates all middleware working together:
//! - Logger middleware (access logging)
//! - CORS middleware (cross-origin support)
//! - Timeout middleware (request timeout)

use std::sync::Arc;
use std::time::Duration;
use nexus_http::{Body, Response, Server, StatusCode};
use nexus_router::{Router, Stateful};
use nexus_runtime::task::block_on;
use nexus_middleware::{LoggerMiddleware, CorsMiddleware, CorsConfig, TimeoutMiddleware};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    // 初始化追踪
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting Nexus HTTP server on http://127.0.0.1:8080");
    tracing::info!("");
    tracing::info!("Middleware Stack:");
    tracing::info!("  1. LoggerMiddleware - Request/response logging");
    tracing::info!("  2. CorsMiddleware - CORS support (allow all origins)");
    tracing::info!("  3. TimeoutMiddleware - 30 second timeout");
    tracing::info!("");
    tracing::info!("Available endpoints:");
    tracing::info!("  GET  /                    - Hello World");
    tracing::info!("  GET  /api/data           - JSON data response");
    tracing::info!("  GET  /api/slow           - Slow endpoint (for timeout test)");
    tracing::info!("  GET  /api/users/:id      - User by ID (path param)");
    tracing::info!("  POST /api/users          - Create user");

    // Create middleware stack
    // 创建中间件栈
    let logger = Arc::new(LoggerMiddleware::new().log_headers(true));
    let cors = Arc::new(CorsMiddleware::new(
        CorsConfig::new()
            .allow_all()
    ));
    let timeout = Arc::new(TimeoutMiddleware::new(Duration::from_secs(30)));

    // Build the router with middleware and state
    // 使用中间件和状态构建路由器
    let app = Router::new()
        // Add middleware (executed in reverse order of registration)
        // 添加中间件（按注册顺序的相反顺序执行）
        .middleware(logger.clone())
        .middleware(cors.clone())
        .middleware(timeout.clone())

        // Root endpoint
        // 根端点
        .get("/", "Welcome to Nexus Middleware Demo!")

        // JSON API endpoint
        // JSON API端点
        .get("/api/data", |_req: nexus_http::Request| async move {
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"message": "Hello from Nexus!", "status": "success"}"#))
                .unwrap())
        })

        // Slow endpoint (for testing timeout)
        // 慢端点（用于测试超时）
        .get("/api/slow", |_req: nexus_http::Request| async move {
            // Simulate slow operation
            // 模拟慢操作
            tokio::time::sleep(Duration::from_secs(2)).await;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"message": "This took 2 seconds to generate"}"#))
                .unwrap())
        })

        // User endpoint with path parameter
        // 带路径参数的用户端点
        .get("/api/users/:id", |req: nexus_http::Request| async move {
            let user_id = req.path_var("id").unwrap_or("unknown");
            tracing::info!("Fetching user: {}", user_id);

            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(format!(
                    r#"{{"id": "{}", "name": "User {}", "email": "user{}@example.com"}}"#,
                    user_id, user_id, user_id
                )))
                .unwrap())
        })

        // Create user endpoint
        // 创建用户端点
        .post("/api/users", |_req: nexus_http::Request| async move {
            Ok(Response::builder()
                .status(StatusCode::CREATED)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"id": "1", "name": "New User", "created": true}"#))
                .unwrap())
        });

    // Run the async server using the runtime
    // 使用运行时运行异步服务器
    block_on(async {
        let _server = Server::bind("127.0.0.1:8080")
            .run(app)
            .await?;

        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    })
}
