# 使用 R2DBC 进行数据访问

Data Access with R2DBC

R2DBC（Reactive Relational Database Connectivity）是 Spring Framework 提供的响应式数据库访问方式，允许以非阻塞的方式与关系型数据库进行交互。

## 使用 DatabaseClient

`DatabaseClient` 是 R2DBC 核心包中的中心类。它负责资源的创建和释放，有助于避免常见的错误，例如忘记关闭连接。

### DatabaseClient 的功能

- 运行 SQL 查询
- 更新语句和存储过程调用
- 对 `Result` 实例执行迭代
- 捕获 R2DBC 异常并将其转换为在 `org.springframework.dao` 包中定义的通用、信息量更大的异常层级结构

该客户端具有函数式、流式的 API，使用响应式类型进行声明式组合。

### 创建 DatabaseClient

**Java:**
```java
DatabaseClient client = DatabaseClient.create(connectionFactory);
```

**Kotlin:**
```kotlin
val client = DatabaseClient.create(connectionFactory)
```

> `ConnectionFactory` 应始终在 Spring IoC 容器中配置为一个 Bean。

### 自定义 DatabaseClient

你可以从 `DatabaseClient.builder()` 获取一个 `Builder` 实例。可以通过调用以下方法来自定义客户端：

- `bindMarkers(...)` - 提供特定的 `BindMarkersFactory`
- `executeFunction(...)` - 设置 `ExecuteFunction`
- `namedParameters(false)` - 禁用命名参数展开（默认启用）

### 支持的数据库

目前支持的数据库有：
- H2
- MariaDB
- Microsoft SQL Server
- MySQL
- Postgres

## 执行语句

### 创建表

**Java:**
```java
Mono<Void> completion = client.sql("CREATE TABLE person (id VARCHAR(255) PRIMARY KEY, name VARCHAR(255), age INTEGER);")
        .then();
```

**Kotlin:**
```kotlin
client.sql("CREATE TABLE person (id VARCHAR(255) PRIMARY KEY, name VARCHAR(255), age INTEGER);")
        .await()
```

## 查询（SELECT）

### 基本查询

**Java:**
```java
Mono<Map<String, Object>> first = client.sql("SELECT id, name FROM person")
        .fetch().first();
```

**Kotlin:**
```kotlin
val first = client.sql("SELECT id, name FROM person")
        .fetch().awaitSingle()
```

### 使用绑定变量

**Java:**
```java
Mono<Map<String, Object>> first = client.sql("SELECT id, name FROM person WHERE first_name = :fn")
        .bind("fn", "Joe")
        .fetch().first();
```

**Kotlin:**
```kotlin
val first = client.sql("SELECT id, name FROM person WHERE first_name = :fn")
        .bind("fn", "Joe")
        .fetch().awaitSingle()
```

### 结果映射

**Java:**
```java
Flux<String> names = client.sql("SELECT name FROM person")
        .map(row -> row.get("name", String.class))
        .all();
```

**Kotlin:**
```kotlin
val names = client.sql("SELECT name FROM person")
        .map{ row: Row -> row.get("name", String.class) }
        .flow()
```

## 更新（INSERT、UPDATE 和 DELETE）

**Java:**
```java
Mono<Integer> affectedRows = client.sql("UPDATE person SET first_name = :fn")
        .bind("fn", "Joe")
        .fetch().rowsUpdated();
```

**Kotlin:**
```kotlin
val affectedRows = client.sql("UPDATE person SET first_name = :fn")
        .bind("fn", "Joe")
        .fetch().awaitRowsUpdated()
```

## 将值绑定到查询

### 按名称绑定

```java
db.sql("INSERT INTO person (id, name, age) VALUES(:id, :name, :age)")
        .bind("id", "joe")
        .bind("name", "Joe")
        .bind("age", 34);
```

### 使用 Map 绑定

```java
Map<String, Object> params = new LinkedHashMap<>();
params.put("id", "joe");
params.put("name", "Joe");
params.put("age", 34);
db.sql("INSERT INTO person (id, name, age) VALUES(:id, :name, :age)")
        .bindValues(params);
```

### 使用对象属性绑定

```java
db.sql("INSERT INTO person (id, name, age) VALUES(:id, :name, :age)")
        .bindProperties(new Person("joe", "Joe", 34));
```

### IN 子句支持

**Java:**
```java
client.sql("SELECT id, name, state FROM table WHERE age IN (:ages)")
        .bind("ages", Arrays.asList(35, 50));
```

**Kotlin:**
```kotlin
client.sql("SELECT id, name, state FROM table WHERE age IN (:ages)")
        .bind("ages", arrayOf(35, 50))
```

## DatabaseClient 最佳实践

一旦配置完成，`DatabaseClient` 类的实例是线程安全的。你可以配置一个 `DatabaseClient` 的单例实例，然后安全地将这个共享引用注入到多个 DAO（或 Repository）中。

### 使用依赖注入

**Java:**
```java
@Component
public class R2dbcCorporateEventDao implements CorporateEventDao {

    private DatabaseClient databaseClient;

    @Autowired
    public void setConnectionFactory(ConnectionFactory connectionFactory) {
        this.databaseClient = DatabaseClient.create(connectionFactory);
    }

    // R2DBC-backed implementations...
}
```

**Kotlin:**
```kotlin
@Component
class R2dbcCorporateEventDao(connectionFactory: ConnectionFactory) : CorporateEventDao {

    private val databaseClient = DatabaseClient.create(connectionFactory)

    // R2DBC-backed implementations...
}
```

---

*来源: [Spring Framework 官方文档](https://docs.springframework.org.cn/spring-framework/reference/data-access/r2dbc.html)*
