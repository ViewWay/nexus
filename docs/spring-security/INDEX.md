# Spring Security Documentation Index / Spring Security 文档索引

This directory contains the Spring Security documentation fetched from https://docs.springframework.org.cn/spring-security/reference/

本目录包含从 https://docs.springframework.org.cn/spring-security/reference/ 获取的 Spring Security 文档

## Directory Structure / 目录结构

```
spring-security/
├── 01-overview/
│   └── 00-main-index.md          # Main overview and project modules
├── 02-architecture/              # Architecture documentation (pending)
├── servlet/                      # Servlet application documentation
│   ├── getting-started.md        # Getting started with Servlet apps
│   ├── authentication/
│   │   ├── index.md              # Authentication architecture overview
│   │   ├── passwords/
│   │   │   ├── index.md          # Username/password authentication
│   │   │   └── storage.md        # Password storage mechanisms
│   │   ├── rememberme.md         # Remember-me authentication
│   │   ├── jaas.md              # JAAS authentication provider
│   │   ├── cas.md               # CAS authentication
│   │   ├── x509.md              # X.509 certificate authentication
│   │   ├── logout.md            # Logout handling
│   │   └── session-management.md # Session management
│   ├── authorization/
│   │   ├── index.md             # Authorization overview
│   │   └── method-security.md   # Method-level security
│   ├── oauth2/
│   │   ├── index.md             # OAuth2 overview
│   │   └── login/
│   │       ├── index.md         # OAuth2 login overview
│   │       └── core.md          # OAuth2 core configuration
│   └── saml2/
│       └── index.md             # SAML2 overview
├── reactive/                     # Reactive application documentation
│   └── getting-started.md        # Getting started with WebFlux
├── 06-cors-csrf/                 # CORS and CSRF documentation (pending)
├── 07-headers/                   # Security headers documentation (pending)
├── 08-testing/                   # Testing documentation (pending)
├── 09-reactive/                  # Reactive security (partial)
├── 10-graalvm/                   # GraalVM native image support (pending)
└── 11-appendix/                  # Appendix documentation (pending)
```

## Completed Sections / 已完成的部分

### Overview / 概述
- Main index with project modules and features
- Version information

### Servlet Applications / Servlet 应用程序

#### Authentication / 认证
- Authentication architecture overview
- Username/password authentication
- Password storage (BCrypt, Argon2, etc.)
- Remember-me authentication
- JAAS authentication provider
- CAS authentication
- X.509 certificate authentication
- Logout handling
- Session management

#### Authorization / 授权
- Authorization architecture
- Method-level security (@PreAuthorize, @PostAuthorize, etc.)

#### OAuth2 / OAuth2
- OAuth2 overview
- OAuth2 login configuration
- OAuth2 client configuration
- OAuth2 resource server (JWT, opaque tokens)

#### SAML2 / SAML2
- SAML2 overview
- SAML2 login configuration

### Reactive Applications / 响应式应用程序
- WebFlux getting started guide

## Pending Sections / 待完成的部分

1. **Architecture / 架构** - Filter chain, SecurityContext, etc.
2. **CORS/CSRF** - Cross-origin and CSRF protection
3. **Security Headers** - HTTP security headers
4. **Testing** - Testing with MockMvc and WebTestClient
5. **Reactive Security** - Complete reactive security documentation
6. **GraalVM** - Native image support
7. **Appendix** - Database schemas, FAQ, etc.

## Documentation Format / 文档格式

All documentation is in Markdown format with:
- Bilingual content (English and Chinese / 英文和中文)
- Code examples in Java and Kotlin
- Configuration examples (YAML, properties, XML)

所有文档采用 Markdown 格式，包含：
- 双语内容（英文和中文）
- Java 和 Kotlin 代码示例
- 配置示例（YAML、properties、XML）

## Source / 来源

Documentation source: https://docs.springframework.org.cn/spring-security/reference/

文档来源：https://docs.springframework.org.cn/spring-security/reference/
