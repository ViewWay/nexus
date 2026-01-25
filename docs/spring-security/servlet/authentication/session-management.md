# Session Management / 会话管理

## Overview / 概述

Once your application can authenticate requests, you need to consider how to persist and recover the authentication result in subsequent requests.

一旦你的应用能够验证请求，就需要考虑如何持久化并恢复后续请求中产生的认证结果。

By default, this happens automatically, so no additional code is required. However, it's important to understand the meaning of `requireExplicitSave` in `HttpSecurity`.

默认情况下，这会自动完成，因此不需要额外的代码，尽管了解 `HttpSecurity` 中的 `requireExplicitSave` 的含义很重要。

## Understanding Session Management Components / 了解会话管理的组件

Session management support is composed of several components that work together. These components include `SecurityContextHolderFilter`, `SecurityContextPersistenceFilter`, and `SessionManagementFilter`.

会话管理支持由几个协同工作的组件构成。这些组件包括 `SecurityContextHolderFilter`、`SecurityContextPersistenceFilter` 和 `SessionManagementFilter`。

### SecurityContextPersistenceFilter (Deprecated)

In Spring Security 5, the default behavior relied on `SessionManagementFilter` to detect when a user had just authenticated and call `SessionAuthenticationStrategy`. The problem was that this meant reading `HttpSession` on every request in a typical setup.

在 Spring Security 5 中，默认配置依赖于 `SessionManagementFilter` 来检测用户是否刚刚完成认证，并调用 `SessionAuthenticationStrategy`。问题在于，这意味着在典型的设置中，每次请求都必须读取 `HttpSession`。

In Spring Security 6, the default is that authentication mechanisms themselves must call `SessionAuthenticationStrategy`. This means there's no need to detect when an `Authentication` has completed, and therefore no need to read `HttpSession` on every request.

在 Spring Security 6 中，默认情况下，身份验证机制本身必须调用 `SessionAuthenticationStrategy`。这意味着无需检测何时完成 `Authentication`，因此每次请求都不需要读取 `HttpSession`。

### Customizing Authentication Storage Location / 自定义认证存储位置

By default, Spring Security stores the security context in the HTTP session. However, you might need to customize this for:

默认情况下，Spring Security 会将安全上下文存储在 HTTP session 中。但是，您可能出于以下几个原因需要自定义它：

- You may wish to call various setter methods on an `HttpSessionSecurityContextRepository` instance
- You may wish to store the security context in a cache or database to enable horizontal scaling

- 您可能希望在 `HttpSessionSecurityContextRepository` 实例上调用各个 setter 方法
- 您可能希望将安全上下文存储在缓存或数据库中以启用水平扩展

```java
@Bean
public SecurityFilterChain filterChain(HttpSecurity http) {
    SecurityContextRepository repo = new MyCustomSecurityContextRepository();
    http
        .securityContext((context) -> context
            .securityContextRepository(repo)
        );
    return http.build();
}
```

### Manual Authentication Storage / 手动存储认证

If you're manually authenticating users instead of relying on Spring Security filters:

如果您正在手动验证用户，而不是依赖于 Spring Security 过滤器：

```java
private SecurityContextRepository securityContextRepository =
        new HttpSessionSecurityContextRepository();

@PostMapping("/login")
public void login(@RequestBody LoginRequest loginRequest, HttpServletRequest request, HttpServletResponse response) {
    UsernamePasswordAuthenticationToken token = UsernamePasswordAuthenticationToken.unauthenticated(
        loginRequest.getUsername(), loginRequest.getPassword());
    Authentication authentication = authenticationManager.authenticate(token);
    SecurityContext context = securityContextHolderStrategy.createEmptyContext();
    context.setAuthentication(authentication);
    securityContextHolderStrategy.setContext(context);
    securityContextRepository.saveContext(context, request, response);
}
```

### Configuring Concurrency Control / 配置并发控制

If you want to restrict a user's ability to login to your application, Spring Security provides the following simple additions to support this functionality. First, you need to add the following listener to make Spring Security aware of session lifecycle events:

如果您希望限制单个用户登录应用程序的能力，Spring Security 提供了以下简单的附加功能来支持此功能。首先，您需要将以下侦听器添加到您的配置中，以使 Spring Security 了解会话生命周期事件：

```java
@Bean
public HttpSessionEventPublisher httpSessionEventPublisher() {
    return new HttpSessionEventPublisher();
}
```

Then add the following lines to your security configuration:

然后将以下几行添加到您的安全配置中：

```java
@Bean
public SecurityFilterChain filterChain(HttpSecurity http) {
    http
        .sessionManagement(session -> session
            .maximumSessions(1)
        );
    return http.build();
}
```

This will prevent a user from logging in multiple times - a second login will invalidate the first.

这将阻止用户多次登录 - 第二次登录将导致第一次登录失效。

You may prefer to block the second login instead:

您可能更愿意阻止第二次登录：

```java
http
    .sessionManagement(session -> session
        .maximumSessions(1)
        .maxSessionsPreventsLogin(true)
    );
```

Then the second login will be refused.

然后将拒绝第二次登录。

### Detecting Timeouts / 检测超时

Spring Security can detect when a session has expired and take specific action. For example, you may want to redirect to a specific endpoint when a user makes a request with an expired session. This is accomplished via the `invalidSessionUrl` in `HttpSecurity`.

Spring Security 可以检测到会话何时过期，并采取您指定的特定操作。例如，当用户使用已过期的会话发出请求时，您可能希望重定向到特定的端点。

```java
http
    .sessionManagement(session -> session
        .invalidSessionUrl("/invalidSession")
    );
```

### Session Fixation Attack Protection / 会话固定攻击防护

Session fixation attacks are a potential risk where an attacker can access a site, create a session, and then persuade another user to log in with the same session (for example, by sending them a link with the session identifier as a parameter). Spring Security protects against this automatically by creating a new session or changing the session ID when the user logs in.

会话固定攻击是一种潜在的风险，攻击者可能通过访问站点创建会话，然后诱导其他用户使用相同的会话登录（例如，通过向他们发送包含会话标识符作为参数的链接）。Spring Security 通过在用户登录时创建新会话或更改会话 ID 来自动防止此问题。

You can control the session fixation protection strategy:

您可以通过选择三个推荐的选项来控制会话固定防护的策略：

- `changeSessionId` - Don't create a new session. Instead, use the session fixation protection provided by the Servlet container (default for Servlet 3.1+)
- `newSession` - Create a new "clean" session without copying existing session data
- `migrateSession` - Create a new session and copy all existing session attributes to the new session (default for Servlet 3.0)

```java
http
    .sessionManagement((session) -> session
        .sessionFixation((sessionFixation) -> sessionFixation
            .newSession()
        )
    );
```

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/authentication/session-management.html*
