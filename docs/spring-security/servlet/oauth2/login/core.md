# OAuth 2.0 Login Core Configuration / OAuth 2.0 登录核心配置

## Spring Boot Auto-Configuration / Spring Boot 自动配置

The `OAuth2ClientAutoConfiguration` class performs the following tasks:

`OAuth2ClientAutoConfiguration` 类执行以下任务：

1. Registers a `ClientRegistrationRepository` `@Bean` composed of `ClientRegistration` instances from the configured OAuth client properties
2. Registers a `SecurityFilterChain` `@Bean` and enables OAuth 2.0 Login via `httpSecurity.oauth2Login()`

1. 注册一个由配置的 OAuth 客户端属性中的 `ClientRegistration` 组成的 `ClientRegistrationRepository` `@Bean`
2. 注册一个 `SecurityFilterChain` `@Bean` 并通过 `httpSecurity.oauth2Login()` 启用 OAuth 2.0 登录

## Registering a ClientRegistrationRepository @Bean / 注册 ClientRegistrationRepository @Bean

The following example shows how to register a `ClientRegistrationRepository` `@Bean`:

以下示例显示了如何注册一个 `ClientRegistrationRepository` `@Bean`：

```java
@Configuration
public class OAuth2LoginConfig {

    @Bean
    public ClientRegistrationRepository clientRegistrationRepository() {
        return new InMemoryClientRegistrationRepository(this.googleClientRegistration());
    }

    private ClientRegistration googleClientRegistration() {
        return ClientRegistration.withRegistrationId("google")
            .clientId("google-client-id")
            .clientSecret("google-client-secret")
            .clientAuthenticationMethod(ClientAuthenticationMethod.CLIENT_SECRET_BASIC)
            .authorizationGrantType(AuthorizationGrantType.AUTHORIZATION_CODE)
            .redirectUri("{baseUrl}/login/oauth2/code/{registrationId}")
            .scope("openid", "profile", "email", "address", "phone")
            .authorizationUri("https://accounts.google.com/o/oauth2/v2/auth")
            .tokenUri("https://www.googleapis.com/oauth2/v4/token")
            .userInfoUri("https://www.googleapis.com/oauth2/v3/userinfo")
            .userNameAttributeName(IdTokenClaimNames.SUB)
            .jwkSetUri("https://www.googleapis.com/oauth2/v3/certs")
            .clientName("Google")
            .build();
    }
}
```

## Registering a SecurityFilterChain @Bean / 注册 SecurityFilterChain @Bean

The following example shows how to register a `SecurityFilterChain` `@Bean` with `@EnableWebSecurity` and enable OAuth 2.0 Login via `httpSecurity.oauth2Login()`:

以下示例显示了如何使用 `@EnableWebSecurity` 注册一个 `SecurityFilterChain` `@Bean` 并通过 `httpSecurity.oauth2Login()` 启用 OAuth 2.0 登录：

```java
@Configuration
@EnableWebSecurity
public class OAuth2LoginSecurityConfig {

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

## Completely Overriding Auto-Configuration / 完全覆盖自动配置

The following example shows how to completely override the auto-configuration by registering both a `ClientRegistrationRepository` `@Bean` and a `SecurityFilterChain` `@Bean`:

以下示例显示了如何通过同时注册 `ClientRegistrationRepository` `@Bean` 和 `SecurityFilterChain` `@Bean` 来完全覆盖自动配置：

```java
@Configuration
public class OAuth2LoginConfig {

    @Bean
    public SecurityFilterChain filterChain(HttpSecurity http) throws Exception {
        http
            .authorizeHttpRequests(authorize -> authorize
                .anyRequest().authenticated()
            )
            .oauth2Login(withDefaults());
        return http.build();
    }

    @Bean
    public ClientRegistrationRepository clientRegistrationRepository() {
        return new InMemoryClientRegistrationRepository(this.googleClientRegistration());
    }

    private ClientRegistration googleClientRegistration() {
        return ClientRegistration.withRegistrationId("google")
            .clientId("google-client-id")
            .clientSecret("google-client-secret")
            .clientAuthenticationMethod(ClientAuthenticationMethod.CLIENT_SECRET_BASIC)
            .authorizationGrantType(AuthorizationGrantType.AUTHORIZATION_CODE)
            .redirectUri("{baseUrl}/login/oauth2/code/{registrationId}")
            .scope("openid", "profile", "email", "address", "phone")
            .authorizationUri("https://accounts.google.com/o/oauth2/v2/auth")
            .tokenUri("https://www.googleapis.com/oauth2/v4/token")
            .userInfoUri("https://www.googleapis.com/oauth2/v3/userinfo")
            .userNameAttributeName(IdTokenClaimNames.SUB)
            .jwkSetUri("https://www.googleapis.com/oauth2/v3/certs")
            .clientName("Google")
            .build();
    }
}
```

## Common Providers / 常见提供商

### Google / 谷歌

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

### GitHub / GitHub

```yaml
spring:
  security:
    oauth2:
      client:
        registration:
          github:
            client-id: your-client-id
            client-secret: your-client-secret
```

### Facebook / Facebook

```yaml
spring:
  security:
    oauth2:
      client:
        registration:
          facebook:
            client-id: your-client-id
            client-secret: your-client-secret
```

### Okta / Okta

```yaml
spring:
  security:
    oauth2:
      client:
        registration:
          okta:
            client-id: your-client-id
            client-secret: your-client-secret
        provider:
          okta:
            issuer-uri: https://your-okta-domain.okta.com/oauth2/default
```

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/oauth2/login/core.html*
