# Spring Framework 概述

Spring 使创建 Java 企业应用变得容易。它提供了在企业环境中拥抱 Java 语言所需的一切，支持 Groovy 和 Kotlin 作为 JVM 上的替代语言，并且具有根据应用需求创建多种架构的灵活性。从 Spring Framework 6.0 开始，Spring 要求使用 Java 17+。

Spring 支持广泛的应用场景。在大型企业中，应用通常存在很长时间，并且必须在开发者无法控制升级周期的 JDK 和应用服务器上运行。其他应用可能以内嵌服务器的单个 jar 形式运行，可能在云环境中。还有一些可能是独立的应用程序（例如批处理或集成工作负载），不需要服务器。

Spring 是开源的。它拥有一个庞大而活跃的社区，根据各种实际用例提供持续反馈。这帮助 Spring 在漫长的岁月中成功演进。

## "Spring" 的含义

术语 "Spring" 在不同上下文中具有不同的含义。它可以指 Spring Framework 项目本身，这是它的起点。随着时间的推移，许多其他 Spring 项目已经构建在 Spring Framework 之上。通常，当人们说 "Spring" 时，他们指的是整个项目家族。本参考文档重点介绍基础：Spring Framework 本身。

Spring Framework 被划分为模块。应用可以选择需要的模块。其核心是核心容器模块，包括配置模型和依赖注入机制。除此之外，Spring Framework 为不同的应用架构提供了基础支持，包括消息、事务数据和持久化以及 Web。它还包括基于 Servlet 的 Spring MVC Web 框架，以及并行存在的 Spring WebFlux 响应式 Web 框架。

关于模块的说明：Spring Framework 的 jar 包允许部署到模块路径 (Java 模块系统)。对于在启用模块的应用中使用，Spring Framework 的 jar 包带有 `Automatic-Module-Name` manifest 条目，它们定义了独立于 jar 工件名称的稳定的语言级别模块名称（例如 `spring.core`, `spring.context` 等）。jar 包遵循相同的命名模式，但使用 `-` 代替 `.`，例如 `spring-core` 和 `spring-context`。当然，Spring Framework 的 jar 包在类路径上也能正常工作。

## Spring 和 Spring Framework 的历史

Spring 诞生于 2003 年，旨在应对早期 J2EE 规范的复杂性。虽然有些人认为 Java EE 及其现代继任者 Jakarta EE 与 Spring 存在竞争，但它们实际上是互补的。Spring 编程模型不拥抱 Jakarta EE 平台规范；相反，它集成了传统 EE 伞形下的精心挑选的各个规范：

- Servlet API (JSR 340)
- WebSocket API (JSR 356)
- 并发工具 (JSR 236)
- JSON 绑定 API (JSR 367)
- Bean 验证 (JSR 303)
- JPA (JSR 338)
- JMS (JSR 914)
- 以及用于事务协调的 JTA/JCA 设置（如有必要）。

Spring Framework 还支持依赖注入 (JSR 330) 和通用注解 (JSR 250) 规范，应用开发者可以选择使用它们，而不是 Spring Framework 提供的 Spring 特定机制。最初，这些是基于常用的 `javax` 包。

从 Spring Framework 6.0 开始，Spring 已升级到 Jakarta EE 9 级别（例如，Servlet 5.0+，JPA 3.0+），基于 `jakarta` namespace 而非传统的 `javax` 包。以 EE 9 作为最低版本并已支持 EE 10，Spring 准备为 Jakarta EE API 的进一步演进提供开箱即用的支持。Spring Framework 6.0 完全兼容 Tomcat 10.1、Jetty 11 和 Undertow 2.3 等 Web 服务器，以及 Hibernate ORM 6.1。

随着时间的推移，Java/Jakarta EE 在应用开发中的作用已经演变。在 J2EE 和 Spring 的早期，应用是创建并部署到应用服务器的。如今，借助 Spring Boot，应用以 DevOps 友好和云友好的方式创建，内嵌 Servlet 容器且易于更改。从 Spring Framework 5 开始，WebFlux 应用甚至不直接使用 Servlet API，可以在非 Servlet 容器（例如 Netty）的服务器上运行。

Spring 不断创新和演进。除了 Spring Framework，还有其他项目，例如 Spring Boot、Spring Security、Spring Data、Spring Cloud、Spring Batch 等等。重要的是要记住，每个项目都有自己的源代码仓库、问题跟踪器和发布周期。请访问 spring.io/projects 查看完整的 Spring 项目列表。

## 设计理念

当您学习一个框架时，不仅要了解它做什么，还要了解它遵循的原则。以下是 Spring Framework 的指导原则：

- **在每个层面提供选择** - Spring 允许您尽可能晚地延迟设计决策。例如，您可以通过配置切换持久化提供者而无需更改代码。许多其他基础设施问题以及与第三方 API 的集成也是如此。
- **适应不同的视角** - Spring 具有灵活性，对于事情应该如何做并不固执己见。它支持具有不同视角的广泛应用需求。
- **保持强大的向后兼容性** - Spring 的演进经过精心管理，以尽量减少版本之间的破坏性更改。Spring 支持精心选择的 JDK 版本范围和第三方库，以方便依赖 Spring 的应用和库的维护。
- **关注 API 设计** - Spring 团队投入大量思考和时间来设计直观且能在多个版本和多年中保持稳定的 API。
- **设定高标准的代码质量** - Spring Framework 非常重视有意义、最新且准确的 javadoc。它是少数几个可以声称代码结构清晰、包之间没有循环依赖的项目之一。

## 反馈和贡献

对于操作方法问题或诊断、调试问题，我们建议使用 Stack Overflow。如果您相当确定 Spring Framework 中存在问题或想建议一个功能，请使用 GitHub Issues。

如果您有解决方案或建议的修复方法，可以在 Github 上提交 pull request。

## 入门

如果您刚开始使用 Spring，您可能希望通过创建一个基于 Spring Boot 的应用来开始使用 Spring Framework。Spring Boot 提供了一种快速（且规范）的方式来创建生产就绪的基于 Spring 的应用。它基于 Spring Framework，偏爱约定优于配置，旨在让您尽快上手。

您可以使用 start.spring.io 生成一个基本项目，或者遵循其中一个"入门"指南。

## 文档结构

本 Spring Framework 文档包含以下主要部分：

1. **核心技术** (Core Technology)
   - IoC 容器
   - 资源
   - 验证、数据绑定和类型转换
   - Spring Expression Language (SpEL)
   - 面向切面编程 (AOP)
   - 空安全
   - 数据缓冲区和编解码器
   - 日志记录
   - AOT 优化

2. **数据访问** (Data Access)
   - 事务管理
   - DAO 支持
   - JDBC 数据访问
   - R2DBC 数据访问
   - ORM 数据访问

3. **Servlet 栈上的 Web** (Web on Servlet Stack)
   - Spring Web MVC
   - REST 客户端
   - WebSockets

4. **响应式栈上的 Web** (Web on Reactive Stack)
   - Spring WebFlux
   - WebClient
   - HTTP Interface Client
   - RSocket

5. **测试** (Testing)
   - 单元测试
   - 集成测试
   - TestContext Framework
   - MockMvc

6. **集成** (Integration)
   - REST 客户端
   - JMS
   - JMX
   - 电子邮件
   - 任务执行和调度
   - 缓存抽象
   - 可观测性支持

7. **语言支持** (Language Support)
   - Kotlin
   - Groovy
   - 动态语言支持
