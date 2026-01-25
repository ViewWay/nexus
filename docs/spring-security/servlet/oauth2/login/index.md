# OAuth 2.0 Login / OAuth 2.0 登录

## Overview / 概述

OAuth 2.0 Login功能允许应用程序用户使用他们在OAuth 2.0提供商（例如GitHub）或OpenID Connect 1.0提供商（例如Google）上的现有账户登录应用程序。OAuth 2.0登录实现了两种用例："使用Google登录"或"使用GitHub登录"。

The OAuth 2.0 Login feature allows an application to have the user log in using their existing account at an OAuth 2.0 Provider (e.g. GitHub) or OpenID Connect 1.0 Provider (e.g. Google). OAuth 2.0 Login implements the use cases: "Login with Google" or "Login with GitHub".

OAuth 2.0 登录是根据 OAuth 2.0 授权框架 和 OpenID Connect Core 1.0 中指定的 _授权码授权_ 来实现的。

OAuth 2.0 Login is implemented using the _Authorization Code Grant_ specified in OAuth 2.0 Authorization Framework and OpenID Connect Core 1.0.

## Chapter Summary / 章节摘要

- Core Configuration / 核心配置
- Advanced Configuration / 高级配置
- OIDC Logout / OIDC 注销

## Basic Use Cases / 基本用例

### Login with OAuth 2.0 Provider / 使用 OAuth 2.0 提供商登录

Enable OAuth 2.0 Login in your SecurityFilterChain:

在您的 SecurityFilterChain 中启用 OAuth 2.0 登录：

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
            .oauth2Login(withDefaults());
        return http.build();
    }
}
```

### Configure Client Registration / 配置客户端注册

```yaml
spring:
  security:
    oauth2:
      client:
        registration:
          google:
            client-id: your-client-id
            client-secret: your-client-secret
```

## How it Works / 工作原理

1. User clicks "Login with Google"
2. Application redirects to Google's authorization endpoint
3. User authenticates with Google and grants consent
4. Google redirects back to application with authorization code
5. Application exchanges code for access token
6. Application retrieves user information
7. User is authenticated in the application

1. 用户点击"使用 Google 登录"
2. 应用程序重定向到 Google 的授权端点
3. 用户使用 Google 进行身份验证并授予同意
4. Google 带着授权码重定向回应用程序
5. 应用程序交换代码以获取访问令牌
6. 应用程序检索用户信息
7. 用户在应用程序中完成身份验证

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/oauth2/login/index.html*
