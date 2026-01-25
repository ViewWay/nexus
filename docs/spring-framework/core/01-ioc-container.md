# IoC 容器

本章涵盖 Spring 的控制反转 (IoC) 容器。

## 章节摘要

- Spring IoC 容器和 Bean 简介
- 容器概述
- Bean 概述
- 依赖项
  - 依赖注入
  - 依赖项与配置详解
  - 使用 `depends-on`
  - 延迟初始化 Bean
  - 自动装配协作者
  - 方法注入
- Bean 作用域
- 定制 Bean 的特性
- Bean 定义继承
- 容器扩展点
- 基于注解的容器配置
  - 使用 `@Autowired`
  - 使用 `@Primary` 或 `@Fallback` 微调基于注解的自动装配
  - 使用 Qualifiers 微调基于注解的自动装配
  - 使用泛型作为自动装配限定符
  - 使用 `CustomAutowireConfigurer`
  - 使用 `@Resource` 进行注入
  - 使用 `@Value`
  - 使用 `@PostConstruct` 和 `@PreDestroy`
- 类路径扫描和托管组件
- 使用 JSR 330 标准注解
- 基于 Java 的容器配置
  - 基本概念：`@Bean` 和 `@Configuration`
  - 使用 `@AnnotationConfigApplicationContext` 实例化 Spring 容器
  - 使用 `@Bean` 注解
  - 使用 `@Configuration` 注解
  - 组合基于 Java 的配置
- 环境抽象
- 注册 `LoadTimeWeaver`
- `ApplicationContext` 的附加功能
- `BeanFactory` API

## Spring IoC 容器和 Bean 简介

本节介绍控制反转 (IoC) 原则的 Spring Framework 实现。IoC 也称为依赖注入 (DI)。这是一个过程，其中对象仅通过构造函数参数、工厂方法的参数或在对象实例被构造或从工厂方法返回后在其上设置的属性来定义它们的依赖项（即与它们一起工作的其他对象）。然后容器在创建 bean 时注入这些依赖项。这个过程基本上是 bean 本身的逆过程（因此称为控制反转），通过使用直接构造类或诸如服务定位器模式之类的机制来控制其依赖项的实例化或位置的实例化。

`org.springframework.beans` 和 `org.springframework.context` 包是 Spring Framework 的 IoC 容器的基础。`BeanFactory` 接口提供了一种高级配置机制，能够管理任何类型的对象。`ApplicationContext` 是 `BeanFactory` 的子接口。它添加了：

- 更容易与 Spring 的 AOP 功能集成
- 消息资源处理（用于国际化）
- 事件发布
- 应用层特定上下文，例如用于 Web 应用程序的 `WebApplicationContext`

简而言之，`BeanFactory` 提供了配置框架和基本功能，而 `ApplicationContext` 添加了更多特定于企业的功能。`ApplicationContext` 是 `BeanFactory` 的完整超集，在本章对 IoC 容器的描述中专门使用。

---

*来源：https://docs.springframework.org.cn/spring-framework/reference/core/beans.html*
