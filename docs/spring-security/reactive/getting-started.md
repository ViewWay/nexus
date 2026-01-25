# Getting Started - WebFlux Applications / WebFlux 应用程序入门

## Overview / 概述

This section describes how to use Spring Security with Spring Boot in a reactive application with minimal setup.

本节介绍如何在响应式应用程序中使用 Spring Security 与 Spring Boot 的最小设置。

## Update Dependencies / 更新依赖项

You can add Spring Security to your Spring Boot project by adding `spring-boot-starter-security`.

您可以通过添加 `spring-boot-starter-security` 将 Spring Security 添加到您的 Spring Boot 项目中。

### Maven

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-security</artifactId>
</dependency>
```

### Gradle

```groovy
implementation 'org.springframework.boot:spring-boot-starter-security'
```

## Running Hello Spring Security Boot / 启动 Hello Spring Security Boot

You can now run the Spring Boot application using the `run` goal of the Maven plugin:

您现在可以使用 Maven 插件的 `run` 目标运行 Spring Boot 应用程序：

### Maven

```bash
$ ./mvnw spring-boot:run
...
INFO 23689 --- [  restartedMain] .s.s.UserDetailsServiceAutoConfiguration :
Using generated security password: 8e557245-73e2-4286-969a-ff57fe326336
...
```

### Gradle

```bash
$ ./gradlew bootRun
...
INFO 23689 --- [  restartedMain] .s.s.UserDetailsServiceAutoConfiguration :
Using generated security password: 8e557245-73e2-4286-969a-ff57fe326336
...
```

## Authentication / 认证

You can access the application at localhost:8080/, which will redirect the browser to the default login page. You can authenticate with the default username `user` and a random-generated password that is logged to the console. The browser will then be taken to the originally requested page.

您可以通过 localhost:8080/ 访问应用程序，这将把浏览器重定向到默认的登录页面。您可以使用默认用户名 `user` 和控制台记录的随机生成的密码进行身份验证。然后，浏览器将被带到最初请求的页面。

To logout, you can visit localhost:8080/logout and then confirm that you wish to logout.

要注销，您可以访问 localhost:8080/logout，然后确认您希望注销。

## Spring Boot Auto-Configuration / Spring Boot 自动配置

Spring Boot automatically adds Spring Security that requires all requests to be authenticated. It also generates a user with a randomly-generated password that is logged to the console that can be used to authenticate using form or basic authentication.

Spring Boot 自动添加 Spring Security，它要求所有请求都经过身份验证。它还会生成一个用户，并使用记录到控制台的随机生成的密码，该密码可用于使用表单或基本身份验证进行身份验证。

## WebFlux Configuration / WebFlux 配置

For WebFlux applications, Spring Security uses `ServerHttpSecurity` instead of `HttpSecurity`:

对于 WebFlux 应用程序，Spring Security 使用 `ServerHttpSecurity` 而不是 `HttpSecurity`：

```java
@Configuration
@EnableWebFluxSecurity
public class SecurityConfig {

    @Bean
    public SecurityWebFilterChain securityWebFilterChain(ServerHttpSecurity http) {
        http
            .authorizeExchange(authorize -> authorize
                .anyExchange().authenticated()
            )
            .httpBasic(withDefaults())
            .formLogin(withDefaults());
        return http.build();
    }

    @Bean
    public MapReactiveUserDetailsService userDetailsService() {
        UserDetails user = User.withDefaultPasswordEncoder()
            .username("user")
            .password("password")
            .roles("USER")
            .build();
        return new MapReactiveUserDetailsService(user);
    }
}
```

## Key Differences from Servlet / 与 Servlet 的主要区别

1. Uses `SecurityWebFilterChain` instead of `SecurityFilterChain`
2. Uses `ServerHttpSecurity` instead of `HttpSecurity`
3. Uses `ReactiveUserDetailsService` instead of `UserDetailsService`
4. Returns reactive types (Mono/Flux) for authentication operations

1. 使用 `SecurityWebFilterChain` 而不是 `SecurityFilterChain`
2. 使用 `ServerHttpSecurity` 而不是 `HttpSecurity`
3. 使用 `ReactiveUserDetailsService` 而不是 `UserDetailsService`
4. 为认证操作返回响应式类型 (Mono/Flux)

*Source: https://docs.springframework.org.cn/spring-security/reference/reactive/getting-started.html*
