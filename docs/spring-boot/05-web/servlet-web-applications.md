# Servlet Web Applications / Servlet Web 应用

Source: https://docs.springframework.org.cn/spring-boot/reference/web/servlet.html

---

## English

If you want to build a servlet-based web application, you can take advantage of Spring Boot's auto-configuration for Spring MVC or Jersey.

## Spring Web MVC Framework

The Spring Web MVC framework (often referred to as "Spring MVC") is a rich "model-view-controller" web framework. Spring MVC lets you create special `@Controller` or `@RestController` beans to handle incoming HTTP requests. Methods in your controller are mapped to HTTP by using `@RequestMapping` annotations.

The following code shows a typical `@RestController` that serves JSON data:

- Java
- Kotlin

```java
import java.util.List;

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
    public User getUser(@PathVariable Long userId) {
        return this.userRepository.findById(userId).get();
    }

    @GetMapping("/{userId}/customers")
    public List<Customer> getUserCustomers(@PathVariable Long userId) {
        return this.userRepository.findById(userId).map(this.customerRepository::findByUser).get();
    }

    @DeleteMapping("/{userId}")
    public void deleteUser(@PathVariable Long userId) {
        this.userRepository.deleteById(userId);
    }

}
```

```kotlin
import org.springframework.web.bind.annotation.DeleteMapping
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/users/")
class MyRestController(private val userRepository: UserRepository, private val customerRepository: CustomerRepository) {

    @GetMapping("/{userId}")
    fun getUser(@PathVariable userId: Long): User {
        return userRepository.findById(userId).get()
    }

    @GetMapping("/{userId}/customers")
    fun getUserCustomers(@PathVariable userId: Long): List<Customer> {
        return userRepository.findById(userId).map(customerRepository::findByUser).get()
    }

    @DeleteMapping("/{userId}")
    fun deleteUser(@PathVariable userId: Long) {
        userRepository.deleteById(userId)
    }

}
```

The functional variant "WebMvc.fn" separates the routing configuration from the actual handling of requests, as shown in the following example:

- Java
- Kotlin

```java
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.MediaType;
import org.springframework.web.servlet.function.RequestPredicate;
import org.springframework.web.servlet.function.RouterFunction;
import org.springframework.web.servlet.function.ServerResponse;

import static org.springframework.web.servlet.function.RequestPredicates.accept;
import static org.springframework.web.servlet.function.RouterFunctions.route;

@Configuration(proxyBeanMethods = false)
public class MyRoutingConfiguration {

    private static final RequestPredicate ACCEPT_JSON = accept(MediaType.APPLICATION_JSON);

    @Bean
    public RouterFunction<ServerResponse> routerFunction(MyUserHandler userHandler) {
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
import org.springframework.web.servlet.function.RequestPredicates.accept
import org.springframework.web.servlet.function.RouterFunction
import org.springframework.web.servlet.function.RouterFunctions
import org.springframework.web.servlet.function.ServerResponse

@Configuration(proxyBeanMethods = false)
class MyRoutingConfiguration {

    @Bean
    fun routerFunction(userHandler: MyUserHandler): RouterFunction<ServerResponse> {
        return RouterFunctions.route()
            .GET("/{user}", ACCEPT_JSON, userHandler::getUser)
            .GET("/{user}/customers", ACCEPT_JSON, userHandler::getUserCustomers)
            .DELETE("/{user}", ACCEPT_JSON, userHandler::deleteUser)
            .build()
    }

    companion object {
        private val ACCEPT_JSON = accept(MediaType.APPLICATION_JSON)
    }

}
```

### Spring MVC Auto-configuration

Spring Boot provides auto-configuration for Spring MVC that works well with most applications. It does not require `@EnableWebMvc`, and the two cannot be used together. In addition to Spring MVC's defaults, the auto-configuration provides the following features:

- Inclusion of `ContentNegotiatingViewResolver` and `BeanNameViewResolver` beans.
- Support for serving static resources, including support for WebJars (covered later in this document).
- Automatic registration of `Converter`, `GenericConverter`, and `Formatter` beans.
- Support for `HttpMessageConverters` (covered later in this document).
- Automatic registration of `MessageCodesResolver` (covered later in this document).
- Static `index.html` support.
- Automatic use of `ConfigurableWebBindingInitializer` bean (covered later in this document).

If you want to keep those Spring Boot MVC customizations and make more MVC customizations (interceptors, formatters, view controllers, and other features), you can add your own `@Configuration` class of type `WebMvcConfigurer` but **without** `@EnableWebMvc`.

### HttpMessageConverters

Spring MVC uses the `HttpMessageConverter` interface to convert HTTP requests and responses. Sensible defaults are included out of the box. For example, objects can be automatically converted to JSON (by using the Jackson library) or to XML (by using the Jackson XML extension, if available, or by using JAXB if the Jackson XML extension is not available). By default, strings are encoded in `UTF-8`.

Any `HttpMessageConverter` bean that is present in the context is added to the list of converters. You can also override default converters in the same way.

### Static Content

By default, Spring Boot serves static content from a directory called `/static` (or `/public` or `/resources` or `/META-INF/resources`) in the classpath or from the root of the `ServletContext`. It uses the `ResourceHttpRequestHandler` from Spring MVC, so you can modify that behavior by adding your own `WebMvcConfigurer` and overriding the `addResourceHandlers` method.

To do so in a standalone web application, you also need to enable the `DefaultServlet` by using the `server.servlet.register-default-servlet` property.

### Template Engines

As well as REST web services, you can also use Spring MVC to serve dynamic HTML content. Spring MVC supports a variety of template technologies, including Thymeleaf, FreeMarker, and JSP. Also, many other template engines include their own Spring MVC integrations.

Spring Boot includes auto-configuration support for the following template engines:

- FreeMarker
- Groovy
- Thymeleaf
- Mustache

When you use one of these template engines with the default configuration, your templates are automatically picked up from `src/main/resources/templates`.

### Error Handling

By default, Spring Boot provides an `/error` mapping that handles all errors in a sensible way, and it is registered as a "global" error page in the servlet container. For machine clients, it produces a JSON response with details of the error, the HTTP status, and an exception message. For browser clients, there is a "whitelabel" error view that renders the same data in HTML format (to customize it, add a `View` that resolves to `error`).

To completely replace the default behavior, you can implement `ErrorController` and register a bean definition of that type or add a bean of type `ErrorAttributes` to use the existing mechanism but replace the contents.

### Custom Error Pages

If you want to display a custom HTML error page for a given status code, you can add files to an `/error` directory. Error pages can either be static HTML (that is, added under any of the static resource directories) or be built by using templates.

### CORS Support

Cross-Origin Resource Sharing (CORS) is a W3C specification implemented by most browsers that lets you specify in a flexible way what kind of cross-domain requests are authorized, instead of using some less secure and less powerful approaches such as IFRAME or JSONP.

Starting from version 4.2, Spring MVC supports CORS. CORS configuration for controller methods is done with `@CrossOrigin` annotations in a Spring Boot application, which requires no specific configuration. Global CORS configuration can be defined by registering a `WebMvcConfigurer` bean with a customized `addCorsMappings(CorsRegistry)` method, as shown in the following example:

- Java
- Kotlin

```java
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.CorsRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

@Configuration(proxyBeanMethods = false)
public class MyCorsConfiguration {

    @Bean
    public WebMvcConfigurer corsConfigurer() {
        return new WebMvcConfigurer() {

            @Override
            public void addCorsMappings(CorsRegistry registry) {
                registry.addMapping("/api/**");
            }

        };
    }

}
```

```kotlin
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.web.servlet.config.annotation.CorsRegistry
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer

@Configuration(proxyBeanMethods = false)
class MyCorsConfiguration {

    @Bean
    fun corsConfigurer(): WebMvcConfigurer {
        return object : WebMvcConfigurer {
            override fun addCorsMappings(registry: CorsRegistry) {
                registry.addMapping("/api/**")
            }
        }
    }

}
```

## JAX-RS and Jersey

If you prefer the JAX-RS programming model for REST endpoints, you can use one of the available implementations instead of Spring MVC. Jersey and Apache CXF work quite well out of the box. CXF requires you to register its `Servlet` or `Filter` as a `@Bean` in your application context. Jersey has some native Spring support, so we also provide auto-configuration support for it in Spring Boot, along with a starter.

To get started with Jersey, include the `spring-boot-starter-jersey` as a dependency and then you need one `@Bean` of type `ResourceConfig` in which you register all the endpoints, as shown in the following example:

```java
import org.glassfish.jersey.server.ResourceConfig;

import org.springframework.stereotype.Component;

@Component
public class MyJerseyConfig extends ResourceConfig {

    public MyJerseyConfig() {
        register(MyEndpoint.class);
    }

}
```

All registered endpoints should be `@Components` with HTTP resource annotations (`@GET`, and others), as shown in the following example:

```java
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;

import org.springframework.stereotype.Component;

@Component
@Path("/hello")
public class MyEndpoint {

    @GET
    public String message() {
        return "Hello";
    }

}
```

## Embedded Servlet Container Support

For servlet applications, Spring Boot includes support for embedded Tomcat, Jetty, and Undertow servers. Most developers use the appropriate starter to obtain a fully configured instance. By default, the embedded server listens for HTTP requests on port `8080`.

### Servlets, Filters, and Listeners

When using an embedded servlet container, you can register servlets, filters, and all listeners from the servlet spec (such as `HttpSessionListener`) either as Spring beans or by scanning for servlet components.

Any `Servlet`, `Filter`, or servlet `*Listener` instance that is a Spring bean is registered with the embedded container. This can be particularly convenient if you want to refer to a value from your `application.properties` during configuration.

### Customizing Embedded Servlet Containers

Common servlet container settings can be configured by using Spring `Environment` properties. Typically, you would define properties in your `application.properties` or `application.yaml` file.

Common server settings include:

- Network settings: The listen port for incoming HTTP requests (`server.port`), the interface address to bind to (`server.address`), and more.
- Session settings: Whether the session is persistent (`server.servlet.session.persistent`), the session timeout (`server.servlet.session.timeout`), the location of session data (`server.servlet.session.store-dir`), and session cookie configuration (`server.servlet.session.cookie.*`).
- Error management: The location of the error page (`server.error.path`), and more.
- SSL
- HTTP compression

### SameSite Cookie

The `SameSite` cookie attribute is available for use by web browsers to control whether and how cookies are submitted in cross-site requests. This attribute is particularly important for modern web browsers which are starting to change the default value when the attribute is missing.

### JSP Limitations

When running a Spring Boot application that uses an embedded servlet container (and is packaged as an executable archive), there are some limitations to JSP support:

- With Jetty and Tomcat, if you use a war packaging, it should work. Executable war files can be started using `java -jar` and can also be deployed to any standard container. JSPs are not supported when using an executable jar file.
- Undertow does not support JSPs.
- Creating a custom `error.jsp` page does not override the default view for error handling. Custom error pages should be used instead.

---

## 中文 / Chinese

如果你想构建基于 servlet 的 Web 应用，你可以利用 Spring Boot 对 Spring MVC 或 Jersey 的自动配置。

## Spring Web MVC 框架

Spring Web MVC 框架（通常称为"Spring MVC"）是一个功能丰富的"模型-视图-控制器"Web 框架。Spring MVC 允许你创建特殊的 `@Controller` 或 `@RestController` bean 来处理传入的 HTTP 请求。你的控制器中的方法通过使用 `@RequestMapping` 注解映射到 HTTP。

以下代码显示了一个典型的提供 JSON 数据的 `@RestController`：

- Java
- Kotlin

```java
import java.util.List;

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
    public User getUser(@PathVariable Long userId) {
        return this.userRepository.findById(userId).get();
    }

    @GetMapping("/{userId}/customers")
    public List<Customer> getUserCustomers(@PathVariable Long userId) {
        return this.userRepository.findById(userId).map(this.customerRepository::findByUser).get();
    }

    @DeleteMapping("/{userId}")
    public void deleteUser(@PathVariable Long userId) {
        this.userRepository.deleteById(userId);
    }

}
```

```kotlin
import org.springframework.web.bind.annotation.DeleteMapping
import org.springframework.web.bind.annotation.GetMapping
import org.springframework.web.bind.annotation.PathVariable
import org.springframework.web.bind.annotation.RequestMapping
import org.springframework.web.bind.annotation.RestController

@RestController
@RequestMapping("/users/")
class MyRestController(private val userRepository: UserRepository, private val customerRepository: CustomerRepository) {

    @GetMapping("/{userId}")
    fun getUser(@PathVariable userId: Long): User {
        return userRepository.findById(userId).get()
    }

    @GetMapping("/{userId}/customers")
    fun getUserCustomers(@PathVariable userId: Long): List<Customer> {
        return userRepository.findById(userId).map(customerRepository::findByUser).get()
    }

    @DeleteMapping("/{userId}")
    fun deleteUser(@PathVariable userId: Long) {
        userRepository.deleteById(userId)
    }

}
```

函数式变体"WebMvc.fn"将路由配置与请求的实际处理分开，如下例所示：

- Java
- Kotlin

```java
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.http.MediaType;
import org.springframework.web.servlet.function.RequestPredicate;
import org.springframework.web.servlet.function.RouterFunction;
import org.springframework.web.servlet.function.ServerResponse;

import static org.springframework.web.servlet.function.RequestPredicates.accept;
import static org.springframework.web.servlet.function.RouterFunctions.route;

@Configuration(proxyBeanMethods = false)
public class MyRoutingConfiguration {

    private static final RequestPredicate ACCEPT_JSON = accept(MediaType.APPLICATION_JSON);

    @Bean
    public RouterFunction<ServerResponse> routerFunction(MyUserHandler userHandler) {
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
import org.springframework.web.servlet.function.RequestPredicates.accept
import org.springframework.web.servlet.function.RouterFunction
import org.springframework.web.servlet.function.RouterFunctions
import org.springframework.web.servlet.function.ServerResponse

@Configuration(proxyBeanMethods = false)
class MyRoutingConfiguration {

    @Bean
    fun routerFunction(userHandler: MyUserHandler): RouterFunction<ServerResponse> {
        return RouterFunctions.route()
            .GET("/{user}", ACCEPT_JSON, userHandler::getUser)
            .GET("/{user}/customers", ACCEPT_JSON, userHandler::getUserCustomers)
            .DELETE("/{user}", ACCEPT_JSON, userHandler::deleteUser)
            .build()
    }

    companion object {
        private val ACCEPT_JSON = accept(MediaType.APPLICATION_JSON)
    }

}
```

### Spring MVC 自动配置

Spring Boot 提供了适用于大多数应用程序的 Spring MVC 自动配置。它无需使用 `@EnableWebMvc`，两者不能一起使用。除了 Spring MVC 的默认值外，自动配置还提供以下功能：

- 包含 `ContentNegotiatingViewResolver` 和 `BeanNameViewResolver` bean。
- 支持提供静态资源，包括对 WebJars 的支持（本文档后面部分会介绍）。
- 自动注册 `Converter`、`GenericConverter` 和 `Formatter` bean。
- 支持 `HttpMessageConverters`（本文档后面部分会介绍）。
- 自动注册 `MessageCodesResolver`（本文档后面部分会介绍）。
- 静态 `index.html` 支持。
- 自动使用 `ConfigurableWebBindingInitializer` bean（本文档后面部分会介绍）。

如果你想保留这些 Spring Boot MVC 自定义项并进行更多 MVC 自定义（拦截器、格式化程序、视图控制器和其他功能），你可以添加你自己的 `@Configuration` 类型的类 `WebMvcConfigurer`，但**不要**使用 `@EnableWebMvc`。

### HttpMessageConverters

Spring MVC 使用 `HttpMessageConverter` 接口转换 HTTP 请求和响应。开箱即用地包含合理的默认值。例如，对象可以自动转换为 JSON（使用 Jackson 库）或 XML（如果可用，则使用 Jackson XML 扩展，或者如果 Jackson XML 扩展不可用，则使用 JAXB）。默认情况下，字符串以 `UTF-8` 编码。

上下文中存在的任何 `HttpMessageConverter` bean 都将添加到转换器列表中。您也可以以相同的方式覆盖默认转换器。

### 静态内容

默认情况下，Spring Boot 从类路径中的名为 `/static`（或 `/public` 或 `/resources` 或 `/META-INF/resources`）的目录或 `ServletContext` 的根目录提供静态内容。它使用 Spring MVC 中的 `ResourceHttpRequestHandler`，因此您可以通过添加您自己的 `WebMvcConfigurer` 并覆盖 `addResourceHandlers` 方法来修改该行为。

在独立 Web 应用程序中，容器的默认 servlet 未启用。可以使用 `server.servlet.register-default-servlet` 属性启用它。

### 模板引擎

除了 REST web 服务之外，还可以使用 Spring MVC 来服务动态 HTML 内容。Spring MVC 支持多种模板技术，包括 Thymeleaf、FreeMarker 和 JSP。此外，许多其他模板引擎都包含其自身的 Spring MVC 集成。

Spring Boot 包含对以下模板引擎的自动配置支持：

- FreeMarker
- Groovy
- Thymeleaf
- Mustache

当使用默认配置使用这些模板引擎之一时，系统会自动从 `src/main/resources/templates` 中获取模板。

### 错误处理

默认情况下，Spring Boot 提供了一个 `/error` 映射，它以合理的方式处理所有错误，并在 servlet 容器中注册为"全局"错误页面。对于机器客户端，它会生成一个 JSON 响应，其中包含错误的详细信息、HTTP 状态和异常消息。对于浏览器客户端，有一个"白标"错误视图，它以 HTML 格式呈现相同的数据（要自定义它，请添加一个解析为 `error` 的 `View`）。

如果要完全替换默认行为，可以实现 `ErrorController` 并注册该类型的 bean 定义，或添加类型为 `ErrorAttributes` 的 bean 以使用现有机制但替换内容。

### 自定义错误页面

如果要为给定的状态代码显示自定义 HTML 错误页面，可以将文件添加到 `/error` 目录。错误页面可以是静态 HTML（即，添加到任何静态资源目录下），也可以使用模板构建。

### CORS 支持

跨源资源共享 (CORS) 是 W3C 规范，由大多数浏览器实现，它允许您以灵活的方式指定授权哪些类型的跨域请求，而不是使用一些安全性较低且功能较弱的方法，例如 IFRAME 或 JSONP。

从 4.2 版本开始，Spring MVC 支持 CORS。在 Spring Boot 应用程序中使用控制器方法 CORS 配置和 `@CrossOrigin` 注解不需要任何特定配置。全局 CORS 配置可以通过注册具有自定义 `addCorsMappings(CorsRegistry)` 方法的 `WebMvcConfigurer` bean 来定义，如下例所示：

- Java
- Kotlin

```java
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.CorsRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

@Configuration(proxyBeanMethods = false)
public class MyCorsConfiguration {

    @Bean
    public WebMvcConfigurer corsConfigurer() {
        return new WebMvcConfigurer() {

            @Override
            public void addCorsMappings(CorsRegistry registry) {
                registry.addMapping("/api/**");
            }

        };
    }

}
```

```kotlin
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.web.servlet.config.annotation.CorsRegistry
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer

@Configuration(proxyBeanMethods = false)
class MyCorsConfiguration {

    @Bean
    fun corsConfigurer(): WebMvcConfigurer {
        return object : WebMvcConfigurer {
            override fun addCorsMappings(registry: CorsRegistry) {
                registry.addMapping("/api/**")
            }
        }
    }

}
```

## JAX-RS 和 Jersey

如果您更喜欢用于 REST 端点的 JAX-RS 编程模型，可以使用可用的实现之一来代替 Spring MVC。Jersey 和 Apache CXF 开箱即用效果很好。CXF 需要您在其应用程序上下文中将 `Servlet` 或 `Filter` 注册为 `@Bean`。Jersey 有一些原生 Spring 支持，因此我们还在 Spring Boot 中为其提供了自动配置支持以及启动器。

要开始使用 Jersey，请包含 `spring-boot-starter-jersey` 作为依赖项，然后需要一个类型为 `ResourceConfig` 的 `@Bean`，在其中注册所有端点，如下例所示：

```java
import org.glassfish.jersey.server.ResourceConfig;

import org.springframework.stereotype.Component;

@Component
public class MyJerseyConfig extends ResourceConfig {

    public MyJerseyConfig() {
        register(MyEndpoint.class);
    }

}
```

所有已注册的端点都应该是带有 HTTP 资源注释（`@GET` 等）的 `@Components`，如下例所示：

```java
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;

import org.springframework.stereotype.Component;

@Component
@Path("/hello")
public class MyEndpoint {

    @GET
    public String message() {
        return "Hello";
    }

}
```

## 嵌入式 Servlet 容器支持

对于 servlet 应用程序，Spring Boot 包括对嵌入式 Tomcat、Jetty 和 Undertow 服务器的支持。大多数开发人员使用相应的启动器来获取完全配置的实例。默认情况下，嵌入式服务器在端口 `8080` 上侦听 HTTP 请求。

### Servlets、过滤器和监听器

使用嵌入式 servlet 容器时，您可以注册 servlet、过滤器和所有 servlet 规范中的监听器（例如 `HttpSessionListener`），方法是使用 Spring bean 或扫描 servlet 组件。

任何作为 Spring bean 的 `Servlet`、`Filter` 或 servlet `*Listener` 实例都将向嵌入式容器注册。如果您想在配置期间引用 `application.properties` 中的值，这尤其方便。

### 自定义嵌入式 Servlet 容器

可以使用 Spring `Environment` 属性配置常见的 servlet 容器设置。通常，您会在 `application.properties` 或 `application.yaml` 文件中定义属性。

常见的服务器设置包括：

- 网络设置：用于传入 HTTP 请求的侦听端口 (`server.port`)、要绑定到的接口地址 (`server.address`) 等。
- 会话设置：会话是否持久 (`server.servlet.session.persistent`)、会话超时 (`server.servlet.session.timeout`)、会话数据的位置 (`server.servlet.session.store-dir`) 和会话 cookie 配置 (`server.servlet.session.cookie.*`)。
- 错误管理：错误页面的位置 (`server.error.path`) 等。
- SSL
- HTTP 压缩

### SameSite Cookie

`SameSite` cookie 属性可供 Web 浏览器使用，以控制是否以及如何在跨站点请求中提交 cookie。对于开始更改使用缺失属性时的默认值的现代 Web 浏览器，此属性尤其重要。

### JSP 限制

当运行使用嵌入式 servlet 容器（并打包为可执行归档文件）的 Spring Boot 应用程序时，JSP 支持存在一些限制：

- 使用 Jetty 和 Tomcat 时，如果使用 war 包，则应该可以正常工作。可执行 war 文件可以使用 `java -jar` 启动，也可以部署到任何标准容器中。使用可执行 jar 文件时不支持 JSP。
- Undertow 不支持 JSP。
- 创建自定义 `error.jsp` 页面不会覆盖错误处理的默认视图。应改用自定义错误页面。
