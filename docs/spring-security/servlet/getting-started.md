# Getting Started - Servlet Applications

## Overview / 概述

This guide will help you get started with Spring Security for Servlet applications.

本指南将帮助您开始使用 Servlet 应用的 Spring Security。

## Spring Boot Integration / Spring Boot 集成

Spring Security provides Spring Boot auto-configuration. Add the dependency:

Spring Security 提供 Spring Boot 自动配置。添加依赖：

```xml
<dependency>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-starter-security</artifactId>
</dependency>
```

## Default Security Configuration / 默认安全配置

With Spring Boot, the following defaults are automatically configured:

使用 Spring Boot 时，会自动配置以下默认值：

- All HTTP paths require authentication
- A default user with username "user" and a generated password
- Form-based login
- HTTP Basic authentication
- Default logout

所有 HTTP 路径都需要认证
用户名为 "user" 的默认用户和生成的密码
基于表单的登录
HTTP Basic 认证
默认登出

## Creating Your First Secure Application / 创建第一个安全应用

```java
@RestController
public class HomeController {

    @GetMapping("/")
    public String home() {
        return "Welcome to the secure application!";
    }
}
```

## Customizing Security Configuration / 自定义安全配置

Create a security configuration class:

创建安全配置类：

```java
@Configuration
@EnableWebSecurity
public class SecurityConfig {

    @Bean
    public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
        http
            .authorizeHttpRequests(auth -> auth
                .requestMatchers("/public/**").permitAll()
                .anyRequest().authenticated()
            )
            .formLogin(form -> form
                .loginPage("/login")
                .permitAll()
            );
        return http.build();
    }

    @Bean
    public UserDetailsService userDetailsService() {
        UserDetails user = User.builder()
            .username("user")
            .password(passwordEncoder().encode("password"))
            .roles("USER")
            .build();
        return new InMemoryUserDetailsManager(user);
    }

    @Bean
    public PasswordEncoder passwordEncoder() {
        return new BCryptPasswordEncoder();
    }
}
```

## Testing Your Application / 测试应用

Start your Spring Boot application and access `http://localhost:8080`. You will be redirected to the login page.

启动 Spring Boot 应用并访问 `http://localhost:8080`。您将被重定向到登录页面。

Use the default credentials or your configured credentials:
- Username: `user` (or your configured username)
- Password: Check console for generated password or use your configured password

使用默认凭据或您配置的凭据：
- 用户名：`user`（或您配置的用户名）
- 密码：查看控制台获取生成的密码或使用您配置的密码

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/getting-started.html*
