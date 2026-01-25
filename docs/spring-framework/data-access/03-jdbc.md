# 使用 JDBC 访问数据

Data Access with JDBC

Spring Framework JDBC 抽象提供的价值也许最好通过下表中概述的操作序列来展示。该表显示了哪些操作由 Spring 处理，哪些操作是您的责任。

## Spring JDBC - 职责划分

| 操作 | Spring | 您 |
|------|--------|-----|
| 定义连接参数。 | | X |
| 打开连接。 | X | |
| 指定 SQL 语句。 | | X |
| 声明参数并提供参数值 | | X |
| 准备并运行语句。 | X | |
| 设置循环迭代结果集（如果存在）。 | X | |
| 为每次迭代执行工作。 | | X |
| 处理任何异常。 | X | |
| 处理事务。 | X | |
| 关闭连接、语句和结果集。 | X | |

Spring Framework 负责所有可能使 JDBC 成为繁琐 API 的底层细节。

## 本节内容

### 选择 JDBC 数据库访问方式

选择适合您应用的 JDBC 数据库访问方法。

### 包层级

了解 JDBC 相关类的包组织结构。

### 使用 JDBC 核心类控制基本 JDBC 处理和错误处理

使用 `JdbcTemplate` 和相关类来简化 JDBC 操作和错误处理。

### 控制数据库连接

学习如何管理和控制数据库连接。

### JDBC 批量操作

了解如何执行高效的批量操作。

### 使用 `SimpleJdbc` 类简化 JDBC 操作

使用 `SimpleJdbcInsert`、`SimpleJdbcCall` 等类简化常见的 JDBC 操作。

### 将 JDBC 操作建模为 Java 对象

将 JDBC 操作封装为可重用的 Java 对象。

### 参数和数据值处理的常见问题

解决参数绑定和数据值处理中的常见问题。

### 嵌入式数据库支持

使用 H2、HSQL、Derby 等嵌入式数据库进行开发和测试。

### 初始化 `DataSource`

了解如何配置和初始化 `DataSource`。

---

*来源: [Spring Framework 官方文档](https://docs.springframework.org.cn/spring-framework/reference/data-access/jdbc.html)*
