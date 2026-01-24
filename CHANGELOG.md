# Changelog / 更新日志

All notable changes to this project will be documented in this file.
本文件记录项目的所有重要变更。

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

---

## [Unreleased] / 未发布

### Added / 新增
- Phase 2 HTTP Core development in progress / Phase 2 HTTP 核心开发中

---

## [0.1.0-alpha.1] - 2026-01-23

### Phase 1: Runtime Core Complete / 第一阶段：运行时核心完成

This release marks the completion of Phase 1, delivering a fully functional async runtime.
此版本标志着第一阶段完成，提供了功能完整的异步运行时。

### Added / 新增

#### Runtime Core / 运行时核心
- **I/O Drivers / I/O 驱动器**
  - io-uring driver for Linux (kernel 5.1+) / Linux io-uring 驱动（内核 5.1+）
  - epoll driver as fallback for older Linux / epoll 回退驱动（旧版 Linux）
  - kqueue driver for macOS/BSD / macOS/BSD kqueue 驱动
  - Automatic driver selection based on platform / 基于平台自动选择驱动

- **Task Scheduler / 任务调度器**
  - Thread-per-core scheduler for maximum performance / Thread-per-core 调度器
  - Work-stealing scheduler for load balancing / Work-stealing 调度器
  - Configurable scheduler selection / 可配置的调度器选择

- **Timer System / 定时器系统**
  - Hierarchical timer wheel (4 levels) / 层次化时间轮（4层）
  - Efficient timer management / 高效定时器管理
  - `sleep()` and `sleep_until()` APIs / `sleep()` 和 `sleep_until()` API

- **Channels / 通道**
  - MPSC bounded channel / 有界 MPSC 通道
  - MPSC unbounded channel / 无界 MPSC 通道
  - Async send/receive operations / 异步发送/接收操作

- **Task Management / 任务管理**
  - `spawn()` for spawning async tasks / `spawn()` 生成异步任务
  - `JoinHandle` for awaiting task results / `JoinHandle` 等待任务结果
  - Task cancellation support / 任务取消支持

- **Select Macro / Select 宏**
  - `select_two()` for waiting on two futures / `select_two()` 等待两个 future
  - `select_multiple()` for waiting on multiple futures / `select_multiple()` 等待多个 future

- **Runtime Builder / 运行时构建器**
  - `RuntimeBuilder` for custom configuration / `RuntimeBuilder` 自定义配置
  - `block_on()` for running futures / `block_on()` 运行 future

#### HTTP Foundation / HTTP 基础
- Basic HTTP types (Request, Response, Body) / 基础 HTTP 类型
- Status codes and HTTP methods / 状态码和 HTTP 方法
- `IntoResponse` trait / `IntoResponse` trait
- `FromRequest` trait / `FromRequest` trait

#### Router Foundation / 路由基础
- Trie-based route matching / 基于 Trie 的路由匹配
- Path parameter extraction / 路径参数提取
- HTTP method routing / HTTP 方法路由

#### Middleware Foundation / 中间件基础
- Middleware trait definition / 中间件 trait 定义
- CORS middleware / CORS 中间件
- Compression middleware / 压缩中间件
- Timeout middleware / 超时中间件
- Logger middleware / 日志中间件

#### Core Infrastructure / 核心基础设施
- IoC Container foundation / IoC 容器基础
- Bean definition and factory / Bean 定义和工厂
- Extensions system / 扩展系统
- Error handling types / 错误处理类型

### Tests / 测试
- 49 unit tests passing / 49 个单元测试通过
- 22 doc tests passing / 22 个文档测试通过
- Multi-platform CI (Linux, macOS, Windows) / 多平台 CI

---

## [0.0.1] - 2026-01-21

### Phase 0: Foundation / 第零阶段：基础设施

### Added / 新增
- Project workspace structure / 项目工作区结构
- CI/CD pipeline with GitHub Actions / GitHub Actions CI/CD 流水线
- Code quality tools (rustfmt, clippy) / 代码质量工具
- Documentation infrastructure (mdBook) / 文档基础设施
- Apache 2.0 License / Apache 2.0 许可证
- CLA agreement / CLA 协议
- Contributing guidelines / 贡献指南

---

## Roadmap / 路线图

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 0 | ✅ Complete | Foundation / 基础设施 |
| Phase 1 | ✅ Complete | Runtime Core / 运行时核心 |
| Phase 2 | 🔄 In Progress | HTTP Core / HTTP 核心 |
| Phase 3 | 📋 Planned | Middleware & Extensions / 中间件与扩展 |
| Phase 4 | 📋 Planned | Resilience & HA / 弹性与高可用 |
| Phase 5 | 📋 Planned | Observability / 可观测性 |
| Phase 6 | 📋 Planned | Web3 Support / Web3 支持 |
| Phase 7 | 📋 Planned | Production Ready / 生产就绪 |

---

[Unreleased]: https://github.com/nexus-framework/nexus/compare/v0.1.0-alpha.1...HEAD
[0.1.0-alpha.1]: https://github.com/nexus-framework/nexus/compare/v0.0.1...v0.1.0-alpha.1
[0.0.1]: https://github.com/nexus-framework/nexus/releases/tag/v0.0.1
