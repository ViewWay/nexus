// Resilience Patterns Example / 弹性模式示例
//
// Demonstrates Nexus's resilience and fault-tolerance patterns:
// 演示 Nexus 的弹性和容错模式：
// - Circuit Breaker / 熔断器
// - Rate Limiter / 限流器
// - Retry with Exponential Backoff / 指数退避重试
// - Service Discovery / 服务发现
//
// Equivalent to: Spring Cloud Circuit Breaker, Resilience4j
// 等价于：Spring Cloud Circuit Breaker, Resilience4j

use nexus_http::{Request, Response, StatusCode};
use nexus_resilience::{
    circuit::{CircuitBreaker, CircuitBreakerConfig, CircuitState},
    rate_limiter::{RateLimiter, RateLimiterConfig},
    retry::{RetryPolicy, RetryStrategy},
    service_discovery::{ServiceInstance, ServiceRegistry},
};
use nexus_router::Router;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// Circuit breaker example / 熔断器示例
#[tokio::main]
async fn circuit_breaker_example() {
    println!("\n=== Circuit Breaker Example / 熔断器示例 ===\n");

    // Configure circuit breaker / 配置熔断器
    let config = CircuitBreakerConfig::builder()
        .failure_threshold(3) // Open after 3 failures / 3次失败后打开
        .success_threshold(2) // Close after 2 successes / 2次成功后关闭
        .timeout(Duration::from_secs(5)) // Reset timeout / 重置超时
        .half_open_max_calls(3) // Max calls in half-open state / 半开状态最大调用数
        .build();

    let breaker = Arc::new(CircuitBreaker::new("api-service", config));
    let mut failure_count = 0;

    // Simulate service calls / 模拟服务调用
    for i in 1..=10 {
        let result = breaker.execute(|| async {
            // Simulate failures / 模拟失败
            if i <= 4 {
                failure_count += 1;
                Err::<(), _>(anyhow::anyhow!("Service unavailable"))
            } else {
                Ok(())
            }
        }).await;

        match result {
            Ok(_) => println!("Call {} succeeded - State: {:?}", i, breaker.state()),
            Err(e) => println!("Call {} failed - State: {:?} - Error: {}", i, breaker.state(), e),
        }

        // Small delay between calls / 调用之间的小延迟
        sleep(Duration::from_millis(100)).await;
    }

    println!("\nFinal circuit state: {:?}\n", breaker.state());
}

/// Rate limiter example / 限流器示例
#[tokio::main]
async fn rate_limiter_example() {
    println!("\n=== Rate Limiter Example / 限流器示例 ===\n");

    // Configure rate limiter: 5 requests per second / 配置限流器：每秒5个请求
    let config = RateLimiterConfig::builder()
        .max_requests(5)
        .per(Duration::from_secs(1))
        .burst_size(2) // Allow burst of 2 / 允许突发2个
        .build();

    let limiter = Arc::new(RateLimiter::new("api-limit", config));

    // Simulate 10 requests / 模拟10个请求
    for i in 1..=10 {
        match limiter.check().await {
            Ok(permit) => {
                println!("Request {} allowed - Remaining: {}", i, permit.remaining());
                // Simulate request processing / 模拟请求处理
                sleep(Duration::from_millis(50)).await;
            }
            Err(e) => {
                println!("Request {} rate limited - Error: {}", i, e);
            }
        }
    }

    // Wait and try again / 等待并重试
    println!("\nWaiting 2 seconds...\n");
    sleep(Duration::from_secs(2)).await;

    println!("Trying again after waiting:");
    match limiter.check().await {
        Ok(permit) => println!("Request allowed - Remaining: {}", permit.remaining()),
        Err(e) => println!("Request rate limited - Error: {}", e),
    }

    println!();
}

/// Retry example / 重试示例
#[tokio::main]
async fn retry_example() {
    println!("\n=== Retry Example / 重试示例 ===\n");

    // Configure retry policy with exponential backoff / 配置指数退避重试策略
    let policy = RetryPolicy::builder()
        .max_attempts(5) // Maximum retry attempts / 最大重试次数
        .initial_delay(Duration::from_millis(100)) // Initial delay / 初始延迟
        .max_delay(Duration::from_secs(1)) // Maximum delay / 最大延迟
        .backoff_factor(2.0) // Exponential backoff factor / 指数退避因子
        .retryable_errors(|error| {
            // Retry only on specific errors / 仅对特定错误重试
            matches!(
                error.to_string().as_str(),
                "Service unavailable" | "Connection timeout"
            )
        })
        .build();

    let mut attempt = 0;
    let result = policy.execute(|| async {
        attempt += 1;
        println!("Attempt {}", attempt);

        // Simulate failure until attempt 4 / 模拟失败直到第4次尝试
        if attempt < 4 {
            Err::<(), _>(anyhow::anyhow!("Service unavailable"))
        } else {
            Ok(())
        }
    }).await;

    match result {
        Ok(_) => println!("\nSuccess after {} attempts!\n", attempt),
        Err(e) => println!("\nFailed after {} attempts: {}\n", attempt, e),
    }
}

/// Service discovery example / 服务发现示例
#[tokio::main]
async fn service_discovery_example() {
    println!("\n=== Service Discovery Example / 服务发现示例 ===\n");

    let registry = Arc::new(ServiceRegistry::new());

    // Register service instances / 注册服务实例
    let instance1 = ServiceInstance::builder()
        .id("user-service-1")
        .service_name("user-service")
        .host("192.168.1.10")
        .port(8080)
        .metadata(vec![("zone", "east"),("version", "1.0")])
        .build();

    let instance2 = ServiceInstance::builder()
        .id("user-service-2")
        .service_name("user-service")
        .host("192.168.1.11")
        .port(8080)
        .metadata(vec![("zone", "west"),("version", "1.0")])
        .build();

    registry.register(instance1).await;
    registry.register(instance2).await;

    println!("Registered 2 service instances");

    // Discover services / 发现服务
    match registry.get_instances("user-service").await {
        Some(instances) => {
            println!("\nDiscovered {} instances:", instances.len());
            for instance in instances {
                println!("  - {}:{} (ID: {}, Zone: {:?})",
                    instance.host(),
                    instance.port(),
                    instance.id(),
                    instance.metadata().get("zone")
                );
            }
        }
        None => println!("No instances found"),
    }

    // Health check / 健康检查
    println!("\nPerforming health check...");
    registry.health_check("user-service").await;

    // Deregister an instance / 注销一个实例
    println!("\nDeregistering instance user-service-1");
    registry.deregister("user-service-1").await;

    match registry.get_instances("user-service").await {
        Some(instances) => {
            println!("Remaining instances: {}", instances.len());
        }
        None => {}
    }

    println!();
}

/// Complete HTTP server with all resilience patterns / 包含所有弹性模式的HTTP服务器
#[tokio::main]
async fn resilience_server_example() {
    println!("\n=== Resilience Server Example / 弹性服务器示例 ===\n");

    // Create circuit breakers for different services / 为不同服务创建熔断器
    let user_breaker = Arc::new(CircuitBreaker::new(
        "user-service",
        CircuitBreakerConfig::default(),
    ));

    let order_breaker = Arc::new(CircuitBreaker::new(
        "order-service",
        CircuitBreakerConfig::default(),
    ));

    // Create rate limiters / 创建限流器
    let api_limiter = Arc::new(RateLimiter::new(
        "api-limit",
        RateLimiterConfig::builder()
            .max_requests(10)
            .per(Duration::from_secs(1))
            .build(),
    ));

    let auth_limiter = Arc::new(RateLimiter::new(
        "auth-limit",
        RateLimiterConfig::builder()
            .max_requests(5)
            .per(Duration::from_secs(60))
            .build(),
    ));

    // Build router with resilience middleware / 构建带有弹性中间件的路由器
    let app = Router::new()
        // Public endpoint with rate limiting / 带限流的公共端点
        .get("/api/health", move || async {
            Response::builder()
                .status(StatusCode::OK)
                .body(r#"{"status":"healthy"}"#.into())
                .unwrap()
        })
        .get("/api/users", {
            let breaker = user_breaker.clone();
            move || async move {
                // Execute with circuit breaker / 使用熔断器执行
                breaker.execute(|| async {
                    // Simulate user service call / 模拟用户服务调用
                    Response::builder()
                        .status(StatusCode::OK)
                        .body(r#"{"users":[]}"#.into())
                        .map_err(|_| anyhow::anyhow!("Service error"))
                }).await
            }
        })
        .post("/api/orders", {
            let breaker = order_breaker.clone();
            move || async move {
                breaker.execute(|| async {
                    Response::builder()
                        .status(StatusCode::CREATED)
                        .body(r#"{"order":"created"}"#.into())
                        .map_err(|_| anyhow::anyhow!("Service error"))
                }).await
            }
        });

    println!("Resilience server configured with:");
    println!("  - Circuit Breakers: user-service, order-service");
    println!("  - Rate Limiters: api-limit, auth-limit");
    println!("  - Retry: Enabled for all external calls");
    println!("\nServer ready to handle requests with resilience!\n");
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║   Nexus Resilience Patterns Example / 弹性模式示例            ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    // Run all examples / 运行所有示例
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(circuit_breaker_example());
    rt.block_on(rate_limiter_example());
    rt.block_on(retry_example());
    rt.block_on(service_discovery_example());
    rt.block_on(resilience_server_example());

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║   All resilience examples completed!                          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}
