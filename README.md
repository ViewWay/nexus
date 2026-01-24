# Nexus Framework
# Nexus 框架

[![CI](https://github.com/nexus-framework/nexus/workflows/CI/badge.svg)](https://github.com/nexus-framework/nexus/actions)
[![codecov](https://codecov.io/gh/nexus-framework/nexus/branch/main/graph/badge.svg)](https://codecov.io/gh/nexus-framework/nexus)
[![Crates.io](https://img.shields.io/crates/v/nexus)](https://crates.io/crates/nexus)
[![Documentation](https://docs.rs/nexus/badge.svg)](https://docs.rs/nexus)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

> A production-grade, high-availability web framework written in Rust with a custom async runtime.
>
> 用 Rust 编写的生产级、高可用 Web 框架，具有自定义异步运行时。

## Overview / 概述

**Nexus** is a modern web framework designed for high-performance, high-availability applications. Unlike other frameworks that use Tokio, Nexus features a custom async runtime built from scratch using io-uring for maximum performance.

**Nexus** 是一个为高性能、高可用应用设计的现代 Web 框架。与其他使用 Tokio 的框架不同，Nexus 具有从零开始构建的自定义异步运行时，使用 io-uring 以实现最高性能。

## Key Features / 核心特性

| Feature / 特性 | Description / 描述 |
|----------------|-------------------|
| **Custom Runtime** / **自定义运行时** | Thread-per-core architecture with io-uring / Thread-per-core 架构与 io-uring |
| **High Availability** / **高可用性** | Circuit breakers, rate limiters, retry logic / 熔断器、限流器、重试逻辑 |
| **Web3 Native** / **原生 Web3** | Built-in blockchain and smart contract support / 内置区块链和智能合约支持 |
| **Observability** / **可观测性** | OpenTelemetry-compatible tracing/metrics / 兼容 OpenTelemetry 的追踪/指标 |
| **Type Safety** / **类型安全** | Leverages Rust's type system / 利用 Rust 类型系统 |

## Project Status / 项目状态

> **⚠️ Alpha Version / Alpha版本**
>
> Nexus is currently in **Phase 1: Runtime Core** (completed). The async runtime is fully functional with io-uring/epoll/kqueue support. Phase 2 (HTTP Core) is in development.
>
> Nexus 目前处于 **第1阶段：运行时核心**（已完成）。异步运行时已完全可用，支持 io-uring/epoll/kqueue。第2阶段（HTTP核心）正在开发中。

See [implementation plan](docs/implementation-plan.md) for the roadmap.
请参阅 [实施计划](docs/implementation-plan.md) 了解路线图。

## Quick Example / 快速示例

> **Note / 注意**: This example will work starting Phase 1 (currently in Phase 0).
>
> **注意**：此示例将在第1阶段开始时生效（目前处于第0阶段）。

```rust
use nexus::prelude::*;

#[nexus::main]
async fn main() -> Result<()> {
    // Create router / 创建路由
    let app = Router::new()
        .route("/", get(hello))
        .route("/users/:id", get(get_user));

    // Start server / 启动服务器
    Server::bind("0.0.0.0:3000")
        .serve(app)
        .await?;

    Ok(())
}

// Handler / 处理器
async fn hello() -> &'static str {
    "Hello, World! / 你好，世界！"
}

// With path parameter / 带路径参数
async fn get_user(Path(id): Path<u64>) -> Json<User> {
    Json(User { id, name: "Alice".into() })
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}
```

## Installation / 安装

Add to your `Cargo.toml`:
添加到您的 `Cargo.toml`：

```toml
[dependencies]
nexus = "0.1"
```

## Documentation / 文档

| Resource / 资源 | Link / 链接 |
|-----------------|-------------|
| **Book / 指南** | [docs.nexus-framework.org](https://docs.nexus-framework.org) |
| **API Docs / API 文档** | [docs.rs/nexus](https://docs.rs/nexus) |
| **Design Spec / 设计规范** | [design-spec.md](docs/design-spec.md) |
| **Implementation Plan / 实施计划** | [implementation-plan.md](docs/implementation-plan.md) |

## Architecture / 架构

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│                        应用层                                 │
├─────────────────────────────────────────────────────────────┤
│  Handlers  │  Middleware  │  Extractors  │  Response        │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     Nexus Runtime                            │
│                      Nexus运行时                              │
├─────────────────────────────────────────────────────────────┤
│  Task Scheduler  │  I/O Driver  │  Timer  │  Executor       │
│  (Thread-per-Core)  │  (io-uring)   │                          │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     System Layer                             │
│                       系统层                                 │
├─────────────────────────────────────────────────────────────┤
│       io-uring (Linux) / epoll / kqueue                      │
└─────────────────────────────────────────────────────────────┘
```

## Development / 开发

```bash
# Clone repository / 克隆仓库
git clone https://github.com/nexus-framework/nexus.git
cd nexus

# Build / 构建
cargo build --workspace

# Test / 测试
cargo test --workspace

# Format / 格式化
cargo fmt --all

# Lint / 检查
cargo clippy --workspace -- -D warnings
```

## Contributing / 贡献

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.
我们欢迎贡献！请参阅 [CONTRIBUTING.md](CONTRIBUTING.md) 了解指南。

## Roadmap / 路线图

| Phase / 阶段 | Status / 状态 | Description / 描述 |
|---------------|---------------|-------------------|
| Phase 0 | ✅ Complete / 已完成 | Foundation / 基础 |
| Phase 1 | ✅ Complete / 已完成 | Runtime Core / 运行时核心 |
| Phase 2 | 🔄 In Progress / 进行中 | HTTP Server / HTTP服务器 |
| Phase 3 | 📋 Planned / 计划中 | Router & Middleware / 路由和中间件 |
| Phase 4 | 📋 Planned / 计划中 | Resilience / 弹性 |
| Phase 5 | 📋 Planned / 计划中 | Observability / 可观测性 |
| Phase 6 | 📋 Planned / 计划中 | Web3 Integration / Web3集成 |
| Phase 7 | 📋 Planned / 计划中 | Performance & Hardening / 性能和加固 |

See [implementation plan](docs/implementation-plan.md) for details.
详情请参阅 [实施计划](docs/implementation-plan.md)。

## License / 许可证

Apache License 2.0 / Apache 许可证 2.0

See [LICENSE](LICENSE) for details.
详情请参阅 [LICENSE](LICENSE)。

## Acknowledgments / 致谢

Nexus is inspired by excellent frameworks across multiple languages:
Nexus 受多种语言中优秀框架的启发：

- **Rust**: Axum, Actix Web, Monoio
- **Go**: Gin, Echo
- **Java**: Spring Boot, WebFlux
- **Python**: FastAPI, Starlette

---

**Nexus Framework** — Built for the future of web development.
**Nexus 框架** — 为 Web 开发的未来而构建。
