//! Hello World example for Nexus framework
//! Nexus框架的Hello World示例
//!
//! Run with: cargo run --bin hello_world
//! 运行: cargo run --bin hello_world

use nexus_http::{Body, Response, Server, StatusCode};
use nexus_runtime::task::block_on;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing
    // 初始化追踪
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Starting Nexus HTTP server on http://127.0.0.1:8080");

    // Run the async server using the runtime
    // 使用运行时运行异步服务器
    block_on(async {
        let _server = Server::bind("127.0.0.1:8080").run(handle_request).await?;

        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    })
}

/// Handle incoming HTTP requests
/// 处理传入的HTTP请求
async fn handle_request(req: nexus_http::Request) -> Result<Response, nexus_http::Error> {
    let path = req.path();
    let method = req.method();

    tracing::info!("Received {} {}", method, path);

    // Simple routing
    // 简单路由
    match path {
        "/" | "/health" => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "text/plain")
            .body(Body::from("Hello, Nexus!"))
            .unwrap()),

        "/api/hello" => Ok(Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "application/json")
            .body(Body::from(r#"{"message": "Hello from Nexus!"}"#))
            .unwrap()),

        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not Found"))
            .unwrap()),
    }
}
