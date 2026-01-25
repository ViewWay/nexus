# Spring Security

## Overview / 概述

Spring Security is a framework that provides authentication, authorization, and protection against common attacks. With first-class support for securing both imperative and reactive applications, it is the de-facto standard for securing Spring-based applications.

Spring Security 是一个框架，它提供了认证、授权以及防范常见攻击的能力。通过对保护命令式和响应式应用提供一流支持，它已成为保护基于 Spring 的应用的事实标准。

For a complete feature list, please refer to the features section of the reference documentation.

有关完整特性列表，请参阅参考文档的特性部分。

## Getting Started / 入门

If you are ready to start protecting your application, please see the getting started chapters for Servlet and Reactive. These chapters will walk you through creating your first Spring Security application.

如果您准备开始保护应用程序，请参阅Servlet和响应式的入门章节。这些章节将指导您创建第一个 Spring Security 应用程序。

If you want to understand how Spring Security works, please refer to the architecture chapters.

如果您想了解 Spring Security 的工作原理，可以参考架构章节。

If you are already familiar with Spring Security or are upgrading, please review the new features in the latest release.

如果您已经熟悉 Spring Security 或正在升级，请查看最新版本的新特性。

If you have any questions, there is a great community that is happy to help!

如果您有任何问题，这里有一个很棒的社区乐于帮助您！

## Project Modules / 项目模块

Spring Security is organized into the following modules:

Spring Security 组织为以下模块：

### Core Modules / 核心模块

- **spring-security-core** - Core authentication and access control classes, interfaces, and support objects
- **spring-security-remoting** - Integration with Spring Remoting
- **spring-security-web** - Filters and web-related security infrastructure

### Data Modules / 数据模块

- **spring-security-crypto** - Cryptographic encoding and password encoding
- **spring-security-data** - Integration with Spring Data
- **spring-security-ldap** - LDAP authentication
- **spring-security-acl** - Access Control List (ACL) security

### OAuth2 Modules / OAuth2 模块

- **spring-security-oauth2-core** - OAuth 2.0 core support
- **spring-security-oauth2-client** - OAuth 2.0 client support
- **spring-security-oauth2-jose** - JOSE (Javascript Object Signing and Encryption) framework support
- **spring-security-oauth2-resource-server** - OAuth 2.0 resource server support

### SAML Module / SAML 模块

- **spring-security-saml2-service-provider** - SAML 2.0 service provider support

### Testing Module / 测试模块

- **spring-security-test** - Testing support

## Features / 特性

### Authentication / 认证

- Password storage (multiple encoding schemes supported)
- LDAP authentication
- JDBC authentication
- Form-based authentication
- HTTP Basic authentication
- Remember-me authentication
- JAAS authentication
- OpenID Connect
- SAML 2.0
- X.509 authentication

### Authorization / 授权

- Web request authorization
- Method-level authorization
- Domain object access control (ACLs)
- Expression-based access control

### Attack Protection / 防范攻击

- CSRF (Cross-Site Request Forgery) protection
- HTTP Response Headers protection
- HTTP request protection

### Integration / 集成

- Cryptography (encryption/decryption)
- Spring Data integration
- Java Concurrency API integration
- Jackson integration
- Localization (i18n)

## Versions / 版本

### Stable Versions / 稳定版

- 6.4.5
- 6.3.9
- 6.2.8
- 6.1.9
- 6.0.8
- 5.8.16
- 5.7.9

### Preview Versions / 预览版

- 6.5.0-RC1

### Snapshot Versions / 快照版

- 6.5.0-SNAPSHOT
- 6.4.6-SNAPSHOT
- 6.3.10-SNAPSHOT

## Related Projects / 相关项目

- Spring Authorization Server
- Spring LDAP
- Spring Security Kerberos
- Spring Session
- Spring Vault

---

*Source: https://docs.springframework.org.cn/spring-security/reference/*
