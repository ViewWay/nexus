# System Requirements / 系统要求

Source: https://docs.springframework.org.cn/spring-boot/system-requirements.html

---

## English

Spring Boot 3.3.5 requires at least Java 17 and is compatible up to and including Java 23. Spring Framework 6.1.14 or later is also required.

Explicit build support is provided for the following build tools:

| Build Tool | Version |
|------------|---------|
| Maven | 3.6.3 or later |
| Gradle | 7.x (7.5 or later) and 8.x |

## Servlet Containers

Spring Boot supports the following embedded servlet containers:

| Name | Servlet Version |
|------|-----------------|
| Tomcat 10.1 (10.1.25 or later) | 6.0 |
| Jetty 12.0 | 6.0 |
| Undertow 2.3 | 6.0 |

You can also deploy Spring Boot applications to any Servlet 5.0+ compatible container.

## GraalVM Native Images

Spring Boot applications can be converted to native images using GraalVM 22.3 or later.

Images can be created using the Native Build Tools Gradle/Maven plugins or the `native-image` tool provided by GraalVM. You can also build native images with the Native Image Paketo Buildpack.

The following versions are supported:

| Name | Version |
|------|---------|
| GraalVM Community Edition | 22.3 |
| Native Build Tools | 0.10.3 |

---

## 中文 / Chinese

Spring Boot 3.3.5 至少需要 Java 17，并兼容最高到 Java 23 的版本。也需要 Spring Framework 6.1.14 或更高版本。

为以下构建工具提供显式构建支持：

| 构建工具 | 版本 |
|----------|------|
| Maven | 3.6.3 或更高版本 |
| Gradle | 7.x (7.5 或更高版本) 和 8.x |

## Servlet 容器

Spring Boot 支持以下嵌入式 Servlet 容器：

| 名称 | Servlet 版本 |
|------|--------------|
| Tomcat 10.1 (10.1.25 或更高版本) | 6.0 |
| Jetty 12.0 | 6.0 |
| Undertow 2.3 | 6.0 |

您还可以将 Spring Boot 应用程序部署到任何兼容 Servlet 5.0+ 的容器。

## GraalVM 原生镜像

Spring Boot 应用程序可以使用 GraalVM 22.3 或更高版本转换为原生镜像。

可以使用原生构建工具 Gradle/Maven 插件或 GraalVM 提供的 `native-image` 工具创建镜像。您也可以使用原生镜像 Paketo buildpack 创建原生镜像。

支持以下版本：

| 名称 | 版本 |
|------|------|
| GraalVM 社区版 | 22.3 |
| 原生构建工具 | 0.10.3 |
