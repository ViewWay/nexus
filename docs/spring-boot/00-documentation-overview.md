# Documentation Overview / 文档概述

Source: https://docs.springframework.org.cn/spring-boot/documentation.html

---

## English

This section provides a brief overview of the Spring Boot reference documentation. It serves as a guide for the rest of the documentation.

### Getting Started

If you are just starting out with Spring Boot or 'Spring' in general, start with these topics:

- **From Scratch:** Overview | Requirements | Installation
- **Tutorial:** Part 1 | Part 2
- **Running Your Example:** Part 1 | Part 2

### Upgrading from Earlier Releases

You should always make sure that you are running a supported version of Spring Boot.

Depending on the version to which you are upgrading, you can find some additional hints here:

- **Upgrading from 1.x to 2.x:** Upgrading from 1.x
- **Upgrading from 2.x:** Upgrading from 2.x
- **Upgrading to a New Feature Version:** Upgrading to a New Feature Version
- **Spring Boot CLI:** Upgrading the Spring Boot CLI

### Developing with Spring Boot

Ready to start using Spring Boot? We have you covered:

- **Build Systems:** Maven | Gradle | Ant | Starters
- **Best Practices:** Code Structure | @Configuration | @EnableAutoConfiguration | Beans and Dependency Injection
- **Running Your Code:** IDE | Packaged | Maven | Gradle
- **Packaging Your Application:** Production jar
- **Spring Boot CLI:** Using the CLI

### Learning Spring Boot Features

Want to know more about Spring Boot core features? We have got you covered:

- **Spring Application:** SpringApplication
- **External Configuration:** External Configuration
- **Profiles:** Profiles
- **Logging:** Logging

### Web

If you are developing Spring Boot web applications, check these out:

- **Servlet Web Applications:** Spring MVC, Jersey, Embedded Servlet Containers
- **Reactive Web Applications:** Spring WebFlux, Embedded Servlet Containers
- **Graceful Shutdown:** Graceful Shutdown
- **Spring Security:** Default Security Configuration, OAuth2 Auto-configuration, SAML
- **Spring Session:** Spring Session Auto-configuration
- **Spring HATEOAS:** Spring HATEOAS Auto-configuration

### Data

If your application involves data storage, you can check here how to configure:

- **SQL:** Configuration of SQL data stores, Embedded database support, Connection pooling, etc.
- **NoSQL:** Auto-configuration for Redis, MongoDB, Neo4j and other NoSQL stores.

### Messaging

If your application uses any messaging protocol, check one or more of the following sections:

- **JMS:** ActiveMQ and Artemis auto-configuration, Sending and receiving messages over JMS
- **AMQP:** RabbitMQ auto-configuration
- **Kafka:** Spring Kafka auto-configuration
- **Pulsar:** Spring for Apache Pulsar auto-configuration
- **RSocket:** Auto-configuration for Spring Framework RSocket support
- **Spring Integration:** Spring Integration auto-configuration

### IO

If your application needs IO capabilities, check one or more of the following sections:

- **Caching:** Support for EhCache, Hazelcast, Infinispan and any JSR-107 compliant cache
- **Quartz:** Quartz scheduling
- **Mail:** Sending email
- **Validation:** JSR-303 validation
- **REST Clients:** Calling REST services with RestTemplate and WebClient
- **Web Services:** Spring Web Services auto-configuration
- **JTA:** Distributed transactions with JTA

### Container Images

Spring Boot provides first class support for building efficient container images. You can read more about this here:

- **Efficient Container Images:** Tips for optimizing container images such as Docker images
- **Dockerfiles:** Building container images using dockerfiles
- **Cloud Native Buildpacks:** Maven and Gradle support for Cloud Native Buildpacks

### Moving to Production

When you are ready to push your Spring Boot application to production, we have some tricks you might like:

- **Management Endpoints:** Overview
- **Connectivity Options:** HTTP | JMX
- **Monitoring:** Metrics | Auditing | HTTP Exchanges | Process

### Optimizing for Production

The techniques described in these chapters can be used to optimize a Spring Boot application for production:

- **Efficient Deployment:** Unpacking the executable JAR
- **GraalVM Native Images:** Introduction | Advanced Topics | Getting Started | Testing
- **Class Data Sharing:** Overview
- **Checkpoint and Restore** Overview

### Advanced Topics

Finally, we have some more advanced topics for the power users:

- **Spring Boot Application Deployment:** Cloud Deployment | Operating System Service
- **Build Tool Plugins:** Maven | Gradle
- **Appendices:** Application Properties | Configuration Metadata | Auto-configuration Classes | Test Auto-configuration Annotations | Executable Jar | Dependency Versions

---

## 中文 / Chinese

本节提供了 Spring Boot 参考文档的简要概述。它充当了文档其余部分的指南。

### 第一步

如果你刚开始接触 Spring Boot 或一般性的 'Spring'，请从以下主题开始：

- **从零开始：** 概述 | 要求 | 安装
- **教程：** 第一部分 | 第二部分
- **运行你的示例：** 第一部分 | 第二部分

### 从早期版本升级

你应始终确保你正在运行 Spring Boot 的受支持版本。

根据你要升级到的版本，你可以在此处找到一些额外的提示：

- **从 1.x 升级到 2.x：** 从 1.x 升级
- **从 2.x 升级：** 从 2.x 升级
- **升级到新功能版本：** 升级到新功能版本
- **Spring Boot CLI：** 升级 Spring Boot CLI

### 使用 Spring Boot 进行开发

准备好开始使用 Spring Boot 了吗？我们为你准备好了：

- **构建系统：** Maven | Gradle | Ant | Starters
- **最佳实践：** 代码结构 | @Configuration | @EnableAutoConfiguration | Bean 和依赖注入
- **运行你的代码：** IDE | 打包后 | Maven | Gradle
- **打包你的应用：** 生产环境 jar 包
- **Spring Boot CLI：** 使用 CLI

### 学习 Spring Boot 特性

想了解 Spring Boot 核心特性的更多细节？以下内容为你准备好了：

- **Spring 应用：** SpringApplication
- **外部配置：** 外部配置
- **Profiles：** Profiles
- **日志：** 日志

### Web

如果你开发 Spring Boot Web 应用，请查看以下内容：

- **Servlet Web 应用：** Spring MVC, Jersey, 嵌入式 Servlet 容器
- **响应式 Web 应用：** Spring WebFlux, 嵌入式 Servlet 容器
- **优雅停机：** 优雅停机
- **Spring Security：** 默认安全配置, OAuth2 自动配置, SAML
- **Spring Session：** Spring Session 自动配置
- **Spring HATEOAS：** Spring HATEOAS 自动配置

### 数据

如果你的应用涉及数据存储，你可以在此处查看如何配置：

- **SQL：** 配置 SQL 数据存储，嵌入式数据库支持，连接池等。
- **NoSQL：** Redis, MongoDB, Neo4j 等 NoSQL 存储的自动配置。

### 消息

如果你的应用使用任何消息协议，请查看以下一个或多个部分：

- **JMS：** ActiveMQ 和 Artemis 自动配置，通过 JMS 发送和接收消息
- **AMQP：** RabbitMQ 自动配置
- **Kafka：** Spring Kafka 自动配置
- **Pulsar：** Spring for Apache Pulsar 自动配置
- **RSocket：** Spring Framework RSocket 支持的自动配置
- **Spring Integration：** Spring Integration 自动配置

### IO

如果你的应用需要 IO 能力，请查看以下一个或多个部分：

- **缓存：** 支持 EhCache, Hazelcast, Infinispan 等缓存
- **Quartz：** Quartz 调度
- **邮件：** 发送邮件
- **校验：** JSR-303 校验
- **REST 客户端：** 使用 RestTemplate 和 WebClient 调用 REST 服务
- **Web 服务：** Spring Web Services 自动配置
- **JTA：** 使用 JTA 的分布式事务

### 容器镜像

Spring Boot 为构建高效容器镜像提供了第一类支持。你可以在此处阅读更多相关内容：

- **高效容器镜像：** 优化 Docker 镜像等容器镜像的技巧
- **Dockerfiles：** 使用 dockerfiles 构建容器镜像
- **Cloud Native Buildpacks：** Maven 和 Gradle 对 Cloud Native Buildpacks 的支持

### 迁移到生产环境

当你准备将 Spring Boot 应用推送到生产环境时，我们有一些你可能喜欢的技巧：

- **管理端点：** 概述
- **连接选项：** HTTP | JMX
- **监控：** 指标 | 审计 | HTTP 交互 | 进程

### 针对生产环境进行优化

可以使用这些章节描述的技术来优化 Spring Boot 应用以便用于生产环境：

- **高效部署：** 解压可执行 JAR
- **GraalVM Native Images：** 介绍 | 高级主题 | 入门 | 测试
- **类数据共享：** 概述
- **检查点和恢复** 概述

### 高级主题

最后，我们为更高级的用户准备了一些主题：

- **Spring Boot 应用部署：** 云部署 | 操作系统服务
- **构建工具插件：** Maven | Gradle
- **附录：** 应用属性 | 配置元数据 | 自动配置类 | 测试自动配置注解 | 可执行 Jar | 依赖版本
