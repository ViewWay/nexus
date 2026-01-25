# Spring Web MVC (Spring Web MVC)

Spring Web MVC 是基于 Servlet API 构建的原始 Web 框架，从一开始就包含在 Spring Framework 中。正式名称为 "Spring Web MVC"，来源于其源模块名称（`spring-webmvc`），但通常被称为 "Spring MVC"。

与 Spring Web MVC 并行，Spring Framework 5.0 引入了一个响应式栈 Web 框架，名为 "Spring WebFlux"。

---

## 概述

### 什么是 Spring MVC

Spring MVC 是一个设计良好的 Web 框架，提供了：

- **DispatcherServlet** - 前端控制器模式的核心 Servlet
- **注解驱动的控制器** - 使用 `@Controller` 和 `@RestController`
- **灵活的方法签名** - 控制器方法不需要继承基类或实现特定接口
- **视图解析** - 支持多种视图技术（Thymeleaf、FreeMarker、JSP 等）
- **强大的配置** - 通过 Java 配置或 XML 配置
- **消息转换** - HTTP 消息转换器支持 JSON、XML 等

### 架构

```
客户端请求
    |
    v
DispatcherServlet (前端控制器)
    |
    +-- HandlerMapping (请求映射)
    +-- Controller (处理器)
    +-- ViewResolver (视图解析)
    +-- View (视图)
    |
    v
响应
```

---

## DispatcherServlet

### 简介

Spring MVC 与许多其他 Web 框架一样，是围绕**前端控制器模式**设计的，其中中央 `Servlet`，即 `DispatcherServlet`，提供了请求处理的共享算法，而实际工作由可配置的委托组件执行。

### Java 配置注册 DispatcherServlet

```java
public class MyWebApplicationInitializer implements WebApplicationInitializer {

    @Override
    public void onStartup(ServletContext servletContext) {

        // 加载 Spring Web 应用程序配置
        AnnotationConfigWebApplicationContext context = new AnnotationConfigWebApplicationContext();
        context.register(AppConfig.class);

        // 创建并注册 DispatcherServlet
        DispatcherServlet servlet = new DispatcherServlet(context);
        ServletRegistration.Dynamic registration = servletContext.addServlet("app", servlet);
        registration.setLoadOnStartup(1);
        registration.addMapping("/app/*");
    }
}
```

### XML 配置注册 DispatcherServlet

```xml
<web-app>

    <listener>
        <listener-class>org.springframework.web.context.ContextLoaderListener</listener-class>
    </listener>

    <context-param>
        <param-name>contextConfigLocation</param-name>
        <param-value>/WEB-INF/app-context.xml</param-value>
    </context-param>

    <servlet>
        <servlet-name>app</servlet-name>
        <servlet-class>org.springframework.web.servlet.DispatcherServlet</servlet-class>
        <init-param>
            <param-name>contextConfigLocation</param-name>
            <param-value></param-value>
        </init-param>
        <load-on-startup>1</load-on-startup>
    </servlet>

    <servlet-mapping>
        <servlet-name>app</servlet-name>
        <url-pattern>/app/*</url-pattern>
    </servlet-mapping>

</web-app>
```

### Spring Boot 初始化

Spring Boot 使用不同的初始化序列。它不是挂钩到 Servlet 容器的生命周期，而是使用 Spring 配置来引导自己和嵌入式 Servlet 容器。

```java
@SpringBootApplication
public class Application {
    public static void main(String[] args) {
        SpringApplication.run(Application.class, args);
    }
}
```

---

## 特殊 Bean 类型

`DispatcherServlet` 委托给特殊 bean 来处理请求和呈现响应。这些特殊 bean 是：

| Bean 类型 | 描述 |
|-----------|------|
| `HandlerMapping` | 将请求映射到处理器（带注解的控制器方法） |
| `HandlerAdapter` | 帮助 `DispatcherServlet` 调用映射到请求的处理器，而不考虑处理器是如何实际调用的 |
| `ViewResolver` | 将从处理器返回的逻辑视图名称解析为实际的视图 |
| `ExceptionHandler` | 解析异常，将其映射到处理器，也称为错误视图 |
| `LocaleResolver`, `LocaleContextResolver` | 解析客户端正在使用的区域设置和时区，以便能够呈现国际化视图 |
| `MultipartResolver` | 在 multipart 请求（例如文件上传）中解析 `multipart` 请求 |
| `FlashMapManager` | 存储和检索 "input" 和 "output" `FlashMap`，可用于将属性从一个请求传递到另一个请求（通常通过重定向） |

---

## 注解控制器

### @Controller 注解

Spring MVC 提供基于注解的编程模型，其中 `@Controller` 和 `@RestController` 组件使用注解来表达请求映射、请求输入、异常处理等。

```java
@Controller
public class HelloController {

    @GetMapping("/hello")
    public String handle(Model model) {
        model.addAttribute("message", "Hello World!");
        return "index";  // 视图名称
    }
}
```

### @RestController 注解

`@RestController` 是一个组合注解，结合了 `@Controller` 和 `@ResponseBody`，用于创建 RESTful 控制器：

```java
@RestController
@RequestMapping("/api")
public class UserController {

    @GetMapping("/users/{id}")
    public User getUser(@PathVariable Long id) {
        return userService.findById(id);
    }

    @PostMapping("/users")
    public User createUser(@RequestBody User user) {
        return userService.save(user);
    }
}
```

### 请求映射注解

| 注解 | 描述 |
|------|------|
| `@RequestMapping` | 通用的请求处理映射 |
| `@GetMapping` | 处理 HTTP GET 请求 |
| `@PostMapping` | 处理 HTTP POST 请求 |
| `@PutMapping` | 处理 HTTP PUT 请求 |
| `@DeleteMapping` | 处理 HTTP DELETE 请求 |
| `@PatchMapping` | 处理 HTTP PATCH 请求 |

### 请求参数注解

| 注解 | 描述 |
|------|------|
| `@PathVariable` | 访问 URI 模板变量 |
| `@RequestParam` | 访问 Servlet 请求参数 |
| `@RequestHeader` | 访问请求标头 |
| `@CookieValue` | 访问 Cookie 值 |
| `@RequestBody` | 访问请求正文 |
| `@RequestAttribute` | 访问请求属性 |
| `@SessionAttribute` | 访问会话属性 |
| `@ModelAttribute` | 访问模型属性 |

### 方法参数示例

```java
@RestController
@RequestMapping("/users")
public class UserController {

    // 路径变量
    @GetMapping("/{id}")
    public User getUser(@PathVariable Long id) {
        return userService.findById(id);
    }

    // 请求参数
    @GetMapping
    public List<User> getUsers(@RequestParam(defaultValue = "0") int page,
                              @RequestParam(defaultValue = "10") int size) {
        return userService.findAll(page, size);
    }

    // 请求体
    @PostMapping
    public User createUser(@RequestBody @Valid User user) {
        return userService.save(user);
    }

    // 请求头
    @GetMapping("/check")
    public boolean checkAuth(@RequestHeader("Authorization") String authHeader) {
        return authService.validate(authHeader);
    }

    // 多值映射
    @GetMapping("/search")
    public List<User> searchUsers(@RequestParam Map<String, String> params) {
        return userService.search(params);
    }
}
```

### 返回值类型

| 返回值 | 描述 |
|--------|------|
| `@ResponseBody` | 返回值通过消息转换器写入响应 |
| `String` | 通过 ViewResolver 解析的逻辑视图名称 |
| `View` | 渲染的 View 实例 |
| `Model` | 要添加到模型的模型属性 |
| `ModelAndView` | 视图和模型 |
| `ResponseEntity<?>` | 响应实体，包含状态码、标头和正文 |
| `HttpEntity<?>` | 与 ResponseEntity 相似，但没有状态码 |

---

## 过滤器

### 注册过滤器

使用 Spring Boot：

```java
@Configuration
public class FilterConfig {

    @Bean
    public FilterRegistrationBean<LoggingFilter> loggingFilter() {
        FilterRegistrationBean<LoggingFilter> registrationBean = new FilterRegistrationBean<>();
        registrationBean.setFilter(new LoggingFilter());
        registrationBean.addUrlPatterns("/*");
        return registrationBean;
    }
}
```

使用 `@Component`：

```java
@Component
@Order(Ordered.HIGHEST_PRECEDENCE)
public class LoggingFilter implements Filter {

    @Override
    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) {
        HttpServletRequest req = (HttpServletRequest) request;
        System.out.println("Request: " + req.getRequestURI());
        chain.doFilter(request, response);
    }
}
```

---

## HTTP 消息转换

### HttpMessageConverter

`spring-web` 模块包含用于读取和写入 HTTP 请求和响应正文的 `HttpMessageConverter` 接口。

| 消息转换器 | 描述 |
|-----------|------|
| `StringHttpMessageConverter` | 读取/写入 `String`，支持 `text/*` |
| `FormHttpMessageConverter` | 读取/写入表单数据，支持 `application/x-www-form-urlencoded` |
| `ByteArrayHttpMessageConverter` | 读取/写入字节数组，支持 `*/*` |
| `MappingJackson2HttpMessageConverter` | 使用 Jackson 读取/写入 JSON |
| `MappingJackson2XmlHttpMessageConverter` | 使用 Jackson XML 扩展读取/写入 XML |

### 配置消息转换器

```java
@Configuration
@EnableWebMvc
public class WebConfig implements WebMvcConfigurer {

    @Override
    public void configureMessageConverters(List<HttpMessageConverter<?>> converters) {
        Jackson2ObjectMapperBuilder builder = new Jackson2ObjectMapperBuilder()
                .indentOutput(true)
                .dateFormat(new SimpleDateFormat("yyyy-MM-dd"));
        converters.add(new MappingJackson2HttpMessageConverter(builder.build()));
    }
}
```

---

## MVC 配置

### MVC Java 配置

```java
@Configuration
@EnableWebMvc
@ComponentScan(basePackages = "com.example.web")
public class WebConfig implements WebMvcConfigurer {

    // 视图控制器
    @Override
    public void addViewControllers(ViewControllerRegistry registry) {
        registry.addViewController("/").setViewName("home");
        registry.addViewController("/login").setViewName("login");
    }

    // 视图解析器
    @Bean
    public ViewResolver viewResolver() {
        InternalResourceViewResolver bean = new InternalResourceViewResolver();
        bean.setViewClass(JstlView.class);
        bean.setPrefix("/WEB-INF/views/");
        bean.setSuffix(".jsp");
        return bean;
    }

    // 静态资源
    @Override
    public void addResourceHandlers(ResourceHandlerRegistry registry) {
        registry.addResourceHandler("/resources/**")
                .addResourceLocations("/resources/", "/static/");
    }

    // 拦截器
    @Override
    public void addInterceptors(InterceptorRegistry registry) {
        registry.addInterceptor(new LocaleChangeInterceptor());
        registry.addInterceptor(new ThemeChangeInterceptor()).addPathPatterns("/**").excludePathPatterns("/admin/**");
    }

    // 默认 Servlet 处理
    @Override
    public void configureDefaultServletHandling(DefaultServletHandlerConfigurer configurer) {
        configurer.enable();
    }
}
```

---

## 异步请求

### Controller 异步方法

```java
@Controller
public class AsyncController {

    @GetMapping("/async")
    public Callable<String> asyncRequest() {
        return () -> {
            // 长时间运行的任务
            Thread.sleep(2000);
            return "async-result";
        };
    }

    @GetMapping("/async-deferred")
    public DeferredResult<String> asyncDeferredRequest() {
        DeferredResult<String> deferredResult = new DeferredResult<>();
        // 在另一个线程中设置结果
        new Thread(() -> {
            try {
                Thread.sleep(2000);
                deferredResult.setResult("async-deferred-result");
            } catch (InterruptedException e) {
                deferredResult.setErrorResult(e);
            }
        }).start();
        return deferredResult;
    }
}
```

### 配置异步支持

```java
@Configuration
@EnableWebMvc
public class WebConfig implements WebMvcConfigurer {

    @Override
    public void configureAsyncSupport(AsyncSupportConfigurer configurer) {
        configurer.setDefaultTimeout(5000);
        configurer.setTaskExecutor(mvcTaskExecutor());
    }

    @Bean
    public ThreadPoolTaskExecutor mvcTaskExecutor() {
        ThreadPoolTaskExecutor executor = new ThreadPoolTaskExecutor();
        executor.setCorePoolSize(5);
        executor.setMaxPoolSize(10);
        executor.setQueueCapacity(25);
        executor.setThreadNamePrefix("mvc-async-");
        return executor;
    }
}
```

---

## CORS (跨域资源共享)

### Controller 级别 CORS

```java
@CrossOrigin(origins = "https://example.com", maxAge = 3600)
@RestController
@RequestMapping("/api")
public class UserController {

    @GetMapping("/users")
    public List<User> getUsers() {
        return userService.findAll();
    }

    // 方法级别覆盖
    @CrossOrigin(origins = "https://another-domain.com")
    @GetMapping("/users/{id}")
    public User getUser(@PathVariable Long id) {
        return userService.findById(id);
    }
}
```

### 全局 CORS 配置

```java
@Configuration
@EnableWebMvc
public class WebConfig implements WebMvcConfigurer {

    @Override
    public void addCorsMappings(CorsRegistry registry) {
        registry.addMapping("/api/**")
                .allowedOrigins("https://example.com", "https://another-domain.com")
                .allowedMethods("GET", "POST", "PUT", "DELETE", "OPTIONS")
                .allowedHeaders("*")
                .allowCredentials(true)
                .maxAge(3600);
    }
}
```

---

## 错误处理

### @ExceptionHandler

```java
@Controller
public class UserController {

    @GetMapping("/users/{id}")
    public String getUser(@PathVariable Long id, Model model) {
        User user = userService.findById(id);
        if (user == null) {
            throw new UserNotFoundException(id);
        }
        model.addAttribute("user", user);
        return "user";
    }

    @ExceptionHandler(UserNotFoundException.class)
    public ResponseEntity<ErrorResponse> handleUserNotFound(UserNotFoundException ex) {
        ErrorResponse error = new ErrorResponse("USER_NOT_FOUND", ex.getMessage());
        return ResponseEntity.status(HttpStatus.NOT_FOUND).body(error);
    }

    @ExceptionHandler(Exception.class)
    public ResponseEntity<ErrorResponse> handleAllExceptions(Exception ex) {
        ErrorResponse error = new ErrorResponse("INTERNAL_ERROR", "An error occurred");
        return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body(error);
    }
}
```

### @ControllerAdvice

```java
@ControllerAdvice
public class GlobalExceptionHandler {

    @ExceptionHandler(UserNotFoundException.class)
    public ResponseEntity<ErrorResponse> handleUserNotFound(UserNotFoundException ex) {
        ErrorResponse error = new ErrorResponse("USER_NOT_FOUND", ex.getMessage());
        return ResponseEntity.status(HttpStatus.NOT_FOUND).body(error);
    }

    @ExceptionHandler(MethodArgumentNotValidException.class)
    public ResponseEntity<ErrorResponse> handleValidationExceptions(MethodArgumentNotValidException ex) {
        List<String> errors = ex.getBindingResult()
                .getFieldErrors()
                .stream()
                .map(FieldError::getDefaultMessage)
                .collect(Collectors.toList());
        ErrorResponse error = new ErrorResponse("VALIDATION_ERROR", errors.toString());
        return ResponseEntity.badRequest().body(error);
    }
}
```

---

## HTTP 缓存

### Cache-Control

```java
@RestController
@RequestMapping("/api")
public class ResourceController {

    @GetMapping("/resources/{id}")
    public ResponseEntity<Resource> getResource(@PathVariable Long id) {
        Resource resource = resourceService.findById(id);
        return ResponseEntity.ok()
                .cacheControl(CacheControl.maxAge(3600, TimeUnit.SECONDS))
                .eTag(String.valueOf(resource.getVersion()))
                .body(resource);
    }
}
```

---

## 视图技术

### 支持的视图技术

- **Thymeleaf** - 现代的服务器端 Java 模板引擎
- **FreeMarker** - 经典的模板引擎
- **Groovy Markup** - 使用 Groovy 的标记模板引擎
- **JSP 和 JSTL** - JavaServer Pages
- **RSS 和 Atom** - 用于 RSS 和 Atom 源
- **PDF 和 Excel** - 用于生成文档
- **Jackson** - 用于 JSON 生成
- **XML Marshalling** - 用于 XML 生成

### Thymeleaf 配置

```java
@Configuration
public class ThymeleafConfig {

    @Bean
    public SpringTemplateEngine templateEngine() {
        SpringTemplateEngine templateEngine = new SpringTemplateEngine();
        templateEngine.setTemplateResolver(thymeleafTemplateResolver());
        return templateEngine;
    }

    @Bean
    public SpringResourceTemplateResolver thymeleafTemplateResolver() {
        SpringResourceTemplateResolver templateResolver = new SpringResourceTemplateResolver();
        templateResolver.setPrefix("classpath:/templates/");
        templateResolver.setSuffix(".html");
        templateResolver.setTemplateMode(TemplateMode.HTML);
        return templateResolver;
    }
}
```

---

## 参考资料

- [Spring Framework Documentation](https://docs.spring.io/spring-framework/reference/web/webmvc.html)
- [DispatcherServlet Documentation](https://docs.spring.io/spring-framework/reference/web/webmvc/mvc-servlet.html)
- [Annotated Controllers](https://docs.spring.io/spring-framework/reference/web/webmvc/mvc-controller.html)
- [Spring MVC Test Framework](https://docs.spring.io/spring-framework/reference/web/webmvc/test.html)
