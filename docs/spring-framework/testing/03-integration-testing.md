# Integration Testing / 集成测试

> Source: https://docs.springframework.org.cn/spring-framework/reference/testing/integration.html

在无需部署到应用服务器或连接到其他企业基础设施的情况下，进行集成测试非常重要。这样做可以让你测试以下方面，例如：

Integration testing without deploying to an application server or connecting to other enterprise infrastructure is important. This allows you to test aspects such as:

- Spring IoC 容器上下文的正确连接（wiring）。
  Correct wiring of the Spring IoC container context.

- 使用 JDBC 或 ORM 工具进行数据访问。这可以包括 SQL 语句的正确性、Hibernate 查询、JPA 实体映射等。
  Data access using JDBC or ORM tools. This can include correctness of SQL statements, Hibernate queries, JPA entity mappings, etc.

Spring Framework 在 `spring-test` 模块中提供了对集成测试的一流支持。实际的 JAR 文件名可能包含版本号，也可能采用完整的 `org.springframework.test` 形式，这取决于你从哪里获取（有关说明，请参见依赖管理章节）。此库包含 `org.springframework.test` 包，其中包含用于与 Spring 容器进行集成测试的有价值的类。这种测试不依赖于应用服务器或其他部署环境。此类测试的运行速度比单元测试慢，但比依赖于部署到应用服务器的等效 Selenium 测试或远程测试快得多。

The Spring Framework provides first-class support for integration testing through the `spring-test` module. The actual JAR file name may include a version number or may be in the form `org.springframework.test`, depending on where you get it from (see the dependency management chapter for details). This library contains the `org.springframework.test` package, which contains valuable classes for integration testing with the Spring container. These tests do not depend on an application server or other deployment environment. Such tests run slower than unit tests but much faster than equivalent Selenium tests or remote tests that rely on deployment to an application server.

单元测试和集成测试支持以注解驱动的 Spring TestContext Framework 的形式提供。TestContext Framework 不依赖于实际使用的测试框架，这允许在各种环境（包括 JUnit、TestNG 等）中对测试进行检测。

Unit and integration testing support is provided in the form of the annotation-driven Spring TestContext Framework. The TestContext Framework does not depend on the actual testing framework used, which allows tests to be instrumented in various environments (including JUnit, TestNG, etc.).

以下部分概述了 Spring 集成支持的高级目标，本章的其余部分将重点介绍专门主题：

The following sections outline the high-level goals of Spring's integration support, and the rest of this chapter will focus on specific topics:

- JDBC 测试支持
  JDBC Test Support

- Spring TestContext Framework
  Spring TestContext Framework

- WebTestClient
  WebTestClient

- MockMvc
  MockMvc

- 测试客户端应用程序
  Testing Client Applications

- 注解
  Annotations

## 集成测试的目标 / Integration Testing Goals

Spring 的集成测试支持具有以下主要目标：

Spring's integration testing support has the following primary goals:

- 管理测试之间的 Spring IoC 容器缓存。
  Manage Spring IoC container caching between tests.

- 提供测试夹具实例的依赖注入。
  Provide dependency injection for test fixture instances.

- 提供适合集成测试的事务管理。
  Provide transaction management suitable for integration tests.

- 提供帮助开发者编写集成测试的Spring 特定基类。
  Provide Spring-specific base classes that help developers write integration tests.

接下来的几个部分将描述每个目标，并提供实现和配置细节的链接。

The next few sections will describe each goal and provide links to implementation and configuration details.

### 上下文管理和缓存 / Context Management and Caching

Spring TestContext Framework 提供 Spring `ApplicationContext` 实例和 `WebApplicationContext` 实例的一致加载以及这些上下文的缓存。对加载的上下文进行缓存的支持非常重要，因为启动时间可能成为一个问题——并非由于 Spring 本身的开销，而是由于 Spring 容器实例化的对象需要时间进行实例化。例如，一个包含 50 到 100 个 Hibernate 映射文件的项目可能需要 10 到 20 秒来加载映射文件，在每个测试夹具中运行每个测试之前都产生此成本会导致整体测试运行变慢，从而降低开发者的生产力。

The Spring TestContext Framework provides consistent loading of Spring `ApplicationContext` and `WebApplicationContext` instances and caching of these contexts. Support for caching loaded contexts is important because startup time can become an issue—not due to Spring's overhead, but because the objects instantiated by the Spring container take time to instantiate. For example, a project with 50 to 100 Hibernate mapping files may take 10 to 20 seconds to load the mapping files, and incurring this cost before running each test in every test fixture would result in slower overall test runs, reducing developer productivity.

测试类通常声明用于 XML 或 Groovy 配置元数据的资源位置数组（通常在类路径中），或者用于配置应用程序的组件类数组。这些位置或类与生产部署中 `web.xml` 或其他配置文件中指定的相同或相似。

Test classes typically declare an array of resource locations for XML or Groovy configuration metadata (usually on the classpath) or an array of component classes for configuring the application. These locations or classes are the same as or similar to those specified in `web.xml` or other configuration files in production deployment.

默认情况下，一旦加载，配置好的 `ApplicationContext` 会在每个测试中被复用。因此，设置成本只在每个测试套件中发生一次，随后的测试执行会快得多。在此上下文中，"测试套件"一词指在同一个 JVM 中运行的所有测试——例如，给定项目或模块通过 Ant、Maven 或 Gradle 构建运行的所有测试。万一测试破坏了应用程序上下文并需要重新加载（例如，通过修改 Bean 定义或应用程序对象的状态），TestContext Framework 可以配置为在执行下一个测试之前重新加载配置并重建应用程序上下文。

By default, once loaded, the configured `ApplicationContext` is reused in each test. Therefore, the setup cost occurs only once per test suite, and subsequent test execution is much faster. In this context, the term "test suite" refers to all tests running in the same JVM—for example, all tests run by Ant, Maven, or Gradle for a given project or module. In case a test corrupts the application context and requires reloading (for example, by modifying a bean definition or the state of an application object), the TestContext Framework can be configured to reload the configuration and rebuild the application context before executing the next test.

请参见 TestContext Framework 中的上下文管理和上下文缓存。

See context management and context caching in the TestContext Framework.

### 测试夹具的依赖注入 / Dependency Injection of Test Fixtures

当 TestContext Framework 加载你的应用程序上下文时，它可以选择性地使用依赖注入来配置测试类的实例。这提供了一种方便的机制，通过使用应用程序上下文中的预配置 Bean 来设置测试夹具。这里的一个显著优势是，你可以在各种测试场景中复用应用程序上下文（例如，用于配置 Spring 管理的对象图、事务代理、`DataSource` 实例等），从而避免为每个单独的测试用例重复复杂的测试夹具设置。

When the TestContext Framework loads your application context, it can optionally use dependency injection to configure instances of your test classes. This provides a convenient mechanism for setting up test fixtures by using pre-configured beans from the application context. A significant advantage here is that you can reuse the application context across various test scenarios (for example, for configuring a Spring-managed object graph, transaction proxies, `DataSource` instances, etc.), avoiding the need to repeat complex test fixture setup for each individual test case.

举个例子，考虑这样一个场景：我们有一个类 (`HibernateTitleRepository`) 实现了 `Title` 领域实体的数据访问逻辑。我们希望编写集成测试来测试以下方面：

For example, consider a scenario where we have a class (`HibernateTitleRepository`) that implements data access logic for the `Title` domain entity. We want to write integration tests to test the following aspects:

- Spring 配置：基本上，所有与 `HibernateTitleRepository` Bean 配置相关的内容是否正确且存在？
  Spring configuration: Basically, is everything related to the `HibernateTitleRepository` bean configuration correct and present?

- Hibernate 映射文件配置：是否所有内容都正确映射，并且延迟加载设置是否正确？
  Hibernate mapping file configuration: Is everything mapped correctly, and are the lazy loading settings correct?

- `HibernateTitleRepository` 的逻辑：这个类的配置实例是否按预期工作？
  The logic of `HibernateTitleRepository`: Does the configured instance of this class work as expected?

请参见 TestContext Framework 中的测试夹具依赖注入。

See test fixture dependency injection in the TestContext Framework.

### 事务管理 / Transaction Management

访问真实数据库的测试中的一个常见问题是它们对持久化存储状态的影响。即使你使用开发数据库，状态的改变也可能影响未来的测试。此外，许多操作——例如插入或修改持久化数据——不能在事务外部执行（或验证）。

A common problem in tests that access a real database is their impact on the state of persistent storage. Even if you use a development database, changes to state can affect future tests. Additionally, many operations—such as inserting or modifying persistent data—cannot be performed (or verified) outside a transaction.

TestContext Framework 解决了这个问题。默认情况下，该框架为每个测试创建一个事务并回滚。你可以编写代码，假设事务的存在。如果在测试中调用经事务代理的对象，它们会根据其配置的事务语义正确运行。此外，如果测试方法在为该测试管理的事务中运行时删除了选定表的内容，事务会默认回滚，数据库会恢复到执行测试之前的状态。通过使用测试的应用程序上下文中定义的 `PlatformTransactionManager` Bean，为测试提供事务支持。

The TestContext Framework addresses this problem. By default, the framework creates a transaction for each test and rolls it back. You can write code assuming the existence of a transaction. If you call a transactionally-proxied object in a test, it will execute correctly according to its configured transaction semantics. Additionally, if a test method deletes the contents of selected tables while running in a transaction managed for that test, the transaction will be rolled back by default, and the database will be restored to its state before the test was executed. Transaction support for tests is provided through the `PlatformTransactionManager` bean defined in the test application context.

如果你希望事务提交（这不常见，但在你希望特定测试填充或修改数据库时偶尔有用），你可以通过使用 `@Commit` 注解来告诉 TestContext Framework 提交事务而不是回滚。

If you wish for a transaction to be committed (which is uncommon but occasionally useful when you want a specific test to populate or modify the database), you can tell the TestContext Framework to commit the transaction instead of rolling it back by using the `@Commit` annotation.

请参见 TestContext Framework 中的事务管理。

See transaction management in the TestContext Framework.

### 集成测试的支持类 / Integration Testing Support Classes

Spring TestContext Framework 提供了一些 `abstract` 支持类，简化了集成测试的编写。这些基础测试类提供了与测试框架的良好定义的连接点以及方便的实例变量和方法，使你可以访问：

The Spring TestContext Framework provides some `abstract` support classes that simplify writing integration tests. These base test classes provide well-defined connection points with testing frameworks as well as convenient instance variables and methods that give you access to:

- `ApplicationContext`，用于执行显式 Bean 查找或测试整个上下文的状态。
  `ApplicationContext`, for performing explicit bean lookups or testing the state of the entire context.

- `JdbcTemplate`，用于执行 SQL 语句查询数据库。你可以使用此类查询在数据库相关应用代码执行前后确认数据库状态，并且 Spring 确保此类查询与应用代码运行在同一事务范围内。与 ORM 工具结合使用时，请务必避免假阳性。
  `JdbcTemplate`, for executing SQL statements to query the database. You can use this class to query and confirm database state before and after execution of database-related application code, and Spring ensures that such queries run in the same transaction scope as the application code. Be careful to avoid false positives when using this with ORM tools.

此外，你可能希望创建自己的自定义、应用程序范围的超类，其中包含特定于你项目的实例变量和方法。

Additionally, you may wish to create your own custom, application-wide superclass containing instance variables and methods specific to your project.

请参见 TestContext Framework 的支持类。

See support classes in the TestContext Framework.
