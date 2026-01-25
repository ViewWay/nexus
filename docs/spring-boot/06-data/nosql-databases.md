# NoSQL Technologies / 使用 NoSQL 技术

Source: https://docs.springframework.org.cn/spring-boot/reference/data/nosql.html

---

## English

Spring Boot provides auto-configuration support for the following NoSQL technologies:

- Redis
- MongoDB
- Neo4j
- Elasticsearch
- Cassandra
- Couchbase
- LDAP

## Redis

A `spring-boot-starter-data-redis` starter is available to collect dependencies in a convenient way. By default, it uses Lettuce. The starter handles both traditional and reactive applications.

### Connecting to Redis

You can inject an auto-configured `StringRedisTemplate` or any other Spring Data bean as you would any other Spring Bean. By default, the instance tries to connect to a Redis server at `localhost:6379`.

- Java
- Kotlin

```java
import org.springframework.data.redis.core.StringRedisTemplate;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final StringRedisTemplate template;

    public MyBean(StringRedisTemplate template) {
        this.template = template;
    }

    // ...
    public Boolean someMethod() {
        return this.template.hasKey("spring");
    }

}
```

```kotlin
import org.springframework.data.redis.core.StringRedisTemplate
import org.springframework.stereotype.Component

@Component
class MyBean(private val template: StringRedisTemplate) {

    // ...
    fun someMethod(): Boolean {
        return template.hasKey("spring")
    }

}
```

You can specify custom connection details using `spring.data.redis.*` properties, as shown in the following example:

- Properties
- YAML

```properties
spring.data.redis.host=localhost
spring.data.redis.port=6379
spring.data.redis.database=0
spring.data.redis.username=user
spring.data.redis.password=secret
```

```yaml
spring:
  data:
    redis:
      host: "localhost"
      port: 6379
      database: 0
      username: "user"
      password: "secret"
```

## MongoDB

MongoDB is an open-source NoSQL document database that uses a JSON-like schema instead of traditional table-based relational data. Spring Boot provides several conveniences for working with MongoDB, including the `spring-boot-starter-data-mongodb` and `spring-boot-starter-data-mongodb-reactive` starters.

### Connecting to a MongoDB Database

To access a MongoDB database, you can inject an auto-configured `MongoDatabaseFactory`. By default, the instance tries to connect to a MongoDB server at `mongodb:///test`.

- Java
- Kotlin

```java
import com.mongodb.client.MongoCollection;
import com.mongodb.client.MongoDatabase;
import org.bson.Document;

import org.springframework.data.mongodb.MongoDatabaseFactory;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final MongoDatabaseFactory mongo;

    public MyBean(MongoDatabaseFactory mongo) {
        this.mongo = mongo;
    }

    // ...
    public MongoCollection<Document> someMethod() {
        MongoDatabase db = this.mongo.getMongoDatabase();
        return db.getCollection("users");
    }

}
```

```kotlin
import com.mongodb.client.MongoCollection
import org.bson.Document
import org.springframework.data.mongodb.MongoDatabaseFactory
import org.springframework.stereotype.Component

@Component
class MyBean(private val mongo: MongoDatabaseFactory) {

    // ...
    fun someMethod(): MongoCollection<Document> {
        val db = mongo.mongoDatabase
        return db.getCollection("users")
    }

}
```

You can set the `spring.mongodb.uri` property to change the URL and configure additional settings, such as replica sets.

- Properties
- YAML

```properties
spring.mongodb.uri=mongodb://user:secret@mongoserver1.example.com:27017,mongoserver2.example.com:23456/test
```

```yaml
spring:
  mongodb:
    uri: "mongodb://user:secret@mongoserver1.example.com:27017,mongoserver2.example.com:23456/test"
```

### Spring Data MongoDB Repositories

Spring Data includes repository support for MongoDB. As with the JPA repositories discussed earlier, the basic principle is that queries are constructed for you automatically based on method names.

In fact, both Spring Data JPA and Spring Data MongoDB share the same common infrastructure. You can take the JPA example from earlier and, assuming that `City` is now a MongoDB data class rather than a JPA `@Entity`, it works in the same way.

- Java
- Kotlin

```java
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.repository.Repository;

public interface CityRepository extends Repository<City, Long> {

    Page<City> findAll(Pageable pageable);

    City findByNameAndStateAllIgnoringCase(String name, String state);

}
```

```kotlin
import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable
import org.springframework.data.repository.Repository

interface CityRepository :
    Repository<City, Long> {
    fun findAll(pageable: Pageable?): Page<City>
    fun findByNameAndStateAllIgnoringCase(name: String, state: String): City?
}
```

## Neo4j

Neo4j is an open-source NoSQL graph database that uses a rich data model of nodes connected by first class relationships, which is better suited for connected big data than traditional RDBMS approaches. Spring Boot provides several conveniences for working with Neo4j, including the `spring-boot-starter-data-neo4j` starter.

### Connecting to a Neo4j Database

To access a Neo4j server, you can inject an auto-configured `Driver`. By default, the instance tries to connect to a Neo4j server at `localhost:7687` using the Bolt protocol.

- Java
- Kotlin

```java
import org.neo4j.driver.Driver;
import org.neo4j.driver.Session;
import org.neo4j.driver.Values;

import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final Driver driver;

    public MyBean(Driver driver) {
        this.driver = driver;
    }

    // ...
    public String someMethod(String message) {
        try (Session session = this.driver.session()) {
            return session.executeWrite(
                    (transaction) -> transaction
                            .run("CREATE (a:Greeting) SET a.message = $message RETURN a.message + ', from node ' + id(a)",
                                    Values.parameters("message", message))
                            .single()
                            .get(0)
                            .asString());
        }
    }

}
```

```kotlin
import org.neo4j.driver.Driver
import org.neo4j.driver.TransactionContext
import org.neo4j.driver.Values
import org.springframework.stereotype.Component

@Component
class MyBean(private val driver: Driver) {

    // ...
    fun someMethod(message: String?): String {
        driver.session().use { session ->
            return@someMethod session.executeWrite { transaction: TransactionContext ->
                transaction
                    .run(
                        "CREATE (a:Greeting) SET a.message = $message RETURN a.message + ', from node ' + id(a)",
                        Values.parameters("message", message)
                    )
                    .single()[0].asString()
            }
        }
    }

}
```

You can configure various aspects of the driver using `spring.neo4j.*` properties.

- Properties
- YAML

```properties
spring.neo4j.uri=bolt://my-server:7687
spring.neo4j.authentication.username=neo4j
spring.neo4j.authentication.password=secret
```

```yaml
spring:
  neo4j:
    uri: "bolt://my-server:7687"
    authentication:
      username: "neo4j"
      password: "secret"
```

## Elasticsearch

Elasticsearch is an open source, distributed, RESTful search and analytics engine. Spring Boot provides basic auto-configuration for the Elasticsearch clients.

Spring Boot supports several clients:

- Official low-level REST Client
- Official Java API Client
- `ReactiveElasticsearchClient` provided by Spring Data Elasticsearch

Spring Boot provides a dedicated starter `spring-boot-starter-data-elasticsearch`.

### Connecting to Elasticsearch by Using REST Clients

Elasticsearch provides two distinct REST Clients that you can use to query a cluster: the low-level client and the Java API client. The Java API client is provided by the `co.elastic.clients:elasticsearch-java` module, and the low-level client is provided by the `co.elastic.clients:elasticsearch-rest5-client` module.

By default, the clients target `localhost:9200`. You can further tune how the clients are configured using `spring.elasticsearch.*` properties.

- Properties
- YAML

```properties
spring.elasticsearch.uris=https://search.example.com:9200
spring.elasticsearch.socket-timeout=10s
spring.elasticsearch.username=user
spring.elasticsearch.password=secret
```

```yaml
spring:
  elasticsearch:
    uris: "https://search.example.com:9200"
    socket-timeout: "10s"
    username: "user"
    password: "secret"
```

### Spring Data Elasticsearch Repositories

Spring Data includes repository support for Elasticsearch. As with the JPA repositories discussed earlier, the basic principle is that queries are constructed for you automatically based on method names.

## Cassandra

Cassandra is an open source, distributed database management system designed to handle large amounts of data across many commodity servers. Spring Boot provides auto-configuration for Cassandra and the abstractions built on top of it provided by Spring Data Cassandra. A `spring-boot-starter-data-cassandra` starter is available to collect dependencies in a convenient way.

### Connecting to Cassandra

You can inject an auto-configured `CqlTemplate`, `CassandraTemplate`, or Cassandra `CqlSession` instance as you would any other Spring Bean. The `spring.cassandra.*` properties can be used to customize the connection. Typically, you provide the `keyspace-name` and `contact-points` as well as the local datacenter name.

- Properties
- YAML

```properties
spring.cassandra.keyspace-name=mykeyspace
spring.cassandra.contact-points=cassandrahost1:9042,cassandrahost2:9042
spring.cassandra.local-datacenter=datacenter1
```

```yaml
spring:
  cassandra:
    keyspace-name: "mykeyspace"
    contact-points: "cassandrahost1:9042,cassandrahost2:9042"
    local-datacenter: "datacenter1"
```

## Couchbase

Couchbase is an open-source, distributed, multi-model NoSQL document-oriented database that is optimized for interactive applications. Spring Boot provides auto-configuration for Couchbase and the abstractions built on top of it provided by Spring Data Couchbase. `spring-boot-starter-data-couchbase` and `spring-boot-starter-data-couchbase-reactive` starters are available to collect dependencies in a convenient way.

### Connecting to Couchbase

You can get a `Cluster` by adding the Couchbase SDK and some configuration. The `spring.couchbase.*` properties can be used to customize the connection. Typically, you provide the connection string and credentials for authentication.

- Properties
- YAML

```properties
spring.couchbase.connection-string=couchbase://192.168.1.123
spring.couchbase.username=user
spring.couchbase.password=secret
```

```yaml
spring:
  couchbase:
    connection-string: "couchbase://192.168.1.123"
    username: "user"
    password: "secret"
```

### Spring Data Couchbase Repositories

Spring Data includes repository support for Couchbase.

You can configure the bucket name as follows:

- Properties
- YAML

```properties
spring.data.couchbase.bucket-name=my-bucket
```

```yaml
spring:
  data:
    couchbase:
      bucket-name: "my-bucket"
```

## LDAP

LDAP (Lightweight Directory Access Protocol) is an open, vendor-neutral, industry standard application protocol for accessing and maintaining distributed directory信息服务 over an IP network. Spring Boot provides auto-configuration for any compliant LDAP server as well as for an in-memory embedded LDAP server from UnboundID.

The LDAP abstraction is provided by Spring Data LDAP. A `spring-boot-starter-data-ldap` starter is available to collect dependencies in a convenient way.

### Connecting to an LDAP Server

To connect to an LDAP server, make sure you declare a dependency on the `spring-boot-starter-data-ldap` starter or `spring-ldap-core` and then declare the URL of your server in your application.properties.

- Properties
- YAML

```properties
spring.ldap.urls=ldap://myserver:1235
spring.ldap.username=admin
spring.ldap.password=secret
```

```yaml
spring:
  ldap:
    urls: "ldap://myserver:1235"
    username: "admin"
    password: "secret"
```

---

## 中文 / Chinese

Spring Boot 为以下 NoSQL 技术提供自动配置支持：

- Redis
- MongoDB
- Neo4j
- Elasticsearch
- Cassandra
- Couchbase
- LDAP

## Redis

有一个 `spring-boot-starter-data-redis` 启动器，用于以方便的方式收集依赖项。默认情况下，它使用 Lettuce。该启动器同时处理传统和响应式应用程序。

### 连接到 Redis

您可以像处理任何其他 Spring Bean 一样注入自动配置的 `StringRedisTemplate` 或任何其他 Spring Data bean。默认情况下，该实例尝试连接到 `localhost:6379` 上的 Redis 服务器。

- Java
- Kotlin

```java
import org.springframework.data.redis.core.StringRedisTemplate;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final StringRedisTemplate template;

    public MyBean(StringRedisTemplate template) {
        this.template = template;
    }

    // ...
    public Boolean someMethod() {
        return this.template.hasKey("spring");
    }

}
```

```kotlin
import org.springframework.data.redis.core.StringRedisTemplate
import org.springframework.stereotype.Component

@Component
class MyBean(private val template: StringRedisTemplate) {

    // ...
    fun someMethod(): Boolean {
        return template.hasKey("spring")
    }

}
```

您可以使用 `spring.data.redis.*` 属性指定自定义连接详细信息。

- 属性
- YAML

```properties
spring.data.redis.host=localhost
spring.data.redis.port=6379
spring.data.redis.database=0
spring.data.redis.username=user
spring.data.redis.password=secret
```

```yaml
spring:
  data:
    redis:
      host: "localhost"
      port: 6379
      database: 0
      username: "user"
      password: "secret"
```

## MongoDB

MongoDB 是一个开源的 NoSQL 文档数据库，它使用类似 JSON 的模式而不是传统的基于表的关联数据。Spring Boot 为使用 MongoDB 提供了多种便利，包括 `spring-boot-starter-data-mongodb` 和 `spring-boot-starter-data-mongodb-reactive` 启动器。

### 连接到 MongoDB 数据库

要访问 MongoDB 数据库，您可以注入一个自动配置的 `MongoDatabaseFactory`。默认情况下，实例尝试连接到 `mongodb:///test` 上的 MongoDB 服务器。

- Java
- Kotlin

```java
import com.mongodb.client.MongoCollection;
import com.mongodb.client.MongoDatabase;
import org.bson.Document;

import org.springframework.data.mongodb.MongoDatabaseFactory;
import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final MongoDatabaseFactory mongo;

    public MyBean(MongoDatabaseFactory mongo) {
        this.mongo = mongo;
    }

    // ...
    public MongoCollection<Document> someMethod() {
        MongoDatabase db = this.mongo.getMongoDatabase();
        return db.getCollection("users");
    }

}
```

```kotlin
import com.mongodb.client.MongoCollection
import org.bson.Document
import org.springframework.data.mongodb.MongoDatabaseFactory
import org.springframework.stereotype.Component

@Component
class MyBean(private val mongo: MongoDatabaseFactory) {

    // ...
    fun someMethod(): MongoCollection<Document> {
        val db = mongo.mongoDatabase
        return db.getCollection("users")
    }

}
```

您可以设置 `spring.mongodb.uri` 属性来更改 URL 并配置其他设置，例如副本集。

- 属性
- YAML

```properties
spring.mongodb.uri=mongodb://user:secret@mongoserver1.example.com:27017,mongoserver2.example.com:23456/test
```

```yaml
spring:
  mongodb:
    uri: "mongodb://user:secret@mongoserver1.example.com:27017,mongoserver2.example.com:23456/test"
```

### Spring Data MongoDB 存储库

Spring Data 包含对 MongoDB 的存储库支持。与前面讨论的 JPA 存储库一样，基本原则是查询根据方法名称自动构建。

实际上，Spring Data JPA 和 Spring Data MongoDB 都共享相同的通用基础设施。您可以采用前面的 JPA 示例，并假设 `City` 现在是 MongoDB 数据类而不是 JPA `@Entity`，它以相同的方式工作。

- Java
- Kotlin

```java
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.repository.Repository;

public interface CityRepository extends Repository<City, Long> {

    Page<City> findAll(Pageable pageable);

    City findByNameAndStateAllIgnoringCase(String name, String state);

}
```

```kotlin
import org.springframework.data.domain.Page
import org.springframework.data.domain.Pageable
import org.springframework.data.repository.Repository

interface CityRepository :
    Repository<City, Long> {
    fun findAll(pageable: Pageable?): Page<City>
    fun findByNameAndStateAllIgnoringCase(name: String, state: String): City?
}
```

## Neo4j

Neo4j 是一个开源的 NoSQL 图数据库，它使用丰富的由一流关系连接的节点数据模型，这比传统的关系型数据库方法更适合连接大数据。Spring Boot 为使用 Neo4j 提供了多种便利，包括 `spring-boot-starter-data-neo4j` 启动器。

### 连接到 Neo4j 数据库

要访问 Neo4j 服务器，您可以注入一个自动配置的 `Driver`。默认情况下，该实例尝试使用 Bolt 协议连接到 `localhost:7687` 上的 Neo4j 服务器。

- Java
- Kotlin

```java
import org.neo4j.driver.Driver;
import org.neo4j.driver.Session;
import org.neo4j.driver.Values;

import org.springframework.stereotype.Component;

@Component
public class MyBean {

    private final Driver driver;

    public MyBean(Driver driver) {
        this.driver = driver;
    }

    // ...
    public String someMethod(String message) {
        try (Session session = this.driver.session()) {
            return session.executeWrite(
                    (transaction) -> transaction
                            .run("CREATE (a:Greeting) SET a.message = $message RETURN a.message + ', from node ' + id(a)",
                                    Values.parameters("message", message))
                            .single()
                            .get(0)
                            .asString());
        }
    }

}
```

```kotlin
import org.neo4j.driver.Driver
import org.neo4j.driver.TransactionContext
import org.neo4j.driver.Values
import org.springframework.stereotype.Component

@Component
class MyBean(private val driver: Driver) {

    // ...
    fun someMethod(message: String?): String {
        driver.session().use { session ->
            return@someMethod session.executeWrite { transaction: TransactionContext ->
                transaction
                    .run(
                        "CREATE (a:Greeting) SET a.message = $message RETURN a.message + ', from node ' + id(a)",
                        Values.parameters("message", message)
                    )
                    .single()[0].asString()
            }
        }
    }

}
```

您可以使用 `spring.neo4j.*` 属性配置驱动程序的各个方面。

- 属性
- YAML

```properties
spring.neo4j.uri=bolt://my-server:7687
spring.neo4j.authentication.username=neo4j
spring.neo4j.authentication.password=secret
```

```yaml
spring:
  neo4j:
    uri: "bolt://my-server:7687"
    authentication:
      username: "neo4j"
      password: "secret"
```

## Elasticsearch

Elasticsearch 是一个开源、分布式、RESTful 搜索和分析引擎。Spring Boot 为 Elasticsearch 客户端提供了基本的自动配置。

Spring Boot 支持多种客户端：

- 官方低级 REST 客户端
- 官方 Java API 客户端
- Spring Data Elasticsearch 提供的 `ReactiveElasticsearchClient`

Spring Boot 提供了一个专用的启动器 `spring-boot-starter-data-elasticsearch`。

### 使用 REST 客户端连接到 Elasticsearch

Elasticsearch 提供了两个不同的 REST 客户端，您可以使用它们来查询集群：低级客户端和 Java API 客户端。Java API 客户端由 `co.elastic.clients:elasticsearch-java` 模块提供，低级客户端由 `co.elastic.clients:elasticsearch-rest5-client` 模块提供。

默认情况下，客户端将目标设置为 `localhost:9200`。您可以使用 `spring.elasticsearch.*` 属性进一步调整客户端的配置方式。

- 属性
- YAML

```properties
spring.elasticsearch.uris=https://search.example.com:9200
spring.elasticsearch.socket-timeout=10s
spring.elasticsearch.username=user
spring.elasticsearch.password=secret
```

```yaml
spring:
  elasticsearch:
    uris: "https://search.example.com:9200"
    socket-timeout: "10s"
    username: "user"
    password: "secret"
```

## Cassandra

Cassandra 是一个开源的分布式数据库管理系统，旨在处理大量数据跨多个商用服务器。Spring Boot 为 Cassandra 和 Spring Data Cassandra 提供的基于它的抽象提供了自动配置。有一个 `spring-boot-starter-data-cassandra` 启动器，用于以方便的方式收集依赖项。

### 连接到 Cassandra

您可以像处理任何其他 Spring Bean 一样注入自动配置的 `CqlTemplate`、`CassandraTemplate` 或 Cassandra `CqlSession` 实例。`spring.cassandra.*` 属性可用于自定义连接。通常，您提供 `keyspace-name` 和 `contact-points` 以及本地数据中心名称。

- 属性
- YAML

```properties
spring.cassandra.keyspace-name=mykeyspace
spring.cassandra.contact-points=cassandrahost1:9042,cassandrahost2:9042
spring.cassandra.local-datacenter=datacenter1
```

```yaml
spring:
  cassandra:
    keyspace-name: "mykeyspace"
    contact-points: "cassandrahost1:9042,cassandrahost2:9042"
    local-datacenter: "datacenter1"
```

## Couchbase

Couchbase 是一款开源、分布式、多模型 NoSQL 文档导向数据库，专为交互式应用而优化。Spring Boot 为 Couchbase 及其上层抽象（由 Spring Data Couchbase 提供）提供了自动配置。有 `spring-boot-starter-data-couchbase` 和 `spring-boot-starter-data-couchbase-reactive` 启动器，用于以便捷的方式收集依赖项。

### 连接到 Couchbase

您可以通过添加 Couchbase SDK 和一些配置来获取 `Cluster`。`spring.couchbase.*` 属性可用于自定义连接。通常，您提供连接字符串和用于身份验证的凭据。

- 属性
- YAML

```properties
spring.couchbase.connection-string=couchbase://192.168.1.123
spring.couchbase.username=user
spring.couchbase.password=secret
```

```yaml
spring:
  couchbase:
    connection-string: "couchbase://192.168.1.123"
    username: "user"
    password: "secret"
```

## LDAP

LDAP（轻量级目录访问协议）是一个开放的、与供应商无关的行业标准应用协议，用于通过 IP 网络访问和维护分布式目录信息服务。Spring Boot 为任何兼容的 LDAP 服务器以及 UnboundID 提供的嵌入式内存中 LDAP 服务器提供了自动配置。

LDAP 抽象由 Spring Data LDAP 提供。有一个 `spring-boot-starter-data-ldap` 启动器，用于以方便的方式收集依赖项。

### 连接到 LDAP 服务器

要连接到 LDAP 服务器，请确保您声明了对 `spring-boot-starter-data-ldap` 启动器或 `spring-ldap-core` 的依赖，然后在您的 application.properties 中声明您的服务器 URL。

- 属性
- YAML

```properties
spring.ldap.urls=ldap://myserver:1235
spring.ldap.username=admin
spring.ldap.password=secret
```

```yaml
spring:
  ldap:
    urls: "ldap://myserver:1235"
    username: "admin"
    password: "secret"
```
