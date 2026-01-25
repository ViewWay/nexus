# Remember Me Authentication / 记住我认证

## Overview / 概述

Remember me functionality allows users to remain authenticated across sessions even after the browser is closed. Spring Security provides two main implementations for remember-me authentication.

"记住我"功能允许用户在浏览器关闭后仍然保持跨会话的认证状态。Spring Security 为记住我认证提供了两种主要实现。

## RememberMeServices Interface / RememberMeServices 接口

The remember-me functionality works with `UsernamePasswordAuthenticationFilter` and is implemented through hooks in the `AbstractAuthenticationProcessingFilter` superclass. It's also used in `BasicAuthenticationFilter`.

记住我功能与 `UsernamePasswordAuthenticationFilter` 一起工作，并通过 `AbstractAuthenticationProcessingFilter` 超类的钩子函数实现。它也用于 `BasicAuthenticationFilter` 中。

```java
public interface RememberMeServices {
    Authentication autoLogin(HttpServletRequest request, HttpServletResponse response);

    void loginFail(HttpServletRequest request, HttpServletResponse response);

    void loginSuccess(HttpServletRequest request, HttpServletResponse response,
        Authentication successfulAuthentication);
}
```

The `autoLogin()` method is called by `RememberMeAuthenticationFilter` when `SecurityContextHolder` does not contain an `Authentication`.

`autoLogin()` 方法在 `SecurityContextHolder` 不包含 `Authentication` 时由 `RememberMeAuthenticationFilter` 调用。

## TokenBasedRememberMeServices / 基于令牌的记住我服务

This implementation supports a simple hash-based token approach. `TokenBasedRememberMeServices` generates a `RememberMeAuthenticationToken` that is processed by `RememberMeAuthenticationProvider`.

此实现支持简单的基于哈希的令牌方法。`TokenBasedRememberMeServices` 生成一个由 `RememberMeAuthenticationProvider` 处理的 `RememberMeAuthenticationToken`。

Features:
- Uses SHA-256 algorithm by default for token signing
- Shared key between provider and service
- Requires `UserDetailsService` for password verification
- Also implements `LogoutHandler` to clear cookies

特性：
- 默认使用 SHA-256 算法对令牌签名
- 提供者和服务之间共享密钥
- 需要 `UserDetailsService` 进行密码验证
- 同时实现 `LogoutHandler` 以清除 Cookie

### Configuration / 配置

```java
@Bean
SecurityFilterChain securityFilterChain(HttpSecurity http, RememberMeServices rememberMeServices) throws Exception {
    http
        .authorizeHttpRequests((authorize) -> authorize
            .anyRequest().authenticated()
        )
        .rememberMe((remember) -> remember
            .rememberMeServices(rememberMeServices)
        );
    return http.build();
}

@Bean
RememberMeServices rememberMeServices(UserDetailsService userDetailsService) {
    RememberMeTokenAlgorithm encodingAlgorithm = RememberMeTokenAlgorithm.SHA256;
    TokenBasedRememberMeServices rememberMe =
        new TokenBasedRememberMeServices(myKey, userDetailsService, encodingAlgorithm);
    rememberMe.setMatchingAlgorithm(RememberMeTokenAlgorithm.MD5);
    return rememberMe;
}
```

## PersistentTokenBasedRememberMeServices / 持久化令牌记住我服务

This implementation requires a `PersistentTokenRepository` to store tokens. It's more secure as it doesn't require the password in the token.

此实现需要一个 `PersistentTokenRepository` 来存储令牌。它更安全，因为令牌中不需要密码。

### Token Repository Implementations / 令牌仓库实现

- **InMemoryTokenRepositoryImpl** - For testing only (仅用于测试)
- **JdbcTokenRepositoryImpl** - Stores tokens in database (将令牌存储在数据库中)

### Database Schema / 数据库架构

```sql
CREATE TABLE persistent_logins (
    username VARCHAR(64) NOT NULL,
    series VARCHAR(64) NOT NULL,
    token VARCHAR(64) NOT NULL,
    last_used TIMESTAMP NOT NULL,
    PRIMARY KEY (series)
);
```

### Configuration / 配置

```java
@Bean
SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
    http
        .rememberMe((remember) -> remember
            .tokenRepository(persistentTokenRepository())
            .userDetailsService(userDetailsService)
            .tokenValiditySeconds(86400)  // 1 day
        );
    return http.build();
}

@Bean
PersistentTokenRepository persistentTokenRepository(DataSource dataSource) {
    JdbcTokenRepositoryImpl tokenRepository = new JdbcTokenRepositoryImpl();
    tokenRepository.setDataSource(dataSource);
    return tokenRepository;
}
```

## Required Beans / 必需的 Bean

To enable remember-me services, the application context needs the following beans:

要启用记住我服务，应用程序上下文需要以下 Bean：

```java
@Bean
RememberMeAuthenticationFilter rememberMeFilter() {
    RememberMeAuthenticationFilter rememberMeFilter = new RememberMeAuthenticationFilter();
    rememberMeFilter.setRememberMeServices(rememberMeServices());
    rememberMeFilter.setAuthenticationManager(theAuthenticationManager);
    return rememberMeFilter;
}

@Bean
TokenBasedRememberMeServices rememberMeServices() {
    TokenBasedRememberMeServices rememberMeServices = new TokenBasedRememberMeServices();
    rememberMeServices.setUserDetailsService(myUserDetailsService);
    rememberMeServices.setKey("springRocks");
    return rememberMeServices;
}

@Bean
RememberMeAuthenticationProvider rememberMeAuthenticationProvider() {
    RememberMeAuthenticationProvider rememberMeAuthenticationProvider =
        new RememberMeAuthenticationProvider();
    rememberMeAuthenticationProvider.setKey("springRocks");
    return rememberMeAuthenticationProvider;
}
```

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/authentication/rememberme.html*
