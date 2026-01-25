# Handling Logout / 处理注销

## Overview / 概述

In applications that allow end users to login, they should also be allowed to logout.

在允许最终用户登录的应用程序中，也应该允许他们注销。

By default, Spring Security enables a `/logout` endpoint, so no additional code is required.

默认情况下，Spring Security 会启动一个 `/logout` 端点，因此无需任何额外代码。

## Customizing Logout URI / 自定义注销 URI

Since `LogoutFilter` appears before `AuthorizationFilter` in the filter chain, by default there's no need to explicitly allow the `/logout` endpoint.

由于 `LogoutFilter` 出现在过滤器链中的 `AuthorizationFilter` 之前，因此默认情况下无需显式允许 `/logout` 端点。

If you want to change the URI that Spring Security is matching:

如果您只想更改 Spring Security 正在匹配的 URI：

```java
http
    .logout((logout) -> logout.logoutUrl("/my/logout/uri"))
```

## Custom Logout Endpoints / 自定义注销端点

If you create your own logout success endpoint (or in rare cases, your own logout endpoint), for example using Spring MVC, you need to allow it in Spring Security.

如果您启动自己的注销成功端点（或在极少数情况下，您自己的注销端点），例如使用 Spring MVC，则需要在 Spring Security 中允许它。

```java
http
    .authorizeHttpRequests((authorize) -> authorize
        .requestMatchers("/my/success/endpoint").permitAll()
        // ...
    )
    .logout((logout) -> logout.logoutSuccessUrl("/my/success/endpoint"))
```

Or use the `permitAll` attribute in the logout DSL:

或者使用注销 DSL 中的 `permitAll` 属性：

```java
http
    .authorizeHttpRequests((authorize) -> authorize
        // ...
    )
    .logout((logout) -> logout
        .logoutSuccessUrl("/my/success/endpoint")
        .permitAll()
    )
```

This will add all logout URIs to the allow list for you.

这将为您将所有注销 URI 添加到允许列表中。

## Adding Cleanup Handlers / 添加清理操作

You can add your own cleanup operations by calling the `addLogoutHandler` method:

您可以通过调用 `addLogoutHandler` 方法添加您自己的清理操作：

```java
CookieClearingLogoutHandler cookies = new CookieClearingLogoutHandler("our-custom-cookie");
http
    .logout((logout) -> logout.addLogoutHandler(cookies))
```

Or use the built-in shortcut:

或者使用内置的快捷方式：

```java
http
    .logout((logout) -> logout.deleteCookies("our-custom-cookie"))
```

### Using Clear-Site-Data Header / 使用 Clear-Site-Data 头部

The `Clear-Site-Data` HTTP header is a browser-supported header that acts as an instruction to clear cookies, storage, and cache belonging to the owning website.

`Clear-Site-Data` HTTP 头部是浏览器支持的一种头部，作为清除属于拥有网站的 Cookie、存储和缓存的指令。

```java
HeaderWriterLogoutHandler clearSiteData = new HeaderWriterLogoutHandler(new ClearSiteDataHeaderWriter());
http
    .logout((logout) -> logout.addLogoutHandler(clearSiteData))
```

To clear only cookies:

要仅清除 Cookie：

```java
HeaderWriterLogoutHandler clearSiteData = new HeaderWriterLogoutHandler(new ClearSiteDataHeaderWriter(Directive.COOKIES));
http
    .logout((logout) -> logout.addLogoutHandler(clearSiteData))
```

## Customizing Logout Success / 自定义注销成功

While `logoutSuccessUrl` is sufficient for most cases, you may need to perform a different action than redirecting to a URL when logout completes. The `LogoutSuccessHandler` is the Spring Security component for customizing logout success actions.

虽然在大多数情况下使用 `logoutSuccessUrl` 就足够了，但您可能需要在注销完成后执行与重定向到 URL 不同的操作。`LogoutSuccessHandler` 是用于自定义注销成功操作的 Spring Security 组件。

For example, to return a status code instead of redirecting:

例如，您可能希望仅返回状态代码，而不是重定向：

```java
http
    .logout((logout) -> logout.logoutSuccessHandler(new HttpStatusReturningLogoutSuccessHandler()))
```

## Creating Custom Logout Endpoints / 创建自定义注销端点

It's highly recommended to use the provided `logout` DSL configuration for logout. One reason is that it's easy to forget to invoke the necessary Spring Security components to ensure correct and complete logout.

强烈建议您使用提供的 `logout` DSL 配置注销。原因之一是，很容易忘记调用必要的 Spring Security 组件以确保正确且完整的注销。

If you need a custom logout endpoint:

如果您发现自己处于需要自定义注销端点的情况下：

```java
SecurityContextLogoutHandler logoutHandler = new SecurityContextLogoutHandler();

@PostMapping("/my/logout")
public String performLogout(Authentication authentication, HttpServletRequest request, HttpServletResponse response) {
    this.logoutHandler.doLogout(request, response, authentication);
    return "redirect:/home";
}
```

Additionally, you will need to explicitly allow the endpoint.

此外，您需要显式允许端点。

## Logout with OAuth 2.0 / 使用 OAuth 2.0 注销

When using OAuth 2.0, you may want to coordinate logout with the authorization server.

当使用 OAuth 2.0 时，您可能希望与授权服务器协调注销。

When using SAML 2.0, you may want to coordinate logout with the identity provider.

当使用 SAML 2.0 时，您可能希望与身份提供程序协调注销。

When using CAS, you may want to coordinate logout with the identity provider.

当使用 CAS 时，您可能希望与身份提供程序协调注销。

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/authentication/logout.html*
