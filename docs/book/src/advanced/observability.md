# Observability / 可观测性

> **Status**: Phase 5 In Progress 🔄  
> **状态**: 第5阶段进行中 🔄

Nexus provides comprehensive observability including distributed tracing, metrics, and structured logging.

Nexus 提供全面的可观测性，包括分布式追踪、指标和结构化日志。

---

## Overview / 概述

Observability helps you understand what's happening in your application:

可观测性帮助您了解应用程序中发生的情况：

```
┌─────────────────────────────────────────────────────────────┐
│              Observability Stack                             │
│              可观测性堆栈                                     │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Tracing ──► Distributed request tracking                  │
│  追踪 ──► 分布式请求追踪                                      │
│                                                              │
│  Metrics ──► Performance and business metrics              │
│  指标 ──► 性能和业务指标                                      │
│                                                              │
│  Logging ──► Structured application logs                   │
│  日志 ──► 结构化应用程序日志                                  │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## Distributed Tracing / 分布式追踪

Track requests across services:

跨服务追踪请求：

```rust
use nexus_observability::{Tracer, Span};

// Create tracer / 创建追踪器
let tracer = Tracer::new("my-service");

// Start span / 开始span
let span = tracer.span("handle_request")
    .with_attribute("user_id", "123")
    .with_attribute("method", "GET")
    .start();

// Enter span context / 进入span上下文
let _guard = span.enter();

// Do work / 执行工作
process_request().await;

// End span / 结束span
span.end();
```

**OpenTelemetry Compatibility** / **OpenTelemetry兼容性**:
- Compatible with OpenTelemetry standard
- Export to Jaeger, Zipkin, etc.

---

## Metrics / 指标

Collect application metrics:

收集应用程序指标：

```rust
use nexus_observability::{MetricsRegistry, Counter, Gauge, Histogram};

// Get metrics registry / 获取指标注册表
let metrics = MetricsRegistry::default();

// Counter - Incrementing value / 计数器 - 递增值
let requests = metrics.counter("http_requests_total")
    .with_label("method", "GET")
    .with_label("status", "200")
    .build();

requests.inc();

// Gauge - Current value / 仪表 - 当前值
let active_connections = metrics.gauge("active_connections").build();
active_connections.set(42);

// Histogram - Distribution / 直方图 - 分布
let request_duration = metrics.histogram("request_duration_seconds")
    .with_buckets(vec![0.1, 0.5, 1.0, 2.0, 5.0])
    .build();

let start = Instant::now();
process_request().await;
request_duration.observe(start.elapsed().as_secs_f64());
```

**Prometheus Compatibility** / **Prometheus兼容性**:
- Prometheus-compatible metrics
- Expose `/metrics` endpoint

---

## Structured Logging / 结构化日志

Structured logging with context:

带上下文的结构化日志：

```rust
use nexus_observability::log;
use tracing::{info, error, warn};

// Basic logging / 基本日志
log::info!("User logged in");
log::error!("Failed to process request: {}", error);

// Structured logging / 结构化日志
log::info!(
    user_id = 123,
    action = "login",
    ip = "127.0.0.1",
    "User logged in"
);

// Log levels / 日志级别
log::trace!("Detailed debug info");
log::debug!("Debug information");
log::info!("Informational message");
log::warn!("Warning message");
log::error!("Error message");
```

**Log Formats** / **日志格式**:
- **Text** - Human-readable / 人类可读
- **JSON** - Machine-readable / 机器可读

### Nexus Logging / Nexus 日志

Nexus supports structured console logging with banner, colored output, and startup information:

Nexus 支持结构化控制台日志，包括横幅、彩色输出和启动信息：

```rust
use nexus_observability::log::Logger;
#[cfg(feature = "nexus-format")]
use nexus_observability::{Banner, StartupLogger};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "nexus-format")]
    {
        // Print banner / 打印横幅
        Banner::print("MyApp", "0.1.0", 8080);

        // Initialize Nexus logging / 初始化 Nexus 日志
        Logger::init_spring_style()?;

        // Log startup information / 记录启动信息
        let startup = StartupLogger::new();
        startup.log_starting("MyApplication");
        startup.log_profile(None);
        startup.log_server_started(8080, startup.elapsed_ms());
    }

    // Use tracing macros / 使用 tracing 宏
    tracing::info!(target: "my.app", "Application running");

    Ok(())
}
```

**Output Format** / **输出格式**:
```
  _   _           ___     ___
 | | | | ___  ___| |_   / _ \ _ __ ___
 | |_| |/ _ \/ __| __| | | | | '_ ` _ \
 |  _  | (_) \__ \ |_  | |_| | | | | | |
 |_| |_|\___/|___/\__|  \___/|_| |_| |_|
MyApp v0.1.0 | port: 8080 | profile: active

2026-01-24 19:35:25.785 |INFO| 10500 [main             ] n                                : Starting Nexus application
2026-01-24 19:35:25.845 |WARN| 10500 [main             ] n.middleware.http : Client error status=400
2026-01-24 19:35:25.845 |ERR | 10500 [main             ] n.service.user : Database query failed error="User not found"
```

See [Nexus Logging Guide](../../../spring-boot-logging.md) for detailed documentation.

详细文档请参阅 [Nexus 日志指南](../../../spring-boot-logging.md)。

---

## Integration / 集成

### With HTTP Server / 与HTTP服务器集成

```rust
use nexus_observability::{tracer, metrics, log};

async fn handler(req: Request) -> Response {
    // Start span / 开始span
    let span = tracer().span("http_request")
        .with_attribute("method", req.method().as_str())
        .with_attribute("path", req.uri().path())
        .start();
    
    let _guard = span.enter();
    
    // Log request / 记录请求
    log::info!(
        method = req.method().as_str(),
        path = req.uri().path(),
        "Incoming request"
    );
    
    // Record metric / 记录指标
    metrics().counter("http_requests_total")
        .with_label("method", req.method().as_str())
        .inc();
    
    // Process request / 处理请求
    let response = process_request(req).await;
    
    response
}
```

---

## Spring Boot Comparison / Spring Boot 对比

| Spring Boot | Nexus | Description |
|-------------|-------|-------------|
| Spring Cloud Sleuth | `Tracer` | Distributed tracing |
| Micrometer | `MetricsRegistry` | Metrics collection |
| Logback/Log4j | `Logger` | Structured logging |
| Actuator | - | Health/metrics endpoints |

---

*← [Previous / 上一页](./resilience.md) | [Next / 下一页](./web3.md) →*
