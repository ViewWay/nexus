# Annotations / 注解

> Source: https://docs.springframework.org.cn/spring-framework/reference/testing/annotations.html

## Overview / 概述

This section covers the annotations that can be used when testing Spring applications.

本节介绍在测试 Spring 应用程序时可以使用的注解。

## Section Summary / 本节摘要

- Standard Annotation Support / 标准注解支持
- Spring Test Annotations / Spring 测试注解
- Spring JUnit 4 Test Annotations / Spring JUnit 4 测试注解
- Spring JUnit Jupiter Test Annotations / Spring JUnit Jupiter 测试注解
- Meta-Annotation Support for Testing / 测试的元注解支持

## Standard Annotation Support / 标准注解支持

The Spring Framework supports the following standard annotations:

Spring Framework 支持以下标准注解：

- `@JUnit` / `@Test` - JUnit test annotations
  JUnit 测试注解

- `@Before` / `BeforeEach` - Methods to run before each test
  在每个测试之前运行的方法

- `@After` / `AfterEach` - Methods to run after each test
  在每个测试之后运行的方法

- `@BeforeClass` / `BeforeAll` - Methods to run once before all tests
  在所有测试之前运行一次的方法

- `@AfterClass` / `AfterAll` - Methods to run once after all tests
  在所有测试之后运行一次的方法

- `@Ignore` / `@Disabled` - Disable tests
  禁用测试

## Spring Test Annotations / Spring 测试注解

The following are Spring-specific test annotations:

以下是 Spring 特定的测试注解：

### `@BootstrapWith`

`@BootstrapWith` is a class-level annotation that is used to configure how the Spring TestContext Framework is bootstrapped.

`@BootstrapWith` 是一个类级别的注解，用于配置如何引导 Spring TestContext Framework。

### `@ContextConfiguration`

`@ContextConfiguration` defines class-level metadata that is used to determine how to load and configure an `ApplicationContext` for integration tests.

`@ContextConfiguration` 定义类级别的元数据，用于确定如何为集成测试加载和配置 `ApplicationContext`。

### `@WebAppConfiguration`

`@WebAppConfiguration` is a class-level annotation that is used to declare that the `ApplicationContext` loaded for an integration test is a `WebApplicationContext`.

`@WebAppConfiguration` 是一个类级别的注解，用于声明为集成测试加载的 `ApplicationContext` 是一个 `WebApplicationContext`。

### `@ContextHierarchy`

`@ContextHierarchy` is a class-level annotation that is used to define a hierarchy of `ApplicationContext` instances for integration tests.

`@ContextHierarchy` 是一个类级别的注解，用于为集成测试定义 `ApplicationContext` 实例的层次结构。

### `@ActiveProfiles`

`@ActiveProfiles` is a class-level annotation that is used to declare which bean definition profiles should be active when loading an `ApplicationContext` for an integration test.

`@ActiveProfiles` 是一个类级别的注解，用于声明在为集成测试加载 `ApplicationContext` 时应该激活哪些 bean 定义 profile。

### `@TestPropertySource`

`@TestPropertySource` is a class-level annotation that is used to configure the locations of properties files and inlined properties to be added to the set of `PropertySources` in the `Environment` for an `ApplicationContext` for integration tests.

`@TestPropertySource` 是一个类级别的注解，用于配置属性文件的位置和内联属性，这些属性将被添加到集成测试的 `ApplicationContext` 的 `Environment` 中的 `PropertySources` 集合中。

### `@DirtiesContext`

`@DirtiesContext` indicates that the underlying Spring `ApplicationContext` has been dirtied during the execution of a test (i.e., the test modified or corrupted it in some manner) and should be closed.

`@DirtiesContext` 指示在测试执行期间，底层的 Spring `ApplicationContext` 已被"弄脏"（即，测试以某种方式修改或损坏了它），并且应该被关闭。

### `@TestExecutionListeners`

`@TestExecutionListeners` defines class-level metadata for configuring which `TestExecutionListener` implementations should be registered with the `TestContextManager`.

`@TestExecutionListeners` 定义类级别的元数据，用于配置应该向 `TestContextManager` 注册哪些 `TestExecutionListener` 实现。

### `@Commit`

`@Commit` indicates that the transaction for a test-managed transaction should be committed after the test method has completed.

`@Commit` 指示测试管理的事务应该在测试方法完成后提交。

### `@Rollback`

`@Rollback` indicates whether the transaction for a test-managed transaction should be rolled back after the test method has completed.

`@Rollback` 指示测试管理的事务是否应该在测试方法完成后回滚。

### `@BeforeTransaction`

`@BeforeTransaction` indicates that the annotated method should be executed before a transaction is started for test methods configured to run within a transaction.

`@BeforeTransaction` 指示在为配置为在事务中运行的测试方法启动事务之前，应该执行带注解的方法。

### `@AfterTransaction`

`@AfterTransaction` indicates that the annotated method should be executed after a transaction has been ended for test methods configured to run within a transaction.

`@AfterTransaction` 指示在为配置为在事务中运行的测试方法结束事务之后，应该执行带注解的方法。

### `@Sql`

`@Sql` is used to annotate a test method or class to configure SQL scripts to be executed against a given database during integration tests.

`@Sql` 用于注解测试方法或类，以配置在集成测试期间针对给定数据库执行的 SQL 脚本。

### `@SqlConfig`

`@SqlConfig` defines metadata that is used to determine how to parse and configure SQL scripts that are executed via the `@Sql` annotation.

`@SqlConfig` 定义元数据，用于确定如何解析和配置通过 `@Sql` 注解执行的 SQL 脚本。

### `@SqlMergeMode`

`@SqlMergeMode` is used to annotate test methods or classes to configure whether method-level `@Sql` declarations are merged with class-level `@Sql` declarations.

`@SqlMergeMode` 用于注解测试方法或类，以配置方法级别的 `@Sql` 声明是否与类级别的 `@Sql` 声明合并。

### `@SqlGroup`

`@SqlGroup` is a container annotation that aggregates several `@Sql` annotations.

`@SqlGroup` 是一个容器注解，用于聚合多个 `@Sql` 注解。

### `@MockitoBean` and `@MockitoSpyBean`

`@MockitoBean` is used to add a Mockito mock to the `ApplicationContext`.

`@MockitoSpyBean` is used to add a Mockito spy to the `ApplicationContext`.

`@MockitoBean` 用于将 Mockito mock 添加到 `ApplicationContext`。

`@MockitoSpyBean` 用于将 Mockito spy 添加到 `ApplicationContext`。

---

**Related Topics / 相关主题**

- Spring TestContext Framework
- Integration Testing
- Unit Testing
