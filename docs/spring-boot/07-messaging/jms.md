# JMS (Java Message Service) / JMS 消息服务

Source: https://docs.springframework.org.cn/spring-boot/reference/messaging/jms.html

---

## English

The `jakarta.jms.ConnectionFactory` interface provides a standard method for creating `jakarta.jms.Connection` for interacting with a JMS broker. Although Spring requires a `ConnectionFactory` to work with JMS, you generally need not use it directly but can instead rely on higher level messaging abstractions.

Spring Boot also auto-configures the necessary infrastructure to send and receive messages.

## ActiveMQ "Classic" Support

Spring Boot can configure a `ConnectionFactory` when ActiveMQ "Classic" is present on the classpath.

|  |  |
| --- | --- |
|  | If you use `spring-boot-starter-activemq`, the necessary dependencies to connect to an ActiveMQ "Classic" instance are provided, as well as the Spring infrastructure to integrate with JMS. |

Configuration of ActiveMQ "Classic" is controlled by external configuration properties in `spring.activemq.*`. By default, ActiveMQ "Classic" is auto-configured to use the TCP transport and defaults to connecting to `tcp://127.0.0.1:61616`.

You can specify the broker URL, user, and password as shown in the following example:

- Properties
- YAML

```properties
spring.activemq.broker-url=tcp://192.168.1.210:9876
spring.activemq.user=admin
spring.activemq.password=secret
```

```yaml
spring:
  activemq:
    broker-url: "tcp://192.168.1.210:9876"
    user: "admin"
    password: "secret"
```

By default, a `CachingConnectionFactory` wraps the native `ConnectionFactory` with sensible settings that you can control by external configuration properties in `spring.jms.*`.

## ActiveMQ Artemis Support

Spring Boot can auto-configure a `ConnectionFactory` when it detects that ActiveMQ Artemis is present on the classpath. If a broker is present, an embedded broker is automatically started and configured (unless the mode property has been explicitly set).

|  |  |
| --- | --- |
|  | If you use `spring-boot-starter-artemis`, the necessary dependencies to connect to an existing ActiveMQ Artemis instance are provided, as well as the Spring infrastructure to integrate with JMS. Adding `org.apache.activemq:artemis-jakarta-server` to your application lets you use embedded mode. |

ActiveMQ Artemis configuration is controlled by external configuration properties in `spring.artemis.*`.

- Properties
- YAML

```properties
spring.artemis.mode=native
spring.artemis.broker-url=tcp://192.168.1.210:9876
spring.artemis.user=admin
spring.artemis.password=secret
```

```yaml
spring:
  artemis:
    mode: native
    broker-url: "tcp://192.168.1.210:9876"
    user: "admin"
    password: "secret"
```

## Using JNDI ConnectionFactory

If you run your application in an application server, Spring Boot tries to locate a JMS `ConnectionFactory` by using JNDI. By default, the `java:/JmsXA` and `java:/XAConnectionFactory` locations are checked. If you need to specify an alternative location, you can use the `spring.jms.jndi-name` property.

- Properties
- YAML

```properties
spring.jms.jndi-name=java:/MyConnectionFactory
```

```yaml
spring:
  jms:
    jndi-name: "java:/MyConnectionFactory"
```

## Sending Messages

Spring's `JmsTemplate` is auto-configured, and you can autowire it directly into your own beans, as shown in the following example:

- Java
- Kotlin

```java
import org.springframework.jms.core.JmsTemplate;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final JmsTemplate jmsTemplate;

    public MyBean(JmsTemplate jmsTemplate) {
        this.jmsTemplate = jmsTemplate;
    }

    // ...
    public void someMethod() {
        this.jmsTemplate.convertAndSend("hello");
    }

}
```

```kotlin
import org.springframework.jms.core.JmsTemplate
import org.springframework.stereotype.Component

@Component
class MyBean(private val jmsTemplate: JmsTemplate) {

    // ...
    fun someMethod() {
        jmsTemplate.convertAndSend("hello")
    }

}
```

## Receiving Messages

When the JMS infrastructure is present, any bean can be annotated with `@JmsListener` to create a listener endpoint. If no `JmsListenerContainerFactory` is defined, a default one is configured automatically.

The following component creates a listener endpoint on the `someQueue` destination:

- Java
- Kotlin

```java
import org.springframework.jms.annotation.JmsListener;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    @JmsListener(destination = "someQueue")
    public void processMessage(String content) {
        // ...
    }

}
```

```kotlin
import org.springframework.jms.annotation.JmsListener
import org.springframework.stereotype.Component

@Component
class MyBean {

    @JmsListener(destination = "someQueue")
    fun processMessage(content: String?) {
        // ...
    }

}
```

---

## 中文 / Chinese

`jakarta.jms.ConnectionFactory` 接口提供了一种创建 `jakarta.jms.Connection` 的标准方法，用于与 JMS 代理进行交互。尽管 Spring 需要 `ConnectionFactory` 来使用 JMS，但通常您无需直接使用它，而是可以依赖更高级别的消息传递抽象。

Spring Boot 还自动配置发送和接收消息所需的必要基础设施。

## ActiveMQ "Classic" 支持

如果类路径上存在 ActiveMQ "Classic"，Spring Boot 就可以配置一个 `ConnectionFactory`。

|  |  |
| --- | --- |
|  | 如果您使用 `spring-boot-starter-activemq`，则会提供连接到 ActiveMQ "Classic" 实例所需的依赖项，以及与 JMS 集成的 Spring 基础设施。 |

ActiveMQ "Classic" 的配置由 `spring.activemq.*` 中的外部配置属性控制。默认情况下，ActiveMQ "Classic" 会自动配置为使用 TCP 传输，并默认连接到 `tcp://127.0.0.1:61616`。

- 属性
- YAML

```properties
spring.activemq.broker-url=tcp://192.168.1.210:9876
spring.activemq.user=admin
spring.activemq.password=secret
```

```yaml
spring:
  activemq:
    broker-url: "tcp://192.168.1.210:9876"
    user: "admin"
    password: "secret"
```

## ActiveMQ Artemis 支持

当 Spring Boot 检测到类路径上存在 ActiveMQ Artemis 时，它可以自动配置一个 `ConnectionFactory`。如果代理存在，则会自动启动并配置一个嵌入式代理。

|  |  |
| --- | --- |
|  | 如果您使用 `spring-boot-starter-artemis`，则会提供连接到现有 ActiveMQ Artemis 实例所需的依赖项，以及与 JMS 集成的 Spring 基础设施。将 `org.apache.activemq:artemis-jakarta-server` 添加到您的应用程序中，可以让您使用嵌入模式。 |

ActiveMQ Artemis 的配置由 `spring.artemis.*` 中的外部配置属性控制。

- 属性
- YAML

```properties
spring.artemis.mode=native
spring.artemis.broker-url=tcp://192.168.1.210:9876
spring.artemis.user=admin
spring.artemis.password=secret
```

```yaml
spring:
  artemis:
    mode: native
    broker-url: "tcp://192.168.1.210:9876"
    user: "admin"
    password: "secret"
```

## 使用 JNDI ConnectionFactory

如果您在应用程序服务器中运行应用程序，Spring Boot 会尝试使用 JNDI 定位 JMS `ConnectionFactory`。默认情况下，会检查 `java:/JmsXA` 和 `java:/XAConnectionFactory` 位置。如果您需要指定备用位置，可以使用 `spring.jms.jndi-name` 属性。

- 属性
- YAML

```properties
spring.jms.jndi-name=java:/MyConnectionFactory
```

```yaml
spring:
  jms:
    jndi-name: "java:/MyConnectionFactory"
```

## 发送消息

Spring 的 `JmsTemplate` 会自动配置，您可以将其直接自动装配到您自己的 Bean 中。

- Java
- Kotlin

```java
import org.springframework.jms.core.JmsTemplate;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final JmsTemplate jmsTemplate;

    public MyBean(JmsTemplate jmsTemplate) {
        this.jmsTemplate = jmsTemplate;
    }

    // ...
    public void someMethod() {
        this.jmsTemplate.convertAndSend("hello");
    }

}
```

```kotlin
import org.springframework.jms.core.JmsTemplate
import org.springframework.stereotype.Component

@Component
class MyBean(private val jmsTemplate: JmsTemplate) {

    // ...
    fun someMethod() {
        jmsTemplate.convertAndSend("hello")
    }

}
```

## 接收消息

当 JMS 基础设施存在时，任何 Bean 都可以使用 `@JmsListener` 注解来创建侦听器端点。如果没有定义 `JmsListenerContainerFactory`，则会自动配置一个默认的。

以下组件在 `someQueue` 目标上创建侦听器端点：

- Java
- Kotlin

```java
import org.springframework.jms.annotation.JmsListener;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    @JmsListener(destination = "someQueue")
    public void processMessage(String content) {
        // ...
    }

}
```

```kotlin
import org.springframework.jms.annotation.JmsListener
import org.springframework.stereotype.Component

@Component
class MyBean {

    @JmsListener(destination = "someQueue")
    fun processMessage(content: String?) {
        // ...
    }

}
```
