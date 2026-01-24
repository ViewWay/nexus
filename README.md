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

Nexus is a production-grade, high-availability web framework written in Rust with a custom async runtime. Unlike other frameworks that use Tokio, Nexus features a custom async runtime built from scratch using io-uring for maximum performance.

## 🎯 Features

- **Custom Runtime** - Thread-per-core architecture with io-uring support
- **High Availability** - Circuit breakers, rate limiters, retry logic
- **Web3 Native** - Built-in blockchain and smart contract support
- **Observability** - OpenTelemetry-compatible tracing/metrics
- **Type Safety** - Leverages Rust's type system
- **Spring-like** - Familiar patterns for Spring Boot developers

## ⚡️ Quick Start

You can view examples [here](https://github.com/nexus-rs/nexus/tree/main/examples), or view [official documentation](https://docs.nexusframework.com).

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

### Nexus Logging

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

## 🚀 Performance

Nexus is designed for high performance from the ground up:

- **70% fewer syscalls** vs epoll with io-uring
- **40% lower latency** with thread-per-core architecture
- **Zero-copy I/O** for minimal allocations
- **Linear scalability** with no lock contention

Benchmark results will be available in Phase 2.

## 📚 Documentation

| Resource | Link |
|----------|------|
| **Book** | [docs.nexusframework.com](https://docs.nexusframework.com) |
| **API Docs** | [docs.rs/nexus](https://docs.rs/nexus) |
| **Design Spec** | [design-spec.md](docs/design-spec.md) |
| **Implementation Plan** | [implementation-plan.md](docs/implementation-plan.md) |

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
├─────────────────────────────────────────────────────────────┤
│  Handlers  │  Middleware  │  Extractors  │  Response        │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     Nexus Runtime                            │
├─────────────────────────────────────────────────────────────┤
│  Task Scheduler  │  I/O Driver  │  Timer  │  Executor       │
│  (Thread-per-Core)  │  (io-uring)   │                          │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     System Layer                             │
├─────────────────────────────────────────────────────────────┤
│       io-uring (Linux) / epoll / kqueue                      │
└─────────────────────────────────────────────────────────────┘
```

## 🛠️ Development

```bash
# Clone repository
git clone https://github.com/nexus-rs/nexus.git
cd nexus

# Build
cargo build --workspace

# Test
cargo test --workspace

# Format
cargo fmt --all

# Lint
cargo clippy --workspace -- -D warnings
```

## 📋 Project Status

> **⚠️ Alpha Version**
>
> Nexus is currently in **Phase 1: Runtime Core** (completed). The async runtime is fully functional with io-uring/epoll/kqueue support. Phase 2 (HTTP Core) is in development.

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 0 | ✅ Complete | Foundation |
| Phase 1 | ✅ Complete | Runtime Core |
| Phase 2 | 🔄 In Progress | HTTP Server |
| Phase 3 | 📋 Planned | Router & Middleware |
| Phase 4 | 📋 Planned | Resilience |
| Phase 5 | 📋 Planned | Observability |
| Phase 6 | 📋 Planned | Web3 Integration |
| Phase 7 | 📋 Planned | Performance & Hardening |

See [implementation plan](docs/implementation-plan.md) for details.

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

Nexus is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

## 🙏 Acknowledgments

Nexus is inspired by excellent frameworks across multiple languages:

- **Rust**: Axum, Actix Web, Monoio, Salvo
- **Go**: Gin, Echo
- **Java**: Spring Boot, WebFlux
- **Python**: FastAPI, Starlette

---

**Nexus Framework** — Built for the future of web development.
