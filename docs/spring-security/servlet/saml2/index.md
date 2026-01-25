# SAML2 Overview / SAML2 概述

## Overview / 概述

Spring Security provides comprehensive SAML 2 support. This section discusses how to integrate SAML 2 into Servlet-based applications.

Spring Security 提供全面的 SAML 2 支持。本节讨论如何将 SAML 2 集成到基于 Servlet 的应用程序中。

## Chapter Summary / 章节摘要

- SAML2 Login / SAML2 登录
- SAML2 Logout / SAML2 注销
- SAML2 Metadata / SAML2 元数据

## What is SAML 2.0? / 什么是 SAML 2.0？

Security Assertion Markup Language (SAML) is an XML-based standard for exchanging authentication and authorization data between parties.

安全断言标记语言 (SAML) 是一种基于 XML 的标准，用于在各方之间交换身份验证和授权数据。

### Key Concepts / 关键概念

- **Service Provider (SP) / 服务提供商** - The application that wants to authenticate users
- **Identity Provider (IdP) / 身份提供商** - The service that authenticates users
- **SAML Assertion / SAML 断言** - XML message that contains authentication/authorization data
- **Metadata / 元数据** - Configuration information about SP or IdP

## Basic Configuration / 基本配置

### Dependency / 依赖

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-security</artifactId>
</dependency>
```

### Minimal Configuration / 最小配置

```java
@Configuration
@EnableWebSecurity
public class SecurityConfig {

    @Bean
    public SecurityFilterChain filterChain(HttpSecurity http) throws Exception {
        http
            .authorizeHttpRequests(authorize -> authorize
                .anyRequest().authenticated()
            )
            .saml2Login(withDefaults());
        return http.build();
    }
}
```

### Application Properties / 应用程序属性

```yaml
spring:
  security:
    saml2:
      relyingparty:
        registration:
          my-saml:
            assertingparty:
              entity-id: https://idp.example.com/identity
              metadata-uri: https://idp.example.com/identity/metadata
```

## SAML 2.0 Login Flow / SAML 2.0 登录流程

1. User accesses the application
2. Application generates SAML authentication request
3. User is redirected to Identity Provider
4. User authenticates with IdP
5. IdP generates SAML response
6. IdP redirects back to application with SAML response
7. Application validates SAML response
8. User is authenticated

1. 用户访问应用程序
2. 应用程序生成 SAML 认证请求
3. 用户被重定向到身份提供商
4. 用户使用 IdP 进行身份验证
5. IdP 生成 SAML 响应
6. IdP 带着响应重定向回应用程序
7. 应用程序验证 SAML 响应
8. 用户完成身份验证

## SAML 2.0 Logout / SAML 2.0 注销

Spring Security supports Single Logout (SLO) which allows users to logout from all applications when logging out from one.

Spring Security 支持单点注销 (SLO)，允许用户在从一个应用程序注销时从所有应用程序注销。

```java
http
    .saml2Logout((saml2) -> saml2
        .logoutUrl("/logout")
    );
```

## SAML 2.0 Metadata / SAML 2.0 元数据

Spring Security can generate metadata for your service provider:

Spring Security 可以为您的服务提供商生成元数据：

```java
http
    .saml2Metadata((metadata) -> metadata
        .metadataUrl("/saml2/service-provider-metadata/my-saml")
    );
```

## Common Identity Providers / 常见身份提供商

Spring Security works with any SAML 2.0-compliant identity provider including:

Spring Security 可以与任何符合 SAML 2.0 标准的身份提供商一起使用，包括：

- Okta
- Microsoft Azure AD
- Shibboleth
- Ping Identity
- Keycloak

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/saml2/index.html*
