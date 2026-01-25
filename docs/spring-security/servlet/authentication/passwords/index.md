# Username/Password Authentication

## Overview / 概述

Username and password authentication is the most common authentication mechanism. Spring Security provides comprehensive support for username/password authentication.

用户名和密码认证是最常见的认证机制。Spring Security 为用户名/密码认证提供全面支持。

## DaoAuthenticationProvider / DAO 认证提供者

`DaoAuthenticationProvider` is the default implementation that retrieves user details from a `UserDetailsService`.

`DaoAuthenticationProvider` 是从 `UserDetailsService` 检索用户详情的默认实现。

```java
@Bean
public DaoAuthenticationProvider authenticationProvider(
        UserDetailsService userDetailsService,
        PasswordEncoder passwordEncoder) {
    DaoAuthenticationProvider provider = new DaoAuthenticationProvider();
    provider.setUserDetailsService(userDetailsService);
    provider.setPasswordEncoder(passwordEncoder);
    return provider;
}
```

## In-Memory User Details / 内存用户详情

For simple applications:

对于简单的应用程序：

```java
@Bean
public UserDetailsService userDetailsService() {
    UserDetails user = User.builder()
        .username("user")
        .password("{bcrypt}$2a$10$...")
        .roles("USER")
        .build();

    UserDetails admin = User.builder()
        .username("admin")
        .password("{bcrypt}$2a$10$...")
        .roles("ADMIN", "USER")
        .build();

    return new InMemoryUserDetailsManager(user, admin);
}
```

## JDBC User Details / JDBC 用户详情

For database-backed authentication:

对于数据库支持的认证：

```java
@Bean
public UserDetailsService userDetailsService(DataSource dataSource) {
    return new JdbcUserDetailsManager(dataSource);
}
```

Default schema (you can customize):

默认架构（您可以自定义）：

```sql
CREATE TABLE users (
    username VARCHAR(50) NOT NULL PRIMARY KEY,
    password VARCHAR(100) NOT NULL,
    enabled BOOLEAN NOT NULL
);

CREATE TABLE authorities (
    username VARCHAR(50) NOT NULL,
    authority VARCHAR(50) NOT NULL,
    FOREIGN KEY (username) REFERENCES users(username)
);
```

## Custom UserDetailsService / 自定义用户详情服务

```java
@Service
public class CustomUserDetailsService implements UserDetailsService {

    @Autowired
    private UserRepository userRepository;

    @Override
    public UserDetails loadUserByUsername(String username)
            throws UsernameNotFoundException {
        User user = userRepository.findByUsername(username)
            .orElseThrow(() -> new UsernameNotFoundException("User not found"));

        return org.springframework.security.core.userdetails.User.builder()
            .username(user.getUsername())
            .password(user.getPassword())
            .roles(user.getRoles().toArray(new String[0]))
            .build();
    }
}
```

## Login Form / 登录表单

Default login form is provided. To customize:

提供默认登录表单。要自定义：

```java
@Bean
public SecurityFilterChain securityFilterChain(HttpSecurity http) throws Exception {
    http
        .formLogin(form -> form
            .loginPage("/login")
            .permitAll()
            .defaultSuccessUrl("/home")
            .failureUrl("/login?error=true")
        );
    return http.build();
}
```

Custom login page:

自定义登录页面：

```html
<form th:action="@{/login}" method="post">
    <div>
        <label>Username:</label>
        <input type="text" name="username"/>
    </div>
    <div>
        <label>Password:</label>
        <input type="password" name="password"/>
    </div>
    <button type="submit">Login</button>
</form>
```

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/authentication/passwords/index.html*
