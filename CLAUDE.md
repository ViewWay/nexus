# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Nexus** is a production-grade, high-availability web framework written in Rust.

### Key Features

- **Custom async runtime** based on io-uring for maximum performance
- **Built-in HA patterns**: circuit breakers, rate limiters, retry, service discovery
- **Native observability**: distributed tracing, metrics, structured logging
- **Native Web3 support**: smart contract interaction, wallet management
- **Thread-per-core architecture** for linear scalability

### Project Status

**Phase**: Runtime Core Complete (Phase 1) - HTTP Core In Progress (Phase 2)
**Estimated Timeline**: 18-24 months to v1.0

**Completed Features (Phase 1):**
- io-uring/epoll/kqueue I/O drivers
- Thread-per-core + work-stealing schedulers
- Hierarchical timer wheel
- MPSC channels (bounded/unbounded)
- Task spawning with JoinHandle
- Select macro foundation

## Documentation

All design and API documentation is located in `docs/`:

| Document | Description |
|----------|-------------|
| `docs/design-spec.md` | Coding standards, naming conventions, API design principles |
| `docs/api-spec.md` | Complete API specification for all modules |
| `docs/implementation-plan.md` | Detailed 7-phase implementation plan |

## Project Structure (Planned)

```
nexus/
├── Cargo.toml                    # Workspace root
├── CLAUDE.md                     # This file
├── docs/                         # Documentation
├── crates/                       # Workspace crates
│   ├── nexus-runtime/            # Custom async runtime
│   ├── nexus-core/               # Core types
│   ├── nexus-http/               # HTTP server & client
│   ├── nexus-router/             # Router & middleware
│   ├── nexus-extractors/         # Request extractors
│   ├── nexus-response/           # Response builders
│   ├── nexus-resilience/         # HA patterns
│   ├── nexus-observability/      # Tracing, metrics, logging
│   ├── nexus-web3/               # Blockchain & Web3
│   └── nexus-macros/             # Procedural macros
├── examples/                     # Example applications
├── tests/                        # Integration tests
└── benches/                      # Benchmarks
```

## Development Guidelines

### Code Style

- Use `rustfmt` with default settings
- Enable all `clippy` lints
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **Bilingual documentation**: All public APIs must have both English and Chinese comments

### Naming Conventions

```rust
// Crates: lowercase with hyphens
nexus-runtime, nexus-http, nexus-web3

// Types: PascalCase
Router, CircuitBreaker, TraceContext

// Functions: snake_case, verb-based
get(), post(), is_connected(), get_config()

// Returning bool: use is_, has_, can_ prefix
is_connected(), has_permission(), can_retry()
```

### API Design Principles

1. **Builder pattern** for complex configuration
2. **Extractor pattern** for request data
3. **Middleware pattern** for request/response processing
4. **Type-safe** error handling

### Documentation Template

```rust
/// Brief description in English.
/// 中文简要描述。
///
/// # Description / 描述
///
/// Detailed explanation in English.
/// 中文详细说明。
///
/// # Example / 示例
/// ```rust
/// let app = Router::new().get("/", handler);
/// ```
pub fn example_function() -> ReturnType {
    // ...
}
```

## Build Commands (When Implemented)

```bash
# Build all crates
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run benchmarks
cargo bench

# Check code formatting
cargo fmt --check

# Format code
cargo fmt

# Run linter
cargo clippy -- -D warnings

# Run linter with all features
cargo clippy --all-features -- -D warnings
```

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                         Application Layer                       │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐           │
│  │    HTTP     │  │  Resilience │  │ Observability│          │
│  │   Router    │  │   & HA      │  │   (Tracing)  │          │
│  └─────────────┘  └─────────────┘  └─────────────┘           │
├─────────────────────────────────────────────────────────────────┤
│                       Core Framework                           │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐           │
│  │  Handlers   │  │ Extractors  │  │ Middleware  │           │
│  └─────────────┘  └─────────────┘  └─────────────┘           │
├─────────────────────────────────────────────────────────────────┤
│                      Custom Runtime                            │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐           │
│  │ io-uring    │  │  Thread-    │  │   Timer     │           │
│  │   Driver    │  │ per-core    │  │   Wheel     │           │
│  └─────────────┘  └─────────────┘  └─────────────┘           │
└─────────────────────────────────────────────────────────────────┘
```

## Key Design Decisions

1. **Thread-per-core**: No work stealing, each core has its own task queue
2. **io-uring first**: Linux uses io-uring, falls back to epoll/kqueue
3. **Zero-copy**: Request/response body uses `Bytes` for zero-copy I/O
4. **Observable by default**: Every request is automatically traced

## Performance Goals

| Metric | Target |
|--------|--------|
| QPS (simple GET) | 1M+ |
| P99 latency (no middleware) | < 1ms |
| Base memory | < 10MB |
| Startup time | < 100ms |

## Implementation Phases

1. **Phase 0** (2mo): Infrastructure
2. **Phase 1** (4mo): Runtime Core
3. **Phase 2** (5mo): HTTP Core
4. **Phase 3** (5mo): Middleware & Extensions
5. **Phase 4** (5mo): Resilience & HA
6. **Phase 5** (5mo): Observability
7. **Phase 6** (5mo): Web3 Support
8. **Phase 7** (6mo): Production Ready

See `docs/implementation-plan.md` for details.

## Contributing

Before contributing:
1. Read `docs/design-spec.md` for coding standards
2. Read `docs/api-spec.md` for API conventions
3. Check if your feature is in the implementation plan
4. Follow the commit message format: `feat:`, `fix:`, `docs:`, `refactor:`, etc.

## References

- [Monoio](https://github.com/bytedance/monoio) - io-uring runtime reference
- [Axum](https://github.com/tokio-rs/axum) - HTTP framework reference
- [Alloy](https://alloy.rs/) - Ethereum toolkit reference
- [Spring Boot Resilience4j](https://resilience4j.readme.io/) - Circuit breaker reference
