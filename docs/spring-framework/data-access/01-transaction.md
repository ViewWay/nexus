# 事务管理

Transaction Management

全面的事务支持是使用 Spring Framework 的最令人信服的理由之一。Spring Framework 提供了一致的事务管理抽象，带来了以下优势：

- 跨不同事务 API 的一致编程模型，例如 Java Transaction API (JTA)、JDBC、Hibernate 和 Java Persistence API (JPA)。
- 支持声明式事务管理。
- 相比复杂的事务 API（例如 JTA），提供了更简单的用于编程式事务管理的 API。
- 与 Spring 的数据访问抽象有出色的集成。

## 本节内容

以下章节描述了 Spring Framework 的事务特性和技术：

1. **Spring Framework 事务支持模型的优势** - 描述了为什么你应该使用 Spring Framework 的事务抽象，而不是 EJB 容器管理事务 (CMT) 或通过专有 API 驱动事务。

2. **理解 Spring Framework 事务抽象** - 概述了核心类，并描述了如何从各种来源配置和获取 `DataSource` 实例。

3. **将资源与事务同步** - 描述了应用程序代码如何确保资源被正确创建、重用和清理。

4. **声明式事务管理** - 描述了对声明式事务管理的支持：
   - 理解 Spring Framework 的声明式事务实现
   - 声明式事务实现示例
   - 回滚声明式事务
   - 为不同的 Bean 配置不同的事务语义
   - <tx:advice/> 设置
   - 使用 `@Transactional`
   - 事务传播
   - 通知事务操作
   - 将 `@Transactional` 与 AspectJ 结合使用

5. **编程式事务管理** - 涵盖了对编程式（即显式编码）事务管理的支持。

6. **选择编程式还是声明式事务管理** - 指导如何选择合适的事务管理方式。

7. **事务绑定事件** - 描述了如何在事务中使用应用程序事件。

8. **应用程序服务器特定集成** - 介绍与应用程序服务器的集成。

9. **常见问题解决方案** - 提供常见问题的解决方案。

10. **更多资源** - 提供额外的学习资源。

---

*来源: [Spring Framework 官方文档](https://docs.springframework.org.cn/spring-framework/reference/data-access/transaction.html)*
