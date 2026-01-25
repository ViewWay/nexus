# JMX

Spring 中的 JMX (Java Management Extensions) 支持提供了特性，可让你轻松透明地将 Spring 应用集成到 JMX 基础设施中。

具体来说，Spring 的 JMX 支持提供了四个核心特性：

- 将任何 Spring bean 自动注册为 JMX MBean。
- 一个灵活的机制，用于控制你的 bean 的管理接口。
- 通过远程 JSR-160 连接器声明式地暴露 MBean。
- 对本地和远程 MBean 资源进行简单的代理。

这些特性旨在工作，而无需将你的应用组件耦合到 Spring 或 JMX 接口和类。事实上，在大多数情况下，你的应用类无需了解 Spring 或 JMX 即可利用 Spring JMX 特性。

## 将 Bean 导出到 JMX

Spring 的核心 JMX 支持在于 `MBeanExporter` 类，它负责将 Spring bean 导出到 JMX `MBeanServer`。

### 基本配置

```java
import org.springframework.jmx.export.MBeanExporter;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration
public class JmxConfig {

    @Bean
    public MBeanExporter mBeanExporter() {
        MBeanExporter exporter = new MBeanExporter();
        exporter.setBeans(Map.of(
            "bean:name=myBean", myBean()
        ));
        return exporter;
    }

    @Bean
    public MyBean myBean() {
        return new MyBean();
    }
}
```

### 自动检测

Spring 可以自动检测应该导出为 MBean 的 bean：

```java
@Bean
public MBeanExporter mBeanExporter() {
    MBeanExporter exporter = new MBeanExporter();
    exporter.setAutodetect(true);
    return exporter;
}
```

## 控制 Bean 的管理接口

Spring 提供了多种方式来控制暴露给 JMX 的方法和属性。

### 使用接口

```java
public interface MyBeanMBean {
    String getName();
    void setName(String name);
    int getCounter();
    void reset();
}

@Component("myBean")
public class MyBean implements MyBeanMBean {
    // 实现方法
}
```

### 使用方法名称

```java
@Bean
public MBeanExporter mBeanExporter() {
    MBeanExporter exporter = new MBeanExporter();
    exporter.setAssembler(newMethodNameBasedMBeanInfoAssembler(Set.of("getName", "setName")));
    return exporter;
}
```

### 使用注解

```java
import org.springframework.jmx.export.annotation.ManagedResource;
import org.springframework.jmx.export.annotation.ManagedAttribute;
import org.springframework.jmx.export.annotation.ManagedOperation;

@ManagedResource(objectName = "bean:name=myBean")
public class MyBean {

    private String name;

    @ManagedAttribute
    public String getName() {
        return name;
    }

    @ManagedAttribute
    public void setName(String name) {
        this.name = name;
    }

    @ManagedOperation
    public void reset() {
        // 重置操作
    }
}
```

要启用注解支持：

```java
@Bean
public AnnotationMBeanExporter annotationMBeanExporter() {
    AnnotationMBeanExporter exporter = new AnnotationMBeanExporter();
    exporter.setAutodetect(true);
    return exporter;
}
```

## 控制 Bean 的 `ObjectName` 实例

`ObjectName` 是 MBean 的唯一标识符。Spring 提供了多种方式来控制它。

### 手动指定

```java
@Bean
public MBeanExporter mBeanExporter() {
    MBeanExporter exporter = new MBeanExporter();
    exporter.setBeans(Map.of(
        "bean:name=myBean", myBean()
    ));
    return exporter;
}
```

### 使用注解

```java
@ManagedResource(objectName = "com.example:type=MyBean,name=example")
public class MyBean {
    // ...
}
```

### 使用命名策略

```java
@Bean
public MBeanExporter mBeanExporter() {
    MBeanExporter exporter = new MBeanExporter();
    exporter.setNamingStrategy(new KeyNamingStrategy());
    exporter.setBeans(Map.of("myBean", myBean()));
    return exporter;
}
```

## 使用 JSR-160 连接器

Spring 支持通过 JSR-160 JMX 连接器远程访问 MBean。

### 配置连接器

```java
@Bean
public JMXConnectorServer jmxConnectorServer() throws IOException {
    JMXServiceURL url = new JMXServiceURL("service:jmx:rmi:///jndi/rmi://localhost:1099/jmxrmi");
    RMIConnectorServer server = new RMIConnectorServer(url, null, mbeanServer());
    server.start();
    return server;
}
```

## 通过代理访问 MBean

Spring 可以创建 MBean 的代理，允许你像使用普通对象一样使用 MBean：

```java
@Bean
public MyBeanMBean myBeanProxy() {
    return (MyBeanMBean) MBeanServerConnectionFactoryBean
        .createProxy(mbeanServer(), "bean:name=myBean", MyBeanMBean.class);
}
```

## 通知

Spring JMX 支持还提供了发送和接收 JMX 通知的功能。

### 发送通知

```java
import org.springframework.jmx.export.notification.NotificationPublisher;
import org.springframework.jmx.export.notification.NotificationPublisherAware;

@Component
public class MyBean implements NotificationPublisherAware {

    private NotificationPublisher notificationPublisher;

    @Override
    public void setNotificationPublisher(NotificationPublisher notificationPublisher) {
        this.notificationPublisher = notificationPublisher;
    }

    public void doSomething() {
        // 执行操作
        notificationPublisher.sendNotification(
            new Notification("my.event", this, System.currentTimeMillis())
        );
    }
}
```

### 接收通知

```java
@Component
public class MyNotificationListener implements NotificationListener {

    @Override
    public void handleNotification(Notification notification, Object handback) {
        System.out.println("Received notification: " + notification);
    }
}
```
