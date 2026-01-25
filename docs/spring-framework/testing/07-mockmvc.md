# MockMvc

> Source: https://docs.springframework.org.cn/spring-framework/reference/testing/mockmvc.html

## Overview / 概述

MockMvc provides support for testing Spring MVC applications. It performs full Spring MVC request handling but uses mock request and response objects instead of a running server.

MockMvc 为测试 Spring MVC 应用程序提供了支持。它执行完整的 Spring MVC 请求处理，但使用模拟请求和响应对象，而不是运行中的服务器。

MockMvc itself can be used to perform requests and verify responses with Hamcrest, or verify using the fluent API provided by AssertJ through `MockMvcTester`. It can also be used through `WebTestClient`, where MockMvc acts as the server handling requests. The advantage of using `WebTestClient` is that it gives you the option to work with higher level objects instead of raw data, and the ability to switch to full end-to-end HTTP tests against a live server using the same test API.

MockMvc 本身可用于执行请求并使用 Hamcrest 验证响应，或通过 `MockMvcTester` 使用 AssertJ 提供的流畅 API 进行验证。它也可以通过 `WebTestClient` 使用，其中 MockMvc 作为服务器处理请求。使用 `WebTestClient` 的优点在于，它为你提供了使用更高级别对象（而非原始数据）进行操作的选项，以及切换到针对实时服务器进行完整端到端 HTTP 测试并使用相同测试 API 的能力。

## Section Summary / 本节摘要

- Overview / 概览
- Setup Options / 设置选项
- Hamcrest Integration / Hamcrest 集成
- AssertJ Integration / AssertJ 集成
- HtmlUnit Integration / HtmlUnit 集成
- MockMvc vs End-to-End Tests / MockMvc 与端到端测试
- More Examples / 更多示例

---

**Related Topics / 相关主题**

- WebTestClient
- Spring TestContext Framework
- Testing Client Applications
