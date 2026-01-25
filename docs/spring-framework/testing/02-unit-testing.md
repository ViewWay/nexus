# Unit Testing / 单元测试

> Source: https://docs.springframework.org.cn/spring-framework/reference/testing/unit.html

Dependency injection should make your code less dependent on the container than in traditional J2EE / Java EE development. The POJOs that make up your application should be testable in JUnit or TestNG tests, instantiated with the `new` operator, without Spring or any other container. You can use mock objects (in combination with other valuable testing techniques) to test your code in isolation. If you follow Spring's architectural recommendations, the resulting clean layering and componentization of your codebase facilitates easier unit testing. For example, you can test service layer objects by stubbing or mocking DAO or repository interfaces, without needing to access persistent data while running unit tests.

依赖注入应该让你的代码对容器的依赖程度低于传统的 J2EE / Java EE 开发。组成你应用的 POJO 应该可以在 JUnit 或 TestNG 测试中进行测试，通过使用 `new` 运算符实例化对象，而无需 Spring 或任何其他容器。你可以使用 mock 对象（结合其他有价值的测试技术）来隔离地测试你的代码。如果你遵循 Spring 的架构建议，由此产生的清晰分层和代码库组件化有助于更轻松地进行单元测试。例如，你可以通过 stub 或 mock DAO 或 repository 接口来测试服务层对象，而无需在运行单元测试时访问持久化数据。

True unit tests typically run extremely fast, as there is no need to set up a runtime infrastructure. Emphasizing true unit tests as part of your development methodology can boost your productivity. You may not need this section of the testing chapter to help you write effective unit tests for your IoC-based applications. However, for some unit testing scenarios, the Spring Framework provides mock objects and testing support classes, which are described in this section.

真正的单元测试通常运行速度非常快，因为不需要设置运行时基础设施。将真正的单元测试作为开发方法的一部分来强调可以提高你的生产力。你可能不需要测试章节的这一部分来帮助你为基于 IoC 的应用程序编写有效的单元测试。然而，对于某些单元测试场景，Spring Framework 提供了 mock 对象和测试支持类，本章对此进行了描述。

## Mock Objects / Mock 对象

Spring includes a number of packages dedicated to mock objects:

Spring 包含了许多专门用于 mock 的包：

- Environment
- Servlet API
- Spring Web Reactive

### Environment

The `org.springframework.mock.env` package contains mock implementations of the `Environment` and `PropertySource` abstractions (see Bean definition Profiles and the `PropertySource` abstraction). `MockEnvironment` and `MockPropertySource` are useful for developing out-of-container tests that depend on specific environment properties.

`org.springframework.mock.env` 包包含了 `Environment` 和 `PropertySource` 抽象的 mock 实现（参见 Bean 定义 Profile 和 `PropertySource` 抽象）。`MockEnvironment` 和 `MockPropertySource` 对于开发依赖于特定环境属性的容器外测试非常有用。

### Servlet API

The `org.springframework.mock.web` package contains a comprehensive set of Servlet API mock objects that are useful for testing web contexts, controllers, and filters. These mock objects are targeted at usage with Spring's Web MVC framework and are generally more convenient to use than dynamic mock objects (such as EasyMock).

`org.springframework.mock.web` 包包含了一整套 Servlet API mock 对象，这些对象对于测试 web 上下文、控制器和过滤器非常有用。这些 mock 对象主要用于 Spring 的 Web MVC 框架，并且通常比动态 mock 对象（例如 EasyMock）更方便使用。

| Since Spring Framework 6.0, the mock objects in `org.springframework.mock.web` are based on the Servlet 6.0 API. |
| --- |

| 自 Spring Framework 6.0 起，`org.springframework.mock.web` 中的 mock 对象基于 Servlet 6.0 API。 |

MockMvc builds on the mock Servlet API objects to provide an integration testing framework for Spring MVC. See MockMvc.

MockMvc 基于 mock Servlet API 对象构建，为 Spring MVC 提供了一个集成测试框架。参见 MockMvc。

### Spring Web Reactive

The `org.springframework.mock.http.server.reactive` package contains mock implementations of `ServerHttpRequest` and `ServerHttpResponse` for WebFlux applications. The `org.springframework.mock.web.server` package contains a mock `ServerWebExchange` that depends on those mock request and response objects.

`org.springframework.mock.http.server.reactive` 包包含了用于 WebFlux 应用程序的 `ServerHttpRequest` 和 `ServerHttpResponse` 的 mock 实现。`org.springframework.mock.web.server` 包包含了依赖于这些 mock 请求和响应对象的 mock `ServerWebExchange`。

Both `MockServerHttpRequest` and `MockServerHttpResponse` inherit from the same abstract base classes as server-specific implementations and share behavior with them. For example, a mock request is immutable once created, but you can use the `mutate()` method in `ServerHttpRequest` to create a modified instance.

`MockServerHttpRequest` 和 `MockServerHttpResponse` 都继承自与服务器特定实现相同的抽象基类，并与它们共享行为。例如，mock 请求一旦创建就不可变，但你可以使用 `ServerHttpRequest` 中的 `mutate()` 方法创建一个修改后的实例。

To be able to correctly implement the write contract and return a completion handle for writes (i.e., `Mono<Void>`), the mock response by default uses a `Flux` with `cache().then()`, which buffers data and makes it available for assertions in tests. Applications can set a custom write function (for example, to test infinite streams).

为了使 mock 响应能够正确实现写入契约并返回写入完成句柄（即 `Mono<Void>`），它默认使用带有 `cache().then()` 的 `Flux`，这会缓冲数据并使其在测试中可用于断言。应用程序可以设置自定义写入函数（例如，用于测试无限流）。

WebTestClient builds on mock request and response objects to provide support for testing WebFlux apps without an HTTP server. This client can also be used for end-to-end testing with a running server.

WebTestClient 基于 mock 请求和响应构建，为测试 WebFlux 应用提供支持，而无需 HTTP 服务器。该客户端也可用于与运行中的服务器进行端到端测试。

## Unit Testing Support Classes / 单元测试支持类

Spring contains a number of classes that can help with unit testing. They fall into two categories:

Spring 包含许多有助于单元测试的类。它们分为两类：

- General testing utilities
  通用测试工具

- Spring MVC testing utilities
  Spring MVC 测试工具

### General Testing Utilities / 通用测试工具

The `org.springframework.test.util` package contains several general utility methods for use in unit and integration testing.

`org.springframework.test.util` 包包含了一些用于单元测试和集成测试的通用工具类。

`AopTestUtils` is a collection of AOP-related utility methods. You can use these methods to obtain a reference to the underlying target object hidden behind one or more Spring proxies. For example, if you have configured a bean as a dynamic mock by using a library such as EasyMock or Mockito, and the mock is wrapped in a Spring proxy, you may need direct access to the underlying mock to configure expectations on it and perform verifications. See `AopUtils` and `AopProxyUtils` for core Spring AOP utility classes.

`AopTestUtils` 是一系列与 AOP 相关的工具方法集合。你可以使用这些方法获取隐藏在一个或多个 Spring 代理背后的底层目标对象的引用。例如，如果你使用 EasyMock 或 Mockito 等库将一个 bean 配置为动态 mock，并且该 mock 被包装在 Spring 代理中，你可能需要直接访问底层 mock 来配置其期望并执行验证。对于 Spring 的核心 AOP 工具类，请参见 `AopUtils` 和 `AopProxyUtils`。

`ReflectionTestUtils` is a collection of reflection-based utility methods. You can use these methods when you need to invoke a non-`public` setter method, set a non-`public` field, or invoke a non-`public` configuration or lifecycle callback method when testing application code for the following use cases:

`ReflectionTestUtils` 是一系列基于反射的工具方法集合。在以下应用场景中测试应用代码时，如果你需要更改常量的值、设置非 `public` 字段、调用非 `public` setter 方法或调用非 `public` 配置或生命周期回调方法，可以使用这些方法：

- ORM frameworks (such as JPA and Hibernate) that support `private` or `protected` field access (as opposed to `public` setter methods) for properties in domain entities.
  支持 `private` 或 `protected` 字段访问（而非 `public` setter 方法）来访问领域实体属性的 ORM 框架（例如 JPA 和 Hibernate）。

- Spring's support for annotations (such as `@Autowired`, `@Inject`, and `@Resource`) that provide dependency injection for `private` or `protected` fields, setter methods, and configuration methods.
  Spring 对注解（例如 `@Autowired`、`@Inject` 和 `@Resource`）的支持，这些注解为 `private` 或 `protected` 字段、setter 方法和配置方法提供依赖注入。

- Use of lifecycle callbacks such as `@PostConstruct` and `@PreDestroy`.
  使用 `@PostConstruct` 和 `@PreDestroy` 等注解作为生命周期回调方法。

`TestSocketUtils` is a simple utility class that lets you find an available TCP port on `localhost` for use in integration testing scenarios.

`TestSocketUtils` 是一个简单的工具类，用于在集成测试场景中查找 `localhost` 上可用的 TCP 端口。

| `TestSocketUtils` can be used for integration tests that start an external server on an available random port. However, these utilities do not guarantee that a given port remains available and are therefore unreliable. It is recommended to not use `TestSocketUtils` to find an available local port for a server, but to rely on a server to start on a user-specified port or an OS-assigned ephemeral port. To interact with that server, you should query the server for the port it is currently using. |
| --- |

| `TestSocketUtils` 可用于在可用随机端口上启动外部服务器的集成测试。然而，这些工具并不能保证给定端口后续仍然可用，因此是不可靠的。建议不要使用 `TestSocketUtils` 来为服务器查找可用的本地端口，而是依赖服务器自行选择或由操作系统分配的随机临时端口启动。要与该服务器交互，应查询服务器当前正在使用的端口。 |

### Spring MVC Testing Utilities / Spring MVC 测试工具

The `org.springframework.test.web` package contains `ModelAndViewAssert`, which you can use in combination with JUnit, TestNG, or any other testing framework for unit tests that deal with Spring MVC `ModelAndView` objects.

`org.springframework.test.web` 包包含 `ModelAndViewAssert`，你可以将其与 JUnit、TestNG 或任何其他测试框架结合使用，用于处理 Spring MVC `ModelAndView` 对象的单元测试。

| To unit test Spring MVC `Controller` classes as POJOs, use `ModelAndViewAssert` in combination with `MockHttpServletRequest`, `MockHttpSession`, and so on from Spring's Servlet API mocks. For comprehensive integration testing of your Spring MVC and REST `Controller` classes in conjunction with the `WebApplicationContext` configuration for Spring MVC, use MockMvc instead. |
| --- |

| 单元测试 Spring MVC `Controller` 类 要将你的 Spring MVC `Controller` 类作为 POJO 进行单元测试，请结合使用 `ModelAndViewAssert` 和 Spring 的 Servlet API mocks 中的 `MockHttpServletRequest`、`MockHttpSession` 等。要对你的 Spring MVC 和 REST `Controller` 类以及 Spring MVC 的 `WebApplicationContext` 配置进行全面的集成测试，请改为使用 MockMvc。 |
