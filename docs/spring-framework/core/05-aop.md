# Spring 的面向切面编程

面向切面编程 (AOP) 通过提供另一种思考程序结构的方式来补充面向对象编程 (OOP)。OOP 中的模块化关键单元是类，而 AOP 中的模块化单元是 Aspect。Aspect 可以实现关注点（例如事务管理）的模块化，这些关注点贯穿多个类型和对象。（这类关注点在 AOP 文献中通常被称为"横切"关注点。）

Spring 的关键组件之一是 AOP 框架。虽然 Spring IoC 容器不依赖于 AOP（意味着如果你不想使用 AOP，则不需要使用），但 AOP 补充了 Spring IoC，提供了一个非常强大的中间件解决方案。

Spring AOP 与 AspectJ pointcuts

Spring 提供了通过使用基于 schema 的方法或@AspectJ 注解风格来编写自定义 aspect 的简单而强大的方式。这两种风格都提供了完全类型化的 advice，并使用 AspectJ pointcut 语言，同时仍然使用 Spring AOP 进行织入。

本章讨论基于 schema 和 @AspectJ 的 AOP 支持。下一章将讨论更底层的 AOP 支持。

AOP 在 Spring Framework 中用于

- 提供声明式企业服务。其中最重要的服务是声明式事务管理。
- 允许用户实现自定义 aspect，用 AOP 补充他们对 OOP 的使用。

|  |  |
| --- | --- |
|  | 如果你只对通用声明式服务或其他预打包的声明式中间件服务（如连接池）感兴趣，则无需直接使用 Spring AOP，可以跳过本章大部分内容。 |

## 章节摘要

- AOP 概念
- Spring AOP 的能力和目标
- AOP 代理
- @AspectJ 支持
  - 启用 @AspectJ 支持
  - 声明 Aspect
  - 声明 Pointcut
  - 声明 Advice
  - 引介 (Introductions)
  - Aspect 实例化模型
  - AOP 示例
- 基于 Schema 的 AOP 支持
- 选择使用哪种 AOP 声明风格
- 混合 Aspect 类型
- 代理机制
- 编程式创建 @AspectJ 代理
- 在 Spring 应用中使用 AspectJ
- 更多资源

---

*来源：https://docs.springframework.org.cn/spring-framework/reference/core/aop.html*
