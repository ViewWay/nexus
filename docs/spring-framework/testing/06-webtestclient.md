# WebTestClient

> Source: https://docs.springframework.org.cn/spring-framework/reference/testing/webtestclient.html

## Overview / 概述

`WebTestClient` is an HTTP client for testing server applications. It wraps Spring's `WebClient` and uses it to perform requests, but exposes a testing facade for verifying responses. `WebTestClient` can be used to perform end-to-end HTTP tests. It can also be used to test Spring MVC and Spring WebFlux applications without a running server by simulating server request and response objects.

`WebTestClient` 是用于测试服务器应用程序的 HTTP 客户端。它包装了 Spring 的 `WebClient` 并使用它来执行请求，但暴露了用于验证响应的测试门面。`WebTestClient` 可用于执行端到端 HTTP 测试。它还可以通过模拟服务器请求和响应对象来测试 Spring MVC 和 Spring WebFlux 应用程序，而无需运行服务器。

## Setup / 设置

To set up `WebTestClient`, you need to choose a server setup to bind to. This can be one of several mock server setup options, or it can be a connection to a running server.

要设置 `WebTestClient`，你需要选择一个服务器设置进行绑定。这可以是几种模拟服务器设置选项之一，也可以是连接到正在运行的服务器。

### Binding to a Controller / 绑定到控制器

This setting allows you to test a specific controller by simulating request and response objects without the need to run a server.

此设置允许你通过模拟请求和响应对象测试特定的控制器，而无需运行服务器。

For WebFlux applications, use the following configuration, which loads infrastructure equivalent to WebFlux Java configuration, registers the given controller, and creates a WebHandler chain to handle requests:

对于 WebFlux 应用程序，使用以下配置，它加载相当于 WebFlux Java 配置的基础设施，注册给定的控制器，并创建一个 WebHandler 链来处理请求：

```java
WebTestClient client =
    WebTestClient.bindToController(new TestController()).build();
```

For Spring MVC, use the following configuration, which delegates to `StandaloneMockMvcBuilder` to load infrastructure equivalent to WebMvc Java configuration, registers the given controller, and creates a MockMvc instance to handle requests:

对于 Spring MVC，使用以下配置，它委托给 `StandaloneMockMvcBuilder` 加载相当于 WebMvc Java 配置的基础设施，注册给定的控制器，并创建一个 MockMvc 实例来处理请求：

```java
WebTestClient client =
    MockMvcWebTestClient.bindToController(new TestController()).build();
```

### Binding to `ApplicationContext` / 绑定到 `ApplicationContext`

This setting allows you to load Spring configuration that includes Spring MVC or Spring WebFlux infrastructure and controller declarations, and use it to handle requests by simulating request and response objects without running a server.

此设置允许你加载包含 Spring MVC 或 Spring WebFlux 基础设施和控制器声明的 Spring 配置，并使用它通过模拟请求和响应对象处理请求，而无需运行服务器。

For WebFlux, use the following configuration, which passes the Spring `ApplicationContext` to `WebHttpHandlerBuilder` to create a WebHandler chain for handling requests:

对于 WebFlux，使用以下配置，将 Spring `ApplicationContext` 传递给 `WebHttpHandlerBuilder` 以创建用于处理请求的 WebHandler 链：

```java
@SpringJUnitConfig(WebConfig.class)
class MyTests {
    WebTestClient client;
    @BeforeEach
    void setUp(ApplicationContext context) {
        client = WebTestClient.bindToApplicationContext(context).build();
    }
}
```

For Spring MVC, use the following configuration, which passes the Spring `ApplicationContext` to `MockMvcBuilders.webAppContextSetup` to create a MockMvc instance to handle requests:

对于 Spring MVC，使用以下配置，将 Spring `ApplicationContext` 传递给 `MockMvcBuilders.webAppContextSetup` 以创建一个 MockMvc 实例来处理请求：

```java
@ExtendWith(SpringExtension.class)
@WebAppConfiguration("classpath:META-INF/web-resources")
@ContextHierarchy({
    @ContextConfiguration(classes = RootConfig.class),
    @ContextConfiguration(classes = WebConfig.class)
})
class MyTests {
    @Autowired
    WebApplicationContext wac;
    WebTestClient client;
    @BeforeEach
    void setUp() {
        client = MockMvcWebTestClient.bindToApplicationContext(this.wac).build();
    }
}
```

### Binding to a Router Function / 绑定到路由函数

This setting allows you to test functional endpoints by simulating request and response objects without running a server.

此设置允许你通过模拟请求和响应对象测试函数式端点，而无需运行服务器。

For WebFlux, use the following configuration, which delegates to `RouterFunctions.toWebHandler` to create a server setup for handling requests:

对于 WebFlux，使用以下配置，它委托给 `RouterFunctions.toWebHandler` 创建用于处理请求的服务器设置：

```java
RouterFunction<?> route = ...
client = WebTestClient.bindToRouterFunction(route).build();
```

For Spring MVC, there is currently no option to test WebMvc functional endpoints.

对于 Spring MVC，目前没有选项来测试 WebMvc 函数式端点。

### Binding to a Server / 绑定到服务器

This setting connects to a running server to perform full end-to-end HTTP tests:

此设置连接到正在运行的服务器以执行完整的端到端 HTTP 测试：

```java
client = WebTestClient.bindToServer().baseUrl("https://example.org:8080").build();
```

### Client Configuration / 客户端配置

In addition to the server setup options described earlier, you can also configure client options, including base URL, default headers, client filters, etc. These options are available immediately after calling `bindToServer()`. For all other configuration options, you need to use `configureClient()` to switch from server configuration to client configuration, as shown below:

除了前面描述的服务器设置选项外，你还可以配置客户端选项，包括基本 URL、默认请求头、客户端过滤器等。这些选项在调用 `bindToServer()` 后立即可用。对于所有其他配置选项，你需要使用 `configureClient()` 从服务器配置切换到客户端配置，如下所示：

```java
client = WebTestClient.bindToController(new TestController())
        .configureClient()
        .baseUrl("/test")
        .build();
```

## Writing Tests / 编写测试

`WebTestClient` provides the same API as `WebClient` until the request is executed using `exchange()`. See the `WebClient` documentation for examples of how to prepare requests with form data, multipart data, etc.

`WebTestClient` 提供了与 `WebClient` 相同的 API，直到使用 `exchange()` 执行请求。有关如何准备包含表单数据、多部分数据等的请求的示例，请参阅 `WebClient` 文档。

After calling `exchange()`, `WebTestClient` differs from `WebClient` by continuing with a workflow for verifying the response.

调用 `exchange()` 后，`WebTestClient` 与 `WebClient` 不同，而是继续进行验证响应的工作流程。

### Asserting Status and Headers / 断言状态和请求头

To assert response status and headers, use the following methods:

要断言响应状态和请求头，请使用以下方法：

```java
client.get().uri("/persons/1")
    .accept(MediaType.APPLICATION_JSON)
    .exchange()
    .expectStatus().isOk()
    .expectHeader().contentType(MediaType.APPLICATION_JSON);
```

### Decoding Response Body / 解码响应体

You can then choose to decode the response body by:

然后你可以选择通过以下任一方式解码响应体：

- `expectBody(Class<T>)`: Decode to a single object.
  解码为单个对象。

- `expectBodyList(Class<T>)`: Decode and collect objects into a `List<T>`.
  解码并将对象收集到 `List<T>` 中。

- `expectBody()`: For JSON content, decode to `byte[]` or an empty response body.
  对于 JSON 内容，解码为 `byte[]` 或空响应体。

And perform assertions on the resulting high-level objects:

并对生成的高级对象执行断言：

```java
client.get().uri("/persons")
        .exchange()
        .expectStatus().isOk()
        .expectBodyList(Person.class).hasSize(3).contains(person);
```

### No Content / 无内容

If the response should not contain content, you can assert as follows:

如果响应不应包含内容，可以按如下方式断言：

```java
client.post().uri("/persons")
        .body(personMono, Person.class)
        .exchange()
        .expectStatus().isCreated()
        .expectBody().isEmpty();
```

### JSON Content / JSON 内容

You can use `expectBody()` without a target type to assert on the raw content instead of through high-level objects.

你可以使用不带目标类型的 `expectBody()` 来对原始内容进行断言，而不是通过高级对象进行断言。

Using JSONAssert to verify complete JSON content:

使用 JSONAssert 验证完整的 JSON 内容：

```java
client.get().uri("/persons/1")
        .exchange()
        .expectStatus().isOk()
        .expectBody()
        .json("{\"name\":\"Jane\"}")
```

Using JSONPath to verify JSON content:

使用 JSONPath 验证 JSON 内容：

```java
client.get().uri("/persons")
        .exchange()
        .expectStatus().isOk()
        .expectBody()
        .jsonPath("$[0].name").isEqualTo("Jane")
        .jsonPath("$[1].name").isEqualTo("Jason");
```

### Streaming Responses / 流式响应

To test potentially infinite streams such as `"text/event-stream"` or `"application/x-ndjson"`, first verify response status and headers, and then obtain a `FluxExchangeResult`:

要测试潜在的无限流（例如 `"text/event-stream"` 或 `"application/x-ndjson"`），首先验证响应状态和请求头，然后获取一个 `FluxExchangeResult`：

```java
FluxExchangeResult<MyEvent> result = client.get().uri("/events")
        .accept(TEXT_EVENT_STREAM)
        .exchange()
        .expectStatus().isOk()
        .returnResult(MyEvent.class);
```

Now you can use `StepVerifier` from `reactor-test` to consume the response stream:

现在你可以使用 `reactor-test` 中的 `StepVerifier` 来消费响应流：

```java
Flux<Event> eventFlux = result.getResponseBody();

StepVerifier.create(eventFlux)
        .expectNext(person)
        .expectNextCount(4)
        .consumeNextWith(p -> ...)
        .thenCancel()
        .verify();
```

---

**Related Topics / 相关主题**

- Spring TestContext Framework
- MockMvc
- Testing Client Applications
