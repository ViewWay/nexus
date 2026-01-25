# JMS (Java 消息服务)

Spring 提供了一个 JMS 集成框架，以类似于 Spring 对 JDBC API 集成的方式，简化了 JMS API 的使用。

JMS 大致可以分为两个功能领域：消息的生产和消费。`JmsTemplate` 类用于消息生产和同步消息接收。对于类似于 Jakarta EE 的消息驱动 Bean 风格的异步接收，Spring 提供了许多消息监听容器，您可以使用它们来创建消息驱动 POJO (MDP)。Spring 还提供了创建消息监听器的声明式方式。

`org.springframework.jms.core` 包提供了使用 JMS 的核心功能。它包含 JMS 模板类，通过处理资源的创建和释放来简化 JMS 的使用，就像 `JdbcTemplate` 对 JDBC 所做的那样。Spring 模板类的通用设计原则是提供辅助方法来执行常见操作，并且对于更复杂的用法，将处理任务的本质委托给用户实现的 callback 接口。JMS 模板遵循相同的设计。这些类提供了各种便捷方法来发送消息、同步消费消息，并将 JMS session 和消息生产者暴露给用户。

`org.springframework.jms.support` 包提供了 `JMSException` 转换功能。该转换将受检的 `JMSException` 层级转换为对应的非受检异常层级。如果存在任何提供商特定的受检 `jakarta.jms.JMSException` 子类，该异常将被包装在非受检的 `UncategorizedJmsException` 中。

`org.springframework.jms.support.converter` 包提供了 `MessageConverter` 抽象，用于在 Java 对象和 JMS 消息之间进行转换。

`org.springframework.jms.support.destination` 包提供了管理 JMS 目的地（destination）的各种策略，例如为存储在 JNDI 中的目的地提供服务定位器（service locator）。

`org.springframework.jms.annotation` 包提供了必要的基础设施，以支持使用 `@JmsListener` 的注解驱动监听器端点。

`org.springframework.jms.config` 包提供了 `jms` 命名空间的解析器实现，以及配置监听器容器和创建监听器端点的 Java config 支持。

最后，`org.springframework.jms.connection` 包提供了适合在独立应用程序中使用的 `ConnectionFactory` 实现。它还包含 Spring 的 JMS `PlatformTransactionManager` 的实现（巧妙地命名为 `JmsTransactionManager`）。这使得 JMS 作为一个事务资源可以无缝集成到 Spring 的事务管理机制中。

> 从 Spring Framework 5 开始，Spring 的 JMS 包完全支持 JMS 2.0，并且在运行时需要存在 JMS 2.0 API。我们建议使用兼容 JMS 2.0 的提供商。如果您的系统恰好使用较旧的消息 broker，您可以尝试升级到与现有 broker 版本兼容的 JMS 2.0 驱动程序。或者，您也可以尝试使用基于 JMS 1.1 的驱动程序，只需将 JMS 2.0 API jar 放在类路径中，但只针对您的驱动程序使用与 JMS 1.1 兼容的 API。Spring 的 JMS 支持默认遵循 JMS 1.1 约定，因此通过相应的配置确实支持这种情况。但是，请仅将此视为过渡场景。

## 使用 Spring JMS

Spring 框架的 JMS 集成提供了发送和接收消息的抽象。

### 发送消息

使用 `JmsTemplate` 发送消息非常简单：

```java
import jakarta.jms.JMSException;
import jakarta.jms.Message;
import jakarta.jms.Session;
import org.springframework.jms.core.JmsTemplate;
import org.springframework.jms.core.MessageCreator;

public class JmsSender {

    private final JmsTemplate jmsTemplate;

    public JmsSender(JmsTemplate jmsTemplate) {
        this.jmsTemplate = jmsTemplate;
    }

    public void sendMessage(final String message) {
        jmsTemplate.send("queueName", new MessageCreator() {
            @Override
            public Message createMessage(Session session) throws JMSException {
                return session.createTextMessage(message);
            }
        });
    }
}
```

### 接收消息

使用 `JmsTemplate` 同步接收消息：

```java
public class JmsReceiver {

    private final JmsTemplate jmsTemplate;

    public JmsReceiver(JmsTemplate jmsTemplate) {
        this.jmsTemplate = jmsTemplate;
    }

    public String receiveMessage() {
        return (String) jmsTemplate.receiveAndConvert("queueName");
    }
}
```

## JCA 消息端点支持

Spring 提供了对 JCA (Java Connector Architecture) 消息端点的支持，允许您在应用服务器环境中使用 JMS 资源适配器。

## 注解驱动的监听器端点

使用 `@JmsListener` 注解可以轻松创建消息监听器端点：

```java
import org.springframework.jms.annotation.JmsListener;
import org.springframework.stereotype.Component;

@Component
public class MessageListener {

    @JmsListener(destination = "queueName")
    public void receiveMessage(String message) {
        System.out.println("Received message: " + message);
    }
}
```

要启用 `@JmsListener` 支持，需要在配置类上添加 `@EnableJms` 注解：

```java
import org.springframework.context.annotation.Configuration;
import org.springframework.jms.annotation.EnableJms;

@Configuration
@EnableJms
public class JmsConfig {
    // 配置 JMS 相关的 Bean
}
```

## JMS 命名空间支持

Spring 提供了 JMS 命名空间，可以简化 XML 配置：

```xml
<beans xmlns="http://www.springframework.org/schema/beans"
       xmlns:jms="http://www.springframework.org/schema/jms"
       xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
       xsi:schemaLocation="
           http://www.springframework.org/schema/beans
           https://www.springframework.org/schema/beans/spring-beans.xsd
           http://www.springframework.org/schema/jms
           https://www.springframework.org/schema/jms/spring-jms.xsd">

    <jms:listener-container connection-factory="connectionFactory">
        <jms:listener destination="queueName" ref="messageListener" method="receiveMessage"/>
    </jms:listener-container>

</beans>
```
