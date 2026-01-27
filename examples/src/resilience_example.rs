// Resilience Patterns Example / 弹性模式示例
//
// Demonstrates Nexus's resilience and fault-tolerance patterns:
// 演示 Nexus 的弹性和容错模式：
// - Circuit Breaker / 熔断器
//
// Equivalent to: Spring Cloud Circuit Breaker, Resilience4j
// 等价于：Spring Cloud Circuit Breaker, Resilience4j

use nexus_http::{Request, Response, Result, StatusCode};
use nexus_resilience::{
    circuit::{CircuitBreaker, CircuitBreakerConfig, CircuitState},
};
use nexus_router::Router;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Circuit breaker example / 熔断器示例
async fn circuit_breaker_example() {
    println!("\n=== Circuit Breaker Example / 熔断器示例 ===\n");

    // Configure circuit breaker / 配置熔断器
    let config = CircuitBreakerConfig::new()
        .with_error_threshold(0.5) // 50% error rate / 50%错误率
        .with_min_requests(3) // After 3 requests / 3次请求后
        .with_open_duration(Duration::from_secs(5)); // Reset timeout / 重置超时

    let breaker = Arc::new(CircuitBreaker::new("api-service", config));
    let mut failure_count = 0;

    // Simulate service calls / 模拟服务调用
    for i in 1..=10 {
        let result = simulate_service_call(&breaker, i, &mut failure_count).await;

        match result {
            Ok(_) => println!("Call {} succeeded - State: {:?}", i, breaker.state()),
            Err(e) => println!("Call {} failed - State: {:?} - Error: {}", i, breaker.state(), e),
        }

        // Small delay between calls / 调用之间的小延迟
        sleep(Duration::from_millis(100)).await;
    }

    println!("\nFinal circuit state: {:?}\n", breaker.state());
}

/// Simulate a service call with circuit breaker protection
/// 模拟带熔断器保护的服务调用
async fn simulate_service_call(
    breaker: &CircuitBreaker,
    i: usize,
    failure_count: &mut usize,
) -> Result<(), anyhow::Error> {
    // Check if circuit is open / 检查熔断器是否打开
    if !breaker.is_request_permitted() {
        return Err(anyhow::anyhow!("Circuit is open"));
    }

    // Simulate failures / 模拟失败
    if i <= 4 {
        *failure_count += 1;
        Err(anyhow::anyhow!("Service unavailable"))
    } else {
        Ok(())
    }
}

/// Complete HTTP server with circuit breaker / 包含熔断器的HTTP服务器
async fn resilience_server_example() {
    println!("\n=== Resilience Server Example / 弹性服务器示例 ===\n");

    // Create circuit breakers for different services / 为不同服务创建熔断器
    let user_breaker =
        Arc::new(CircuitBreaker::new("user-service", CircuitBreakerConfig::default()));

    let order_breaker =
        Arc::new(CircuitBreaker::new("order-service", CircuitBreakerConfig::default()));

    // Build router with circuit breaker protection / 构建带有熔断器保护的路由器
    let _app = Router::new()
        // Public endpoint / 公共端点
        .get("/api/health", |_req: Request| async {
            Ok::<_, nexus_http::Error>(Response::builder()
                .status(StatusCode::OK)
                .body(r#"{"status":"healthy"}"#.into())
                .unwrap())
        })
        // Users endpoint with circuit breaker / 带熔断器的用户端点
        .get("/api/users", move |_req: Request| {
            let breaker = user_breaker.clone();
            async move {
                if !breaker.is_request_permitted() {
                    return Ok::<_, nexus_http::Error>(Response::builder()
                        .status(StatusCode::SERVICE_UNAVAILABLE)
                        .body(r#"{"error":"Circuit breaker is open"}"#.into())
                        .unwrap());
                }

                // Simulate user service call / 模拟用户服务调用
                Ok::<_, nexus_http::Error>(Response::builder()
                    .status(StatusCode::OK)
                    .body(r#"{"users":[]}"#.into())
                    .unwrap())
            }
        })
        // Orders endpoint with circuit breaker / 带熔断器的订单端点
        .post("/api/orders", move |_req: Request| {
            let breaker = order_breaker.clone();
            async move {
                if !breaker.is_request_permitted() {
                    return Ok::<_, nexus_http::Error>(Response::builder()
                        .status(StatusCode::SERVICE_UNAVAILABLE)
                        .body(r#"{"error":"Circuit breaker is open"}"#.into())
                        .unwrap());
                }

                // Simulate order service call / 模拟订单服务调用
                Ok::<_, nexus_http::Error>(Response::builder()
                    .status(StatusCode::CREATED)
                    .body(r#"{"order":"created"}"#.into())
                    .unwrap())
            }
        });

    println!("Resilience server configured with:");
    println!("  - Circuit Breakers: user-service, order-service");
    println!("\nServer ready to handle requests with resilience!\n");
}

#[tokio::main]
async fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║   Nexus Resilience Patterns Example / 弹性模式示例            ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    // Run all examples / 运行所有示例
    circuit_breaker_example().await;
    resilience_server_example().await;

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║   All resilience examples completed!                          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}
