# Introduction
# 简介

## What is Nexus? / 什么是 Nexus？

Nexus is a modern web framework designed for high-performance, high-availability applications. It combines the best features from various frameworks across multiple programming languages.

Nexus 是一个为高性能、高可用应用设计的现代 Web 框架。它结合了多种编程语言中各种框架的最佳特性。

## Design Philosophy / 设计理念

### Performance First / 性能优先

- **Thread-per-core architecture**: No work stealing overhead / **Thread-per-core 架构**：无工作窃取开销
- **io-uring based I/O**: Zero-copy operations where possible / **基于 io-uring 的 I/O**：尽可能零拷贝操作
- **Ownership-based buffers**: Safe buffer management / **基于所有权的缓冲区**：安全的缓冲区管理

### Developer Experience / 开发者体验

- **Ergonomic API**: Intuitive handlers and extractors / **符合人体工学的 API**：直观的 handlers 和 extractors
- **Bilingual Documentation**: All public APIs documented in English and Chinese / **双语文档**：所有公共 API 都有英文和中文文档
- **Compile-time Safety**: Catch errors before runtime / **编译时安全**：在运行前捕获错误

### Production Ready / 生产就绪

- **Resilience Patterns**: Circuit breakers, retries, rate limiting / **弹性模式**：熔断器、重试、限流
- **Observability**: OpenTelemetry-compatible tracing / **可观测性**：兼容 OpenTelemetry 的追踪
- **Security**: Security-first design principles / **安全**：安全优先的设计原则

## Architecture Overview / 架构概览

```
┌─────────────────────────────────────────────────────────────┐
│                      Application Layer                       │
│                        应用层                                 │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                   Nexus HTTP & Router                        │
│                    Nexus HTTP 和路由                          │
├─────────────────────────────────────────────────────────────┤
│  Handlers  │  Middleware  │  Extractors  │  Response        │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     Nexus Runtime                            │
│                      Nexus运行时                              │
├─────────────────────────────────────────────────────────────┤
│  Task Scheduler  │  I/O Driver  │  Timer  │  Executor       │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                     System Layer                             │
│                       系统层                                 │
├─────────────────────────────────────────────────────────────┤
│       io-uring (Linux) / epoll / kqueue                      │
└─────────────────────────────────────────────────────────────┘
```

## Comparison with Other Frameworks / 与其他框架的比较

| Feature / 特性 | Nexus | Tokio-based | Go (Gin) | Java (Spring) |
|----------------|-------|-------------|----------|---------------|
| Custom Runtime | ✅ Yes | ❌ No | N/A | N/A |
| Thread-per-Core | ✅ Yes | Optional | No | No |
| Web3 Native | ✅ Yes | Partial | No | Partial |
| Zero-Copy I/O | ✅ Yes | Partial | No | No |

---

*← [Previous / 上一页](../index.md) | [Next / 下一页](./installation.md) →*
