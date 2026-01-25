# JDBC Test Support / JDBC 测试支持

> Source: https://docs.springframework.org.cn/spring-framework/reference/testing/support-jdbc.html

## JdbcTestUtils

The `org.springframework.test.jdbc` package contains `JdbcTestUtils`, which is a collection of JDBC-related utility functions designed to simplify standard database testing scenarios. Specifically, `JdbcTestUtils` provides the following static utility methods.

`org.springframework.test.jdbc` 包包含 `JdbcTestUtils`，它是一系列 JDBC 相关的工具函数，旨在简化标准的数据库测试场景。具体而言，`JdbcTestUtils` 提供了以下静态工具方法。

- `countRowsInTable(..)`: Counts the number of rows in the given table.
  计算给定表中的行数。

- `countRowsInTableWhere(..)`: Counts the number of rows in the given table using the provided `WHERE` clause.
  使用提供的 `WHERE` 子句计算给定表中的行数。

- `deleteFromTables(..)`: Deletes all rows from the specified tables.
  删除指定表中的所有行。

- `deleteFromTableWhere(..)`: Deletes rows from the given table using the provided `WHERE` clause.
  使用提供的 `WHERE` 子句删除给定表中的行。

- `dropTables(..)`: Drops the specified tables.
  删除指定的表。

| `AbstractTransactionalJUnit4SpringContextTests` and `AbstractTransactionalTestNGSpringContextTests` provide convenience methods that delegate to the above methods in `JdbcTestUtils`. |
| --- |

| `AbstractTransactionalJUnit4SpringContextTests` 和 `AbstractTransactionalTestNGSpringContextTests` 提供了便捷方法，它们委托给 `JdbcTestUtils` 中提及的方法。 |

## Embedded Databases / 嵌入式数据库

The `spring-jdbc` module provides support for configuring and launching an embedded database, which you can use in integration tests that interact with a database. For details, see Embedded Database Support and Testing Data Access Logic with an Embedded Database.

`spring-jdbc` 模块提供了配置和启动嵌入式数据库的支持，您可以在与数据库交互的集成测试中使用它。详细信息请参阅 嵌入式数据库支持 和 使用嵌入式数据库测试数据访问逻辑。

---

**Related Topics / 相关主题**

- Integration Testing
- Spring TestContext Framework
