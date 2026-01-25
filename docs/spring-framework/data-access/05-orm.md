# 对象关系映射 (ORM) 数据访问

Object-Relational Mapping (ORM) Data Access

本节介绍在使用对象关系映射 (ORM) 时的数据访问。

## 本节内容

### Spring ORM 简介

介绍 Spring Framework 对 ORM 的支持，包括 Spring 与各种 ORM 框架集成的概述。

### 通用 ORM 集成注意事项

讨论在使用 ORM 框架时需要考虑的通用问题，包括：

- 资源管理
- 事务管理
- 异常转换
- 应用上下文中的资源绑定

### Hibernate

详细介绍 Spring 与 Hibernate 的集成，包括：

- 在 Spring 中配置 SessionFactory
- HibernateTemplate 的使用
- 声明式事务管理
- 异常转换

### JPA (Java Persistence API)

详细介绍 Spring 与 JPA 的集成，包括：

- 在 Spring 中配置 EntityManagerFactory
- JPA 的使用方式
- 声明式事务管理
- JPA 异常转换

## ORM 集成的主要优势

Spring 的 ORM 集成提供以下优势：

1. **易于集成** - Spring 提供了与 Hibernate、JPA、JDO、iBATIS 等流行 ORM 框架的无缝集成。

2. **一致的事务管理** - 可以使用 Spring 的统一事务管理 API，无论使用哪种 ORM 框架。

3. **异常转换** - 将特定框架的异常转换为 Spring 的 DataAccessException 层次结构。

4. **资源管理** - 自动管理 ORM 框架的资源，如 Session、EntityManager 等。

5. **线程安全** - 提供线程安全的模板类，简化 DAO 层的实现。

## 基本配置示例

### JPA 配置示例

```java
@Configuration
@EnableTransactionManagement
public class JpaConfig {

    @Bean
    public LocalContainerEntityManagerFactoryBean entityManagerFactory(DataSource dataSource) {
        LocalContainerEntityManagerFactoryBean em = new LocalContainerEntityManagerFactoryBean();
        em.setDataSource(dataSource);
        em.setPackagesToScan("com.example.domain");
        em.setJpaVendorAdapter(new HibernateJpaVendorAdapter());
        return em;
    }

    @Bean
    public PlatformTransactionManager transactionManager(EntityManagerFactory emf) {
        JpaTransactionManager transactionManager = new JpaTransactionManager();
        transactionManager.setEntityManagerFactory(emf);
        return transactionManager;
    }
}
```

### Hibernate 配置示例

```java
@Configuration
@EnableTransactionManagement
public class HibernateConfig {

    @Bean
    public LocalSessionFactoryBean sessionFactory(DataSource dataSource) {
        LocalSessionFactoryBean sessionFactory = new LocalSessionFactoryBean();
        sessionFactory.setDataSource(dataSource);
        sessionFactory.setPackagesToScan("com.example.domain");
        sessionFactory.setHibernateProperties(hibernateProperties());
        return sessionFactory;
    }

    @Bean
    public PlatformTransactionManager transactionManager(SessionFactory sessionFactory) {
        HibernateTransactionManager transactionManager = new HibernateTransactionManager();
        transactionManager.setSessionFactory(sessionFactory);
        return transactionManager;
    }

    private Properties hibernateProperties() {
        Properties properties = new Properties();
        properties.put("hibernate.dialect", "org.hibernate.dialect.MySQLDialect");
        properties.put("hibernate.show_sql", true);
        properties.put("hibernate.format_sql", true);
        return properties;
    }
}
```

---

*来源: [Spring Framework 官方文档](https://docs.springframework.org.cn/spring-framework/reference/data-access/orm.html)*
