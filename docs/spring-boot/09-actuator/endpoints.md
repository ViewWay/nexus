# Actuator Endpoints / 执行器端点

Source: https://docs.springframework.org.cn/spring-boot/reference/actuator/endpoints.html

---

## English

Actuator endpoints let you monitor and interact with your application. Spring Boot includes a number of built-in endpoints and lets you add your own. For example, the `health` endpoint provides basic application health information.

You can enable or disable each individual endpoint and expose them over HTTP or JMX (make them remotely accessible). An endpoint is considered to be available when it is both enabled and exposed. Only built-in endpoints are auto-configured when available. Most applications choose to expose endpoints over HTTP, where the ID of the endpoint and a prefix of `/actuator` is mapped to a URL. For example, by default, the `health` endpoint is mapped to `/actuator/health`.

The following technology-agnostic endpoints are available:

| ID | Description |
| --- | --- |
| `auditevents` | Exposes audit events information for the current application. Requires an `AuditEventRepository` bean. |
| `beans` | Displays a complete list of all the Spring beans in your application. |
| `caches` | Exposes available caches. |
| `conditions` | Shows the conditions that were evaluated on configuration and auto-configuration classes and the reasons why they did or did not match. |
| `configprops` | Displays a collated list of all `@ConfigurationProperties`. Subject to sanitization. |
| `env` | Exposes properties from Spring's `ConfigurableEnvironment`. Subject to sanitization. |
| `flyway` | Shows any Flyway database migrations that have been applied. Requires one or more `Flyway` beans. |
| `health` | Shows application health information. |
| `httpexchanges` | Displays HTTP exchange information (by default, the last 100 HTTP request-response exchanges). Requires an `HttpExchangeRepository` bean. |
| `info` | Displays arbitrary application information. |
| `integrationgraph` | Shows the Spring Integration graph. Requires a dependency on `spring-integration-core`. |
| `loggers` | Shows and modifies the configuration of loggers in the application. |
| `liquibase` | Shows any Liquibase database migrations that have been applied. Requires one or more `Liquibase` beans. |
| `metrics` | Shows "metrics" information for the current application. |
| `mappings` | Displays a collated list of all `@RequestMapping` paths. |
| `quartz` | Shows information about Quartz Scheduler jobs. Subject to sanitization. |
| `scheduledtasks` | Shows the scheduled tasks in the application. |
| `sessions` | Allows retrieval and deletion of user sessions from a Spring Session-backed session store. Requires a servlet-based web application that uses Spring Session. |
| `shutdown` | Lets the application be gracefully shut down. Only works when using jar packaging. Disabled by default. |
| `startup` | Shows the startup steps data collected by `ApplicationStartup`. Requires the `SpringApplication` to be configured with a `BufferingApplicationStartup`. |
| `threaddump` | Performs a thread dump. |

If your application is a web application (Spring MVC, Spring WebFlux, or Jersey), the following additional endpoints are available:

| ID | Description |
| --- | --- |
| `heapdump` | Returns a heap dump file. On HotSpot JVMs, returns a file in `HPROF` format. On OpenJ9 JVMs, returns a file in `PHD` format. |
| `logfile` | Returns the contents of the log file (if the `logging.file.name` or `logging.file.path` property has been set). Supports the use of the HTTP `Range` header to retrieve part of the log file's contents. |
| `prometheus` | Exposes metrics in a format that can be scraped by a Prometheus server. Requires a dependency on `micrometer-registry-prometheus`. |

## Enabling Endpoints

By default, all endpoints except `shutdown` are enabled. To configure the enablement of an endpoint, use its `management.endpoint.<id>.enabled` property. The following example enables the `shutdown` endpoint:

- Properties
- YAML

```properties
management.endpoint.shutdown.enabled=true
```

```yaml
management:
  endpoint:
    shutdown:
      enabled: true
```

If you want endpoint enablement to be opt-in rather than opt-out, set the `management.endpoints.enabled-by-default` property to `false` and use individual endpoint `enabled` properties to opt back in. The following example enables the `info` endpoint and disables all other endpoints:

- Properties
- YAML

```properties
management.endpoints.enabled-by-default=false
management.endpoint.info.enabled=true
```

```yaml
management:
  endpoints:
    enabled-by-default: false
  endpoint:
    info:
      enabled: true
```

## Exposing Endpoints

By default, only the health endpoint is exposed over HTTP and JMX. Since endpoints may contain sensitive information, you should carefully consider when to expose them.

To change which endpoints are exposed, use the following technology-specific `include` and `exclude` properties:

| Property | Default |
| --- | --- |
| `management.endpoints.jmx.exposure.exclude` |  |
| `management.endpoints.jmx.exposure.include` | `health` |
| `management.endpoints.web.exposure.exclude` |  |
| `management.endpoints.web.exposure.include` | `health` |

The `include` property lists the IDs of the endpoints that are exposed. The `exclude` property lists the IDs of the endpoints that should not be exposed. The `exclude` property takes precedence over the `include` property. You can configure both the `include` and the `exclude` properties with a list of endpoint IDs.

For example, to expose only the `health` and `info` endpoints over JMX, use the following property:

- Properties
- YAML

```properties
management.endpoints.jmx.exposure.include=health,info
```

```yaml
management:
  endpoints:
    jmx:
      exposure:
        include: "health,info"
```

The `*` can be used to select all endpoints. For example, to expose everything over HTTP except `env` and `beans` endpoints, use the following properties:

- Properties
- YAML

```properties
management.endpoints.web.exposure.include=*
management.endpoints.web.exposure.exclude=env,beans
```

```yaml
management:
  endpoints:
    web:
      exposure:
        include: "*"
        exclude: "env,beans"
```

## Security

For security reasons, only the `/health` endpoint is exposed over HTTP by default. You can configure the exposed endpoints by using the `management.endpoints.web.exposure.include` property.

If Spring Security is on the classpath and no other `SecurityFilterChain` bean is present, all actuators other than `/health` are secured by Spring Boot auto-configuration. If you define a custom `SecurityFilterChain` bean, Spring Boot auto-configuration backs off and lets you fully control actuator access rules.

A typical Spring Security configuration might look like the following example:

- Java
- Kotlin

```java
import org.springframework.boot.actuate.autoconfigure.security.servlet.EndpointRequest;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.web.SecurityFilterChain;

import static org.springframework.security.config.Customizer.withDefaults;

@Configuration(proxyBeanMethods = false)
public class MySecurityConfiguration {

    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
        http.securityMatcher(EndpointRequest.toAnyEndpoint());
        http.authorizeHttpRequests((requests) -> requests.anyRequest().hasRole("ENDPOINT_ADMIN"));
        http.httpBasic(withDefaults());
        return http.build();
    }

}
```

```kotlin
import org.springframework.boot.actuate.autoconfigure.security.servlet.EndpointRequest
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.security.config.Customizer.withDefaults
import org.springframework.security.config.annotation.web.builders.HttpSecurity
import org.springframework.security.web.SecurityFilterChain

@Configuration(proxyBeanMethods = false)
class MySecurityConfiguration {

    @Bean
    fun securityFilterChain(http: HttpSecurity): SecurityFilterChain {
        http.securityMatcher(EndpointRequest.toAnyEndpoint())
        http.authorizeHttpRequests { requests ->
            requests.anyRequest().hasRole("ENDPOINT_ADMIN")
        }
        http.httpBasic(withDefaults())
        return http.build()
    }

}
```

## Health Information

You can use health information to check the status of your running application. It is often used by monitoring software to alert someone if a production system goes down. The information exposed by the `health` endpoint depends on the `management.endpoint.health.show-details` and `management.endpoint.health.show-components` properties, which can be configured with one of the following values:

| Name | Description |
| --- | --- |
| `never` | Details are never shown. |
| `when-authorized` | Details are shown to authorized users. Authorized roles can be configured by using `management.endpoint.health.roles`. |
| `always` | Details are shown to all users. |

The default value is `never`. A user is considered to be authorized when they are in one or more of the endpoint's roles. If the endpoint has no configured roles (the default), all authenticated users are considered to be authorized. You can configure roles by using the `management.endpoint.health.roles` property.

### Auto-configured HealthIndicators

The following `HealthIndicators` are auto-configured by Spring Boot when appropriate. You can also enable or disable selected indicators by configuring `management.health.key.enabled`, where `key` is listed in the following table:

| Key | Name | Description |
| --- | --- | --- |
| `cassandra` | `CassandraDriverHealthIndicator` | Checks that a Cassandra database is up. |
| `couchbase` | `CouchbaseHealthIndicator` | Checks that a Couchbase cluster is up. |
| `db` | `DataSourceHealthIndicator` | Checks that a connection to `DataSource` can be obtained. |
| `diskspace` | `DiskSpaceHealthIndicator` | Checks for low disk space. |
| `elasticsearch` | `ElasticsearchRestClientHealthIndicator` | Checks that an Elasticsearch cluster is up. |
| `hazelcast` | `HazelcastHealthIndicator` | Checks that a Hazelcast server is up. |
| `influxdb` | `InfluxDbHealthIndicator` | Checks that an InfluxDB server is up. |
| `jms` | `JmsHealthIndicator` | Checks that a JMS broker is up. |
| `ldap` | `LdapHealthIndicator` | Checks that an LDAP server is up. |
| `mail` | `MailHealthIndicator` | Checks that a mail server is up. |
| `mongo` | `MongoHealthIndicator` | Checks that a Mongo database is up. |
| `neo4j` | `Neo4jHealthIndicator` | Checks that a Neo4j database is up. |
| `ping` | `PingHealthIndicator` | Always responds with `UP`. |
| `rabbit` | `RabbitHealthIndicator` | Checks that a Rabbit server is up. |
| `redis` | `RedisHealthIndicator` | Checks that a Redis server is up. |

### Writing Custom HealthIndicators

To provide custom health information, you can register Spring beans that implement the `HealthIndicator` interface. You need to provide an implementation of the `health()` method and return a `Health` response. The `Health` response should include a status and can optionally include additional details to be displayed. The following code shows an example `HealthIndicator` implementation:

- Java
- Kotlin

```java
import org.springframework.boot.actuate.health.Health;
import org.springframework.boot.actuate.health.HealthIndicator;
import org.springframework.stereotype.Component;

@Component
public class MyHealthIndicator implements HealthIndicator {

    @Override
    public Health health() {
        int errorCode = check();
        if (errorCode != 0) {
            return Health.down().withDetail("Error Code", errorCode).build();
        }
        return Health.up().build();
    }

    private int check() {
        // perform some specific health check
        return ...;
    }

}
```

```kotlin
import org.springframework.boot.actuate.health.Health
import org.springframework.boot.actuate.health.HealthIndicator
import org.springframework.stereotype.Component

@Component
class MyHealthIndicator : HealthIndicator {

    override fun health(): Health {
        val errorCode = check()
        if (errorCode != 0) {
            return Health.down().withDetail("Error Code", errorCode).build()
        }
        return Health.up().build()
    }

    private fun check(): Int {
        // perform some specific health check
        return  ...
    }

}
```

## Kubernetes Probes

Applications deployed on Kubernetes can provide information about their internal state using container probes. Depending on your Kubernetes configuration, the kubelet calls these probes and reacts to the result.

By default, Spring Boot manages the availability state of your application. If deployed in a Kubernetes environment, the actuator gathers "Liveness" and "Readiness" information from the `ApplicationAvailability` interface and uses it in dedicated health indicators: `LivenessStateHealthIndicator` and `ReadinessStateHealthIndicator`. These indicators are shown on the global health endpoint (`"/actuator/health"`). They are also exposed as individual HTTP probes using health groups: `"/actuator/health/liveness"` and `"/actuator/health/readiness"`.

You can then configure your Kubernetes infrastructure with the following endpoint information:

```yaml
livenessProbe:
  httpGet:
    path: "/actuator/health/liveness"
    port: <actuator-port>
  failureThreshold: ...
  periodSeconds: ...

readinessProbe:
  httpGet:
    path: "/actuator/health/readiness"
    port: <actuator-port>
  failureThreshold: ...
  periodSeconds: ...
```

## Application Information

Information from arbitrary `InfoContributor` beans is exposed from the `info` endpoint. Spring Boot includes many auto-configured `InfoContributor` beans, and you can write your own.

### Auto-configured InfoContributors

The following `InfoContributor` beans are auto-configated by Spring Boot when appropriate:

| ID | Name | Description | Prerequisites |
| --- | --- | --- | --- |
| `build` | `BuildInfoContributor` | Exposes build information. | A `META-INF/build-info.properties` resource. |
| `env` | `EnvironmentInfoContributor` | Exposes any property from the `Environment` whose name starts with `info.`. | None. |
| `git` | `GitInfoContributor` | Exposes git information. | A `git.properties` resource. |
| `java` | `JavaInfoContributor` | Exposes Java runtime information. | None. |
| `os` | `OsInfoContributor` | Exposes operating system information. | None. |
| `process` | `ProcessInfoContributor` | Exposes process information. | None. |

### Customizing Application Information

Once the `env` contributor is enabled, you can customize the data exposed by the `info` endpoint by setting `info.*` Spring properties. All `Environment` properties under the `info` key are automatically exposed. For example, you could add the following settings to your `application.properties` file:

- Properties
- YAML

```properties
info.app.encoding=UTF-8
info.app.java.source=17
info.app.java.target=17
```

```yaml
info:
  app:
    encoding: "UTF-8"
    java:
      source: "17"
      target: "17"
```

### Git Commit Information

Another useful feature of the `info` endpoint is its ability to publish information about the state of your `git` source code repository when the project was built. If a `GitProperties` bean is available, you can use the `info` endpoint to expose these properties.

By default, the endpoint exposes `git.branch`, `git.commit.id`, and `git.commit.time` properties, if present. If you do not want any of these properties in the endpoint response, you need to exclude them from the `git.properties` file. If you want to display the full git information (that is, the full content of `git.properties`), use the `management.info.git.mode` property, as follows:

- Properties
- YAML

```properties
management.info.git.mode=full
```

```yaml
management:
  info:
    git:
      mode: "full"
```

---

## 中文 / Chinese

执行器端点允许你监控和与你的应用程序交互。Spring Boot 包含许多内置端点，并允许你添加你自己的端点。例如，`health` 端点提供基本的应用程序健康信息。

您可以启用或禁用每个单独的端点，并通过 HTTP 或 JMX 公开它们（使其远程可访问）。当端点同时启用和公开时，则认为该端点可用。内置端点仅在其可用时才会自动配置。大多数应用程序选择通过 HTTP 公开端点，其中端点的 ID 和前缀 `/actuator` 映射到一个 URL。例如，默认情况下，`health` 端点映射到 `/actuator/health`。

以下技术无关的端点可用：

| ID | 描述 |
| --- | --- |
| `auditevents` | 公开当前应用程序的审计事件信息。需要一个 `AuditEventRepository` bean。 |
| `beans` | 显示应用程序中所有 Spring bean 的完整列表。 |
| `caches` | 公开可用的缓存。 |
| `conditions` | 显示在配置和自动配置类上评估的条件以及它们匹配或不匹配的原因。 |
| `configprops` | 显示所有 `@ConfigurationProperties` 的汇总列表。受清理影响。 |
| `env` | 公开来自 Spring 的 `ConfigurableEnvironment` 的属性。受清理影响。 |
| `flyway` | 显示已应用的任何 Flyway 数据库迁移。需要一个或多个 `Flyway` bean。 |
| `health` | 显示应用程序运行状况信息。 |
| `httpexchanges` | 显示 HTTP 交换信息（默认情况下，最近 100 个 HTTP 请求-响应交换）。需要一个 `HttpExchangeRepository` bean。 |
| `info` | 显示任意应用程序信息。 |
| `integrationgraph` | 显示 Spring 集成图。需要依赖于 `spring-integration-core`。 |
| `loggers` | 显示和修改应用程序中日志记录器的配置。 |
| `liquibase` | 显示已应用的任何 Liquibase 数据库迁移。需要一个或多个 `Liquibase` bean。 |
| `metrics` | 显示当前应用程序的"指标"信息。 |
| `mappings` | 显示所有 `@RequestMapping` 路径的汇总列表。 |
| `quartz` | 显示有关 Quartz 调度程序作业的信息。受清理影响。 |
| `scheduledtasks` | 显示应用程序中的计划任务。 |
| `sessions` | 允许从 Spring Session 支持的会话存储中检索和删除用户会话。需要使用 Spring Session 的基于 servlet 的 Web 应用程序。 |
| `shutdown` | 允许优雅地关闭应用程序。仅在使用 jar 包时有效。默认情况下禁用。 |
| `startup` | 显示 `ApplicationStartup` 收集的启动步骤数据。需要将 `SpringApplication` 配置为 `BufferingApplicationStartup`。 |
| `threaddump` | 执行线程转储。 |

如果您的应用程序是 Web 应用程序（Spring MVC、Spring WebFlux 或 Jersey），您可以使用以下附加端点：

| ID | 描述 |
| --- | --- |
| `heapdump` | 返回堆转储文件。在 HotSpot JVM 上，返回 `HPROF` 格式的文件。在 OpenJ9 JVM 上，返回 `PHD` 格式的文件。 |
| `logfile` | 返回日志文件的内容（如果已设置 `logging.file.name` 或 `logging.file.path` 属性）。支持使用 HTTP `Range` 标头来检索日志文件内容的一部分。 |
| `prometheus` | 以 Prometheus 服务器可以抓取的格式公开指标。需要依赖于 `micrometer-registry-prometheus`。 |

## 启用端点

默认情况下，除 `shutdown` 之外的所有端点都已启用。要配置端点的启用，请使用其 `management.endpoint.<id>.enabled` 属性。以下示例启用了 `shutdown` 端点：

- 属性
- YAML

```properties
management.endpoint.shutdown.enabled=true
```

```yaml
management:
  endpoint:
    shutdown:
      enabled: true
```

如果您希望端点启用采用选择加入而不是选择退出，请将 `management.endpoints.enabled-by-default` 属性设置为 `false`，并使用各个端点的 `enabled` 属性重新选择加入。以下示例启用 `info` 端点并禁用所有其他端点：

- 属性
- YAML

```properties
management.endpoints.enabled-by-default=false
management.endpoint.info.enabled=true
```

```yaml
management:
  endpoints:
    enabled-by-default: false
  endpoint:
    info:
      enabled: true
```

## 公开端点

默认情况下，只有 health 端点通过 HTTP 和 JMX 公开。由于端点可能包含敏感信息，因此您应仔细考虑何时公开它们。

要更改公开的端点，请使用以下特定于技术的 `include` 和 `exclude` 属性：

| 属性 | 默认值 |
| --- | --- |
| `management.endpoints.jmx.exposure.exclude` |  |
| `management.endpoints.jmx.exposure.include` | `health` |
| `management.endpoints.web.exposure.exclude` |  |
| `management.endpoints.web.exposure.include` | `health` |

`include` 属性列出公开的端点的 ID。`exclude` 属性列出不应公开的端点的 ID。`exclude` 属性优先于 `include` 属性。您可以使用端点 ID 列表配置 `include` 和 `exclude` 属性。

例如，要仅通过 JMX 公开 `health` 和 `info` 端点，请使用以下属性：

- 属性
- YAML

```properties
management.endpoints.jmx.exposure.include=health,info
```

```yaml
management:
  endpoints:
    jmx:
      exposure:
        include: "health,info"
```

`*` 可用于选择所有端点。例如，要通过 HTTP 公开除 `env` 和 `beans` 端点之外的所有内容，请使用以下属性：

- 属性
- YAML

```properties
management.endpoints.web.exposure.include=*
management.endpoints.web.exposure.exclude=env,beans
```

```yaml
management:
  endpoints:
    web:
      exposure:
        include: "*"
        exclude: "env,beans"
```

## 安全性

出于安全考虑，默认情况下，只有 `/health` 端点通过 HTTP 公开。您可以使用 `management.endpoints.web.exposure.include` 属性来配置公开的端点。

如果 Spring Security 位于类路径上并且没有其他 `SecurityFilterChain` bean，则除 `/health` 之外的所有执行器都将通过 Spring Boot 自动配置进行保护。如果您定义自定义 `SecurityFilterChain` bean，Spring Boot 自动配置将后退，让您完全控制执行器访问规则。

典型的 Spring Security 配置可能如下例所示：

- Java
- Kotlin

```java
import org.springframework.boot.actuate.autoconfigure.security.servlet.EndpointRequest;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.security.config.annotation.web.builders.HttpSecurity;
import org.springframework.security.web.SecurityFilterChain;

import static org.springframework.security.config.Customizer.withDefaults;

@Configuration(proxyBeanMethods = false)
public class MySecurityConfiguration {

    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
        http.securityMatcher(EndpointRequest.toAnyEndpoint());
        http.authorizeHttpRequests((requests) -> requests.anyRequest().hasRole("ENDPOINT_ADMIN"));
        http.httpBasic(withDefaults());
        return http.build();
    }

}
```

```kotlin
import org.springframework.boot.actuate.autoconfigure.security.servlet.EndpointRequest
import org.springframework.context.annotation.Bean
import org.springframework.context.annotation.Configuration
import org.springframework.security.config.Customizer.withDefaults
import org.springframework.security.config.annotation.web.builders.HttpSecurity
import org.springframework.security.web.SecurityFilterChain

@Configuration(proxyBeanMethods = false)
class MySecurityConfiguration {

    @Bean
    fun securityFilterChain(http: HttpSecurity): SecurityFilterChain {
        http.securityMatcher(EndpointRequest.toAnyEndpoint())
        http.authorizeHttpRequests { requests ->
            requests.anyRequest().hasRole("ENDPOINT_ADMIN")
        }
        http.httpBasic(withDefaults())
        return http.build()
    }

}
```

## 健康信息

您可以使用健康信息来检查正在运行的应用程序的状态。监控软件经常使用它来在生产系统宕机时提醒某人。`health` 端点公开的信息取决于 `management.endpoint.health.show-details` 和 `management.endpoint.health.show-components` 属性，这些属性可以使用以下值之一进行配置：

| 名称 | 描述 |
| --- | --- |
| `never` | 从不显示详细信息。 |
| `when-authorized` | 仅向授权用户显示详细信息。可以使用 `management.endpoint.health.roles` 配置授权角色。 |
| `always` | 向所有用户显示详细信息。 |

默认值为 `never`。当用户属于端点的一个或多个角色时，则认为该用户已获授权。如果端点没有配置的角色（默认情况下），则所有已认证的用户都被认为已获授权。您可以使用 `management.endpoint.health.roles` 属性配置角色。

### 自动配置的 HealthIndicators

在适当的情况下，Spring Boot 会自动配置下表中列出的 `HealthIndicators`。您也可以通过配置 `management.health.key.enabled` 来启用或禁用选定的指标，其中 `key` 列在下表中：

| 键 | 名称 | 描述 |
| --- | --- | --- |
| `cassandra` | `CassandraDriverHealthIndicator` | 检查 Cassandra 数据库是否启动。 |
| `couchbase` | `CouchbaseHealthIndicator` | 检查 Couchbase 集群是否启动。 |
| `db` | `DataSourceHealthIndicator` | 检查是否可以获得与 `DataSource` 的连接。 |
| `diskspace` | `DiskSpaceHealthIndicator` | 检查磁盘空间是否不足。 |
| `elasticsearch` | `ElasticsearchRestClientHealthIndicator` | 检查 Elasticsearch 集群是否启动。 |
| `hazelcast` | `HazelcastHealthIndicator` | 检查 Hazelcast 服务器是否启动。 |
| `influxdb` | `InfluxDbHealthIndicator` | 检查 InfluxDB 服务器是否启动。 |
| `jms` | `JmsHealthIndicator` | 检查 JMS 代理是否启动。 |
| `ldap` | `LdapHealthIndicator` | 检查 LDAP 服务器是否启动。 |
| `mail` | `MailHealthIndicator` | 检查邮件服务器是否启动。 |
| `mongo` | `MongoHealthIndicator` | 检查 Mongo 数据库是否启动。 |
| `neo4j` | `Neo4jHealthIndicator` | 检查 Neo4j 数据库是否启动。 |
| `ping` | `PingHealthIndicator` | 始终返回 `UP`。 |
| `rabbit` | `RabbitHealthIndicator` | 检查 Rabbit 服务器是否启动。 |
| `redis` | `RedisHealthIndicator` | 检查 Redis 服务器是否启动。 |

### 编写自定义 HealthIndicators

要提供自定义健康信息，您可以注册实现 `HealthIndicator` 接口的 Spring bean。您需要提供 `health()` 方法的实现并返回 `Health` 响应。`Health` 响应应包含状态，并且可以选择包含要显示的其他详细信息。以下代码显示了一个示例 `HealthIndicator` 实现：

- Java
- Kotlin

```java
import org.springframework.boot.actuate.health.Health;
import org.springframework.boot.actuate.health.HealthIndicator;
import org.springframework.stereotype.Component;

@Component
public class MyHealthIndicator implements HealthIndicator {

    @Override
    public Health health() {
        int errorCode = check();
        if (errorCode != 0) {
            return Health.down().withDetail("Error Code", errorCode).build();
        }
        return Health.up().build();
    }

    private int check() {
        // perform some specific health check
        return ...;
    }

}
```

```kotlin
import org.springframework.boot.actuate.health.Health
import org.springframework.boot.actuate.health.HealthIndicator
import org.springframework.stereotype.Component

@Component
class MyHealthIndicator : HealthIndicator {

    override fun health(): Health {
        val errorCode = check()
        if (errorCode != 0) {
            return Health.down().withDetail("Error Code", errorCode).build()
        }
        return Health.up().build()
    }

    private fun check(): Int {
        // perform some specific health check
        return  ...
    }

}
```

## Kubernetes 探针

部署在 Kubernetes 上的应用程序可以使用容器探针提供有关其内部状态的信息。根据您的 Kubernetes 配置，kubelet 会调用这些探针并对结果做出反应。

默认情况下，Spring Boot 管理您的应用程序可用性状态。如果部署在 Kubernetes 环境中，执行器会从 `ApplicationAvailability` 接口收集"Liveness"和"Readiness"信息，并在专用健康指标中使用这些信息：`LivenessStateHealthIndicator` 和 `ReadinessStateHealthIndicator`。这些指标显示在全局健康端点 (`"/actuator/health"`) 上。它们还可以使用健康组作为单独的 HTTP 探针公开：`"/actuator/health/liveness"` 和 `"/actuator/health/readiness"`。

然后，您可以使用以下端点信息配置 Kubernetes 基础设施：

```yaml
livenessProbe:
  httpGet:
    path: "/actuator/health/liveness"
    port: <actuator-port>
  failureThreshold: ...
  periodSeconds: ...

readinessProbe:
  httpGet:
    path: "/actuator/health/readiness"
    port: <actuator-port>
  failureThreshold: ...
  periodSeconds: ...
```

## 应用程序信息

应用程序信息公开从 `ApplicationContext` 中定义的所有 `InfoContributor` bean 收集的各种信息。Spring Boot 包含许多自动配置的 `InfoContributor` bean，您可以编写自己的 bean。

### 自动配置的 InfoContributors

在适当的情况下，Spring 会自动配置以下 `InfoContributor` bean：

| ID | 名称 | 描述 | 前提条件 |
| --- | --- | --- | --- |
| `build` | `BuildInfoContributor` | 公开构建信息。 | 一个 `META-INF/build-info.properties` 资源。 |
| `env` | `EnvironmentInfoContributor` | 公开 `Environment` 中名称以 `info.` 开头的任何属性。 | 无。 |
| `git` | `GitInfoContributor` | 公开 git 信息。 | 一个 `git.properties` 资源。 |
| `java` | `JavaInfoContributor` | 公开 Java 运行时信息。 | 无。 |
| `os` | `OsInfoContributor` | 公开操作系统信息。 | 无。 |
| `process` | `ProcessInfoContributor` | 公开进程信息。 | 无。 |

### 自定义应用程序信息

启用 `env` 贡献者后，您可以通过设置 `info.*` Spring 属性来自定义 `info` 端点公开的数据。`info` 键下的所有 `Environment` 属性都会自动公开。例如，您可以将以下设置添加到您的 `application.properties` 文件中：

- 属性
- YAML

```properties
info.app.encoding=UTF-8
info.app.java.source=17
info.app.java.target=17
```

```yaml
info:
  app:
    encoding: "UTF-8"
    java:
      source: "17"
      target: "17"
```

### Git 提交信息

`info` 端点的另一个有用功能是它能够在构建项目时发布有关您的 `git` 源代码存储库状态的信息。如果 `GitProperties` bean 可用，则可以使用 `info` 端点公开这些属性。

默认情况下，如果存在，端点会公开 `git.branch`、`git.commit.id` 和 `git.commit.time` 属性。如果您不希望端点响应中包含任何这些属性，则需要将它们从 `git.properties` 文件中排除。如果您想显示完整的 git 信息（即 `git.properties` 的完整内容），请使用 `management.info.git.mode` 属性，如下所示：

- 属性
- YAML

```properties
management.info.git.mode=full
```

```yaml
management:
  info:
    git:
      mode: "full"
```
