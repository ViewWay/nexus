# OAuth2 Overview / OAuth2 概述

## Overview / 概述

Spring Security provides comprehensive OAuth 2.0 support. This section discusses how to integrate OAuth 2.0 into Servlet-based applications.

Spring Security 提供全面的 OAuth 2.0 支持。本节讨论如何将 OAuth 2.0 集成到基于 Servlet 的应用程序中。

## Main Feature Sets / 主要功能集

Spring Security's OAuth 2.0 support consists of two main feature sets:

Spring Security 的 OAuth 2.0 支持包含两个主要功能集：

- **OAuth2 Resource Server / OAuth2 资源服务器**
- **OAuth2 Client / OAuth2 客户端** (including OAuth2 Login / 包括 OAuth2 登录)

These feature sets cover the _Resource Server_ and _Client_ roles defined in the OAuth 2.0 Authorization Framework, while the _Authorization Server_ role is covered by Spring Authorization Server.

这些功能集涵盖了 OAuth 2.0 授权框架中定义的_资源服务器_和_客户端_角色，而_授权服务器_角色则由 Spring Authorization Server 涵盖。

## OAuth2 Resource Server / OAuth2 资源服务器

### Dependency / 依赖

To get started, add the `spring-security-oauth2-resource-server` dependency to your project. With Spring Boot, add the following starter:

```gradle
implementation 'org.springframework.boot:spring-boot-starter-oauth2-resource-server'
```

### JWT Support / JWT 支持

The following example configures a `JwtDecoder` bean using Spring Boot configuration properties:

```yaml
spring:
  security:
    oauth2:
      resourceserver:
        jwt:
          issuer-uri: https://my-auth-server.com
```

With Spring Boot, this is sufficient. The default arrangement provided by Spring Boot is equivalent to:

```java
@Configuration
@EnableWebSecurity
public class SecurityConfig {

    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
        http
            .authorizeHttpRequests((authorize) -> authorize
                .anyRequest().authenticated()
            )
            .oauth2ResourceServer((oauth2) -> oauth2
                .jwt(Customizer.withDefaults())
            );
        return http.build();
    }

    @Bean
    public JwtDecoder jwtDecoder() {
        return JwtDecoders.fromIssuerLocation("https://my-auth-server.com");
    }
}
```

### Opaque Token Support / 不透明令牌支持

The following example configures an `OpaqueTokenIntrospector` bean using Spring Boot configuration properties:

```yaml
spring:
  security:
    oauth2:
      resourceserver:
        opaquetoken:
          introspection-uri: https://my-auth-server.com/oauth2/introspect
          client-id: my-client-id
          client-secret: my-client-secret
```

Equivalent Java configuration:

```java
@Configuration
@EnableWebSecurity
public class SecurityConfig {

    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
        http
            .authorizeHttpRequests((authorize) -> authorize
                .anyRequest().authenticated()
            )
            .oauth2ResourceServer((oauth2) -> oauth2
                .opaqueToken(Customizer.withDefaults())
            );
        return http.build();
    }

    @Bean
    public OpaqueTokenIntrospector opaqueTokenIntrospector() {
        return new SpringOpaqueTokenIntrospector(
            "https://my-auth-server.com/oauth2/introspect", "my-client-id", "my-client-secret");
    }
}
```

## OAuth2 Client / OAuth2 客户端

### Dependency / 依赖

```gradle
implementation 'org.springframework.boot:spring-boot-starter-oauth2-client'
```

### OAuth2 Login / OAuth2 登录

The following example configures an application to act as an OAuth2 client capable of logging in users using OAuth2 or OpenID Connect:

```java
@Configuration
@EnableWebSecurity
public class SecurityConfig {

    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
        http
            .oauth2Login(Customizer.withDefaults());
        return http.build();
    }
}
```

Additionally, the application needs to configure at least one `ClientRegistration`:

```yaml
spring:
  security:
    oauth2:
      client:
        registration:
          my-oidc-client:
            provider: my-oidc-provider
            client-id: my-client-id
            client-secret: my-client-secret
            authorization-grant-type: authorization_code
            scope: openid,profile
        provider:
          my-oidc-provider:
            issuer-uri: https://my-oidc-provider.com
```

### Accessing Protected Resources / 访问受保护的资源

The following example configures an application to act as an OAuth2 client capable of requesting protected resources from a third-party API:

```java
@Configuration
@EnableWebSecurity
public class SecurityConfig {

    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
        http
            .oauth2Client(Customizer.withDefaults());
        return http.build();
    }
}
```

Configuration:

```yaml
spring:
  security:
    oauth2:
      client:
        registration:
          my-oauth2-client:
            provider: my-auth-server
            client-id: my-client-id
            client-secret: my-client-secret
            authorization-grant-type: authorization_code
            scope: message.read,message.write
        provider:
          my-auth-server:
            issuer-uri: https://my-auth-server.com
```

Using `WebClient` to access protected resources:

```java
@Configuration
public class WebClientConfig {

    @Bean
    public WebClient webClient(OAuth2AuthorizedClientManager authorizedClientManager) {
        ServletOAuth2AuthorizedClientExchangeFilterFunction filter =
            new ServletOAuth2AuthorizedClientExchangeFilterFunction(authorizedClientManager);
        return WebClient.builder()
            .apply(filter.oauth2Configuration())
            .build();
    }
}
```

Controller example:

```java
@RestController
public class MessagesController {
    private final WebClient webClient;

    public MessagesController(WebClient webClient) {
        this.webClient = webClient;
    }

    @GetMapping("/messages")
    public ResponseEntity<List<Message>> messages() {
        return this.webClient.get()
            .uri("https://127.0.0.1:8090/messages")
            .attributes(clientRegistrationId("my-oauth2-client"))
            .retrieve()
            .toEntityList(Message.class)
            .block();
    }

    public record Message(String message) { }
}
```

## Grant Types / 授权类型

Spring Security supports multiple OAuth2 grant types:

Spring Security 支持多种 OAuth2 授权类型：

- **Authorization Code / 授权码** - For user-facing applications (用于面向用户的应用程序)
- **Client Credentials / 客户端凭据** - for service-to-service communication (用于服务间通信)
- **Refresh Token / 刷新令牌** - For obtaining new access tokens (用于获取新的访问令牌)
- **Password / 密码** - Legacy, not recommended (传统方式，不推荐)
- **JWT Bearer** - Extension grant type (扩展授权类型)
- **Token Exchange** - Extension grant type (扩展授权类型)

### Enabling Extended Grant Types / 启用扩展授权类型

```java
@Configuration
public class SecurityConfig {
    @Bean
    public OAuth2AuthorizedClientProvider jwtBearer() {
        return new JwtBearerOAuth2AuthorizedClientProvider();
    }
}
```

## Further Reading / 进一步阅读

- OAuth 2.0 Login
- OAuth 2.0 Client
- OAuth 2.0 Resource Server

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/oauth2/index.html*
