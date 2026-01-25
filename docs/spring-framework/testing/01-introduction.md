# Spring Testing Introduction / Spring 测试简介

> Source: https://docs.springframework.org.cn/spring-framework/reference/testing/introduction.html

Testing is an indispensable part of enterprise software development. This chapter focuses on the value of IoC principles for unit testing and the benefits of Spring Framework support for integration testing. (A comprehensive treatment of enterprise testing is beyond the scope of this reference manual.)

测试是企业软件开发不可或缺的一部分。本章重点介绍 IoC 原则为单元测试带来的价值以及 Spring Framework 对集成测试支持的好处。（企业测试的全面论述超出了本参考手册的范围。）

## Overview / 概述

The Spring Framework provides first-class support for integration testing through the `spring-test` module. This support includes:

Spring Framework 在 `spring-test` 模块中提供了对集成测试的一流支持。这种支持包括：

- **Context Management**: Consistent loading and caching of Spring `ApplicationContext` and `WebApplicationContext` instances
  **上下文管理**：一致地加载和缓存 Spring `ApplicationContext` 和 `WebApplicationContext` 实例

- **Dependency Injection**: Ability to inject test fixtures with dependencies from the application context
  **依赖注入**：能够从应用程序上下文中向测试夹具注入依赖项

- **Transaction Management**: Support for transaction management in tests with automatic rollback by default
  **事务管理**：支持测试中的事务管理，默认情况下自动回滚

- **Test Support Classes**: Base classes that simplify writing integration tests
  **测试支持类**：简化集成测试编写的基类

## Key Topics / 主要主题

### Unit Testing / 单元测试

The following sections cover unit testing support:

以下各节介绍单元测试支持：

- Mock objects for Environment, Servlet API, and Spring Web Reactive
  用于 Environment、Servlet API 和 Spring Web Reactive 的 mock 对象

- Testing support classes for general utilities and Spring MVC
  用于通用工具和 Spring MVC 的测试支持类

### Integration Testing / 集成测试

Integration testing support allows you to test:

集成测试支持允许您测试：

- Correct wiring of the Spring IoC container context
  Spring IoC 容器上下文的正确连接

- Data access using JDBC or ORM tools (SQL statements, Hibernate queries, JPA entity mappings, etc.)
  使用 JDBC 或 ORM 工具进行数据访问（SQL 语句、Hibernate 查询、JPA 实体映射等）

### Testing Modules / 测试模块

The Spring Framework provides specialized testing support for various modules:

Spring Framework 为各种模块提供了专门的测试支持：

- JDBC Test Support
  JDBC 测试支持

- Spring TestContext Framework
  Spring TestContext Framework

- WebTestClient
  WebTestClient

- MockMvc
  MockMvc

- Testing Client Applications
  测试客户端应用程序

- Annotations Reference
  注解参考
