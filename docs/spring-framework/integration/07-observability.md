# 可观测性支持

Micrometer 定义了一个可观测性概念，可在应用程序中同时启用 Metrics 和 Traces。Metrics 支持提供了一种创建计时器、量规或计数器的方法，用于收集应用程序运行时行为的统计信息。Metrics 可以帮助您跟踪错误率、使用模式、性能等。Traces 提供整个系统的整体视图，跨越应用程序边界；您可以放大特定用户请求，并跟踪它们在应用程序中的完整完成过程。

如果配置了 `ObservationRegistry`，Spring Framework 会在其自身代码库的各个部分进行检测，以发布可观测性数据。

## 生成的观测数据列表

Spring Framework 对各种功能进行了检测，以实现可观测性：

| 观测数据名称 | 描述 |
| --- | --- |
| `"http.client.requests"` | HTTP 客户端交换所花费的时间 |
| `"http.server.requests"` | Framework 级别 HTTP 服务器交换的处理时间 |
| `"jms.message.publish"` | 消息生产者向目标发送 JMS 消息所花费的时间 |
| `"jms.message.process"` | 消息消费者先前接收到的 JMS 消息的处理时间 |
| `"tasks.scheduled.execution"` | `@Scheduled` 任务执行的处理时间 |

> 观测数据使用 Micrometer 的官方命名约定，但 Metrics 名称将自动转换为监控系统后端偏好的格式 (Prometheus, Atlas, Graphite, InfluxDB 等)。

## Micrometer Observation 概念

- `Observation` - 应用程序中发生事件的实际记录
- `ObservationContext` - 包含所有相关信息的上下文实现
- `KeyValues` - 元数据（低基数用于指标，高基数用于跟踪）
- `ObservationConvention` - 自定义观测数据名称和提取的元数据

## 配置观测数据

全局配置选项可在 `ObservationRegistry#observationConfig()` 级别获得：

### 使用自定义观测数据约定

```java
import io.micrometer.common.KeyValue;
import io.micrometer.common.KeyValues;
import org.springframework.http.server.observation.DefaultServerRequestObservationConvention;
import org.springframework.http.server.observation.ServerRequestObservationContext;

public class ExtendedServerRequestObservationConvention extends DefaultServerRequestObservationConvention {

    @Override
    public KeyValues getLowCardinalityKeyValues(ServerRequestObservationContext context) {
        return super.getLowCardinalityKeyValues(context).and(custom(context));
    }

    private KeyValue custom(ServerRequestObservationContext context) {
        return KeyValue.of("custom.method", context.getCarrier().getMethod());
    }
}
```

### 使用 ObservationFilter

```java
import io.micrometer.common.KeyValue;
import io.micrometer.observation.Observation;
import io.micrometer.observation.ObservationFilter;
import org.springframework.http.server.observation.ServerRequestObservationContext;

public class ServerRequestObservationFilter implements ObservationFilter {

    @Override
    public Observation.Context map(Observation.Context context) {
        if (context instanceof ServerRequestObservationContext serverContext) {
            context.setName("custom.observation.name");
            context.addLowCardinalityKeyValue(KeyValue.of("project", "spring"));
        }
        return context;
    }
}
```

## @Scheduled 任务检测

```java
import io.micrometer.observation.ObservationRegistry;
import org.springframework.scheduling.annotation.SchedulingConfigurer;
import org.springframework.scheduling.config.ScheduledTaskRegistrar;

public class ObservationSchedulingConfigurer implements SchedulingConfigurer {

    private final ObservationRegistry observationRegistry;

    public ObservationSchedulingConfigurer(ObservationRegistry observationRegistry) {
        this.observationRegistry = observationRegistry;
    }

    @Override
    public void configureTasks(ScheduledTaskRegistrar taskRegistrar) {
        taskRegistrar.setObservationRegistry(this.observationRegistry);
    }
}
```

低基数键：

| 名称 | 描述 |
| --- | --- |
| `code.function` | 计划执行的 Java `Method` 的名称 |
| `code.namespace` | 持有计划方法的 bean 实例类的规范名称 |
| `error` | 执行期间抛出的异常类名 |
| `outcome` | 方法执行的结果：`"SUCCESS"`、`"ERROR"` 或 `"UNKNOWN"` |

## HTTP 服务器检测

### Servlet 应用

```java
import org.springframework.web.filter.ServerHttpObservationFilter;
import org.springframework.boot.web.servlet.FilterRegistrationBean;

@Configuration
public class ObservabilityConfig {

    @Bean
    public FilterRegistrationBean<ServerHttpObservationFilter> observationFilter() {
        FilterRegistrationBean<ServerHttpObservationFilter> registration = new FilterRegistrationBean<>();
        registration.setFilter(new ServerHttpObservationFilter());
        registration.addUrlPatterns("/*");
        return registration;
    }
}
```

低基数键：

| 名称 | 描述 |
| --- | --- |
| `error` | 交换期间抛出的异常类名 |
| `method` | HTTP 请求方法的名称 |
| `outcome` | HTTP 服务器交换的结果 |
| `status` | HTTP 响应原始状态码 |
| `uri` | 匹配 handler 的 URI 模式 |

### 响应式应用

```java
import io.micrometer.observation.ObservationRegistry;
import org.springframework.http.server.reactive.HttpHandler;
import org.springframework.web.server.adapter.WebHttpHandlerBuilder;

@Configuration(proxyBeanMethods = false)
public class HttpHandlerConfiguration {

    @Bean
    public HttpHandler httpHandler(ObservationRegistry registry) {
        return WebHttpHandlerBuilder.applicationContext(applicationContext)
            .observationRegistry(registry)
            .build();
    }
}
```

## HTTP 客户端检测

### RestTemplate

```java
@Bean
public RestTemplate restTemplate(ObservationRegistry registry) {
    RestTemplate restTemplate = new RestTemplate();
    restTemplate.setObservationRegistry(registry);
    return restTemplate;
}
```

### RestClient

```java
@Bean
public RestClient.Builder restClientBuilder(ObservationRegistry registry) {
    return RestClient.builder()
        .observationRegistry(registry);
}
```

### WebClient

```java
@Bean
public WebClient.Builder webClientBuilder(ObservationRegistry registry) {
    return WebClient.builder()
        .observationRegistry(registry);
}
```

## 应用事件和 `@EventListener`

Spring Framework 不为 `@EventListener` 调用贡献 Observation。如果需要传播上下文，可以使用 `ContextPropagatingTaskDecorator`：

```java
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.context.event.SimpleApplicationEventMulticaster;
import org.springframework.core.task.SimpleAsyncTaskExecutor;
import org.springframework.core.task.support.ContextPropagatingTaskDecorator;

@Configuration
public class ApplicationEventsConfiguration {

    @Bean(name = "applicationEventMulticaster")
    public SimpleApplicationEventMulticaster simpleApplicationEventMulticaster() {
        SimpleApplicationEventMulticaster eventMulticaster = new SimpleApplicationEventMulticaster();
        SimpleAsyncTaskExecutor taskExecutor = new SimpleAsyncTaskExecutor();
        taskExecutor.setTaskDecorator(new ContextPropagatingTaskDecorator());
        eventMulticaster.setTaskExecutor(taskExecutor);
        return eventMulticaster;
    }
}
```
