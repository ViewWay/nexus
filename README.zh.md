<div align="center">
<p><img alt="Nexus" width="132" style="max-width:40%;min-width:60px;" src="https://via.placeholder.com/132x40/0066CC/FFFFFF?text=Nexus" /></p>
<p>
    <a href="https://github.com/nexus-rs/nexus/blob/main/README.md">English</a>&nbsp;&nbsp;
    <a href="https://github.com/nexus-rs/nexus/blob/main/README.zh.md">简体中文</a>
</p>
<p>
<a href="https://github.com/nexus-rs/nexus/actions">
    <img alt="build status" src="https://github.com/nexus-rs/nexus/workflows/CI/badge.svg" />
</a>
<a href="https://codecov.io/gh/nexus-rs/nexus">
    <img alt="codecov" src="https://codecov.io/gh/nexus-rs/nexus/branch/main/graph/badge.svg" />
</a>
<br>
<a href="https://crates.io/crates/nexus"><img alt="crates.io" src="https://img.shields.io/crates/v/nexus" /></a>
<a href="https://docs.rs/nexus"><img alt="Documentation" src="https://docs.rs/nexus/badge.svg" /></a>
<a href="https://crates.io/crates/nexus"><img alt="Download" src="https://img.shields.io/crates/d/nexus.svg" /></a>
<a href="https://github.com/rust-secure-code/safety-dance/"><img alt="unsafe forbidden" src="https://img.shields.io/badge/unsafe-forbidden-success.svg" /></a>
<br>
<a href="https://nexusframework.com">
    <img alt="Website" src="https://img.shields.io/badge/https-nexusframework.com-%23f00" />
</a>
</p>
</div>

Nexus 是一个用 Rust 编写的生产级、高可用 Web 框架，具有自定义异步运行时。与其他使用 Tokio 的框架不同，Nexus 具有从零开始构建的自定义异步运行时，使用 io-uring 以实现最高性能。

## 🎯 特性

- **自定义运行时** - 支持 io-uring 的 Thread-per-core 架构
- **高可用性** - 熔断器、限流器、重试逻辑
- **原生 Web3** - 内置区块链和智能合约支持
- **可观测性** - 兼容 OpenTelemetry 的追踪/指标
- **类型安全** - 利用 Rust 类型系统
- **类 Spring** - Spring Boot 开发者熟悉的模式

## ⚡️ 快速开始

您可以查看示例 [这里](https://github.com/nexus-rs/nexus/tree/main/examples)，或查看 [官方文档](https://docs.nexusframework.com)。

### Hello World

```rust
use nexus_http::{Body, Response, Server, StatusCode};
use nexus_runtime::task::block_on;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    block_on(async {
        let _server = Server::bind("127.0.0.1:8080")
            .run(handle_request)
            .await?;

        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    })
}

async fn handle_request(req: nexus_http::Request) -> Result<Response, nexus_http::Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/plain")
        .body(Body::from("Hello, Nexus!"))
        .unwrap())
}
```

### Nexus 日志

```rust
use nexus_observability::log::Logger;
#[cfg(feature = "nexus-format")]
use nexus_observability::{Banner, StartupLogger};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "nexus-format")]
    {
        Banner::print("MyApp", "0.1.0", 8080);
        Logger::init_spring_style()?;

        let startup = StartupLogger::new();
        startup.log_starting("MyApplication");
        startup.log_server_started(8080, startup.elapsed_ms());
    }

    tracing::info!(target: "my.app", "Application running");
    Ok(())
}
```

## 🚀 性能

Nexus 从设计之初就追求高性能：

- **相比 epoll 减少 70% 系统调用**（使用 io-uring）
- **延迟降低 40%**（Thread-per-core 架构）
- **零拷贝 I/O**，最小化内存分配
- **线性扩展性**，无锁竞争

基准测试结果将在第2阶段提供。

## 📚 文档

| 资源 | 链接 |
|------|------|
| **指南** | [docs.nexusframework.com](https://docs.nexusframework.com) |
| **API 文档** | [docs.rs/nexus](https://docs.rs/nexus) |
| **设计规范** | [design-spec.md](docs/design-spec.md) |
| **实施计划** | [implementation-plan.md](docs/implementation-plan.md) |

## 🏗️ 架构

```
┌─────────────────────────────────────────────────────────────┐
│                    应用层                                    │
├─────────────────────────────────────────────────────────────┤
│  Handlers  │  Middleware  │  Extractors  │  Response        │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                    Nexus运行时                               │
├─────────────────────────────────────────────────────────────┤
│  Task Scheduler  │  I/O Driver  │  Timer  │  Executor       │
│  (Thread-per-Core)  │  (io-uring)   │                          │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                    系统层                                    │
├─────────────────────────────────────────────────────────────┤
│       io-uring (Linux) / epoll / kqueue                      │
└─────────────────────────────────────────────────────────────┘
```

## 🛠️ 开发

```bash
# 克隆仓库
git clone https://github.com/nexus-rs/nexus.git
cd nexus

# 构建
cargo build --workspace

# 测试
cargo test --workspace

# 格式化
cargo fmt --all

# 检查
cargo clippy --workspace -- -D warnings
```

## 📋 项目状态

> **⚠️ Alpha 版本**
>
> Nexus 目前处于 **第1阶段：运行时核心**（已完成）。异步运行时已完全可用，支持 io-uring/epoll/kqueue。第2阶段（HTTP核心）正在开发中。

| 阶段 | 状态 | 描述 |
|------|------|------|
| Phase 0 | ✅ 已完成 | 基础 |
| Phase 1 | ✅ 已完成 | 运行时核心 |
| Phase 2 | 🔄 进行中 | HTTP服务器 |
| Phase 3 | 📋 计划中 | 路由和中间件 |
| Phase 4 | 📋 计划中 | 弹性 |
| Phase 5 | 📋 计划中 | 可观测性 |
| Phase 6 | 📋 计划中 | Web3集成 |
| Phase 7 | 📋 计划中 | 性能和加固 |

详情请参阅 [实施计划](docs/implementation-plan.md)。

## 🤝 贡献

我们欢迎贡献！请参阅 [CONTRIBUTING.md](CONTRIBUTING.md) 了解指南。

## 📄 许可证

Nexus 采用以下许可证之一：

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) 或 [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT) 或 [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

## 🙏 致谢

Nexus 受多种语言中优秀框架的启发：

- **Rust**: Axum, Actix Web, Monoio, Salvo
- **Go**: Gin, Echo
- **Java**: Spring Boot, WebFlux
- **Python**: FastAPI, Starlette

---

**Nexus 框架** — 为 Web 开发的未来而构建。
