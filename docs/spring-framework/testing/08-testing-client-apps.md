# Testing Client Applications / 测试客户端应用程序

> Source: https://docs.springframework.org.cn/spring-framework/reference/testing/spring-mvc-test-client.html

## Overview / 概述

You can use client-side testing to test code that internally uses `RestTemplate`. The core idea is to declare expected requests and provide "stub" responses so that you can focus on testing the code in isolation (i.e., without a running server).

您可以使用客户端测试来测试内部使用 `RestTemplate` 的代码。核心思想是声明预期的请求并提供"存根"响应，这样您就可以专注于独立地测试代码（即无需运行服务器）。

## Basic Example / 基本示例

The following example shows how to implement this:

以下示例展示了如何实现：

```java
RestTemplate restTemplate = new RestTemplate();

MockRestServiceServer mockServer = MockRestServiceServer.bindTo(restTemplate).build();
mockServer.expect(requestTo("/greeting")).andRespond(withSuccess());

// Test code that uses the above RestTemplate ...

mockServer.verify();
```

In the preceding example, `MockRestServiceServer` (the core class for client REST testing) configures the `RestTemplate` with a custom `ClientHttpRequestFactory` that asserts actual requests against expectations and returns "stub" responses. In this case, we expect a request to `/greeting` and want to return a 200 response with `text/plain` content. We can define additional expected requests and stub responses as needed. When we've defined expected requests and stub responses, the `RestTemplate` can be used as usual in client code. At the end of the test, `mockServer.verify()` can be used to verify that all expectations have been satisfied.

在前面的示例中，`MockRestServiceServer`（客户端 REST 测试的核心类）使用自定义的 `ClientHttpRequestFactory` 配置 `RestTemplate`，该工厂会针对期望断言实际请求并返回"存根"响应。在本例中，我们期望一个对 `/greeting` 的请求，并希望返回一个带有 `text/plain` 内容的 200 响应。我们可以根据需要定义额外的预期请求和存根响应。当我们定义了预期请求和存根响应后，`RestTemplate` 可以在客户端代码中照常使用。测试结束时，可以使用 `mockServer.verify()` 来验证所有期望是否都已满足。

## Request Ordering / 请求顺序

By default, requests are expected to arrive in the order in which expectations are declared. You can set the `ignoreExpectOrder` option when building the server, in which case all expectations are checked (in order) to find a match for a given request. This means that requests can arrive in any order. The following example uses `ignoreExpectOrder`:

默认情况下，请求的顺序与期望声明的顺序一致。您可以在构建服务器时设置 `ignoreExpectOrder` 选项，在这种情况下，所有期望都会被检查（按顺序）以找到与给定请求匹配的那个。这意味着请求可以按任意顺序到达。以下示例使用了 `ignoreExpectOrder`：

```java
server = MockRestServiceServer.bindTo(restTemplate).ignoreExpectOrder(true).build();
```

## Expected Count / 预期计数

Even when requests are not ordered by default, each request can only run once. The `expect` method provides an overloaded variant that accepts an `ExpectedCount` parameter that specifies a count range (e.g., `once`, `manyTimes`, `max`, `min`, `between`, etc.). The following example uses `times`:

即使默认情况下请求是无序的，每个请求也只能运行一次。`expect` 方法提供了一个重载变体，它接受一个 `ExpectedCount` 参数，用于指定计数范围（例如，`once`、`manyTimes`、`max`、`min`、`between` 等）。以下示例使用了 `times`：

```java
RestTemplate restTemplate = new RestTemplate();

MockRestServiceServer mockServer = MockRestServiceServer.bindTo(restTemplate).build();
mockServer.expect(times(2), requestTo("/something")).andRespond(withSuccess());
mockServer.expect(times(3), requestTo("/somewhere")).andRespond(withSuccess());

// ...

mockServer.verify();
```

Note that when `ignoreExpectOrder` is not set (the default), and request expectations are in declared order, that order applies only to the first occurrence of any expected request. For example, if "/something" is expected twice followed by "/somewhere" three times, then there should be one request to "/something" before any requests to "/somewhere", but aside from that, subsequent "/something" and "/somewhere" requests can arrive at any time.

请注意，当 `ignoreExpectOrder` 未设置（默认值），并且请求期望按声明顺序进行时，该顺序仅适用于任何预期请求的第一次出现。例如，如果期望 "/something" 出现两次，然后期望 "/somewhere" 出现三次，那么在 "/somewhere" 的请求出现之前，应该有一个对 "/something" 的请求，但除此之外，后续的 "/something" 和 "/somewhere" 请求可以在任何时候出现。

## Binding to MockMvc / 绑定到 MockMvc

As an alternative to the above approach, client testing support also provides a `ClientHttpRequestFactory` implementation that you can configure into a `RestTemplate` to bind it to a `MockMvc` instance. This allows requests to be handled with actual server-side logic but without running a server. The following example shows how to do this:

作为上述方法的替代方案，客户端测试支持还提供了一个 `ClientHttpRequestFactory` 实现，您可以将其配置到 `RestTemplate` 中，以将其绑定到 `MockMvc` 实例。这允许使用实际的服务器端逻辑处理请求，但无需运行服务器。以下示例展示了如何实现：

```java
MockMvc mockMvc = MockMvcBuilders.webAppContextSetup(this.wac).build();
this.restTemplate = new RestTemplate(new MockMvcClientHttpRequestFactory(mockMvc));

// Test code that uses the above RestTemplate ...
```

## Executing Actual Requests / 执行实际请求

In some cases, you may need to make an actual call to a remote service instead of mocking the response. The following example shows how to do this with `ExecutingResponseCreator`:

在某些情况下，可能需要实际调用远程服务而不是模拟响应。以下示例展示了如何通过 `ExecutingResponseCreator` 来实现：

```java
RestTemplate restTemplate = new RestTemplate();

// Create ExecutingResponseCreator with the original request factory
ExecutingResponseCreator withActualResponse = new ExecutingResponseCreator(restTemplate.getRequestFactory());

MockRestServiceServer mockServer = MockRestServiceServer.bindTo(restTemplate).build();
mockServer.expect(requestTo("/profile")).andRespond(withSuccess());
mockServer.expect(requestTo("/quoteOfTheDay")).andRespond(withActualResponse);

// Test code that uses the above RestTemplate ...

mockServer.verify();
```

In the preceding example, we created the `ExecutingResponseCreator` using the `RestTemplate`'s factory _before_ the `MockRestServiceServer` replaced the `RestTemplate`'s `ClientHttpRequestFactory` with a different factory. We then defined expectations for two types of responses:

在前面的示例中，我们在 `MockRestServiceServer` 用不同的工厂替换 `RestTemplate` 的 `ClientHttpRequestFactory` _之前_，使用 `RestTemplate` 的工厂创建了 `ExecutingResponseCreator`。然后，我们定义了两种响应类型的期望：

- For the `/profile` endpoint, return a stub `200` response (the actual request will not be executed)
  对于 `/profile` 端点，返回一个存根 `200` 响应（不会执行实际请求）

- The response obtained by calling the `/quoteOfTheDay` endpoint
  通过调用 `/quoteOfTheDay` 端点获得的响应

In the second case, the request is executed through the previously captured `ClientHttpRequestFactory`. Depending on how the `RestTemplate` was originally configured, this results in a response that may be from an actual remote server.

在第二种情况下，请求通过先前捕获的 `ClientHttpRequestFactory` 执行。根据 `RestTemplate` 最初的配置方式，这会生成一个响应，该响应可能来自实际的远程服务器。

---

**Related Topics / 相关主题**

- WebTestClient
- MockMvc
- Spring TestContext Framework
