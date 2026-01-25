# Password Storage

## Overview / 概述

Password storage is a critical aspect of application security. Spring Security provides multiple password encoding schemes to securely store user passwords.

密码存储是应用安全的关键方面。Spring Security 提供多种密码编码方案来安全地存储用户密码。

## Password Encoder Interface / 密码编码器接口

```java
public interface PasswordEncoder {
    String encode(CharSequence rawPassword);
    boolean matches(CharSequence rawPassword, String encodedPassword);
    default boolean upgradeEncoding(String encodedPassword) {
        return false;
    }
}
```

## Recommended Encoders / 推荐的编码器

### BCryptPasswordEncoder

BCrypt is the default and recommended password encoder:

BCrypt 是默认且推荐的密码编码器：

```java
@Bean
public PasswordEncoder passwordEncoder() {
    return new BCryptPasswordEncoder();
}
```

BCrypt features:
- Automatic salt generation
- Adaptive cost factor (default 10)
- Built into most databases

BCrypt 特性：
- 自动盐生成
- 自适应成本因子（默认 10）
- 内置于大多数数据库

### Argon2PasswordEncoder

Argon2 is the winner of the Password Hashing Competition:

Argon2 是密码哈希竞赛的获胜者：

```java
@Bean
public PasswordEncoder passwordEncoder() {
    return new Argon2PasswordEncoder(
        16,      // salt length
        32,      // hash length
        1,       // parallelism
        65536,   // memory (KB)
        3        // iterations
    );
}
```

### SCryptPasswordEncoder

```java
@Bean
public PasswordEncoder passwordEncoder() {
    return new SCryptPasswordEncoder(
        16384,   // CPU cost
        8,       // memory cost
        1,       // parallelization
        32,      // key length
        64       // salt length
    );
}
```

## Legacy Encoders / 传统编码器

For compatibility with legacy systems:

为了与传统系统兼容：

### Pbkdf2PasswordEncoder

```java
@Bean
public PasswordEncoder passwordEncoder() {
    return new Pbkdf2PasswordEncoder();
}
```

### StandardPasswordEncoder (SHA-256)

```java
@Bean
public PasswordEncoder passwordEncoder() {
    return new StandardPasswordEncoder();
}
```

## DelegatingPasswordEncoder / 委托密码编码器

`DelegatingPasswordEncoder` allows multiple encoding schemes:

`DelegatingPasswordEncoder` 允许多种编码方案：

```java
String idForEncode = "bcrypt";
Map<String, PasswordEncoder> encoders = new HashMap<>();
encoders.put("bcrypt", new BCryptPasswordEncoder());
encoders.put("argon2", new Argon2PasswordEncoder());
encoders.put("pbkdf2", new Pbkdf2PasswordEncoder());

PasswordEncoder passwordEncoder =
    new DelegatingPasswordEncoder(idForEncode, encoders);
```

Stored passwords are prefixed with the encoding scheme (e.g., `{bcrypt}$2a$10$...`).

存储的密码以编码方案为前缀（例如 `{bcrypt}$2a$10$...`）。

## Password History / 密码历史

To prevent password reuse:

为了防止密码重用：

```java
@Bean
public PasswordEncoder passwordEncoder(PasswordEncoder delegatingPasswordEncoder) {
    return new PasswordEncoder() {
        @Override
        public String encode(CharSequence rawPassword) {
            return delegatingPasswordEncoder.encode(rawPassword);
        }

        @Override
        public boolean matches(CharSequence rawPassword, String encodedPassword) {
            return delegatingPasswordEncoder.matches(rawPassword, encodedPassword);
        }
    };
}
```

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/authentication/passwords/storage.html*
