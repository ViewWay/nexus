# Authentication - Servlet Applications

## Overview / 概述

Authentication is the process of verifying a user's identity. Spring Security provides comprehensive support for various authentication mechanisms.

认证是验证用户身份的过程。Spring Security 为各种认证机制提供全面支持。

## Authentication Architecture / 认证架构

Spring Security's authentication architecture consists of several key components:

Spring Security 的认证架构由以下几个关键组件组成：

### AuthenticationManager / 认证管理器

The `AuthenticationManager` is the main strategy for authentication.

`AuthenticationManager` 是认证的主要策略。

```java
public interface AuthenticationManager {
    Authentication authenticate(Authentication authentication)
        throws AuthenticationException;
}
```

### ProviderManager / 提供者管理器

`ProviderManager` is the most common implementation of `AuthenticationManager`. It delegates to a list of `AuthenticationProvider` instances.

`ProviderManager` 是 `AuthenticationManager` 最常见的实现。它委托给 `AuthenticationProvider` 实例列表。

### AuthenticationProvider / 认证提供者

```java
public interface AuthenticationProvider {
    Authentication authenticate(Authentication authentication)
        throws AuthenticationException;

    boolean supports(Class<?> authentication);
}
```

### UserDetailsService / 用户详情服务

```java
public interface UserDetailsService {
    UserDetails loadUserByUsername(String username)
        throws UsernameNotFoundException;
}
```

## Supported Authentication Mechanisms / 支持的认证机制

### Password-based Authentication / 基于密码的认证

- Form-based login
- HTTP Basic authentication
- Digest authentication

### Token-based Authentication / 基于令牌的认证

- JWT (JSON Web Tokens)
- OAuth2
- SAML 2.0

### Other Mechanisms / 其他机制

- LDAP
- JAAS
- X.509 certificates
- Remember-me authentication
- Anonymous authentication

## Security Context / 安全上下文

The `SecurityContext` holds the authenticated user's information:

`SecurityContext` 保存已认证用户的信息：

```java
SecurityContext context = SecurityContextHolder.getContext();
Authentication authentication = context.getAuthentication();
String username = authentication.getName();
```

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/authentication/index.html*
