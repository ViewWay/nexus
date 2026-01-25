# REST 客户端

Spring Framework 提供以下选项来调用 REST 端点：

- `RestClient` - 带有 fluent API 的同步客户端。
- `WebClient` - 带有 fluent API 的非阻塞、响应式客户端。
- `RestTemplate` - 带有 template method API 的同步客户端。
- HTTP Interface - 带有生成的动态代理实现的注解接口。

## `RestClient`

`RestClient` 是一个提供现代、fluent API 的同步 HTTP 客户端。它提供了一个对 HTTP 库的抽象，可以方便地将 Java 对象转换为 HTTP 请求，并从 HTTP 响应创建对象。

### 创建 `RestClient`

`RestClient` 是使用静态的 `create` 方法之一创建的。您也可以使用 `builder()` 获取一个带有更多选项的构建器，例如指定要使用的 HTTP 库（参见客户端请求工厂）和要使用的消息转换器（参见HTTP 消息转换），设置默认 URI、默认路径变量、默认请求头或 `uriBuilderFactory`，或注册拦截器和初始化器。

一旦创建（或构建）完成，`RestClient` 可以安全地被多个线程使用。

以下示例展示了如何创建一个默认的 `RestClient`，以及如何构建一个自定义的。

**Java**
```java
RestClient defaultClient = RestClient.create();

RestClient customClient = RestClient.builder()
    .requestFactory(new HttpComponentsClientHttpRequestFactory())
    .messageConverters(converters -> converters.add(new MyCustomMessageConverter()))
    .baseUrl("https://example.com")
    .defaultUriVariables(Map.of("variable", "foo"))
    .defaultHeader("My-Header", "Foo")
    .defaultCookie("My-Cookie", "Bar")
    .requestInterceptor(myCustomInterceptor)
    .requestInitializer(myCustomInitializer)
    .build();
```

**Kotlin**
```kotlin
val defaultClient = RestClient.create()

val customClient = RestClient.builder()
    .requestFactory(HttpComponentsClientHttpRequestFactory())
    .messageConverters { converters -> converters.add(MyCustomMessageConverter()) }
    .baseUrl("https://example.com")
    .defaultUriVariables(mapOf("variable" to "foo"))
    .defaultHeader("My-Header", "Foo")
    .defaultCookie("My-Cookie", "Bar")
    .requestInterceptor(myCustomInterceptor)
    .requestInitializer(myCustomInitializer)
    .build()
```

### 使用 `RestClient`

使用 `RestClient` 发送 HTTP 请求时，首先要指定要使用的 HTTP 方法。这可以通过 `method(HttpMethod)` 或便捷方法 `get()`、`head()`、`post()` 等来完成。

#### 请求 URL

接下来，可以使用 `uri` 方法指定请求 URI。此步骤是可选的，如果 `RestClient` 配置了默认 URI，则可以跳过。URL 通常指定为 `String`，带有可选的 URI 模板变量。以下示例配置一个指向 `example.com/orders/42` 的 GET 请求。

**Java**
```java
int id = 42;
restClient.get()
    .uri("https://example.com/orders/{id}", id)
    // ...
```

**Kotlin**
```kotlin
val id = 42
restClient.get()
    .uri("https://example.com/orders/{id}", id)
    // ...
```

也可以使用函数来实现更多控制，例如指定请求参数。

String URL 默认会进行编码，但可以通过使用自定义 `uriBuilderFactory` 构建客户端来更改此行为。URL 也可以通过函数或 `java.net.URI` 提供，这两者都不会进行编码。

#### 请求头和请求体

如有必要，可以通过 `header(String, String)`、`headers(Consumer<HttpHeaders>` 或便捷方法 `accept(MediaType…​)`、`acceptCharset(Charset…​)` 等添加请求头来操作 HTTP 请求。对于可以包含请求体（`POST`、`PUT` 和 `PATCH`）的 HTTP 请求，还有其他可用方法：`contentType(MediaType)` 和 `contentLength(long)`。

请求体本身可以通过 `body(Object)` 设置，它在内部使用HTTP 消息转换。另外，请求体可以使用 `ParameterizedTypeReference` 设置，从而可以使用泛型。最后，请求体可以设置为写入 `OutputStream` 的回调函数。

#### 检索响应

设置好请求后，可以通过在 `retrieve()` 之后链式调用方法来发送。例如，响应体可以通过 `retrieve().body(Class)` 或针对列表等参数化类型使用 `retrieve().body(ParameterizedTypeReference)` 访问。`body` 方法将响应内容转换为各种类型——例如，字节可以转换为 `String`，JSON 可以使用 Jackson 转换为对象，等等。

响应也可以转换为 `ResponseEntity`，从而可以通过 `retrieve().toEntity(Class)` 访问响应头和响应体。

> 单独调用 `retrieve()` 是一个空操作，返回一个 `ResponseSpec`。应用必须在 `ResponseSpec` 上调用一个终端操作才能产生任何副作用。如果您的用例对消费响应不感兴趣，可以使用 `retrieve().toBodilessEntity()`。

此示例展示了如何使用 `RestClient` 执行简单的 `GET` 请求。

**Java**
```java
String result = restClient.get() // (1)
    .uri("https://example.com") // (2)
    .retrieve() // (3)
    .body(String.class); // (4)

System.out.println(result); // (5)
```

| 步骤 | 说明 |
|------|------|
| 1 | 设置 GET 请求 |
| 2 | 指定要连接的 URL |
| 3 | 检索响应 |
| 4 | 将响应转换为字符串 |
| 5 | 打印结果 |

**Kotlin**
```kotlin
val result= restClient.get() // (1)
    .uri("https://example.com") // (2)
    .retrieve() // (3)
    .body<String>() // (4)

println(result) // (5)
```

可以通过 `ResponseEntity` 访问响应状态码和响应头。

**Java**
```java
ResponseEntity<String> result = restClient.get()
    .uri("https://example.com")
    .retrieve()
    .toEntity(String.class);

System.out.println("Response status: " + result.getStatusCode());
System.out.println("Response headers: " + result.getHeaders());
System.out.println("Contents: " + result.getBody());
```

**Kotlin**
```kotlin
val result = restClient.get()
    .uri("https://example.com")
    .retrieve()
    .toEntity<String>()

println("Response status: " + result.statusCode)
println("Response headers: " + result.headers)
println("Contents: " + result.body)
```

`RestClient` 可以使用 Jackson 库将 JSON 转换为对象。注意此示例中 URI 变量的使用，以及 `Accept` 头设置为 JSON。

**Java**
```java
int id = ...;
Pet pet = restClient.get()
    .uri("https://petclinic.example.com/pets/{id}", id)
    .accept(APPLICATION_JSON)
    .retrieve()
    .body(Pet.class);
```

**Kotlin**
```kotlin
val id = ...
val pet = restClient.get()
    .uri("https://petclinic.example.com/pets/{id}", id)
    .accept(APPLICATION_JSON)
    .retrieve()
    .body<Pet>()
```

在下一个示例中，使用 `RestClient` 执行一个包含 JSON 的 POST 请求，JSON 同样使用 Jackson 进行转换。

**Java**
```java
Pet pet = ...
ResponseEntity<Void> response = restClient.post()
    .uri("https://petclinic.example.com/pets/new")
    .contentType(APPLICATION_JSON)
    .body(pet)
    .retrieve()
    .toBodilessEntity();
```

**Kotlin**
```kotlin
val pet: Pet = ...
val response = restClient.post()
    .uri("https://petclinic.example.com/pets/new")
    .contentType(APPLICATION_JSON)
    .body(pet)
    .retrieve()
    .toBodilessEntity()
```

#### 错误处理

默认情况下，当检索到状态码为 4xx 或 5xx 的响应时，`RestClient` 会抛出 `RestClientException` 的子类。此行为可以通过 `onStatus` 进行覆盖。

**Java**
```java
String result = restClient.get()
    .uri("https://example.com/this-url-does-not-exist")
    .retrieve()
    .onStatus(HttpStatusCode::is4xxClientError, (request, response) -> {
        throw new MyCustomRuntimeException(response.getStatusCode(), response.getHeaders());
    })
    .body(String.class);
```

**Kotlin**
```kotlin
val result = restClient.get()
    .uri("https://example.com/this-url-does-not-exist")
    .retrieve()
    .onStatus(HttpStatusCode::is4xxClientError) { _, response ->
        throw MyCustomRuntimeException(response.getStatusCode(), response.getHeaders())
    }
    .body<String>()
```

#### Exchange

对于更高级的场景，`RestClient` 通过 `exchange()` 方法提供了对底层 HTTP 请求和响应的访问。使用 `exchange()` 时不会应用状态处理程序。

**Java**
```java
Pet result = restClient.get()
    .uri("https://petclinic.example.com/pets/{id}", id)
    .accept(APPLICATION_JSON)
    .exchange((request, response) -> {
        if (response.getStatusCode().is4xxClientError()) {
            throw new MyCustomRuntimeException(response.getStatusCode(), response.getHeaders());
        }
        else {
            Pet pet = convertResponse(response);
            return pet;
        }
    });
```

### HTTP 消息转换

请参阅专门章节中支持的 HTTP 消息转换器。

#### Jackson JSON Views

为了仅序列化对象属性的子集，您可以指定一个Jackson JSON View：

```java
MappingJacksonValue value = new MappingJacksonValue(new User("eric", "7!jd#h23"));
value.setSerializationView(User.WithoutPasswordView.class);

ResponseEntity<Void> response = restClient.post()
    .contentType(APPLICATION_JSON)
    .body(value)
    .retrieve()
    .toBodilessEntity();
```

#### Multipart

要发送 multipart 数据，您需要提供一个 `MultiValueMap<String, Object>`：

```java
MultiValueMap<String, Object> parts = new LinkedMultiValueMap<>();

parts.add("fieldPart", "fieldValue");
parts.add("filePart", new FileSystemResource("...logo.png"));
parts.add("jsonPart", new Person("Jason"));

HttpHeaders headers = new HttpHeaders();
headers.setContentType(MediaType.APPLICATION_XML);
parts.add("xmlPart", new HttpEntity<>(myBean, headers));

// send using RestClient.post or RestTemplate.postForEntity
```

### 客户端请求工厂

为了执行 HTTP 请求，`RestClient` 使用客户端 HTTP 库。这些库通过 `ClientRequestFactory` 接口进行适配。提供多种实现：

- `JdkClientHttpRequestFactory` 用于 Java 的 `HttpClient`
- `HttpComponentsClientHttpRequestFactory` 用于 Apache HTTP Components 的 `HttpClient`
- `JettyClientHttpRequestFactory` 用于 Jetty 的 `HttpClient`
- `ReactorNettyClientRequestFactory` 用于 Reactor Netty 的 `HttpClient`
- `SimpleClientHttpRequestFactory` 作为简单的默认实现

> 请注意，当访问表示错误的响应状态（例如 401）时，`SimpleClientHttpRequestFactory` 可能会抛出异常。如果这是一个问题，请使用其他任何请求工厂。

## `WebClient`

`WebClient` 是一个非阻塞、响应式的客户端，用于执行 HTTP 请求。它在 5.0 中引入，提供了 `RestTemplate` 的替代方案，支持同步、异步和流式场景。

`WebClient` 支持以下特性：

- 非阻塞 I/O
- 响应式流背压
- 以更少的硬件资源实现高并发
- 利用 Java 8 lambda 的函数式风格、fluent API
- 同步和异步交互
- 向上游或向下游流式传输

## `RestTemplate`

`RestTemplate` 以经典的 Spring Template 类的形式提供了对 HTTP 客户端库的高级 API。它暴露了以下几组重载方法：

> `RestClient` 为同步 HTTP 访问提供了更现代的 API。对于异步和流式场景，请考虑响应式的 WebClient。

### RestTemplate 方法

| 方法组 | 描述 |
|--------|------|
| `getForObject` | 通过 GET 检索表示。 |
| `getForEntity` | 通过 GET 检索 `ResponseEntity`（即状态、头部和主体）。 |
| `headForHeaders` | 通过 HEAD 检索资源的所有头部。 |
| `postForLocation` | 通过 POST 创建新资源并返回响应中的 `Location` 头。 |
| `postForObject` | 通过 POST 创建新资源并返回响应中的表示。 |
| `postForEntity` | 通过 POST 创建新资源并返回响应中的表示。 |
| `put` | 通过 PUT 创建或更新资源。 |
| `patchForObject` | 通过 PATCH 更新资源并返回响应中的表示。 |
| `delete` | 通过 DELETE 删除指定 URI 的资源。 |
| `optionsForAllow` | 通过 ALLOW 检索资源允许的 HTTP 方法。 |
| `exchange` | 更通用的版本，提供额外的灵活性。 |
| `execute` | 执行请求最通用的方式。 |

### 初始化

`RestTemplate` 使用与 `RestClient` 相同的 HTTP 库抽象。默认情况下，它使用 `SimpleClientHttpRequestFactory`，但这可以通过构造函数更改。

## HTTP 接口

Spring Framework 允许您使用 `@HttpExchange` 方法将 HTTP 服务定义为 Java 接口。

首先创建带有 `@HttpExchange` 方法的接口：

```java
public interface RepositoryService {

    @GetExchange("/repos/{owner}/{repo}")
    Repository getRepository(@PathVariable String owner, @PathVariable String repo);

    // more HTTP exchange methods...
}
```

现在您可以创建一个代理，当方法被调用时执行请求。

**对于 `RestClient`**
```java
RestClient restClient = RestClient.builder().baseUrl("https://api.github.com/").build();
RestClientAdapter adapter = RestClientAdapter.create(restClient);
HttpServiceProxyFactory factory = HttpServiceProxyFactory.builderFor(adapter).build();

RepositoryService service = factory.createClient(RepositoryService.class);
```

**对于 `WebClient`**
```java
WebClient webClient = WebClient.builder().baseUrl("https://api.github.com/").build();
WebClientAdapter adapter = WebClientAdapter.create(webClient);
HttpServiceProxyFactory factory = HttpServiceProxyFactory.builderFor(adapter).build();

RepositoryService service = factory.createClient(RepositoryService.class);
```

`@HttpExchange` 支持在类型级别应用，此时它适用于所有方法：

```java
@HttpExchange(url = "/repos/{owner}/{repo}", accept = "application/vnd.github.v3+json")
public interface RepositoryService {

    @GetExchange
    Repository getRepository(@PathVariable String owner, @PathVariable String repo);

    @PatchExchange(contentType = MediaType.APPLICATION_FORM_URLENCODED_VALUE)
    void updateRepository(@PathVariable String owner, @PathVariable String repo,
            @RequestParam String name, @RequestParam String description, @RequestParam String homepage);
}
```

### 方法参数

注解的 HTTP 交换方法支持灵活的方法签名，使用以下方法参数：

| 方法参数 | 描述 |
|----------|------|
| `URI` | 动态设置请求的 URL |
| `UriBuilderFactory` | 提供一个 `UriBuilderFactory` 来扩展 URI 模板 |
| `HttpMethod` | 动态设置请求的 HTTP 方法 |
| `@RequestHeader` | 添加一个或多个请求头 |
| `@PathVariable` | 添加一个变量以扩展请求 URL 中的占位符 |
| `@RequestAttribute` | 提供一个 `Object` 作为请求属性添加 |
| `@RequestBody` | 提供请求体 |
| `@RequestParam` | 添加一个或多个请求参数 |
| `@RequestPart` | 添加请求部分 |
| `MultipartFile` | 从 `MultipartFile` 添加请求部分 |
| `@CookieValue` | 添加一个或多个 cookie |

### 返回值

**适配 `HttpExchangeAdapter` 的客户端（例如 `RestClient` 和 `RestTemplate`）支持同步返回值：**

| 方法返回值 | 描述 |
|------------|------|
| `void` | 执行给定的请求。 |
| `HttpHeaders` | 执行给定的请求并返回响应头部。 |
| `<T>` | 执行给定的请求并将响应内容解码为声明的返回类型。 |
| `ResponseEntity<Void>` | 执行给定的请求并返回一个包含状态和头部的 `ResponseEntity`。 |
| `ResponseEntity<T>` | 执行给定的请求，将响应内容解码为声明的返回类型，并返回包含状态、头部和解码后的请求体的 `ResponseEntity`。 |

**适配 `ReactorHttpExchangeAdapter` 的客户端（例如 `WebClient`）支持上述所有内容以及响应式变体：**

| 方法返回值 | 描述 |
|------------|------|
| `Mono<Void>` | 执行给定的请求，并释放响应内容。 |
| `Mono<HttpHeaders>` | 执行给定的请求，释放响应内容，并返回响应头部。 |
| `Mono<T>` | 执行给定的请求并将响应内容解码为声明的返回类型。 |
| `Flux<T>` | 执行给定的请求，并将响应内容解码为声明元素类型的流。 |
| `Mono<ResponseEntity<Void>>` | 执行给定的请求，释放响应内容，并返回包含状态和头部的 `ResponseEntity`。 |
| `Mono<ResponseEntity<T>>` | 执行给定的请求，将响应内容解码为声明的返回类型，并返回包含状态、头部和解码后的请求体的 `ResponseEntity`。 |
| `Mono<ResponseEntity<Flux<T>>` | 执行给定的请求，将响应内容解码为声明元素类型的流，并返回包含状态、头部和解码后的响应体流的 `ResponseEntity`。 |
