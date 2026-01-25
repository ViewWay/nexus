# AMQP and RabbitMQ / AMQP 和 RabbitMQ

Source: https://docs.springframework.org.cn/spring-boot/reference/messaging/amqp.html

---

## English

The Advanced Message Queuing Protocol (AMQP) is a platform-neutral, wire-level protocol for message-oriented middleware. The Spring AMQP project applies core Spring concepts to the development of AMQP-based messaging solutions. Spring Boot provides several conveniences for working with AMQP through RabbitMQ, including the `spring-boot-starter-amqp` starter.

## RabbitMQ Support

RabbitMQ is a lightweight, reliable, scalable, and portable message broker based on the AMQP protocol. Spring uses RabbitMQ to communicate through the AMQP protocol.

RabbitMQ configuration is controlled by external configuration properties in `spring.rabbitmq.*`. For example, you might declare the following section in your `application.properties`:

- Properties
- YAML

```properties
spring.rabbitmq.host=localhost
spring.rabbitmq.port=5672
spring.rabbitmq.username=admin
spring.rabbitmq.password=secret
```

```yaml
spring:
  rabbitmq:
    host: "localhost"
    port: 5672
    username: "admin"
    password: "secret"
```

Alternatively, you can configure the same connection using the `addresses` property:

- Properties
- YAML

```properties
spring.rabbitmq.addresses=amqp://admin:secret@localhost
```

```yaml
spring:
  rabbitmq:
    addresses: "amqp://admin:secret@localhost"
```

## Sending Messages

Spring's `AmqpTemplate` and `AmqpAdmin` are auto-configured, and you can autowire them directly into your own beans, as shown in the following example:

- Java
- Kotlin

```java
import org.springframework.amqp.core.AmqpAdmin;
import org.springframework.amqp.core.AmqpTemplate;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final AmqpAdmin amqpAdmin;
    private final AmqpTemplate amqpTemplate;

    public MyBean(AmqpAdmin amqpAdmin, AmqpTemplate amqpTemplate) {
        this.amqpAdmin = amqpAdmin;
        this.amqpTemplate = amqpTemplate;
    }

    // ...
    public void someMethod() {
        this.amqpAdmin.getQueueInfo("someQueue");
    }

    public void someOtherMethod() {
        this.amqpTemplate.convertAndSend("hello");
    }

}
```

```kotlin
import org.springframework.amqp.core.AmqpAdmin
import org.springframework.amqp.core.AmqpTemplate
import org.springframework.stereotype.Component

@Component
class MyBean(private val amqpAdmin: AmqpAdmin, private val amqpTemplate: AmqpTemplate) {

    // ...
    fun someMethod() {
        amqpAdmin.getQueueInfo("someQueue")
    }

    fun someOtherMethod() {
        amqpTemplate.convertAndSend("hello")
    }

}
```

To retry operations (for example, in the event of broker connection loss), you can enable retries on the `AmqpTemplate`:

- Properties
- YAML

```properties
spring.rabbitmq.template.retry.enabled=true
spring.rabbitmq.template.retry.initial-interval=2s
```

```yaml
spring:
  rabbitmq:
    template:
      retry:
        enabled: true
        initial-interval: "2s"
```

Retries are disabled by default. You can also customize the `RetryTemplate` programmatically by declaring a `RabbitRetryTemplateCustomizer` bean.

## Receiving Messages

When Rabbit infrastructure is present, any bean can be annotated with `@RabbitListener` to create a listener endpoint. If no `RabbitListenerContainerFactory` is defined, a default `SimpleRabbitListenerContainerFactory` is auto-configured.

The following example component creates a listener endpoint on the `someQueue` queue:

- Java
- Kotlin

```java
import org.springframework.amqp.rabbit.annotation.RabbitListener;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    @RabbitListener(queues = "someQueue")
    public void processMessage(String content) {
        // ...
    }

}
```

```kotlin
import org.springframework.amqp.rabbit.annotation.RabbitListener
import org.springframework.stereotype.Component

@Component
class MyBean {

    @RabbitListener(queues = ["someQueue"])
    fun processMessage(content: String?) {
        // ...
    }

}
```

---

## 中文 / Chinese

高级消息队列协议 (AMQP) 是一种与平台无关的、用于面向消息的中间件的线级协议。Spring AMQP 项目将核心 Spring 概念应用于基于 AMQP 的消息传递解决方案的开发。Spring Boot 通过 RabbitMQ 提供了若干用于处理 AMQP 的便利功能，包括 `spring-boot-starter-amqp` 启动器。

## RabbitMQ 支持

RabbitMQ 是一个基于 AMQP 协议的轻量级、可靠、可扩展和可移植的消息代理。Spring 使用 RabbitMQ 通过 AMQP 协议进行通信。

RabbitMQ 配置由 `spring.rabbitmq.*` 中的外部配置属性控制。例如，您可以在 `application.properties` 中声明以下部分：

- 属性
- YAML

```properties
spring.rabbitmq.host=localhost
spring.rabbitmq.port=5672
spring.rabbitmq.username=admin
spring.rabbitmq.password=secret
```

```yaml
spring:
  rabbitmq:
    host: "localhost"
    port: 5672
    username: "admin"
    password: "secret"
```

或者，您可以使用 `addresses` 属性配置相同的连接：

- 属性
- YAML

```properties
spring.rabbitmq.addresses=amqp://admin:secret@localhost
```

```yaml
spring:
  rabbitmq:
    addresses: "amqp://admin:secret@localhost"
```

## 发送消息

Spring 的 `AmqpTemplate` 和 `AmqpAdmin` 是自动配置的，您可以将它们直接自动装配到您自己的 bean 中。

- Java
- Kotlin

```java
import org.springframework.amqp.core.AmqpAdmin;
import org.springframework.amqp.core.AmqpTemplate;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final AmqpAdmin amqpAdmin;
    private final AmqpTemplate amqpTemplate;

    public MyBean(AmqpAdmin amqpAdmin, AmqpTemplate amqpTemplate) {
        this.amqpAdmin = amqpAdmin;
        this.amqpTemplate = amqpTemplate;
    }

    // ...
    public void someMethod() {
        this.amqpAdmin.getQueueInfo("someQueue");
    }

    public void someOtherMethod() {
        this.amqpTemplate.convertAndSend("hello");
    }

}
```

```kotlin
import org.springframework.amqp.core.AmqpAdmin
import org.springframework.amqp.core.AmqpTemplate
import org.springframework.stereotype.Component

@Component
class MyBean(private val amqpAdmin: AmqpAdmin, private val amqpTemplate: AmqpTemplate) {

    // ...
    fun someMethod() {
        amqpAdmin.getQueueInfo("someQueue")
    }

    fun someOtherMethod() {
        amqpTemplate.convertAndSend("hello")
    }

}
```

要对 `AmqpTemplate` 进行重试操作（例如，在代理连接丢失的情况下），您可以启用重试：

- 属性
- YAML

```properties
spring.rabbitmq.template.retry.enabled=true
spring.rabbitmq.template.retry.initial-interval=2s
```

```yaml
spring:
  rabbitmq:
    template:
      retry:
        enabled: true
        initial-interval: "2s"
```

默认情况下禁用重试。您还可以通过声明 `RabbitRetryTemplateCustomizer` bean 以编程方式自定义 `RetryTemplate`。

## 接收消息

当 Rabbit 基础设施存在时，任何 bean 都可以使用 `@RabbitListener` 进行注释以创建侦听器端点。如果未定义 `RabbitListenerContainerFactory`，则会自动配置一个默认的 `SimpleRabbitListenerContainerFactory`。

以下示例组件在 `someQueue` 队列上创建了一个侦听器端点：

- Java
- Kotlin

```java
import org.springframework.amqp.rabbit.annotation.RabbitListener;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    @RabbitListener(queues = "someQueue")
    public void processMessage(String content) {
        // ...
    }

}
```

```kotlin
import org.springframework.amqp.rabbit.annotation.RabbitListener
import org.springframework.stereotype.Component

@Component
class MyBean {

    @RabbitListener(queues = ["someQueue"])
    fun processMessage(content: String?) {
        // ...
    }

}
```
