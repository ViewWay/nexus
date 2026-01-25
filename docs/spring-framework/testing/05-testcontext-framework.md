# Spring TestContext Framework / Spring TestContext 框架

> Source: https://docs.springframework.org.cn/spring-framework/reference/testing/testcontext-framework.html

## Overview / 概述

The Spring TestContext Framework (located in the `org.springframework.test.context` package) provides generic, annotation-driven unit and integration testing support that is independent of the testing framework used. The TestContext framework also places a high value on "convention over configuration," providing reasonable defaults that can be overridden through annotation-based configuration.

Spring TestContext Framework（位于 `org.springframework.test.context` 包中）提供了通用的、注解驱动的单元和集成测试支持，并且与所使用的测试框架无关。TestContext framework 也非常重视"约定优于配置"，提供了可以通过基于注解的配置来覆盖的合理默认值。

In addition to general testing infrastructure, the TestContext framework provides explicit support for JUnit 4, JUnit Jupiter (also known as JUnit 5), and TestNG. For JUnit 4 and TestNG, Spring provides `abstract` support classes. Additionally, Spring provides custom JUnit `Runner` and custom JUnit `Rules` for JUnit 4, and custom `Extension` for JUnit Jupiter, which allow you to write so-called POJO test classes. POJO test classes do not need to inherit from specific class hierarchies, such as `abstract` support classes.

除了通用的测试基础设施外，TestContext framework 还提供了对 JUnit 4、JUnit Jupiter（也称为 JUnit 5）和 TestNG 的明确支持。对于 JUnit 4 和 TestNG，Spring 提供了 `abstract` 支持类。此外，Spring 还为 JUnit 4 提供了自定义的 JUnit `Runner` 和自定义的 JUnit `Rules`，并为 JUnit Jupiter 提供了自定义的 `Extension`，这些使得你可以编写所谓的 POJO 测试类。POJO 测试类不需要继承特定的类层级结构，例如 `abstract` 支持类。

## Section Summary / 本节摘要

- Key Abstractions / 关键抽象
- Bootstrapping the TestContext Framework / 引导 TestContext Framework
- `TestExecutionListener` Configuration / `TestExecutionListener` 配置
- Application Events / 应用程序事件
- Test Execution Events / 测试执行事件
- Context Management / 上下文管理
- Dependency Injection of Test Fixtures / 测试夹具的依赖注入
- Bean Overriding in Tests / 测试中的 Bean 覆盖
- Testing Request and Session Scoped Beans / 测试请求和会话作用域 Bean
- Transaction Management / 事务管理
- Executing SQL Scripts / 执行 SQL 脚本
- Parallel Test Execution / 并行测试执行
- TestContext Framework Support Classes / TestContext Framework 支持类
- AOT Testing Support / 测试的预先支持

---

**Related Topics / 相关主题**

- JDBC Test Support
- WebTestClient
- MockMvc
