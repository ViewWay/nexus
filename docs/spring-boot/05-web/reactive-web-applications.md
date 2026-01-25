# Reactive Web Applications / 响应式 Web 应用

Source: https://docs.springframework.org.cn/spring-boot/reference/web/reactive.html

---

## English

Spring WebFlux is the new reactive web framework introduced in Spring Framework 5.0. Unlike Spring MVC, it does not require the servlet API, is fully asynchronous and non-blocking, and implements the Reactive Streams specification through the Reactor project.

Spring WebFlux comes in two flavors: functional and annotation-based. The annotation-based model is very close to the Spring MVC model, as shown in the following example:

- Java
- Kotlin

```java
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/users/")
public class MyRestController {

    private final UserRepository userRepository;
    private final CustomerRepository customerRepository;

    public MyRestController(UserRepository userRepository, CustomerRepository customerRepository) {
        this.userRepository = userRepository;
        this.customerRepository = customerRepository;
    }

    @GetMapping("/{userId}")
    public Mono<User> getUser(@PathVariable Long userId) {
        return this.userRepository.findById(userId);
    }

    @GetMapping("/{userId}/customers")
    public Flux<Customer> getUserCustomers(@PathVariable Long userId) {
        return this.userRepository.findById(userId).flatMapMany(this.customerRepository::findByUser);
    }

    @DeleteMapping("/{userId}")
    public Mono<Void> deleteUser(@PathVariable Long userId) {
        return this.userRepository.deleteById(userId);
    }

}
```

```kotlin
import org.springframework.web.bind.annotation.DeleteMapping
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController
import reactor.core.publisher.Flux
import reactor.core.publisher.Mono

@RestController
@RequestMapping("/users/")
class MyRestController(private val userRepository: UserRepository, private val customerRepository: CustomerRepository) {

    @GetMapping("/{userId}")
    fun getUser(@PathVariable userId: Long): Mono<User?> {
        return userRepository.findById(userId)
    }

    @GetMapping("/{userId}/customers")
    fun getUserCustomers(@PathVariable userId: Long): Flux<Customer> {
        return userRepository.findById(userId).flatMapMany { user: User? ->
            customerRepository.findByUser(user)
        }
    }

    @DeleteMapping("/{userId}")
    fun deleteUser(@PathVariable userId: Long): Mono<Void> {
        return userRepository.deleteById(userId)
    }

}
```

WebFlux is part of the Spring Framework and detailed information is available in its reference documentation.

"WebFlux.fn" is the functional variant that separates the routing configuration from the actual handling of requests, as shown in the following example:

- Java
- Kotlin

```java
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.MediaType;
import org.springframework.web.reactive.function.server.RequestPredicate;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerResponse;

import static org.springframework.web.reactive.function.server.RequestPredicates.accept;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Configuration(proxyBeanMethods = false)
public class MyRoutingConfiguration {

    private static final RequestPredicate ACCEPT_JSON = accept(MediaType.APPLICATION_JSON);

    @Bean
    public RouterFunction<ServerResponse> monoRouterFunction(MyUserHandler userHandler) {
        return route()
            .GET("/{user}", ACCEPT_JSON, userHandler::getUser)
            .GET("/{user}/customers", ACCEPT_JSON, userHandler::getUserCustomers)
            .DELETE("/{user}", ACCEPT_JSON, userHandler::deleteUser)
            .build();
    }

}
```

```kotlin
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.http.MediaType
import org.springframework.web.reactive.function.server.RequestPredicates.DELETE
import org.springframework.web.reactive.function.server.RequestPredicates.GET
import org.springframework.web.reactive.function.server.RequestPredicates.accept
import org.springframework.web.reactive.function.server.RouterFunction
import org.springframework.web.reactive.function.server.RouterFunctions
import org.springframework.web.reactive.function.server.ServerResponse

@Configuration(proxyBeanMethods = false)
class MyRoutingConfiguration {

    @Bean
    fun monoRouterFunction(userHandler: MyUserHandler): RouterFunction<ServerResponse> {
        return RouterFunctions.route(
            GET("/{user}").and(ACCEPT_JSON), userHandler::getUser).andRoute(
            GET("/{user}/customers").and(ACCEPT_JSON), userHandler::getUserCustomers).andRoute(
            DELETE("/{user}").and(ACCEPT_JSON), userHandler::deleteUser)
    }

    companion object {
        private val ACCEPT_JSON = accept(MediaType.APPLICATION_JSON)
    }

}
```

- Java
- Kotlin

```java
import reactor.core.publisher.Mono;

import org.springframework.stereotype.Component;
import org.springframework.web.reactive.function.server.ServerRequest;
import org.springframework.web.reactive.function.server.ServerResponse;

@Component
public class MyUserHandler {

    public Mono<ServerResponse> getUser(ServerRequest request) {
        ...
    }

    public Mono<ServerResponse> getUserCustomers(ServerRequest request) {
        ...
    }

    public Mono<ServerResponse> deleteUser(ServerRequest request) {
        ...
    }

}
```

```kotlin
import org.springframework.stereotype.Component
import org.springframework.web.reactive.function.server.ServerRequest
import org.springframework.web.reactive.function.server.ServerResponse
import reactor.core.publisher.Mono

@Component
class MyUserHandler {

    fun getUser(request: ServerRequest?): Mono<ServerResponse> {
        ...
    }

    fun getUserCustomers(request: ServerRequest?): Mono<ServerResponse> {
        ...
    }

    fun deleteUser(request: ServerRequest?): Mono<ServerResponse> {
        ...
    }

}
```

"WebFlux.fn" is part of the Spring Framework and detailed information is available in its reference documentation.

|  |  |
| --- | --- |
|  | You can define as many `RouterFunction` beans as you like to modularize the router definitions. Beans can be ordered if you need to apply a precedence. |

To get started, add the `spring-boot-starter-webflux` module to your application.

|  |  |
| --- | --- |
|  | Adding both the `spring-boot-starter-web` and `spring-boot-starter-webflux` modules in your application results in Spring Boot auto-configuring Spring MVC, not WebFlux. This behavior has been chosen because many Spring developers add `spring-boot-starter-webflux` to their Spring MVC application to use the reactive `WebClient`. You can still enforce your choice by setting the preferred application type to `SpringApplication.setWebApplicationType(WebApplicationType.REACTIVE)`. |

### Spring WebFlux Auto-configuration

Spring Boot provides auto-configuration for Spring WebFlux that works well with most applications.

The auto-configuration adds the following features on top of Spring's defaults:

- Configuring codecs for `HttpMessageReader` and `HttpMessageWriter` instances (described later in this document).
- Support for serving static resources, including support for WebJars (described later in this document).

If you want to keep the Spring Boot WebFlux features and you want to add additional WebFlux configuration, you can add your own `@Configuration` class of type `WebFluxConfigurer` but **without** `@EnableWebFlux`.

If you want to add extra customizations to the auto-configured `HttpHandler`, you can define beans of type `WebHttpHandlerBuilderCustomizer` and use them to modify the `WebHttpHandlerBuilder`.

If you want to have complete control over Spring WebFlux, you can add your own `@Configuration` annotated with `@EnableWebFlux`.

### Static Content

By default, Spring Boot serves static content from a directory called `/static` (or `/public` or `/resources` or `/META-INF/resources`) in the classpath. It uses the `ResourceWebHandler` from Spring WebFlux so that you can modify that behavior by adding your own `WebFluxConfigurer` and overriding the `addResourceHandlers` method.

By default, resources are mapped on `/**`, but you can tune that by setting the `spring.webflux.static-path-pattern` property. For instance, relocating all resources to `/resources/**` can be achieved as follows:

- Properties
- YAML

```properties
spring.webflux.static-path-pattern=/resources/**
```

```yaml
spring:
  webflux:
    static-path-pattern: "/resources/**"
```

You can also customize the static resource locations by using `spring.web.resources.static-locations`.

In addition to the "standard" static resource locations listed earlier, a special case is made for Webjars content. By default, any resource with a path in `/webjars/**` is served from a jar file if it is packaged in the Webjars format. The path can be customized with the `spring.webflux.webjars-path-pattern` property.

|  |  |
| --- | --- |
|  | Spring WebFlux applications do not strictly depend on the servlet API, so they cannot be deployed as war files and do not use the `src/main/webapp` directory. |

### Welcome Page

Spring Boot supports both static and templated welcome pages. It first looks for an `index.html` file in the configured static content locations. If none is found, it looks for an `index` template. If either is found, it is automatically used as the welcome page for the application.

This serves only as a fallback for actual index routes defined by the application. The ordering is defined by the order of `HandlerMapping` beans, which defaults to the following:

|  |  |
| --- | --- |
| `RouterFunctionMapping` | Endpoints declared with `RouterFunction` beans |
| `RequestMappingHandlerMapping` | Endpoints declared in `@Controller` beans |
| Welcome page `RouterFunctionMapping` | Welcome page support |

### Template Engines

In addition to REST web services, you can also use Spring WebFlux to serve dynamic HTML content. Spring WebFlux supports a variety of template technologies, including Thymeleaf, FreeMarker, and Mustache.

Spring Boot includes auto-configuration support for the following template engines:

- FreeMarker
- Thymeleaf
- Mustache

When you use one of these template engines with the default configuration, your templates are automatically picked up from `src/main/resources/templates`.

### Error Handling

Spring Boot provides a `WebExceptionHandler` that handles all errors in a sensible way. Its position in the processing order is immediately before the handlers provided by WebFlux, which are considered last. For machine clients, it creates a JSON response with details of the error, the HTTP status, and the exception message. For browser clients, there is a "whitelabel" error handler that renders the same data in HTML format. You can also provide your own HTML templates to display errors (see the next section).

Before directly customizing error handling in Spring Boot, you can leverage the RFC 9457 Problem Details support available in Spring WebFlux. Spring WebFlux can produce custom error messages using the `application/problem+json` media type, for example:

```json
{
    "type": "https://example.org/problems/unknown-project",
    "title": "Unknown project",
    "status": 404,
    "detail": "No project found for id 'spring-unknown'",
    "instance": "/projects/spring-unknown"
}
```

This support can be enabled by setting `spring.webflux.problemdetails.enabled` to `true`.

### Web Filters

Spring WebFlux provides a `WebFilter` interface that you can implement to filter HTTP request-response exchanges. `WebFilter` beans found in the application context are automatically used to filter each exchange.

Where filters are important for ordering, they can implement `Ordered` or be annotated with `@Order`. When Spring Boot auto-configures your web filters, it applies the ordering shown in the following table:

| Web Filter | Order |
| --- | --- |
| `WebFilterChainProxy` (Spring Security) | `-100` |
| `HttpExchangesWebFilter` | `Ordered.LOWEST_PRECEDENCE - 10` |

---

## 中文 / Chinese

Spring WebFlux 是 Spring Framework 5.0 中引入的新响应式 Web 框架。与 Spring MVC 不同，它不需要 servlet API，是完全异步和非阻塞的，并通过 响应式流 规范实现 Reactor 项目。

Spring WebFlux 有两种形式：函数式和基于注解的。基于注解的形式非常接近 Spring MVC 模型，如下例所示：

- Java
- Kotlin

```java
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;

import org.springframework.web.bind.annotation.DeleteMapping;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/users/")
public class MyRestController {

    private final UserRepository userRepository;
    private final CustomerRepository customerRepository;

    public MyRestController(UserRepository userRepository, CustomerRepository customerRepository) {
        this.userRepository = userRepository;
        this.customerRepository = customerRepository;
    }

    @GetMapping("/{userId}")
    public Mono<User> getUser(@PathVariable Long userId) {
        return this.userRepository.findById(userId);
    }

    @GetMapping("/{userId}/customers")
    public Flux<Customer> getUserCustomers(@PathVariable Long userId) {
        return this.userRepository.findById(userId).flatMapMany(this.customerRepository::findByUser);
    }

    @DeleteMapping("/{userId}")
    public Mono<Void> deleteUser(@PathVariable Long userId) {
        return this.userRepository.deleteById(userId);
    }

}
```

```kotlin
import org.springframework.web.bind.annotation.DeleteMapping
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController
import reactor.core.publisher.Flux
import reactor.core.publisher.Mono

@RestController
@RequestMapping("/users/")
class MyRestController(private val userRepository: UserRepository, private val customerRepository: CustomerRepository) {

    @GetMapping("/{userId}")
    fun getUser(@PathVariable userId: Long): Mono<User?> {
        return userRepository.findById(userId)
    }

    @GetMapping("/{userId}/customers")
    fun getUserCustomers(@PathVariable userId: Long): Flux<Customer> {
        return userRepository.findById(userId).flatMapMany { user: User? ->
            customerRepository.findByUser(user)
        }
    }

    @DeleteMapping("/{userId}")
    fun deleteUser(@PathVariable userId: Long): Mono<Void> {
        return userRepository.deleteById(userId)
    }

}
```

WebFlux 是 Spring Framework 的一部分，详细信息可在其参考文档中找到。

"WebFlux.fn" 是函数式变体，它将路由配置与请求的实际处理分离，如下例所示：

- Java
- Kotlin

```java
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.MediaType;
import org.springframework.web.reactive.function.server.RequestPredicate;
import org.springframework.web.reactive.function.server.RouterFunction;
import org.springframework.web.reactive.function.server.ServerResponse;

import static org.springframework.web.reactive.function.server.RequestPredicates.accept;
import static org.springframework.web.reactive.function.server.RouterFunctions.route;

@Configuration(proxyBeanMethods = false)
public class MyRoutingConfiguration {

    private static final RequestPredicate ACCEPT_JSON = accept(MediaType.APPLICATION_JSON);

    @Bean
    public RouterFunction<ServerResponse> monoRouterFunction(MyUserHandler userHandler) {
        return route()
            .GET("/{user}", ACCEPT_JSON, userHandler::getUser)
            .GET("/{user}/customers", ACCEPT_JSON, userHandler::getUserCustomers)
            .DELETE("/{user}", ACCEPT_JSON, userHandler::deleteUser)
            .build();
    }

}
```

```kotlin
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.http.MediaType
import org.springframework.web.reactive.function.server.RequestPredicates.DELETE
import org.springframework.web.reactive.function.server.RequestPredicates.GET
import org.springframework.web.reactive.function.server.RequestPredicates.accept
import org.springframework.web.reactive.function.server.RouterFunction
import org.springframework.web.reactive.function.server.RouterFunctions
import org.springframework.web.reactive.function.server.ServerResponse

@Configuration(proxyBeanMethods = false)
class MyRoutingConfiguration {

    @Bean
    fun monoRouterFunction(userHandler: MyUserHandler): RouterFunction<ServerResponse> {
        return RouterFunctions.route(
            GET("/{user}").and(ACCEPT_JSON), userHandler::getUser).andRoute(
            GET("/{user}/customers").and(ACCEPT_JSON), userHandler::getUserCustomers).andRoute(
            DELETE("/{user}").and(ACCEPT_JSON), userHandler::deleteUser)
    }

    companion object {
        private val ACCEPT_JSON = accept(MediaType.APPLICATION_JSON)
    }

}
```

|  |  |
| --- | --- |
|  | 您可以定义任意数量的 `RouterFunction` Bean 来模块化路由的定义。如果需要应用优先级，则可以对 Bean 进行排序。 |

要开始使用，请将 `spring-boot-starter-webflux` 模块添加到您的应用程序中。

|  |  |
| --- | --- |
|  | 在您的应用程序中同时添加 `spring-boot-starter-web` 和 `spring-boot-starter-webflux` 模块会导致 Spring Boot 自动配置 Spring MVC，而不是 WebFlux。之所以选择此行为，是因为许多 Spring 开发人员将 `spring-boot-starter-webflux` 添加到他们的 Spring MVC 应用程序中以使用反应式 `WebClient`。您仍然可以通过将选定的应用程序类型设置为 `SpringApplication.setWebApplicationType(WebApplicationType.REACTIVE)` 来强制您的选择。 |

### Spring WebFlux 自动配置

Spring Boot 为 Spring WebFlux 提供了自动配置，适用于大多数应用程序。

自动配置在 Spring 的默认设置之上添加了以下功能：

- 为 `HttpMessageReader` 和 `HttpMessageWriter` 实例配置编解码器（在本文档后面描述）。
- 支持提供静态资源，包括对 WebJars 的支持（在本文档后面描述）。

如果您想保留 Spring Boot WebFlux 功能并想添加其他 WebFlux 配置，您可以添加您自己的 `@Configuration` 类，类型为 `WebFluxConfigurer`，但**无需** `@EnableWebFlux`。

如果您想对自动配置的 `HttpHandler` 添加其他自定义，您可以定义类型为 `WebHttpHandlerBuilderCustomizer` 的 Bean 并使用它们来修改 `WebHttpHandlerBuilder`。

如果您想完全控制 Spring WebFlux，您可以添加您自己的用 `@EnableWebFlux` 注解的 `@Configuration`。

### 静态内容

默认情况下，Spring Boot 从类路径中名为 `/static`（或 `/public` 或 `/resources` 或 `/META-INF/resources`）的目录提供静态内容。它使用 Spring WebFlux 中的 `ResourceWebHandler`，以便您可以通过添加您自己的 `WebFluxConfigurer` 并覆盖 `addResourceHandlers` 方法来修改该行为。

默认情况下，资源映射到 `/**`，但您可以通过设置 `spring.webflux.static-path-pattern` 属性来调整它。例如，将所有资源重新定位到 `/resources/**` 可以通过以下方式实现：

- 属性
- YAML

```properties
spring.webflux.static-path-pattern=/resources/**
```

```yaml
spring:
  webflux:
    static-path-pattern: "/resources/**"
```

您还可以使用 `spring.web.resources.static-locations` 自定义静态资源位置。这样做会用目录位置列表替换默认值。

除了前面列出的"标准"静态资源位置之外，还为 Webjars 内容做了特殊处理。默认情况下，路径在 `/webjars/**` 中的任何资源都将从 jar 文件中提供服务，前提是它们以 Webjars 格式打包。路径可以使用 `spring.webflux.webjars-path-pattern` 属性进行自定义。

|  |  |
| --- | --- |
|  | Spring WebFlux 应用程序不严格依赖于 servlet API，因此它们不能作为 war 文件部署，也不使用 `src/main/webapp` 目录。 |

### 欢迎页面

Spring Boot 支持静态和模板化欢迎页面。它首先在配置的静态内容位置中查找 `index.html` 文件。如果未找到，则查找 `index` 模板。如果找到其中任何一个，它将自动用作应用程序的欢迎页面。

这仅作为应用程序定义的实际索引路由的回退。排序由 `HandlerMapping` Bean 的顺序定义，默认情况下如下所示：

|  |  |
| --- | --- |
| `RouterFunctionMapping` | 使用 `RouterFunction` Bean 声明的端点 |
| `RequestMappingHandlerMapping` | 在 `@Controller` Bean 中声明的端点 |
| 欢迎页面的 `RouterFunctionMapping` | 欢迎页面支持 |

### 模板引擎

除了 REST Web 服务之外，您还可以使用 Spring WebFlux 提供动态 HTML 内容。Spring WebFlux 支持各种模板技术，包括 Thymeleaf、FreeMarker 和 Mustache。

Spring Boot 包括以下模板引擎的自动配置支持：

- FreeMarker
- Thymeleaf
- Mustache

当您将其中一个模板引擎与默认配置一起使用时，您的模板将自动从 `src/main/resources/templates` 中获取。

### 错误处理

Spring Boot 提供了一个 `WebExceptionHandler`，它以合理的方式处理所有错误。它在处理顺序中的位置紧接在 WebFlux 提供的处理程序之前，这些处理程序被视为最后处理。对于机器客户端，它会生成一个包含错误详细信息、HTTP 状态和异常消息的 JSON 响应。对于浏览器客户端，有一个"白标"错误处理程序，它以 HTML 格式呈现相同的数据。您还可以提供您自己的 HTML 模板来显示错误（请参阅下一节）。

在直接自定义 Spring Boot 中的错误处理之前，您可以利用 Spring WebFlux 中的 RFC 9457 问题详细信息 支持。Spring WebFlux 可以使用 `application/problem+json` 媒体类型生成自定义错误消息，例如：

```json
{
    "type": "https://example.org/problems/unknown-project",
    "title": "Unknown project",
    "status": 404,
    "detail": "No project found for id 'spring-unknown'",
    "instance": "/projects/spring-unknown"
}
```

可以通过将 `spring.webflux.problemdetails.enabled` 设置为 `true` 来启用此支持。

### Web 过滤器

Spring WebFlux 提供了一个 `WebFilter` 接口，可以实现它来过滤 HTTP 请求-响应交换。应用程序上下文中找到的 `WebFilter` Bean 将自动用于过滤每个交换。

在过滤器顺序很重要的位置，它们可以实现 `Ordered` 或用 `@Order` 注解。Spring Boot 自动配置可能会为您配置 Web 过滤器。当它这样做时，将使用下表中显示的顺序：

| Web 过滤器 | 顺序 |
| --- | --- |
| `WebFilterChainProxy`（Spring Security） | `-100` |
| `HttpExchangesWebFilter` | `Ordered.LOWEST_PRECEDENCE - 10` |
