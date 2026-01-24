# Resilience / 弹性

> **Status**: Phase 4 In Progress 🔄  
> **状态**: 第4阶段进行中 🔄

Nexus provides comprehensive resilience patterns to make your applications fault-tolerant and highly available.

Nexus 提供全面的弹性模式，使您的应用程序具有容错性和高可用性。

---

## Overview / 概述

Resilience patterns help your application handle failures gracefully:

弹性模式帮助您的应用程序优雅地处理故障：

```
┌─────────────────────────────────────────────────────────────┐
│              Resilience Patterns                            │
│              弹性模式                                         │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Circuit Breaker ──► Fail fast when service is down         │
│  熔断器 ──► 服务关闭时快速失败                                 │
│                                                              │
│  Rate Limiter ──► Control request rate                      │
│  限流器 ──► 控制请求速率                                      │
│                                                              │
│  Retry ──► Automatic retry with backoff                     │
│  重试 ──► 带退避的自动重试                                    │
│                                                              │
│  Timeout ──► Prevent hanging requests                       │
│  超时 ──► 防止挂起请求                                        │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Circuit Breaker / 熔断器

Protect against cascading failures:

防止级联故障：

```rust
use nexus_resilience::{CircuitBreaker, CircuitBreakerConfig};
use std::time::Duration;

// Create circuit breaker / 创建熔断器
let circuit = CircuitBreaker::new(CircuitBreakerConfig {
    error_threshold: 0.5,              // Open after 50% errors
    min_requests: 10,                   // Need 10 requests
    timeout: Duration::from_secs(60),   // Stay open for 60s
    half_open_max_calls: 3,             // Test with 3 calls
});

// Use with async operation / 与异步操作一起使用
async fn call_external_api() -> Result<String, Error> {
    circuit.call(|| async {
        http_client.get("https://api.example.com/data").await
    }).await
}
```

**Circuit States** / **熔断器状态**:
- **CLOSED** - Normal operation / 正常运行
- **OPEN** - Failing fast / 快速失败
- **HALF-OPEN** - Testing recovery / 测试恢复

---

## Rate Limiter / 限流器

Control request rate:

控制请求速率：

```rust
use nexus_resilience::{RateLimiter, RateLimiterType};

// Token bucket rate limiter / Token bucket限流器
let limiter = RateLimiter::builder()
    .rate(100)                         // 100 requests per second
    .capacity(200)                     // Burst up to 200
    .limiter_type(RateLimiterType::TokenBucket)
    .build();

// Use with async operation / 与异步操作一起使用
async fn process_request() -> Result<Response, Error> {
    limiter.acquire().await?;  // Wait for permit
    handle_request().await
}
```

---

## Retry Policy / 重试策略

Automatic retry with backoff:

带退避的自动重试：

```rust
use nexus_resilience::{RetryPolicy, retry};
use std::time::Duration;

// Exponential backoff / 指数退避
let policy = RetryPolicy::exponential_backoff(
    3,                              // Max 3 retries
    Duration::from_secs(1),          // Initial delay 1s
);

// Use with async operation / 与异步操作一起使用
async fn call_api() -> Result<String, Error> {
    retry(policy, || async {
        http_client.get("https://api.example.com").await
    }).await
}
```

---

## Timeout / 超时

Enforce request timeouts:

强制执行请求超时：

```rust
use nexus_resilience::Timeout;
use std::time::Duration;

// Create timeout / 创建超时
let timeout = Timeout::new(Duration::from_secs(5));

// Use with async operation / 与异步操作一起使用
async fn call_slow_api() -> Result<String, Error> {
    timeout.timeout(|| async {
        slow_api_call().await
    }).await
}
```

---

## Combining Patterns / 组合模式

Combine multiple resilience patterns:

组合多个弹性模式：

```rust
use nexus_resilience::{CircuitBreaker, RateLimiter, RetryPolicy};

struct ResilientClient {
    circuit: CircuitBreaker,
    limiter: RateLimiter,
    retry: RetryPolicy,
}

impl ResilientClient {
    async fn call(&self, request: Request) -> Result<Response, Error> {
        // 1. Rate limit / 限流
        self.limiter.acquire().await?;
        
        // 2. Circuit breaker / 熔断器
        let result = self.circuit.call(|| async {
            // 3. Retry / 重试
            retry(self.retry.clone(), || async {
                http_client.send(request.clone()).await
            }).await
        }).await;
        
        result
    }
}
```

---

## Spring Boot Comparison / Spring Boot 对比

| Spring Boot | Nexus | Description |
|-------------|-------|-------------|
| `@CircuitBreaker` | `CircuitBreaker` | Circuit breaker pattern |
| `@RateLimiter` | `RateLimiter` | Rate limiting |
| `@Retry` | `RetryPolicy` | Retry with backoff |
| `@Timeout` | `Timeout` | Request timeout |
| Resilience4j | `nexus-resilience` | Resilience library |

---

## Best Practices / 最佳实践

1. **Use circuit breakers for external calls** / **对外部调用使用熔断器**
2. **Rate limit per client** / **每客户端限流**
3. **Retry with exponential backoff** / **指数退避重试**
4. **Set appropriate timeouts** / **设置适当的超时**

---

*← [Previous / 上一页](../core-concepts/extractors.md) | [Next / 下一页](./observability.md) →*
