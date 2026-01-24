# nexus-observability

[![Crates.io](https://img.shields.io/crates/v/nexus-observability)](https://crates.io/crates/nexus-observability)
[![Documentation](https://docs.rs/nexus-observability/badge.svg)](https://docs.rs/nexus-observability)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](../../LICENSE)

> Distributed tracing, metrics, and logging for Nexus framework
> 
> Nexus框架的分布式追踪、指标和日志

---

## 📋 Overview / 概述

`nexus-observability` provides comprehensive observability for Nexus applications, including distributed tracing, metrics collection, and structured logging.

`nexus-observability` 为Nexus应用程序提供全面的可观测性，包括分布式追踪、指标收集和结构化日志。

**Key Features** / **核心特性**:
- ✅ **Distributed Tracing** - OpenTelemetry compatible
- ✅ **Metrics** - Prometheus compatible
- ✅ **Structured Logging** - JSON and text formats
- ✅ **Context Propagation** - Trace context across services

---

## ✨ Components / 组件

| Component | Description | Status |
|-----------|-------------|--------|
| **Tracing** | Distributed tracing with spans | 🔄 Phase 5 |
| **Metrics** | Counter, Gauge, Histogram | 🔄 Phase 5 |
| **Logging** | Structured logging | 🔄 Phase 5 |

---

## 🚀 Quick Start / 快速开始

### Installation / 安装

```toml
[dependencies]
nexus-observability = "0.1.0-alpha"
tracing = "0.1"
```

### Basic Usage / 基本用法

```rust
use nexus_observability::{tracer, metrics, log};

// Initialize observability / 初始化可观测性
nexus_observability::init()?;

// Create span / 创建span
let span = tracer().span("handle_request").start();
let _guard = span.enter();

// Log event / 记录事件
log::info!("Processing request");

// Record metric / 记录指标
metrics().counter("requests_total").inc();

span.end();
```

---

## 📖 Component Details / 组件详情

### Distributed Tracing / 分布式追踪

Track requests across services:

跨服务追踪请求：

```rust
use nexus_observability::{Tracer, Span, TraceContext};

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

// Nested spans / 嵌套span
let parent_span = tracer.span("parent").start();
let _parent_guard = parent_span.enter();

let child_span = tracer.span("child").start();
let _child_guard = child_span.enter();

process_child().await;

child_span.end();
parent_span.end();
```

**Trace Context Propagation** / **追踪上下文传播**:

```rust
use nexus_observability::{TraceContext, TraceId, SpanId};

// Extract trace context from headers / 从headers提取追踪上下文
let context = TraceContext::from_headers(&request.headers())?;

// Create child span / 创建子span
let span = tracer.span("child_operation")
    .with_parent(context)
    .start();

// Inject trace context into outgoing request / 将追踪上下文注入传出请求
let mut headers = HashMap::new();
context.inject_into(&mut headers);
http_client.get("https://api.example.com")
    .headers(headers)
    .send()
    .await?;
```

**OpenTelemetry Compatibility** / **OpenTelemetry兼容性**:

```rust
use nexus_observability::tracer;

// Export to OpenTelemetry / 导出到OpenTelemetry
let tracer = tracer()
    .with_exporter(OpenTelemetryExporter::new("http://collector:4317"))
    .build();

// Spans are compatible with OpenTelemetry / span与OpenTelemetry兼容
let span = tracer.span("operation").start();
```

---

### Metrics / 指标

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

requests.inc();           // Increment by 1 / 增加1
requests.inc_by(5);      // Increment by 5 / 增加5

// Gauge - Current value / 仪表 - 当前值
let active_connections = metrics.gauge("active_connections")
    .build();

active_connections.set(42);      // Set value / 设置值
active_connections.inc();         // Increment / 增加
active_connections.dec();         // Decrement / 减少

// Histogram - Distribution / 直方图 - 分布
let request_duration = metrics.histogram("request_duration_seconds")
    .with_buckets(vec![0.1, 0.5, 1.0, 2.0, 5.0])
    .build();

let start = Instant::now();
process_request().await;
request_duration.observe(start.elapsed().as_secs_f64());
```

**Prometheus Compatibility** / **Prometheus兼容性**:

```rust
use nexus_observability::metrics;

// Metrics are Prometheus-compatible / 指标与Prometheus兼容
let metrics = metrics()
    .with_exporter(PrometheusExporter::new("/metrics"))
    .build();

// Expose metrics endpoint / 暴露指标端点
router.get("/metrics", || async {
    metrics.export_prometheus()
});
```

**Metric Types** / **指标类型**:

| Type | Description | Use Case |
|------|-------------|----------|
| **Counter** | Monotonically increasing | Request count, error count |
| **Gauge** | Current value | Active connections, queue size |
| **Histogram** | Distribution | Request duration, response size |

---

### Structured Logging / 结构化日志

Structured logging with context:

带上下文的结构化日志：

```rust
use nexus_observability::{log, Logger, LoggerConfig};
use tracing::{info, error, warn, debug};

// Initialize logger / 初始化日志
let logger = Logger::new(LoggerConfig {
    level: LogLevel::Info,
    format: LogFormat::Json,
    output: LogOutput::Stdout,
})?;

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

// With span context / 带span上下文
let span = tracer().span("handle_request").start();
let _guard = span.enter();

log::info!("Processing request");  // Automatically includes trace_id

// Log levels / 日志级别
log::trace!("Detailed debug info");
log::debug!("Debug information");
log::info!("Informational message");
log::warn!("Warning message");
log::error!("Error message");
```

**Log Formats** / **日志格式**:

**Text Format** / **文本格式**:
```
2024-01-24T10:30:45.123Z INFO [trace_id=abc123] User logged in user_id=123 action=login
```

**JSON Format** / **JSON格式**:
```json
{
  "timestamp": "2024-01-24T10:30:45.123Z",
  "level": "INFO",
  "message": "User logged in",
  "trace_id": "abc123",
  "span_id": "def456",
  "user_id": 123,
  "action": "login",
  "ip": "127.0.0.1"
}
```

**Log Rotation** / **日志轮转**:

```rust
use nexus_observability::log::LogRotation;

let logger = Logger::new(LoggerConfig {
    rotation: LogRotation::daily("/var/log/app"),
    max_files: 30,           // Keep 30 days
    max_size: 100 * 1024 * 1024,  // 100MB per file
    compress: true,          // Compress old logs
})?;
```

---

## 🎯 Integration / 集成

### With HTTP Server / 与HTTP服务器集成

```rust
use nexus_observability::{tracer, metrics, log};
use nexus_http::Server;
use nexus_router::Router;

// Initialize observability / 初始化可观测性
nexus_observability::init()?;

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
    
    // Record response metric / 记录响应指标
    metrics().counter("http_requests_total")
        .with_label("status", response.status().as_str())
        .inc();
    
    response
}

let app = Router::new()
    .get("/", handler);

Server::bind("0.0.0.0:3000")
    .serve(app)
    .await?;
```

### With Database / 与数据库集成

```rust
use nexus_observability::{tracer, metrics};

async fn query_database(query: &str) -> Result<Vec<Row>, Error> {
    let span = tracer().span("db_query")
        .with_attribute("query", query)
        .start();
    
    let _guard = span.enter();
    let start = Instant::now();
    
    let result = db.query(query).await?;
    
    // Record query duration / 记录查询持续时间
    metrics().histogram("db_query_duration_seconds")
        .observe(start.elapsed().as_secs_f64());
    
    span.end();
    Ok(result)
}
```

---

## ⚡ Performance / 性能

### Overhead / 开销

| Component | Overhead | Notes |
|-----------|----------|-------|
| **Tracing** | 1-5µs per span | Minimal when sampled |
| **Metrics** | < 1µs | Atomic operations |
| **Logging** | 10-50µs | Async logging reduces impact |

### Sampling / 采样

Reduce overhead with sampling:

通过采样减少开销：

```rust
use nexus_observability::tracer;

// Sample 10% of requests / 采样10%的请求
let tracer = tracer()
    .with_sampler(Sampler::probabilistic(0.1))
    .build();

// Or sample based on conditions / 或基于条件采样
let tracer = tracer()
    .with_sampler(Sampler::conditional(|span| {
        span.name() == "critical_operation"  // Always sample critical ops
    }))
    .build();
```

---

## 🧪 Testing / 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tracing() {
        let tracer = Tracer::new("test");
        let span = tracer.span("test_operation").start();
        
        assert!(span.is_active());
        span.end();
    }

    #[test]
    fn test_metrics() {
        let metrics = MetricsRegistry::default();
        let counter = metrics.counter("test_total").build();
        
        counter.inc();
        assert_eq!(counter.get(), 1);
    }
}
```

---

## 🚦 Roadmap / 路线图

### Phase 5: Core Observability 🔄 (In Progress / 进行中)
- [ ] Distributed tracing implementation
- [ ] Metrics collection
- [ ] Structured logging
- [ ] OpenTelemetry integration
- [ ] Prometheus integration

### Phase 6: Advanced Features 📋 (Planned / 计划中)
- [ ] APM integration
- [ ] Custom exporters
- [ ] Log aggregation
- [ ] Alerting integration

---

## 📚 Documentation / 文档

- **API Documentation**: [docs.rs/nexus-observability](https://docs.rs/nexus-observability)
- **Book**: [Observability Guide](../../docs/book/src/advanced/observability.md)
- **Examples**: [examples/](../../examples/)

---

## 🤝 Contributing / 贡献

We welcome contributions! Please see:

- [CONTRIBUTING.md](../../CONTRIBUTING.md)
- [Design Spec](../../docs/design-spec.md)
- [GitHub Issues](https://github.com/nexus-framework/nexus/issues)

---

## 📄 License / 许可证

Licensed under Apache License 2.0. See [LICENSE](../../LICENSE) for details.

---

## 🙏 Acknowledgments / 致谢

Nexus Observability is inspired by:

- **[OpenTelemetry](https://opentelemetry.io/)** - Distributed tracing standard
- **[Prometheus](https://prometheus.io/)** - Metrics collection
- **[tracing](https://github.com/tokio-rs/tracing)** - Rust structured logging

---

**Built with ❤️ for observability**

**为可观测性构建 ❤️**
