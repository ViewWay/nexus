# Method Security

## Overview / 概述

Spring Security supports method security using annotations like `@PreAuthorize`, `@PostAuthorize`, `@PreFilter`, and `@PostFilter`. These annotations allow fine-grained access control at the method level.

Spring Security 支持使用 `@PreAuthorize`、`@PostAuthorize`、`@PreFilter` 和 `@PostFilter` 等注解实现方法级安全。这些注解允许在方法级别进行细粒度的访问控制。

## Enable Method Security / 启用方法安全

To enable method security, add the `@EnableMethodSecurity` annotation to a configuration class:

要启用方法安全，请在配置类上添加 `@EnableMethodSecurity` 注解：

```java
@Configuration
@EnableMethodSecurity
public class SecurityConfig {
    // ...
}
```

## @PreAuthorize / 预授权

`@PreAuthorize` is used to check authorization before method execution. It supports SpEL expressions.

`@PreAuthorize` 用于在方法执行前检查授权。它支持 SpEL 表达式。

```java
@Service
public class DocumentService {

    @PreAuthorize("hasRole('ADMIN')")
    public Document getDocument(String id) {
        // ...
    }

    @PreAuthorize("#userId == authentication.principal.id")
    public User getUser(String userId) {
        // ...
    }
}
```

## @PostAuthorize / 后授权

`@PostAuthorize` is used to check authorization after method execution. This is useful when you need to filter the return value.

`@PostAuthorize` 用于在方法执行后检查授权。这在需要对返回值进行过滤时很有用。

```java
@PostAuthorize("returnObject.owner == authentication.principal.username")
public Document getDocument(String id) {
    // ...
}
```

## @PreFilter / 预过滤

`@PreFilter` filters the input collection before method execution.

`@PreFilter` 在方法执行前过滤输入集合。

```java
@PreFilter("filterObject.owner == authentication.principal.username")
public void updateDocuments(List<Document> documents) {
    // ...
}
```

## @PostFilter / 后过滤

`@PostFilter` filters the returned collection after method execution.

`@PostFilter` 在方法执行后过滤返回的集合。

```java
@PostFilter("filterObject.owner == authentication.principal.username")
public List<Document> getAllDocuments() {
    // ...
}
```

## Secured Annotations / @Secured 注解

The `@Secured` annotation provides a simpler alternative for role-based checks.

`@Secured` 注解为基于角色的检查提供了更简单的替代方案。

```java
@Secured("ROLE_ADMIN")
public void adminOperation() {
    // ...
}
```

## JSR-250 Annotations / JSR-250 注解

Spring Security also supports JSR-250 annotations like `@RolesAllowed`.

Spring Security 还支持 `@RolesAllowed` 等 JSR-250 注解。

```java
@RolesAllowed("ROLE_USER")
public void userOperation() {
    // ...
}
```

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/authorization/method-security.html*
