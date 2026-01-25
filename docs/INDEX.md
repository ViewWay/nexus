# Nexus Documentation Index
# Nexus 文档索引

**Last Updated**: 2026-01-25
**最后更新**: 2026-01-25

---

## 📚 Documentation Structure / 文档结构

```
docs/
├── design/           # Project design documents / 项目设计文档
├── api/              # API specifications and references / API 规范和参考
├── reports/          # Development reports / 开发报告
├── guides/           # User guides and tutorials / 用户指南和教程
├── spring-boot/      # Spring Boot reference / Spring Boot 参考
├── book/             # Comprehensive book / 完整书籍
└── bug-fixes/        # Bug fix documentation / Bug 修复文档
```

---

## 🎯 Quick Navigation / 快速导航

| Category / 分类 | Description / 描述 | Link / 链接 |
|----------------|-------------------|-------------|
| **[Design Docs](#design-documents---项目设计文档)** | Architecture & design specifications / 架构和设计规范 | [design/](design/) |
| **[API Docs](#api-documents---api文档)** | API specifications & references / API 规范和参考 | [api/](api/) |
| **[Reports](#development-reports---开发报告)** | Progress & completion reports / 进度和完成报告 | [reports/](reports/) |
| **[Guides](#user-guides---用户指南)** | Tutorials & guides / 教程和指南 | [guides/](guides/) |
| **[Spring Boot](#spring-boot-reference---spring-boot参考)** | Spring Boot comparisons / Spring Boot 对比 | [spring-boot/](spring-boot/) |
| **[Book](#book---书籍)** | Comprehensive documentation / 完整文档 | [book/](book/) |

---

## 📐 Design Documents / 项目设计文档

**Location**: [`design/`](design/)

| Document / 文档 | Description / 描述 |
|----------------|-------------------|
| [design-spec.md](design/design-spec.md) | Coding standards, naming conventions, API design principles / 编码规范、命名约定、API设计原则 |
| [implementation-plan.md](design/implementation-plan.md) | Detailed 7-phase implementation plan / 详细的7阶段实现计划 |
| [implementation-roadmap-data.md](design/implementation-roadmap-data.md) | Data layer implementation roadmap / 数据层实现路线图 |
| [MASTER-ROADMAP.md](design/MASTER-ROADMAP.md) | Master project roadmap / 项目主路线图 |
| [STRATEGY-OVERVIEW.md](design/STRATEGY-OVERVIEW.md) | Project strategy overview / 项目策略概览 |

---

## 📋 API Documents / API 文档

**Location**: [`api/`](api/)

| Document / 文档 | Description / 描述 |
|----------------|-------------------|
| [api-spec.md](api/api-spec.md) | Complete API specification / 完整的 API 规范 |
| [api-quick-reference.md](api/api-quick-reference.md) | API quick reference / API 快速参考 |
| [annotations-reference.md](api/annotations-reference.md) | Annotations reference / 注解参考 |

---

## 📊 Development Reports / 开发报告

**Location**: [`reports/`](reports/)

### Phase Completion Reports / 阶段完成报告

| Report / 报告 | Description / 描述 |
|--------------|-------------------|
| [phase0-completion.md](reports/phase0-completion.md) | Phase 0 (Infrastructure) completion / Phase 0（基础设施）完成 |
| [phase1-completion.md](reports/phase1-completion.md) | Phase 1 (Runtime Core) completion / Phase 1（运行时核心）完成 |
| [phase2-completion.md](reports/phase2-completion.md) | Phase 2 (HTTP Core) completion / Phase 2（HTTP 核心）完成 |
| [phase3-completion.md](reports/phase3-completion.md) | Phase 3 (Middleware) completion / Phase 3（中间件）完成 |
| [phase4-completion.md](reports/phase4-completion.md) | Phase 4 (Resilience) completion / Phase 4（弹性）完成 |
| [phase5-completion.md](reports/phase5-completion.md) | Phase 5 (Observability) completion / Phase 5（可观测性）完成 |
| [phase6-completion.md](reports/phase6-completion.md) | Phase 6 (Web3) completion / Phase 6（Web3）完成 |

### Feature Implementation Reports / 功能实现报告

| Report / 报告 | Description / 描述 |
|--------------|-------------------|
| [ANNOTATION-COMPARISON.md](reports/ANNOTATION-COMPARISON.md) | Annotation feature comparison / 注解功能对比 |
| [ANNOTATION-GUIDE.md](reports/ANNOTATION-GUIDE.md) | Annotation usage guide / 注解使用指南 |
| [ANNOTATIONS-COMPLETE-REPORT.md](reports/ANNOTATIONS-COMPLETE-REPORT.md) | Complete annotations report / 完整注解报告 |
| [ANNOTATIONS-PROGRESS-REPORT.md](reports/ANNOTATIONS-PROGRESS-REPORT.md) | Annotations progress report / 注解进度报告 |
| [JWT-AUTHENTICATION-REPORT.md](reports/JWT-AUTHENTICATION-REPORT.md) | JWT authentication implementation / JWT 认证实现 |
| [DOCUMENTATION-UPDATE-REPORT.md](reports/DOCUMENTATION-UPDATE-REPORT.md) | Documentation update summary / 文档更新摘要 |
| [TRANSACTIONAL-UPGRADE-REPORT.md](reports/TRANSACTIONAL-UPGRADE-REPORT.md) | @Transactional upgrade report / @Transactional 升级报告 |
| [LOMBOK-IMPLEMENTATION.md](reports/LOMBOK-IMPLEMENTATION.md) | Lombok features implementation / Lombok 功能实现 |
| [SPRING-ANNOTATIONS-STATUS.md](reports/SPRING-ANNOTATIONS-STATUS.md) | Spring annotations status / Spring 注解状态 |

### Progress Tracking / 进度跟踪

| Report / 报告 | Description / 描述 |
|--------------|-------------------|
| [MISSING-FEATURES.md](reports/MISSING-FEATURES.md) | Missing features list / 缺失功能列表 |
| [MISSING-FEATURES-PROGRESS.md](reports/MISSING-FEATURES-PROGRESS.md) | Missing features progress / 缺失功能进度 |
| [MISSING-FEATURES-QUICK-REF.md](reports/MISSING-FEATURES-QUICK-REF.md) | Missing features quick reference / 缺失功能快速参考 |
| [FINAL-PROGRESS-REPORT.md](reports/FINAL-PROGRESS-REPORT.md) | Final progress report / 最终进度报告 |
| [RUNTIME-INTEGRATION-PROGRESS.md](reports/RUNTIME-INTEGRATION-PROGRESS.md) | Runtime integration progress / 运行时集成进度 |
| [README-UPDATE-REPORT.md](reports/README-UPDATE-REPORT.md) | README update report / README 更新报告 |
| [code-review-report.md](reports/code-review-report.md) | Code review report / 代码审查报告 |
| [security-audit-report.md](reports/security-audit-report.md) | Security audit report / 安全审计报告 |

### Data Layer Reports / 数据层报告

| Report / 报告 | Description / 描述 |
|--------------|-------------------|
| [DATA-LAYER-ADDENDUM.md](reports/DATA-LAYER-ADDENDUM.md) | Data layer addendum / 数据层附录 |
| [nexus-data-full-implementation.md](reports/nexus-data-full-implementation.md) | Nexus data full implementation / Nexus 数据完整实现 |
| [nexus-mybatis-plus-style.md](reports/nexus-mybatis-plus-style.md) | MyBatis-Plus style implementation / MyBatis-Plus 风格实现 |
| [LOMBOK-QUICK-REF.md](reports/LOMBOK-QUICK-REF.md) | Lombok quick reference / Lombok 快速参考 |

---

## 📖 User Guides / 用户指南

**Location**: [`guides/`](guides/)

| Guide / 指南 | Description / 描述 |
|-------------|-------------------|
| [user-guide.md](guides/user-guide.md) | User guide / 用户指南 |
| [migration-guide.md](guides/migration-guide.md) | Migration guide from other frameworks / 从其他框架迁移指南 |
| [benchmarking.md](guides/benchmarking.md) | Benchmarking guide / 性能测试指南 |
| [rust-challenges-solutions.md](guides/rust-challenges-solutions.md) | Rust challenges and solutions / Rust 挑战和解决方案 |

---

## 🌱 Spring Boot Reference / Spring Boot 参考

**Location**: [`spring-boot/`](spring-boot/)

### Core Spring Boot / 核心 Spring Boot

| Document / 文档 | Description / 描述 |
|----------------|-------------------|
| [spring-boot-basics.md](spring-boot/spring-boot-basics.md) | Spring Boot basics / Spring Boot 基础 |
| [spring-boot-core.md](spring-boot/spring-boot-core.md) | Spring Boot core concepts / Spring Boot 核心概念 |
| [spring-boot-advanced.md](spring-boot/spring-boot-advanced.md) | Spring Boot advanced features / Spring Boot 高级特性 |
| [spring-boot-enterprise.md](spring-boot/spring-boot-enterprise.md) | Spring Boot enterprise features / Spring Boot 企业特性 |

### Learning & Practice / 学习与实践

| Document / 文档 | Description / 描述 |
|----------------|-------------------|
| [spring-boot-learning-index.md](spring-boot/spring-boot-learning-index.md) | Spring Boot learning index / Spring Boot 学习索引 |
| [spring-boot-practice.md](spring-boot/spring-boot-practice.md) | Spring Boot practice / Spring Boot 实践 |
| [spring-boot-feature-matrix.md](spring-boot/spring-boot-feature-matrix.md) | Spring Boot feature matrix / Spring Boot 功能矩阵 |

### Comparison & Gap Analysis / 对比与差距分析

| Document / 文档 | Description / 描述 |
|----------------|-------------------|
| [spring-comparison.md](spring-boot/spring-comparison.md) | Spring Boot vs Nexus comparison / Spring Boot vs Nexus 对比 |
| [spring-boot-gap-analysis.md](spring-boot/spring-boot-gap-analysis.md) | Spring Boot gap analysis / Spring Boot 差距分析 |
| [spring-ecosystem-gap-analysis.md](spring-boot/spring-ecosystem-gap-analysis.md) | Spring ecosystem gap analysis / Spring 生态系统差距分析 |
| [spring-features-gap-analysis.md](spring-boot/spring-features-gap-analysis.md) | Spring features gap analysis / Spring 功能差距分析 |
| [spring-missing-features.md](spring-boot/spring-missing-features.md) | Spring missing features / Spring 缺失功能 |
| [spring-modules-deep-analysis.md](spring-boot/spring-modules-deep-analysis.md) | Spring modules deep analysis / Spring 模块深度分析 |
| [spring-boot-logging.md](spring-boot/spring-boot-logging.md) | Spring Boot logging / Spring Boot 日志 |

---

## 📕 Book / 书籍

**Location**: [`book/`](book/)

### Getting Started / 入门

| Chapter / 章节 | Description / 描述 |
|---------------|-------------------|
| [introduction.md](book/src/getting-started/introduction.md) | Introduction to Nexus / Nexus 介绍 |
| [installation.md](book/src/getting-started/installation.md) | Installation guide / 安装指南 |
| [quick-start.md](book/src/getting-started/quick-start.md) | Quick start tutorial / 快速开始教程 |
| [tutorial.md](book/src/getting-started/tutorial.md) | Tutorial / 教程 |

### Core Concepts / 核心概念

| Chapter / 章节 | Description / 描述 |
|---------------|-------------------|
| [runtime.md](book/src/core-concepts/runtime.md) | Runtime concepts / 运行时概念 |
| [http.md](book/src/core-concepts/http.md) | HTTP concepts / HTTP 概念 |
| [router.md](book/src/core-concepts/router.md) | Router concepts / 路由器概念 |
| [middleware.md](book/src/core-concepts/middleware.md) | Middleware concepts / 中间件概念 |
| [extractors.md](book/src/core-concepts/extractors.md) | Extractors concepts / 提取器概念 |

### Advanced / 高级

| Chapter / 章节 | Description / 描述 |
|---------------|-------------------|
| [resilience.md](book/src/advanced/resilience.md) | Resilience patterns / 弹性模式 |
| [observability.md](book/src/advanced/observability.md) | Observability features / 可观测性功能 |
| [testing.md](book/src/advanced/testing.md) | Testing guide / 测试指南 |
| [web3.md](book/src/advanced/web3.md) | Web3 features / Web3 功能 |

### Reference / 参考

| Chapter / 章节 | Description / 描述 |
|---------------|-------------------|
| [api.md](book/src/reference/api.md) | API reference / API 参考 |
| [configuration.md](book/src/reference/configuration.md) | Configuration reference / 配置参考 |
| [security.md](book/src/reference/security.md) | Security reference / 安全参考 |
| [performance.md](book/src/reference/performance.md) | Performance reference / 性能参考 |

---

## 🐛 Bug Fixes / Bug 修复

**Location**: [`bug-fixes/`](bug-fixes/)

| Document / 文档 | Description / 描述 |
|----------------|-------------------|
| [phase0.md](bug-fixes/phase0.md) | Phase 0 bug fixes / Phase 0 bug 修复 |

---

## 🔍 Search & Navigation / 搜索与导航

### By Feature / 按功能查找

- **Authentication / 认证**: See [Security chapter in book](book/src/reference/security.md), [JWT authentication report](reports/JWT-AUTHENTICATION-REPORT.md)
- **Annotations / 注解**: See [Annotations reference](api/annotations-reference.md), [Annotation guide](reports/ANNOTATION-GUIDE.md)
- **Middleware / 中间件**: See [Middleware concepts](book/src/core-concepts/middleware.md), [Middleware README](../crates/nexus-middleware/README.md)
- **Resilience / 弹性**: See [Resilience chapter](book/src/advanced/resilience.md)
- **Web3**: See [Web3 chapter](book/src/advanced/web3.md)

### By Phase / 按阶段查找

- **Phase 0 (Infrastructure)**: [Implementation plan § Phase 0](design/implementation-plan.md), [Phase 0 completion](reports/phase0-completion.md)
- **Phase 1 (Runtime Core)**: [Implementation plan § Phase 1](design/implementation-plan.md), [Phase 1 completion](reports/phase1-completion.md)
- **Phase 2 (HTTP Core)**: [Implementation plan § Phase 2](design/implementation-plan.md), [Phase 2 completion](reports/phase2-completion.md)
- **Phase 3 (Middleware)**: [Implementation plan § Phase 3](design/implementation-plan.md), [Phase 3 completion](reports/phase3-completion.md)
- **Phase 4 (Resilience)**: [Implementation plan § Phase 4](design/implementation-plan.md), [Phase 4 completion](reports/phase4-completion.md)
- **Phase 5 (Observability)**: [Implementation plan § Phase 5](design/implementation-plan.md), [Phase 5 completion](reports/phase5-completion.md)
- **Phase 6 (Web3)**: [Implementation plan § Phase 6](design/implementation-plan.md), [Phase 6 completion](reports/phase6-completion.md)

### By Spring Boot Feature / 按 Spring Boot 功能查找

- **Spring Security**: [Security reference](book/src/reference/security.md), [Spring comparison](spring-boot/spring-comparison.md)
- **Spring Data**: [Data layer implementation](reports/nexus-data-full-implementation.md), [MyBatis-Plus style](reports/nexus-mybatis-plus-style.md)
- **Spring Web MVC**: [HTTP concepts](book/src/core-concepts/http.md), [Router concepts](book/src/core-concepts/router.md)
- **Spring Boot Actuator**: [Observability chapter](book/src/advanced/observability.md)

---

## 📝 Documentation Conventions / 文档约定

### Bilingual Documentation / 双语文档

All documentation maintains both English and Chinese versions:
所有文档都保持英文和中文版本：

- Headers: `English / 中文`
- Code comments: `// English / 中文`
- Examples: Bilingual explanations

### Documentation Types / 文档类型

- **Design Docs**: Architecture, specifications, plans / 架构、规范、计划
- **API Docs**: API specifications, references / API 规范、参考
- **Reports**: Progress, completion, analysis / 进度、完成、分析
- **Guides**: Tutorials, how-tos / 教程、操作指南
- **Book**: Comprehensive documentation / 完整文档

---

## 🚀 Getting Started / 快速开始

1. **New to Nexus?** / Nexus 新手？
   - Start with [Introduction](book/src/getting-started/introduction.md)
   - Follow [Quick Start](book/src/getting-started/quick-start.md)

2. **Migrating from Spring Boot?** / 从 Spring Boot 迁移？
   - Read [Migration Guide](guides/migration-guide.md)
   - Check [Spring Comparison](spring-boot/spring-comparison.md)

3. **Looking for API reference?** / 查找 API 参考？
   - See [API Specification](api/api-spec.md)
   - Check [API Quick Reference](api/api-quick-reference.md)

4. **Checking implementation progress?** / 检查实现进度？
   - Review [Master Roadmap](design/MASTER-ROADMAP.md)
   - Check [Final Progress Report](reports/FINAL-PROGRESS-REPORT.md)

---

**Maintained with ❤️ for the Nexus community**

**为 Nexus 社区维护 ❤️**
