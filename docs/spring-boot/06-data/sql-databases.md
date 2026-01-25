# SQL Databases / SQL 数据库

Source: https://docs.springframework.org.cn/spring-boot/reference/data/sql.html

---

## English

The Spring Framework provides extensive support for working with SQL databases, from direct JDBC access using `JdbcClient` or `JdbcTemplate` to complete "object relational mapping" technologies such as Hibernate. Spring Data provides an additional level of functionality: creating `Repository` implementations directly from interfaces and using conventions to generate queries from your method names.

## Configuring a DataSource

Java's `javax.sql.DataSource` interface provides a standard method of working with database connections. Traditionally, a `DataSource` uses a `URL` along with some credentials to establish a database connection.

### Embedded Database Support

It is often convenient to develop applications by using an in-memory embedded database. Obviously, in-memory databases do not provide persistent storage. You need to populate your database when your application starts and be prepared to discard the data when your application ends.

Spring Boot can auto-configure embedded H2, HSQL, and Derby databases. You need not provide any connection URLs. You need only include a build dependency on the embedded database that you want to use. If more than one embedded database is on the classpath, set the `spring.datasource.embedded-database-connection` configuration property to control which one is used. Setting the property to `none` disables the auto-configuration of an embedded database.

For example, typical POM dependencies are as follows:

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-data-jpa</artifactId>
</dependency>
<dependency>
    <groupId>org.hsqldb</groupId>
    <artifactId>hsqldb</artifactId>
    <scope>runtime</scope>
</dependency>
```

|  |  |
| --- | --- |
|  | To auto-configure an embedded database, you need to depend on `spring-jdbc`. In this example, it is pulled in through `spring-boot-starter-data-jpa`. |

### Connecting to a Production Database

Production database connections can also be auto-configured by using a pooling `DataSource`.

### DataSource Configuration

DataSource configuration is controlled by external configuration properties in `spring.datasource.*`. For example, you might declare the following section in your `application.properties`:

- Properties
- YAML

```properties
spring.datasource.url=jdbc:mysql://127.0.0.1/test
spring.datasource.username=dbuser
spring.datasource.password=dbpass
```

```yaml
spring:
  datasource:
    url: "jdbc:mysql://127.0.0.1/test"
    username: "dbuser"
    password: "dbpass"
```

|  |  |
| --- | --- |
|  | You should at least specify the URL by setting the `spring.datasource.url` property. Otherwise, Spring Boot attempts to auto-configure an embedded database. |

Spring Boot can infer the JDBC driver class for most databases from the URL. If you need to specify a specific class, you can use the `spring.datasource.driver-class-name` property.

For more supported options, see the `DataSourceProperties` API documentation. These are standard options that work regardless of the actual implementation. You can also fine-tune implementation-specific settings using their corresponding prefixes (`spring.datasource.hikari.*`, `spring.datasource.tomcat.*`, `spring.datasource.dbcp2.*`, and `spring.datasource.oracleucp.*`).

### Supported Connection Pools

Spring Boot uses the following algorithm for choosing a specific implementation:

1. We prefer HikariCP for its performance and concurrency. If HikariCP is available, we always use it.
2. Otherwise, if the Tomcat pooling `DataSource` is available, we use it.
3. Otherwise, if Commons DBCP2 is available, we use it.
4. If none of HikariCP, Tomcat, and DBCP2 are available and Oracle UCP is available, we use it.

|  |  |
| --- | --- |
|  | If you use the `spring-boot-starter-jdbc` or `spring-boot-starter-data-jpa` starters, you automatically get a dependency to `HikariCP`. |

You can bypass that algorithm completely and specify the connection pool to use by setting the `spring.datasource.type` property. This is especially important if you run your application in a Tomcat container, as `tomcat-jdbc` is provided by default.

You can always manually configure other connection pools by using `DataSourceBuilder`. If you define your own `DataSource` bean, auto-configuration does not occur. The following connection pools are supported by `DataSourceBuilder`:

- HikariCP
- Tomcat pooling `DataSource`
- Commons DBCP2
- Oracle UCP and `OracleDataSource`
- Spring Framework's `SimpleDriverDataSource`
- H2 `JdbcDataSource`
- PostgreSQL `PGSimpleDataSource`
- C3P0

### Connecting to a JNDI DataSource

If you deploy your Spring Boot application to an application server, you might want to configure and manage your DataSource by using your application server's built-in features and access it by using JNDI.

The `spring.datasource.jndi-name` property can be used as an alternative to the `spring.datasource.url`, `spring.datasource.username`, and `spring.datasource.password` properties to access a `DataSource` from a specific JNDI location.

For example, the following section in `application.properties` shows how to access a `DataSource` defined by JBoss AS:

- Properties
- YAML

```properties
spring.datasource.jndi-name=java:jboss/datasources/customers
```

```yaml
spring:
  datasource:
    jndi-name: "java:jboss/datasources/customers"
```

## Using JdbcTemplate

Spring's `JdbcTemplate` and `NamedParameterJdbcTemplate` classes are auto-configured, and you can `@Autowired` them directly into your own beans, as shown in the following example:

- Java
- Kotlin

```java
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final JdbcTemplate jdbcTemplate;

    public MyBean(JdbcTemplate jdbcTemplate) {
        this.jdbcTemplate = jdbcTemplate;
    }

    public void doSomething() {
        this.jdbcTemplate ...
    }

}
```

```kotlin
import org.springframework.jdbc.core.JdbcTemplate
import org.springframework.stereotype.Component

@Component
class MyBean(private val jdbcTemplate: JdbcTemplate) {
    fun doSomething() {
        jdbcTemplate.execute("delete from customer")
    }
}
```

You can customize some properties of the template by using the `spring.jdbc.template.*` properties, as shown in the following example:

- Properties
- YAML

```properties
spring.jdbc.template.max-rows=500
```

```yaml
spring:
  jdbc:
    template:
      max-rows: 500
```

## Using JdbcClient

Spring's `JdbcClient` is auto-configured based on the presence of a `NamedParameterJdbcTemplate`. You can also inject it directly into your own beans, as shown in the following example:

- Java
- Kotlin

```java
import org.springframework.jdbc.core.simple.JdbcClient;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final JdbcClient jdbcClient;

    public MyBean(JdbcClient jdbcClient) {
        this.jdbcClient = jdbcClient;
    }

    public void doSomething() {
        this.jdbcClient ...
    }

}
```

```kotlin
import org.springframework.jdbc.core.simple.JdbcClient
import org.springframework.stereotype.Component

@Component
class MyBean(private val jdbcClient: JdbcClient) {
    fun doSomething() {
        jdbcClient.sql("delete from customer").update()
    }
}
```

## JPA and Spring Data JPA

The Java Persistence API is a standard technology that lets you "map" objects to relational databases. The `spring-boot-starter-data-jpa` POM provides a quick way to get started. It provides the following key dependencies:

- Hibernate: One of the most popular JPA implementations.
- Spring Data JPA: Helps you implement JPA-based repositories.
- Spring ORM: Core ORM support from the Spring Framework.

### Entity Classes

Traditionally, JPA "entity" classes are specified in a `persistence.xml` file. With Spring Boot, this file is not required and "entity scanning" is used instead. By default, the auto-configuration packages are searched.

Any class annotated with `@Entity`, `@Embeddable`, or `@MappedSuperclass` is considered an entity class. A typical entity class resembles the following example:

- Java
- Kotlin

```java
import java.io.Serializable;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.Id;

@Entity
public class City implements Serializable {

    @Id
    @GeneratedValue
    private Long id;

    @Column(nullable = false)
    private String name;

    @Column(nullable = false)
    private String state;

    // ... additional members, often include @OneToMany mappings

    protected City() {
        // no-args constructor required by JPA spec
        // this one is protected since it should not be used directly
    }

    public City(String name, String state) {
        this.name = name;
        this.state = state;
    }

    public String getName() {
        return this.name;
    }

    public String getState() {
        return this.state;
    }

    // ... etc

}
```

```kotlin
import jakarta.persistence.Column
import jakarta.persistence.Entity
import jakarta.persistence.GeneratedValue
import jakarta.persistence.Id
import java.io.Serializable

@Entity
class City : Serializable {

    @Id
    @GeneratedValue
    private val id: Long? = null

    @Column(nullable = false)
    var name: String? = null
        private set

    // ... etc
    @Column(nullable = false)
    var state: String? = null
        private set

    // ... additional members, often include @OneToMany mappings
    protected constructor() {
        // no-args constructor required by JPA spec
        // this one is protected since it should not be used directly
    }

    constructor(name: String?, state: String?) {
        this.name = name
        this.state = state
    }

}
```

### Spring Data JPA Repositories

Spring Data JPA repositories are interfaces that you can define to access data. JPA queries are created automatically from your method names. For example, a `CityRepository` interface might declare a `findAllByState(String state)` method to find all cities in a given state.

For more complex queries, you can annotate your method with Spring Data's `Query` annotation.

Spring Data repositories usually extend from the `Repository` or `CrudRepository` interfaces. If you use auto-configuration, repositories are searched from the auto-configuration package.

The following example shows a typical Spring Data repository interface definition:

- Java
- Kotlin

```java
import org.springframework.boot.docs.data.sql.jpaandspringdata.entityclasses.City;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.repository.Repository;

public interface CityRepository extends Repository<City, Long> {

    Page<City> findAll(Pageable pageable);

    City findByNameAndStateAllIgnoringCase(String name, String state);

}
```

```kotlin
import org.springframework.boot.docs.data.sql.jpaandspringdata.entityclasses.City
import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable
import org.springframework.data.repository.Repository

interface CityRepository : Repository<City?, Long?> {
    fun findAll(pageable: Pageable?): Page<City?>?
    fun findByNameAndStateAllIgnoringCase(name: String?, state: String?): City?
}
```

## Spring Data JDBC

Spring Data JDBC includes repository support for JDBC and will automatically generate SQL for the methods on `CrudRepository`. For more advanced queries, a `@Query` annotation is provided.

Spring Boot will auto-configure Spring Data's JDBC repositories when the necessary dependencies are on the classpath. They can be added to your project by adding a single dependency on `spring-boot-starter-data-jdbc`. If necessary, you can control the configuration of Spring Data JDBC by adding the `@EnableJdbcRepositories` annotation or an `AbstractJdbcConfiguration` subclass to your application.

## Using jOOQ

jOOQ Object Oriented Querying (jOOQ) is a popular product from Data Geekery which generates Java code from your database and lets you build type-safe SQL queries through its fluent API. Spring Boot can auto-configure the jOOQ open source edition as well as the commercial edition.

### Code Generation

To use jOOQ type-safe queries, you need to generate Java classes from your database schema. You can follow the instructions in the jOOQ user manual.

### Using DSLContext

The fluent API offered by jOOQ is initiated through the `org.jooq.DSLContext` interface. Spring Boot auto-configures a `DSLContext` as a Spring Bean and connects it to your application `DataSource`. To use `DSLContext`, you can inject it, as shown in the following example:

- Java
- Kotlin

```java
import java.util.GregorianCalendar;
import java.util.List;
import org.jooq.DSLContext;
import org.springframework.stereotype.Component;
import static org.springframework.boot.docs.data.sql.jooq.dslcontext.Tables.AUTHOR;

@Component
public class MyBean {

    private final DSLContext create;

    public MyBean(DSLContext dslContext) {
        this.create = dslContext;
    }

}
```

```kotlin
import org.jooq.DSLContext
import org.springframework.stereotype.Component
import java.util.GregorianCalendar

@Component
class MyBean(private val create: DSLContext) {
}
```

## Using R2DBC

The Reactive Relational Database Connectivity (R2DBC) project brings reactive programming APIs to relational databases. R2DBC's `io.r2dbc.spi.Connection` provides a standard method of working with non-blocking database connections. Connections are provided by using a `ConnectionFactory`, similar to a `DataSource` in jdbc.

`ConnectionFactory` configuration is controlled by external configuration properties in `spring.r2dbc.*`. For example, you might declare the following section in your `application.properties`:

- Properties
- YAML

```properties
spring.r2dbc.url=r2dbc:postgresql://127.0.0.1/test
spring.r2dbc.username=dbuser
spring.r2dbc.password=dbpass
```

```yaml
spring:
  r2dbc:
    url: "r2dbc:postgresql://127.0.0.1/test"
    username: "dbuser"
    password: "dbpass"
```

### Using DatabaseClient

The `DatabaseClient` bean is auto-configured, and you can `@Autowired` it directly into your own beans, as shown in the following example:

- Java
- Kotlin

```java
import java.util.Map;
import reactor.core.publisher.Flux;
import org.springframework.r2dbc.core.DatabaseClient;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final DatabaseClient databaseClient;

    public MyBean(DatabaseClient databaseClient) {
        this.databaseClient = databaseClient;
    }

    // ...
    public Flux<Map<String, Object>> someMethod() {
        return this.databaseClient.sql("select * from user").fetch().all();
    }

}
```

```kotlin
import org.springframework.r2dbc.core.DatabaseClient
import org.springframework.stereotype.Component
import reactor.core.publisher.Flux

@Component
class MyBean(private val databaseClient: DatabaseClient) {
    // ...
    fun someMethod(): Flux<Map<String, Any>> {
        return databaseClient.sql("select * from user").fetch().all()
    }
}
```

### Spring Data R2DBC Repositories

Spring Data R2DBC repositories are interfaces that you can define to access data. Queries are created automatically from your method names. For example, a `CityRepository` interface might declare a `findAllByState(String state)` method to find all cities in a given state.

For more complex queries, you can annotate your method with Spring Data's `@Query` annotation.

Spring Data repositories usually extend from the `Repository` or `CrudRepository` interfaces. If you use auto-configuration, repositories are searched from the auto-configuration package.

The following example shows a typical Spring Data repository interface definition:

- Java
- Kotlin

```java
import reactor.core.publisher.Mono;
import org.springframework.data.repository.Repository;

public interface CityRepository extends Repository<City, Long> {

    Mono<City> findByNameAndStateAllIgnoringCase(String name, String state);

}
```

```kotlin
import org.springframework.data.repository.Repository
import reactor.core.publisher.Mono

interface CityRepository : Repository<City?, Long?> {
    fun findByNameAndStateAllIgnoringCase(name: String?, state: String?): Mono<City?>?
}
```

---

## 中文 / Chinese

Spring Framework 提供了广泛的支持来处理 SQL 数据库，从使用 `JdbcClient` 或 `JdbcTemplate` 进行直接 JDBC 访问到完整的"对象关系映射"技术（如 Hibernate）。Spring Data 提供了额外的功能级别：直接从接口创建 `Repository` 实现，并使用约定从你的方法名称生成查询。

## 配置数据源

Java 的 `javax.sql.DataSource` 接口提供了一种处理数据库连接的标准方法。传统上，`DataSource` 使用 `URL` 以及一些凭据来建立数据库连接。

### 嵌入式数据库支持

使用内存中的嵌入式数据库来开发应用程序通常很方便。显然，内存数据库不提供持久性存储。您需要在应用程序启动时填充数据库，并准备好应用程序结束时丢弃数据。

Spring Boot 可以自动配置嵌入式 H2、HSQL 和 Derby 数据库。您无需提供任何连接 URL。您只需要在构建依赖项中包含要使用的嵌入式数据库即可。如果类路径上有多个嵌入式数据库，请设置 `spring.datasource.embedded-database-connection` 配置属性以控制使用哪个数据库。将该属性设置为 `none` 将禁用嵌入式数据库的自动配置。

例如，典型的 POM 依赖项如下所示：

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-data-jpa</artifactId>
</dependency>
<dependency>
    <groupId>org.hsqldb</groupId>
    <artifactId>hsqldb</artifactId>
    <scope>runtime</scope>
</dependency>
```

|  |  |
| --- | --- |
|  | 要自动配置嵌入式数据库，您需要依赖 `spring-jdbc`。在本例中，它通过 `spring-boot-starter-data-jpa` 传递引入。 |

### 连接到生产数据库

生产数据库连接也可以通过使用池化 `DataSource` 来自动配置。

### DataSource 配置

DataSource 配置由 `spring.datasource.*` 中的外部配置属性控制。例如，您可以在 `application.properties` 中声明以下部分：

- 属性
- YAML

```properties
spring.datasource.url=jdbc:mysql://127.0.0.1/test
spring.datasource.username=dbuser
spring.datasource.password=dbpass
```

```yaml
spring:
  datasource:
    url: "jdbc:mysql://127.0.0.1/test"
    username: "dbuser"
    password: "dbpass"
```

|  |  |
| --- | --- |
|  | 您至少应通过设置 `spring.datasource.url` 属性来指定 URL。否则，Spring Boot 会尝试自动配置嵌入式数据库。 |

Spring Boot 可以从 URL 中推断出大多数数据库的 JDBC 驱动程序类。如果您需要指定特定的类，可以使用 `spring.datasource.driver-class-name` 属性。

有关支持的更多选项，请参阅 `DataSourceProperties` API 文档。这些是标准选项，无论实际实现如何都适用。还可以使用各自的前缀（`spring.datasource.hikari.*`、`spring.datasource.tomcat.*`、`spring.datasource.dbcp2.*` 和 `spring.datasource.oracleucp.*`）微调特定于实现的设置。

### 支持的连接池

Spring Boot 使用以下算法选择特定的实现：

1. 我们更喜欢 HikariCP，因为它具有良好的性能和并发性。如果 HikariCP 可用，我们始终选择它。
2. 否则，如果 Tomcat 池化 `DataSource` 可用，则我们使用它。
3. 否则，如果 Commons DBCP2 可用，则我们使用它。
4. 如果 HikariCP、Tomcat 和 DBCP2 均不可用，并且 Oracle UCP 可用，则我们使用它。

|  |  |
| --- | --- |
|  | 如果您使用 `spring-boot-starter-jdbc` 或 `spring-boot-starter-data-jpa` 启动器，则会自动获得对 `HikariCP` 的依赖项。 |

您可以完全绕过该算法，并通过设置 `spring.datasource.type` 属性来指定要使用的连接池。如果您在 Tomcat 容器中运行应用程序，这尤其重要，因为 `tomcat-jdbc` 默认提供。

可以使用 `DataSourceBuilder` 始终手动配置其他连接池。如果您定义了自己的 `DataSource` bean，则不会发生自动配置。以下连接池受 `DataSourceBuilder` 支持：

- HikariCP
- Tomcat 池化 `DataSource`
- Commons DBCP2
- Oracle UCP 和 `OracleDataSource`
- Spring 框架的 `SimpleDriverDataSource`
- H2 `JdbcDataSource`
- PostgreSQL `PGSimpleDataSource`
- C3P0

### 连接到 JNDI 数据源

如果您将 Spring Boot 应用程序部署到应用程序服务器，您可能希望使用应用程序服务器的内置功能配置和管理您的 DataSource，并使用 JNDI 访问它。

`spring.datasource.jndi-name` 属性可以用作 `spring.datasource.url`、`spring.datasource.username` 和 `spring.datasource.password` 属性的替代方案，以从特定的 JNDI 位置访问 `DataSource`。

例如，`application.properties` 中的以下部分显示了如何访问 JBoss AS 定义的 `DataSource`：

- 属性
- YAML

```properties
spring.datasource.jndi-name=java:jboss/datasources/customers
```

```yaml
spring:
  datasource:
    jndi-name: "java:jboss/datasources/customers"
```

## 使用 JdbcTemplate

Spring 的 `JdbcTemplate` 和 `NamedParameterJdbcTemplate` 类已自动配置，您可以将它们直接自动装配到您自己的 bean 中，如下例所示：

- Java
- Kotlin

```java
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final JdbcTemplate jdbcTemplate;

    public MyBean(JdbcTemplate jdbcTemplate) {
        this.jdbcTemplate = jdbcTemplate;
    }

    public void doSomething() {
        this.jdbcTemplate ...
    }

}
```

```kotlin
import org.springframework.jdbc.core.JdbcTemplate
import org.springframework.stereotype.Component

@Component
class MyBean(private val jdbcTemplate: JdbcTemplate) {
    fun doSomething() {
        jdbcTemplate.execute("delete from customer")
    }
}
```

您可以使用 `spring.jdbc.template.*` 属性自定义模板的一些属性，如下例所示：

- 属性
- YAML

```properties
spring.jdbc.template.max-rows=500
```

```yaml
spring:
  jdbc:
    template:
      max-rows: 500
```

## 使用 JdbcClient

Spring 的 `JdbcClient` 基于 `NamedParameterJdbcTemplate` 的存在进行自动配置。您也可以将其直接注入到您自己的 bean 中，如下例所示：

- Java
- Kotlin

```java
import org.springframework.jdbc.core.simple.JdbcClient;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final JdbcClient jdbcClient;

    public MyBean(JdbcClient jdbcClient) {
        this.jdbcClient = jdbcClient;
    }

    public void doSomething() {
        this.jdbcClient ...
    }

}
```

```kotlin
import org.springframework.jdbc.core.simple.JdbcClient
import org.springframework.stereotype.Component

@Component
class MyBean(private val jdbcClient: JdbcClient) {
    fun doSomething() {
        jdbcClient.sql("delete from customer").update()
    }
}
```

## JPA 和 Spring Data JPA

Java 持久性 API 是一种标准技术，允许您将对象"映射"到关系数据库。`spring-boot-starter-data-jpa` POM 提供了一种快速入门的方法。它提供以下关键依赖项：

- Hibernate：最流行的 JPA 实现之一。
- Spring Data JPA：帮助您实现基于 JPA 的存储库。
- Spring ORM：来自 Spring 框架的核心 ORM 支持。

### 实体类

传统上，JPA"实体"类是在 `persistence.xml` 文件中指定的。使用 Spring Boot，此文件不是必需的，而是使用"实体扫描"。默认情况下，会扫描自动配置包。

任何用 `@Entity`、`@Embeddable` 或 `@MappedSuperclass` 注解的类都被视为实体类。典型的实体类类似于以下示例：

- Java
- Kotlin

```java
import java.io.Serializable;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.Id;

@Entity
public class City implements Serializable {

    @Id
    @GeneratedValue
    private Long id;

    @Column(nullable = false)
    private String name;

    @Column(nullable = false)
    private String state;

    // ... additional members, often include @OneToMany mappings

    protected City() {
        // no-args constructor required by JPA spec
        // this one is protected since it should not be used directly
    }

    public City(String name, String state) {
        this.name = name;
        this.state = state;
    }

    public String getName() {
        return this.name;
    }

    public String getState() {
        return this.state;
    }

    // ... etc

}
```

```kotlin
import jakarta.persistence.Column
import jakarta.persistence.Entity
import jakarta.persistence.GeneratedValue
import jakarta.persistence.Id
import java.io.Serializable

@Entity
class City : Serializable {

    @Id
    @GeneratedValue
    private val id: Long? = null

    @Column(nullable = false)
    var name: String? = null
        private set

    // ... etc
    @Column(nullable = false)
    var state: String? = null
        private set

    // ... additional members, often include @OneToMany mappings
    protected constructor() {
        // no-args constructor required by JPA spec
        // this one is protected since it should not be used directly
    }

    constructor(name: String?, state: String?) {
        this.name = name
        this.state = state
    }

}
```

### Spring Data JPA 存储库

Spring Data JPA 存储库是您可以定义以访问数据的接口。JPA 查询会根据您的方法名称自动创建。例如，`CityRepository` 接口可能会声明一个 `findAllByState(String state)` 方法来查找给定州中的所有城市。

对于更复杂的查询，您可以使用 Spring Data 的 `Query` 注解来注释您的方法。

Spring Data 存储库通常扩展自 `Repository` 或 `CrudRepository` 接口。如果您使用自动配置，则会搜索自动配置包以查找存储库。

以下示例显示了典型的 Spring Data 存储库接口定义：

- Java
- Kotlin

```java
import org.springframework.boot.docs.data.sql.jpaandspringdata.entityclasses.City;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.repository.Repository;

public interface CityRepository extends Repository<City, Long> {

    Page<City> findAll(Pageable pageable);

    City findByNameAndStateAllIgnoringCase(String name, String state);

}
```

```kotlin
import org.springframework.boot.docs.data.sql.jpaandspringdata.entityclasses.City
import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable
import org.springframework.data.repository.Repository

interface CityRepository : Repository<City?, Long?> {
    fun findAll(pageable: Pageable?): Page<City?>?
    fun findByNameAndStateAllIgnoringCase(name: String?, state: String?): City?
}
```

## Spring Data JDBC

Spring Data 包含对 JDBC 的存储库支持，并将自动为 `CrudRepository` 上的方法生成 SQL。对于更高级的查询，提供了 `@Query` 注解。

当类路径上存在必要的依赖项时，Spring Boot 将自动配置 Spring Data 的 JDBC 存储库。可以通过在项目中添加对 `spring-boot-starter-data-jdbc` 的单个依赖项来添加它们。如有必要，可以通过添加 `@EnableJdbcRepositories` 注解或 `AbstractJdbcConfiguration` 子类到您的应用程序中来控制 Spring Data JDBC 的配置。

## 使用 jOOQ

jOOQ 面向对象查询 (jOOQ) 是 Data Geekery 的一款流行产品，它根据您的数据库模式生成 Java 代码，并允许您通过其流畅的 API 构建类型安全的 SQL 查询。Spring Boot 可以使用商业版和开源版。

### 代码生成

为了使用 jOOQ 类型安全的查询，您需要根据数据库模式生成 Java 类。您可以按照 jOOQ 用户手册中的说明进行操作。

### 使用 DSLContext

jOOQ 提供的流畅 API 通过 `org.jooq.DSLContext` 接口启动。Spring Boot 自动配置 `DSLContext` 作为 Spring Bean，并将其连接到您的应用程序 `DataSource`。要使用 `DSLContext`，您可以将其注入，如下例所示：

- Java
- Kotlin

```java
import java.util.GregorianCalendar;
import java.util.List;
import org.jooq.DSLContext;
import org.springframework.stereotype.Component;
import static org.springframework.boot.docs.data.sql.jooq.dslcontext.Tables.AUTHOR;

@Component
public class MyBean {

    private final DSLContext create;

    public MyBean(DSLContext dslContext) {
        this.create = dslContext;
    }

}
```

```kotlin
import org.jooq.DSLContext
import org.springframework.stereotype.Component
import java.util.GregorianCalendar

@Component
class MyBean(private val create: DSLContext) {
}
```

## 使用 R2DBC

Reactive Relational Database Connectivity (R2DBC) 项目将反应式编程 API 引入关系数据库。R2DBC 的 `io.r2dbc.spi.Connection` 提供了一种使用非阻塞数据库连接的标准方法。连接是通过使用 `ConnectionFactory` 提供的，类似于 jdbc 中的 `DataSource`。

`ConnectionFactory` 配置由 `spring.r2dbc.*` 中的外部配置属性控制。例如，您可以在 `application.properties` 中声明以下部分：

- 属性
- YAML

```properties
spring.r2dbc.url=r2dbc:postgresql://127.0.0.1/test
spring.r2dbc.username=dbuser
spring.r2dbc.password=dbpass
```

```yaml
spring:
  r2dbc:
    url: "r2dbc:postgresql://127.0.0.1/test"
    username: "dbuser"
    password: "dbpass"
```

### 使用 DatabaseClient

`DatabaseClient` bean 是自动配置的，您可以将其直接自动装配到您自己的 bean 中，如下例所示：

- Java
- Kotlin

```java
import java.util.Map;
import reactor.core.publisher.Flux;
import org.springframework.r2dbc.core.DatabaseClient;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final DatabaseClient databaseClient;

    public MyBean(DatabaseClient databaseClient) {
        this.databaseClient = databaseClient;
    }

    // ...
    public Flux<Map<String, Object>> someMethod() {
        return this.databaseClient.sql("select * from user").fetch().all();
    }

}
```

```kotlin
import org.springframework.r2dbc.core.DatabaseClient
import org.springframework.stereotype.Component
import reactor.core.publisher.Flux

@Component
class MyBean(private val databaseClient: DatabaseClient) {
    // ...
    fun someMethod(): Flux<Map<String, Any>> {
        return databaseClient.sql("select * from user").fetch().all()
    }
}
```

### Spring Data R2DBC 存储库

Spring Data R2DBC 存储库是您可以定义以访问数据的接口。查询会根据您的方法名称自动创建。例如，`CityRepository` 接口可能会声明一个 `findAllByState(String state)` 方法来查找给定州中的所有城市。

对于更复杂的查询，您可以使用 Spring Data 的 `@Query` 注解来注释您的方法。

Spring Data 存储库通常扩展自 `Repository` 或 `CrudRepository` 接口。如果您使用自动配置，则会搜索自动配置包以查找存储库。

以下示例显示了典型的 Spring Data 存储库接口定义：

- Java
- Kotlin

```java
import reactor.core.publisher.Mono;
import org.springframework.data.repository.Repository;

public interface CityRepository extends Repository<City, Long> {

    Mono<City> findByNameAndStateAllIgnoringCase(String name, String state);

}
```

```kotlin
import org.springframework.data.repository.Repository
import reactor.core.publisher.Mono

interface CityRepository : Repository<City?, Long?> {
    fun findByNameAndStateAllIgnoringCase(name: String?, state: String?): Mono<City?>?
}
```
