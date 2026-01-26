// Actuator Example / 监控端点示例
//
// Demonstrates Nexus's production-ready monitoring features:
// 演示 Nexus 的生产级监控功能：
// - Health checks / 健康检查
// - Metrics collection / 指标收集
// - System information / 系统信息
// - Application status / 应用状态
//
// Equivalent to: Spring Boot Actuator, Micrometer
// 等价于：Spring Boot Actuator, Micrometer

use nexus_actuator::{
    endpoint::{Endpoint, EndpointHandler},
    health::{HealthEndpoint, HealthIndicator, HealthStatus},
    info::{BuildInfo, GitInfo, InfoEndpoint},
    metrics::{MetricRegistry, MetricsEndpoint},
};
use nexus_http::{Request, Response, StatusCode};
use nexus_router::Router;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Application health indicators / 应用健康指标
struct AppHealthIndicators {
    database: DatabaseHealthIndicator,
    redis: RedisHealthIndicator,
    external_api: ExternalApiHealthIndicator,
}

/// Database health indicator / 数据库健康指标
struct DatabaseHealthIndicator {
    connection_pool_size: u32,
    active_connections: u32,
}

impl HealthIndicator for DatabaseHealthIndicator {
    fn name(&self) -> &str {
        "database"
    }

    fn check_health(&self) -> HealthStatus {
        let status = if self.active_connections < self.connection_pool_size {
            HealthStatus::Up
        } else {
            HealthStatus::Down {
                reason: "Connection pool exhausted".to_string(),
            }
        };

        let mut details = HashMap::new();
        details.insert("pool_size".to_string(), serde_json::json!(self.connection_pool_size));
        details.insert("active".to_string(), serde_json::json!(self.active_connections));
        details.insert(
            "idle".to_string(),
            serde_json::json!(self.connection_pool_size - self.active_connections),
        );

        HealthStatus::with_details(status, details)
    }
}

/// Redis health indicator / Redis 健康指标
struct RedisHealthIndicator {
    connected: bool,
    memory_used: u64,
    memory_max: u64,
}

impl HealthIndicator for RedisHealthIndicator {
    fn name(&self) -> &str {
        "redis"
    }

    fn check_health(&self) -> HealthStatus {
        let status = if self.connected {
            let usage_percent = (self.memory_used as f64 / self.memory_max as f64) * 100.0;
            if usage_percent > 90.0 {
                HealthStatus::Warning {
                    reason: format!("Memory usage high: {:.1}%", usage_percent),
                }
            } else {
                HealthStatus::Up
            }
        } else {
            HealthStatus::Down {
                reason: "Not connected".to_string(),
            }
        };

        let mut details = HashMap::new();
        details.insert("connected".to_string(), serde_json::json!(self.connected));
        details.insert("memory_used".to_string(), serde_json::json!(self.memory_used));
        details.insert("memory_max".to_string(), serde_json::json!(self.memory_max));

        HealthStatus::with_details(status, details)
    }
}

/// External API health indicator / 外部 API 健康指标
struct ExternalApiHealthIndicator {
    last_response_time_ms: u64,
    error_rate_percent: f64,
}

impl HealthIndicator for ExternalApiHealthIndicator {
    fn name(&self) -> &str {
        "externalApi"
    }

    fn check_health(&self) -> HealthStatus {
        let status = if self.error_rate_percent > 50.0 {
            HealthStatus::Down {
                reason: format!("High error rate: {:.1}%", self.error_rate_percent),
            }
        } else if self.error_rate_percent > 10.0 {
            HealthStatus::Warning {
                reason: format!("Elevated error rate: {:.1}%", self.error_rate_percent),
            }
        } else {
            HealthStatus::Up
        };

        let mut details = HashMap::new();
        details
            .insert("response_time_ms".to_string(), serde_json::json!(self.last_response_time_ms));
        details
            .insert("error_rate_percent".to_string(), serde_json::json!(self.error_rate_percent));

        HealthStatus::with_details(status, details)
    }
}

/// Health check example / 健康检查示例
#[tokio::main]
async fn health_check_example() {
    println!("\n=== Health Check Example / 健康检查示例 ===\n");

    let indicators = AppHealthIndicators {
        database: DatabaseHealthIndicator {
            connection_pool_size: 10,
            active_connections: 3,
        },
        redis: RedisHealthIndicator {
            connected: true,
            memory_used: 512 * 1024 * 1024, // 512MB
            memory_max: 1024 * 1024 * 1024, // 1GB
        },
        external_api: ExternalApiHealthIndicator {
            last_response_time_ms: 120,
            error_rate_percent: 2.5,
        },
    };

    let health_endpoint = HealthEndpoint::new();

    // Register health indicators / 注册健康指标
    health_endpoint.register_indicator(indicators.database);
    health_endpoint.register_indicator(indicators.redis);
    health_endpoint.register_indicator(indicators.external_api);

    // Check overall health / 检查整体健康状态
    let health = health_endpoint.health().await;

    println!("Overall Status: {:?}", health.status);
    println!("\nComponent Health:");

    for (name, component) in &health.components {
        println!("  {}:", name);
        println!("    Status: {:?}", component.status);
        if let Some(details) = &component.details {
            println!("    Details:");
            for (key, value) in details {
                println!("      {}: {}", key, value);
            }
        }
    }

    println!();
}

/// Metrics collection example / 指标收集示例
#[tokio::main]
async fn metrics_example() {
    println!("\n=== Metrics Collection Example / 指标收集示例 ===\n");

    let registry = MetricRegistry::new();
    let metrics_endpoint = MetricsEndpoint::new(registry.clone());

    // Record some metrics / 记录一些指标
    let counter = registry.counter("http.requests.total");
    counter.increment();
    counter.increment_by(5.0);

    let gauge = registry.gauge("http.connections.active");
    gauge.set(42.0);

    let histogram = registry.histogram("http.request.duration");
    histogram.record(50.0);
    histogram.record(100.0);
    histogram.record(75.0);

    println!("Metrics collected:");
    println!("  http.requests.total: {}", counter.count());
    println!("  http.connections.active: {}", gauge.value());
    println!("  http.request.duration.count: {}", histogram.count());
    println!("  http.request.duration.mean: {:.2}", histogram.mean());
    println!("  http.request.duration.p95: {:.2}", histogram.percentile(95.0));

    // Get all metrics as JSON / 获取所有指标的 JSON
    let metrics_json = metrics_endpoint.metrics().await;
    println!("\nAll metrics (JSON):");
    println!("{}", serde_json::to_string_pretty(&metrics_json).unwrap());

    println!();
}

/// System information example / 系统信息示例
#[tokio::main]
async fn system_info_example() {
    println!("\n=== System Information Example / 系统信息示例 ===\n");

    let info_endpoint = InfoEndpoint::new();

    // Build information / 构建信息
    let build_info = BuildInfo {
        name: "nexus-example".to_string(),
        version: "0.1.0-alpha".to_string(),
        artifact: "nexus-examples".to_string(),
        group: "io.nexus".to_string(),
        time: "2024-01-24T12:00:00Z".to_string(),
    };

    // Git information / Git 信息
    let git_info = GitInfo {
        branch: "main".to_string(),
        commit: "abc123def456".to_string(),
        commit_time: "2024-01-24T12:00:00Z".to_string(),
    };

    info_endpoint.set_build_info(build_info);
    info_endpoint.set_git_info(git_info);

    // Java/OS information (simulated for Rust) / Java/OS 信息（Rust 模拟）
    let mut info = info_endpoint.info().await;

    println!("Application Information:");
    println!(
        "  Name: {}",
        info.build
            .as_ref()
            .map(|b| &b.name)
            .unwrap_or(&"Unknown".to_string())
    );
    println!(
        "  Version: {}",
        info.build
            .as_ref()
            .map(|b| &b.version)
            .unwrap_or(&"Unknown".to_string())
    );
    println!(
        "  Artifact: {}",
        info.build
            .as_ref()
            .map(|b| &b.artifact)
            .unwrap_or(&"Unknown".to_string())
    );

    if let Some(git) = info.git {
        println!("\nGit Information:");
        println!("  Branch: {}", git.branch);
        println!("  Commit: {}", git.commit);
    }

    // Add system info / 添加系统信息
    let mut sys_info = HashMap::new();
    sys_info.insert("os".to_string(), serde_json::json!(std::env::consts::OS));
    sys_info.insert("arch".to_string(), serde_json::json!(std::env::consts::ARCH));
    sys_info.insert("rust_version".to_string(), serde_json::json!("1.93.0"));

    info.env = Some(sys_info);

    println!("\nSystem Information:");
    if let Some(env) = info.env {
        for (key, value) in env {
            println!("  {}: {}", key, value);
        }
    }

    println!();
}

/// Complete actuator server / 完整的监控服务器
async fn actuator_server_example() {
    println!("\n=== Actuator Server Example / 监控服务器示例 ===\n");

    let health_endpoint = HealthEndpoint::new();
    let metrics_endpoint = MetricsEndpoint::new(MetricRegistry::new());
    let info_endpoint = InfoEndpoint::new();

    // Register health indicators / 注册健康指标
    health_endpoint.register_indicator(DatabaseHealthIndicator {
        connection_pool_size: 10,
        active_connections: 3,
    });

    health_endpoint.register_indicator(RedisHealthIndicator {
        connected: true,
        memory_used: 512 * 1024 * 1024,
        memory_max: 1024 * 1024 * 1024,
    });

    // Build router with actuator endpoints / 构建带有监控端点的路由器
    let app = Router::new()
        // Health endpoint / 健康端点
        .get("/actuator/health", move || async move {
            let health = health_endpoint.health().await;
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&health).unwrap().into())
                .unwrap()
        })
        // Health with detailed breakdown / 带详细分解的健康端点
        .get("/actuator/health/{component}", move |component: String| async move {
            let health = health_endpoint.component_health(&component).await;
            Response::builder()
                .status(match &health.status {
                    HealthStatus::Up => StatusCode::OK,
                    HealthStatus::Down { .. } => StatusCode::SERVICE_UNAVAILABLE,
                    HealthStatus::Warning { .. } => StatusCode::OK,
                })
                .header("content-type", "application/json")
                .body(serde_json::to_string(&health).unwrap().into())
                .unwrap()
        })
        // Metrics endpoint / 指标端点
        .get("/actuator/metrics", move || async move {
            let metrics = metrics_endpoint.metrics().await;
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&metrics).unwrap().into())
                .unwrap()
        })
        // Info endpoint / 信息端点
        .get("/actuator/info", move || async move {
            let info = info_endpoint.info().await;
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&info).unwrap().into())
                .unwrap()
        })
        // Readiness probe / 就绪探针
        .get("/actuator/health/readiness", move || async move {
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(r#"{"status":"UP"}"#.into())
                .unwrap()
        })
        // Liveness probe / 存活探针
        .get("/actuator/health/liveness", move || async move {
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(r#"{"status":"UP"}"#.into())
                .unwrap()
        });

    println!("Actuator endpoints configured:");
    println!("  GET  /actuator/health - Overall health status");
    println!("  GET  /actuator/health/{component} - Component health");
    println!("  GET  /actuator/health/readiness - Readiness probe");
    println!("  GET  /actuator/health/liveness - Liveness probe");
    println!("  GET  /actuator/metrics - Application metrics");
    println!("  GET  /actuator/info - Application information");
    println!("\nAll endpoints return JSON responses!");
    println!("Perfect for Kubernetes probes and monitoring systems!");
    println!();
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║   Nexus Actuator Example / 监控端点示例                      ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    println!("\nProduction-Ready Monitoring:");
    println!("  ✓ Health checks (Up/Down/Warning)");
    println!("  ✓ Component-level health breakdown");
    println!("  ✓ Metrics collection (Counter, Gauge, Histogram)");
    println!("  ✓ Build and Git information");
    println!("  ✓ System information");
    println!("  ✓ Kubernetes probes (readiness, liveness)");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(health_check_example());
    rt.block_on(metrics_example());
    rt.block_on(system_info_example());
    rt.block_on(actuator_server_example());

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║   All actuator examples completed!                           ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}
