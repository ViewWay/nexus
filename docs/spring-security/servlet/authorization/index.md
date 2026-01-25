# Authorization / 授权

## Overview / 概述

在确定用户如何进行身份验证后，您还需要配置应用程序的授权规则。

Spring Security 中的高级授权功能是其广受欢迎的最重要原因之一。无论您选择哪种身份验证方式（使用 Spring Security 提供的机制和提供程序，还是与容器或其他非 Spring Security 身份验证机构集成），都可以在您的应用程序中以一致且简单的方式使用授权服务。

您应该考虑将授权规则附加到请求 URI和方法以开始。在这两种情况下，您都可以监听并响应每次授权检查发布的授权事件。下面还详细介绍了Spring Security 授权的工作原理以及如何在建立基本模型后对其进行微调。

## Chapter Sections / 章节摘要

- **Authorization Architecture / 授权架构** - How Spring Security handles authorization internally
- **Authorize HTTP Requests / 授权 HTTP 请求** - Configuring web-based authorization
- **Method Security / 方法安全** - Fine-grained method-level authorization
- **Domain Object Security (ACLs) / 域对象安全 ACL** - Securing individual domain objects
- **Authorization Events / 授权事件** - Listening to and responding to authorization decisions

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/authorization/index.html*
