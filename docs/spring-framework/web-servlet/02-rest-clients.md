# REST 客户端 (REST Clients)

Spring Framework 提供了以下用于调用 REST 端点的选择：

- `RestClient` - 具有流畅 API 的同步客户端
- `WebClient` - 具有流畅 API 的非阻塞、反应式客户端
- `RestTemplate` - 具有模板方法 API 的同步客户端
- HTTP 接口 - 带有生成的动态代理实现的带注释的接口

---

## RestClient

`RestClient` 是一个同步 HTTP 客户端，提供现代、流畅的 API。它提供了对 HTTP 库的抽象，允许方便地将 Java 对象转换为 HTTP 请求，以及从 HTTP 响应创建对象。

### 创建 RestClient

`RestClient` 是使用静态方法 `create()` 之一创建的。您还可以使用 `builder()` 来获取具有更多选项的构建器，例如：

- 指定要使用的 HTTP 库（请参阅客户端请求工厂）
- 指定要使用的消息转换器（请参阅 HTTP 消息转换）
- 设置默认 URI
- 设置默认路径变量
- 设置默认请求标头
- 注册侦听器和初始值设定项

一旦创建（或构建），`RestClient` 就可以由多个线程安全地使用。

```java
// 默认客户端
RestClient defaultClient = RestClient.create();

// 自定义客户端
RestClient customClient = RestClient.builder()
  .requestFactory(new HttpComponentsClientHttpRequestFactory())
  .messageConverters(converters -> converters.add(new MyCustomMessageConverter()))
  .baseUrl("https://example.com")
  .defaultUriVariables(Map.of("variable", "foo"))
  .defaultHeader("My-Header", "Foo")
  .requestInterceptor(myCustomInterceptor)
  .requestInitializer(myCustomInitializer)
  .build();
```

### 使用 RestClient

#### 请求 URL

使用 `uri()` 方法指定请求 URI。URL 通常指定为 `String`，带有可选的 URI 模板变量：

```java
int id = 42;
restClient.get()
  .uri("https://example.com/orders/{id}", id)
  .retrieve()
  .body(String.class);
```

#### 请求标头和正文

- 使用 `header(String, String)` 添加单个标头
- 使用 `headers(Consumer<HttpHeaders>)` 添加多个标头
- 使用 `accept(MediaType…​)` 设置 Accept 头
- 使用 `contentType(MediaType)` 设置 Content-Type 头

对于可以包含正文的 HTTP 请求（POST、PUT、PATCH）：
- 使用 `body(Object)` 设置请求正文
- 使用 `body(ParameterizedTypeReference)` 处理泛型类型

#### 检索响应

使用 `retrieve()` 访问 HTTP 响应：

```java
// 简单 GET 请求
String result = restClient.get()
  .uri("https://example.com")
  .retrieve()
  .body(String.class);

// 获取完整响应（状态、标头、正文）
ResponseEntity<String> result = restClient.get()
  .uri("https://example.com")
  .retrieve()
  .toEntity(String.class);
```

#### JSON 示例

`RestClient` 使用 Jackson 库将 JSON 转换为对象：

```java
// GET 请求获取 JSON
int id = ...;
Pet pet = restClient.get()
  .uri("https://petclinic.example.com/pets/{id}", id)
  .accept(APPLICATION_JSON)
  .retrieve()
  .body(Pet.class);

// POST 请求发送 JSON
Pet pet = ...;
ResponseEntity<Void> response = restClient.post()
  .uri("https://petclinic.example.com/pets/new")
  .contentType(APPLICATION_JSON)
  .body(pet)
  .retrieve()
  .toBodilessEntity();
```

#### 错误处理

默认情况下，`RestClient` 在检索具有 4xx 或 5xx 状态代码的响应时抛出 `RestClientException` 的子类。可以使用 `onStatus` 覆盖此行为：

```java
String result = restClient.get()
  .uri("https://example.com/this-url-does-not-exist")
  .retrieve()
  .onStatus(HttpStatusCode::is4xxClientError, (request, response) -> {
      throw new MyCustomRuntimeException(response.getStatusCode(), response.getHeaders());
  })
  .body(String.class);
```

#### Exchange

对于更高级的方案，可以通过 `exchange()` 方法访问基础 HTTP 请求和响应：

```java
Pet result = restClient.get()
  .uri("https://petclinic.example.com/pets/{id}", id)
  .accept(APPLICATION_JSON)
  .exchange((request, response) -> {
    if (response.getStatusCode().is4xxClientError()) {
      throw new MyCustomRuntimeException(response.getStatusCode(), response.getHeaders());
    } else {
      Pet pet = convertResponse(response);
      return pet;
    }
  });
```

### HTTP 消息转换

`spring-web` 模块包含用于读取和写入 HTTP 请求和响应正文的 `HttpMessageConverter` 接口。

主要的 `HttpMessageConverter` 实现：

| 消息转换器 | 描述 |
| --- | --- |
| `StringHttpMessageConverter` | 读取和写入 `String` 实例，支持所有文本媒体类型 (`text/*`) |
| `FormHttpMessageConverter` | 读取和写入表单数据，支持 `application/x-www-form-urlencoded` 和 `multipart/form-data` |
| `ByteArrayHttpMessageConverter` | 读取和写入字节数组，支持所有媒体类型 (`*/*`) |
| `MappingJackson2HttpMessageConverter` | 使用 Jackson 读取和写入 JSON，支持 `application/json` |
| `MappingJackson2XmlHttpMessageConverter` | 使用 Jackson XML 扩展读取和写入 XML，支持 `application/xml` |
| `SourceHttpMessageConverter` | 读取和写入 `javax.xml.transform.Source`，支持 `text/xml` 和 `application/xml` |

### 客户端请求工厂

可用的 `ClientRequestFactory` 实现：

- `JdkClientHttpRequestFactory` - 用于 Java 的 `HttpClient`
- `HttpComponentsClientHttpRequestFactory` - 用于 Apache HTTP Components `HttpClient`
- `JettyClientHttpRequestFactory` - 用于 Jetty 的 `HttpClient`
- `ReactorNettyClientRequestFactory` - 用于 Reactor Netty 的 `HttpClient`
- `SimpleClientHttpRequestFactory` - 简单的默认值

如果在构建时未指定请求工厂，它将按以下顺序选择：
1. Apache HttpComponents（如果在类路径上）
2. Jetty（如果在类路径上）
3. Java 的 `HttpClient`（如果 `java.net.http` 模块已加载）
4. 简单的默认值

---

## WebClient

`WebClient` 是用于执行 HTTP 请求的非阻塞反应式客户端。它在 Spring 5.0 中引入，提供了 `RestTemplate` 的替代方案，支持同步、异步和流式处理方案。

`WebClient` 支持以下功能：
- 非阻塞 I/O
- 反应流背压
- 硬件资源少，高并发
- 利用 Java 8 lambda 的功能式流畅 API
- 同步和异步交互
- 向上流式传输到服务器或从服务器向式传输

---

## RestTemplate

`RestTemplate` 以经典的 Spring Template 类的形式提供了基于 HTTP 客户端库的高级 API。

> **注意**: `RestClient` 为同步 HTTP 访问提供了更现代的 API。对于异步和流式处理方案，请考虑反应式 `WebClient`。

### RestTemplate 方法

| 方法组 | 描述 |
| --- | --- |
| `getForObject` | 通过 GET 检索表示 |
| `getForEntity` | 使用 GET 检索 `ResponseEntity`（即 status、headers 和 body） |
| `headForHeaders` | 使用 HEAD 检索资源的所有标头 |
| `postForLocation` | 使用 POST 创建新资源，并从响应中返回 `Location` 标头 |
| `postForObject` | 使用 POST 创建新资源，并从响应中返回表示形式 |
| `postForEntity` | 使用 POST 创建新资源，并从响应中返回表示形式 |
| `put` | 使用 PUT 创建或更新资源 |
| `patchForObject` | 使用 PATCH 更新资源 |
| `delete` | 使用 DELETE 删除指定 URI 处的资源 |
| `optionsForAllow` | 使用 ALLOW 检索资源允许的 HTTP 方法 |
| `exchange` | 更通用的版本，提供额外的灵活性 |
| `execute` | 执行请求的最通用方式 |

### 从 RestTemplate 迁移到 RestClient

| `RestTemplate` 方法 | `RestClient` 等效 |
| --- | --- |
| `getForObject(String, Class, Object…​)` | `get().uri(String, Object…​).retrieve().body(Class)` |
| `getForEntity(String, Class, Object…​)` | `get().uri(String, Object…​).retrieve().toEntity(Class)` |
| `postForObject(String, Object, Class, Object…​)` | `post().uri(String, Object…​).body(Object).retrieve().body(Class)` |
| `postForEntity(String, Object, Class, Object…​)` | `post().uri(String, Object…​).body(Object).retrieve().toEntity(Class)` |
| `put(String, Object, Object…​)` | `put().uri(String, Object…​).body(Object).retrieve().toBodilessEntity()` |
| `delete(String, Object…​)` | `delete().uri(String, Object…​).retrieve().toBodilessEntity()` |
| `exchange(...)` | `method(HttpMethod).uri(...).headers(...).body(...).retrieve().toEntity(Class)` |

---

## HTTP 接口

Spring Framework 允许您将 HTTP 服务定义为带有方法的 Java 接口。您可以将此类接口传递给 `HttpServiceProxyFactory` 来创建一个代理，该代理通过 HTTP 客户端（如 `RestClient` 或 `WebClient`）执行请求。

### 定义接口

```java
interface RepositoryService {

    @GetExchange("/repos/{owner}/{repo}")
    Repository getRepository(@PathVariable String owner, @PathVariable String repo);

    // 更多 HTTP 交换方法...
}
```

### 为 RestClient 创建代理

```java
RestClient restClient = RestClient.builder().baseUrl("https://api.github.com/").build();
RestClientAdapter adapter = RestClientAdapter.create(restClient);
HttpServiceProxyFactory factory = HttpServiceProxyFactory.builderFor(adapter).build();

RepositoryService service = factory.createClient(RepositoryService.class);
```

### 为 WebClient 创建代理

```java
WebClient webClient = WebClient.builder().baseUrl("https://api.github.com/").build();
WebClientAdapter adapter = WebClientAdapter.create(webClient);
HttpServiceProxyFactory factory = HttpServiceProxyFactory.builderFor(adapter).build();

RepositoryService service = factory.createClient(RepositoryService.class);
```

### 方法参数

带注释的 HTTP 交换方法支持以下方法参数：

| 方法参数 | 描述 |
| --- | --- |
| `URI` | 动态设置请求的 URL |
| `UriBuilderFactory` | 提供用于扩展 URI 模板和 URI 变量的工厂 |
| `HttpMethod` | 动态设置请求的 HTTP 方法 |
| `@RequestHeader` | 添加一个或多个请求标头 |
| `@PathVariable` | 添加一个变量，以便在请求 URL 中展开占位符 |
| `@RequestAttribute` | 提供要添加为请求的属性 |
| `@RequestBody` | 提供请求的正文作为要序列化的对象 |
| `@RequestParam` | 添加一个或多个请求参数 |
| `@RequestPart` | 添加请求部分（表单字段、文件等） |
| `@CookieValue` | 添加一个或多个 Cookie |

### 返回值

支持的返回值取决于基础客户端：

**同步返回值**（适用于 `RestClient` 和 `RestTemplate`）：

| 方法返回值 | 描述 |
| --- | --- |
| `void` | 执行给定的请求 |
| `HttpHeaders` | 执行给定的请求并返回响应标头 |
| `<T>` | 执行给定的请求，并将响应内容解码为声明的返回类型 |
| `ResponseEntity<Void>` | 执行给定的请求并返回带有状态和标头的 `ResponseEntity` |
| `ResponseEntity<T>` | 执行给定的请求，将响应内容解码为声明的返回类型 |

**反应式返回值**（适用于 `WebClient`）：

| 方法返回值 | 描述 |
| --- | --- |
| `Mono<Void>` | 执行给定的请求，并发布响应内容（如果有） |
| `Mono<T>` | 执行给定的请求，并将响应内容解码为声明的返回类型 |
| `Flux<T>` | 执行给定的请求并将响应内容解码为声明的流元素类型 |
| `Mono<ResponseEntity<T>>` | 执行给定的请求，将响应内容解码为声明的返回类型 |

### 错误处理

要自定义错误响应处理，您需要配置基础 HTTP 客户端：

**RestClient**：
```java
RestClient restClient = RestClient.builder()
    .defaultStatusHandler(HttpStatusCode::isError, (request, response) -> ...)
    .build();
```

**WebClient**：
```java
WebClient webClient = WebClient.builder()
    .defaultStatusHandler(HttpStatusCode::isError, resp -> ...)
    .build();
```

---

## 参考资料

- [Spring Framework Documentation](https://docs.spring.io/spring-framework/reference/)
- [RestClient API](https://docs.spring.io/spring-framework/docs/current/javadoc-api/org/springframework/web/client/RestClient.html)
- [WebClient API](https://docs.spring.io/spring-framework/docs/current/javadoc-api/org/springframework/web/reactive/function/client/WebClient.html)
- [RestTemplate API](https://docs.spring.io/spring-framework/docs/current/javadoc-api/org/springframework/web/client/RestTemplate.html)
