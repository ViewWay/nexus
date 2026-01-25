# Spring Boot Overview / Spring Boot 概述

Source: https://docs.springframework.org.cn/spring-boot/index.html

---

## English

Spring Boot helps you create stand-alone, production-grade Spring based applications that you can run. We take an opinionated view of the Spring platform and third-party libraries so you can get started with minimum fuss. Most Spring Boot applications need very little Spring configuration.

You can use Spring Boot to create Java applications that can be started using `java -jar` or more traditional war deployments.

Our main goals are:

- Provide a radically faster and more accessible getting-started experience for all Spring development.
- Be opinionated out of the box but get out of the way quickly as requirements start to diverge from the defaults.
- Provide a range of non-functional features that are common to large classes of projects (e.g. embedded servers, security, metrics, health checks, externalized configuration).
- Absolutely no code generation (when not targeting native images) and no requirement for XML configuration.

---

## 中文 / Chinese

Spring Boot 帮助你创建独立的、生产级别的基于 Spring 的应用程序，你可以运行它们。我们对 Spring 平台和第三方库持有一种明确的观点，以便你可以轻松上手。大多数 Spring Boot 应用程序只需要很少的 Spring 配置。

你可以使用 Spring Boot 创建 Java 应用程序，这些应用程序可以通过 `java -jar` 或更传统的 war 部署来启动。

我们的主要目标是：

- 为所有 Spring 开发提供一种从根本上更快且更易访问的入门体验。
- 开箱即用时具有明确的观点，但在需求开始偏离默认值时快速让路。
- 提供一系列对大型项目类通用的非功能特性（例如嵌入式服务器、安全、指标、健康检查和外部化配置）。
- 绝对没有代码生成（在不针对原生镜像时）并且不需要 XML 配置。

---

## Documentation Structure / 文档结构

The Spring Boot documentation includes the following main sections:

### 1. Getting Started / 入门
- Overview / 概述
- System Requirements / 系统要求
- Installing Spring Boot / 安装 Spring Boot
- Upgrading Spring Boot / 升级 Spring Boot
- Developing your first Spring Boot application / 开发你的第一个 Spring Boot 应用

### 2. Using Spring Boot / 使用 Spring Boot
- Build Systems (Maven, Gradle, Ant, Starters) / 构建系统
- Code Structure / 代码结构
- Configuration Classes / 配置类
- Auto-configuration / 自动配置
- Spring Beans and Dependency Injection / Spring Bean 和依赖注入
- Using @SpringBootApplication Annotation / 使用 @SpringBootApplication 注解
- Running your Application / 运行你的应用
- Developer Tools / 开发者工具
- Packaging for Production / 打包应用以用于生产环境

### 3. Core Features / 核心特性
- SpringApplication
- Externalized Configuration / 外部化配置
- Profiles / 配置文件
- Logging / 日志记录
- Internationalization / 国际化
- AOP (Aspect-Oriented Programming) / 面向切面编程
- JSON
- Task Execution and Scheduling / 任务执行和调度
- Devtools Services / 开发时服务
- Creating your own Auto-configuration / 创建你自己的自动配置
- Kotlin Support / Kotlin 支持
- SSL

### 4. Web / Web
- Servlet Web Applications / Servlet Web 应用
- Reactive Web Applications / 响应式 Web 应用
- Graceful Shutdown / 优雅关闭
- Spring Security
- Spring Session
- Spring for GraphQL
- Spring HATEOAS

### 5. Data / 数据
- SQL Databases / SQL 数据库
- Using NoSQL Technologies / 使用 NoSQL 技术

### 6. IO / IO
- Caching / 缓存
- Hazelcast
- Quartz Scheduler / Quartz 调度器
- Sending Email / 发送邮件
- Validation / 验证
- Calling REST Services / 调用 REST 服务
- Web Services / Web 服务
- Distributed Transactions with JTA / 使用 JTA 的分布式事务

### 7. Messaging / 消息
- JMS
- AMQP
- Apache Kafka Support / Apache Kafka 支持
- Apache Pulsar Support / Apache Pulsar 支持
- RSocket
- Spring Integration
- WebSockets

### 8. Testing / 测试
- Test Scope Dependencies / 测试范围依赖
- Testing Spring Applications / 测试 Spring 应用
- Testing Spring Boot Applications / 测试 Spring Boot 应用
- Testcontainers
- Test Utilities / 测试实用程序

### 9. Packaging Spring Boot Applications / 打包 Spring Boot 应用
- Efficient Deployment / 高效部署
- Class Data Sharing / 类数据共享
- Ahead-of-Time Processing with JVM / 使用 JVM 进行提前处理
- GraalVM Native Images / GraalVM 原生镜像
- Checkpoint and Restore with JVM / 使用 JVM 进行检查点和恢复
- Container Images / 容器镜像

### 10. Production-Ready Features / 生产就绪特性
- Enabling Production-Ready Features / 启用生产就绪特性
- Endpoints / 端点
- Monitoring and Management over HTTP / 通过 HTTP 进行监控和管理
- Monitoring and Management over JMX / 通过 JMX 进行监控和管理
- Observability / 可观察性
- Loggers / 日志记录器
- Metrics / 指标
- Tracing / 跟踪
- Auditing / 审计
- Recording HTTP Exchanges / 记录 HTTP 交换
- Process Monitoring / 进程监控
- Cloud Foundry Support

### 11. How-to Guides / 操作指南
- Spring Boot Application / Spring Boot 应用
- Properties and Configuration / 属性和配置
- Embedded Web Servers / 嵌入式 Web 服务器
- Spring MVC
- Jersey
- HTTP Clients / HTTP 客户端
- Logging / 日志记录
- Data Access / 数据访问
- Database Initialization / 数据库初始化
- NoSQL
- Messaging / 消息
- Batch Applications / 批处理应用
- Actuator
- Security / 安全
- Hot Swapping / 热替换
- Testing / 测试
- Building / 构建
- Ahead-of-Time Processing / 提前处理
- GraalVM Native Applications / GraalVM 原生应用
- Class Data Sharing / 类数据共享
- Deploying Spring Boot Applications / 部署 Spring Boot 应用
- Docker Compose

### 12. Build Tool Plugins / 构建工具插件
- Maven Plugin / Maven 插件
- Gradle Plugin / Gradle 插件
- Spring Boot AntLib Module / Spring Boot AntLib 模块
- Support for Other Build Systems / 支持其他构建系统

### 13. Spring Boot CLI

### 14. REST API

### 15. Appendices / 附录
- Common Application Properties / 常见应用属性
- Auto-configuration Classes / 自动配置类
- Test Auto-configuration Annotations / 测试自动配置注解
- Dependency Versions / 依赖版本
