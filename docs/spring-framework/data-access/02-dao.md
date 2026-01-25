# DAO 支持

DAO Support

Spring 的 Data Access Object (DAO) 支持旨在提供一种一致的方式，使处理数据访问技术（如 JDBC、Hibernate 或 JPA）变得容易。这使你可以相当轻松地在上述持久化技术之间切换，并且无需担心捕获特定于每种技术的异常即可编写代码。

## 一致的异常层次结构

Spring 提供了一种便捷的转换，将技术特定的异常（如 `SQLException`）转换为它自己的异常类层次结构，该层次结构的根异常是 `DataAccessException`。这些异常会包装原始异常，因此你绝不会有丢失有关发生错误的信息的风险。

除了 JDBC 异常外，Spring 还可以包装 JPA 和 Hibernate 特定的异常，将它们转换为一组集中的运行时异常。这使得你可以在适当的层中处理大多数不可恢复的持久化异常，而无需在你的 DAO 中编写烦人的样板 catch-and-throw 代码块和异常声明。

前面的讨论对于 Spring 支持各种 ORM 框架中的各种模板类同样适用。如果你使用基于拦截器的类，应用程序本身必须处理 `HibernateExceptions` 和 `PersistenceExceptions`，最好分别委托给 `SessionFactoryUtils` 的 `convertHibernateAccessException(..)` 或 `convertJpaAccessException(..)` 方法。

由于 `PersistenceExceptions` 是 unchecked 异常，它们也可以被抛出（尽管牺牲了异常层面的通用 DAO 抽象）。

下图展示了 Spring 提供的异常层次结构（详细的类层次结构仅显示了整个 `DataAccessException` 层次结构的一部分）。

![DataAccessException 层次结构](https://docs.springframework.org.cn/spring-framework/reference/_images/DataAccessException.png)

## 用于配置 DAO 或 Repository 类的注解

确保你的 Data Access Objects (DAO) 或 repository 提供异常转换的最佳方法是使用 `@Repository` 注解。这个注解还允许组件扫描支持找到并配置你的 DAO 和 repository，而无需为其提供 XML 配置条目。

### 使用 @Repository 注解

**Java:**
```java
@Repository
public class SomeMovieFinder implements MovieFinder {
    // ...
}
```

**Kotlin:**
```kotlin
@Repository
class SomeMovieFinder : MovieFinder {
    // ...
}
```

### 注入持久化资源

任何 DAO 或 repository 实现都需要访问持久化资源，这取决于所使用的持久化技术。

#### JPA Repository 示例

**Java:**
```java
@Repository
public class JpaMovieFinder implements MovieFinder {

    @PersistenceContext
    private EntityManager entityManager;

    // ...
}
```

**Kotlin:**
```kotlin
@Repository
class JpaMovieFinder : MovieFinder {

    @PersistenceContext
    private lateinit var entityManager: EntityManager

    // ...
}
```

#### Hibernate Repository 示例

**Java:**
```java
@Repository
public class HibernateMovieFinder implements MovieFinder {

    private SessionFactory sessionFactory;

    @Autowired
    public void setSessionFactory(SessionFactory sessionFactory) {
        this.sessionFactory = sessionFactory;
    }

    // ...
}
```

**Kotlin:**
```kotlin
@Repository
class HibernateMovieFinder(private val sessionFactory: SessionFactory) : MovieFinder {
    // ...
}
```

#### JDBC Repository 示例

**Java:**
```java
@Repository
public class JdbcMovieFinder implements MovieFinder {

    private JdbcTemplate jdbcTemplate;

    @Autowired
    public void init(DataSource dataSource) {
        this.jdbcTemplate = new JdbcTemplate(dataSource);
    }

    // ...
}
```

**Kotlin:**
```kotlin
@Repository
class JdbcMovieFinder(dataSource: DataSource) : MovieFinder {

    private val jdbcTemplate = JdbcTemplate(dataSource)

    // ...
}
```

---

*来源: [Spring Framework 官方文档](https://docs.springframework.org.cn/spring-framework/reference/data-access/dao.html)*
