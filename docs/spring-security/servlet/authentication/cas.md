# CAS Authentication / CAS 认证

## Overview / 概述

JA-SIG developed an enterprise-wide single sign-on system called CAS. Unlike other initiatives, JA-SIG's Central Authentication Service is open-source, widely used, easy to understand, platform-independent, and supports proxy capabilities. Spring Security has full support for CAS and provides an easy migration path from Spring Security's single-application deployment to multi-application deployment secured by an enterprise CAS server.

JA-SIG 开发了一个名为 CAS 的企业级单点登录系统。与其他举措不同，JA-SIG 的中央认证服务是开源的、广泛使用的、易于理解的、平台无关的，并支持代理功能。Spring Security 完全支持 CAS，并提供了一条简单的迁移路径，从 Spring Security 的单应用程序部署到由企业级 CAS 服务器保护的多应用程序部署。

You can learn more about CAS at www.apereo.org.

您可以在 www.apereo.org 上了解更多关于 CAS 的信息。

## How CAS Works / CAS 的工作原理

### CAS Components / CAS 组件

- **CAS Server** - A standard WAR file that handles authentication
- **Services** - The web applications secured by CAS (there are three types)
  - Services that validate service tickets
  - Services that can obtain proxy tickets
  - Services that validate proxy tickets

- **CAS 服务器** - 处理认证的标准 WAR 文件
- **服务** - 由 CAS 保护的应用程序（有三种类型）
  - 验证服务票证的服务
  - 可以获取代理票证的服务
  - 验证代理票证的服务

### Spring Security and CAS Interaction Sequence / Spring Security 和 CAS 交互序列

1. User browses a public page - CAS or Spring Security not involved
2. User requests a secure page - `ExceptionTranslationFilter` detects `AuthenticationException`
3. `CasAuthenticationEntryPoint` redirects browser to CAS server with `service` parameter
4. User enters credentials on CAS server login page
5. CAS redirects back to the service with a `ticket` parameter
6. `CasAuthenticationFilter` constructs `UsernamePasswordAuthenticationToken` with the ticket
7. `CasAuthenticationProvider` validates the ticket using `TicketValidator`
8. CAS server responds with XML containing username and proxy list
9. `CasAuthenticationProvider` creates `CasAuthenticationToken` and places it in security context
10. User is redirected to the original page

1. 用户浏览公共页面 - CAS 或 Spring Security 未参与
2. 用户请求安全页面 - `ExceptionTranslationFilter` 检测到 `AuthenticationException`
3. `CasAuthenticationEntryPoint` 将浏览器重定向到带有 `service` 参数的 CAS 服务器
4. 用户在 CAS 服务器登录页面上输入凭据
5. CAS 带着票证参数重定向回服务
6. `CasAuthenticationFilter` 构造带有票证的 `UsernamePasswordAuthenticationToken`
7. `CasAuthenticationProvider` 使用 `TicketValidator` 验证票证
8. CAS 服务器响应包含用户名和代理列表的 XML
9. `CasAuthenticationProvider` 创建 `CasAuthenticationToken` 并将其放入安全上下文
10. 用户被重定向到原始页面

## CAS Client Configuration / CAS 客户端配置

### Service Properties / 服务属性

```xml
<bean id="serviceProperties"
    class="org.springframework.security.cas.ServiceProperties">
    <property name="service"
        value="https://127.0.0.1:8443/cas-sample/login/cas"/>
    <property name="sendRenew" value="false"/>
</bean>
```

The `service` must equal the URL that `CasAuthenticationFilter` will monitor. The `sendRenew` defaults to false but should be set to true for highly sensitive applications.

`service` 必须等于 `CasAuthenticationFilter` 将监视的 URL。`sendRenew` 默认为 false，但对于高度敏感的应用程序应设置为 true。

### Security Filter Chain Configuration / 安全过滤器链配置

```xml
<security:http entry-point-ref="casEntryPoint">
    ...
    <security:custom-filter position="CAS_FILTER" ref="casFilter" />
</security:http>

<bean id="casFilter"
    class="org.springframework.security.cas.web.CasAuthenticationFilter">
    <property name="authenticationManager" ref="authenticationManager"/>
</bean>

<bean id="casEntryPoint"
    class="org.springframework.security.cas.web.CasAuthenticationEntryPoint">
    <property name="loginUrl" value="https://127.0.0.1:9443/cas/login"/>
    <property name="serviceProperties" ref="serviceProperties"/>
</bean>
```

### Authentication Provider Configuration / 认证提供者配置

```xml
<security:authentication-manager alias="authenticationManager">
    <security:authentication-provider ref="casAuthenticationProvider" />
</security:authentication-manager>

<bean id="casAuthenticationProvider"
    class="org.springframework.security.cas.authentication.CasAuthenticationProvider">
    <property name="authenticationUserDetailsService">
        <bean class="org.springframework.security.core.userdetails.UserDetailsByNameServiceWrapper">
            <constructor-arg ref="userService" />
        </bean>
    </property>
    <property name="serviceProperties" ref="serviceProperties" />
    <property name="ticketValidator">
        <bean class="org.apereo.cas.client.validation.Cas20ServiceTicketValidator">
            <constructor-arg index="0" value="https://127.0.0.1:9443/cas" />
        </bean>
    </property>
    <property name="key" value="an_id_for_this_auth_provider_only"/>
</bean>

<security:user-service id="userService">
    <security:user name="joe" password="{noop}joe" authorities="ROLE_USER" />
    ...
</security:user-service>
```

## Single Sign-Out / 单点登出

CAS protocol supports single sign-out:

CAS 协议支持单点登出：

```xml
<security:http entry-point-ref="casEntryPoint">
    ...
    <security:logout logout-success-url="/cas-logout.jsp"/>
    <security:custom-filter ref="requestSingleLogoutFilter" before="LOGOUT_FILTER"/>
    <security:custom-filter ref="singleLogoutFilter" before="CAS_FILTER"/>
</security:http>

<!-- This filter handles a Single Logout Request from the CAS Server -->
<bean id="singleLogoutFilter"
    class="org.apereo.cas.client.session.SingleSignOutFilter"/>

<!-- This filter redirects to the CAS Server to signal Single Logout -->
<bean id="requestSingleLogoutFilter"
    class="org.springframework.security.web.authentication.logout.LogoutFilter">
    <constructor-arg value="https://127.0.0.1:9443/cas/logout"/>
    <constructor-arg>
        <bean class="org.springframework.security.web.authentication.logout.SecurityContextLogoutHandler"/>
    </constructor-arg>
    <property name="filterProcessesUrl" value="/logout/cas"/>
</bean>
```

### web.xml Configuration / web.xml 配置

```xml
<filter>
    <filter-name>characterEncodingFilter</filter-name>
    <filter-class>org.springframework.web.filter.CharacterEncodingFilter</filter-class>
    <init-param>
        <param-name>encoding</param-name>
        <param-value>UTF-8</param-value>
    </init-param>
</filter>
<filter-mapping>
    <filter-name>characterEncodingFilter</filter-name>
    <url-pattern>/*</url-pattern>
</filter-mapping>
<listener>
    <listener-class>org.apereo.cas.client.session.SingleSignOutHttpSessionListener</listener-class>
</listener>
```

## Proxy Ticket Authentication / 代理票证认证

### Configuring CAS to Obtain Proxy Granting Tickets / 配置 CAS 以获取代理授予票证

```xml
<bean id="pgtStorage" class="org.apereo.cas.client.proxy.ProxyGrantingTicketStorageImpl"/>

<bean id="casAuthenticationProvider"
    class="org.springframework.security.cas.authentication.CasAuthenticationProvider">
    ...
    <property name="ticketValidator">
        <bean class="org.apereo.cas.client.validation.Cas20ProxyTicketValidator">
            <constructor-arg value="https://127.0.0.1:9443/cas"/>
            <property name="proxyCallbackUrl"
                value="https://127.0.0.1:8443/cas-sample/login/cas/proxyreceptor"/>
            <property name="proxyGrantingTicketStorage" ref="pgtStorage"/>
        </bean>
    </property>
</bean>

<bean id="casFilter" class="org.springframework.security.cas.web.CasAuthenticationFilter">
    ...
    <property name="proxyGrantingTicketStorage" ref="pgtStorage"/>
    <property name="proxyReceptorUrl" value="/login/cas/proxyreceptor"/>
</bean>
```

### Using Proxy Tickets to Call Stateless Services / 使用代理票证调用无状态服务

```java
protected void doGet(HttpServletRequest request, HttpServletResponse response)
        throws ServletException, IOException {
    final CasAuthenticationToken token = (CasAuthenticationToken) request.getUserPrincipal();
    final String proxyTicket = token.getAssertion().getPrincipal().getProxyTicketFor(targetUrl);

    // Make a remote call using the proxy ticket
    final String serviceUrl = targetUrl + "?ticket=" + URLEncoder.encode(proxyTicket, "UTF-8");
    String proxyResponse = CommonUtils.getResponseFromServer(serviceUrl, "UTF-8");
    ...
}
```

### Proxy Ticket Authentication Configuration / 代理票证认证配置

```xml
<bean id="serviceProperties"
    class="org.springframework.security.cas.ServiceProperties">
    ...
    <property name="authenticateAllArtifacts" value="true"/>
</bean>

<bean id="casFilter"
    class="org.springframework.security.cas.web.CasAuthenticationFilter">
    ...
    <property name="serviceProperties" ref="serviceProperties"/>
    <property name="authenticationDetailsSource">
        <bean class="org.springframework.security.cas.web.authentication.ServiceAuthenticationDetailsSource">
            <constructor-arg ref="serviceProperties"/>
        </bean>
    </property>
</bean>

<bean id="casAuthenticationProvider"
    class="org.springframework.security.cas.authentication.CasAuthenticationProvider">
    ...
    <property name="ticketValidator">
        <bean class="org.apereo.cas.client.validation.Cas20ProxyTicketValidator">
            <constructor-arg value="https://127.0.0.1:9443/cas"/>
            <property name="acceptAnyProxy" value="true"/>
        </bean>
    </property>
    <property name="statelessTicketCache">
        <bean class="org.springframework.security.cas.authentication.EhCacheBasedTicketCache">
            <property name="cache">
                <bean class="net.sf.ehcache.Cache"
                    init-method="initialise" destroy-method="dispose">
                    <constructor-arg value="casTickets"/>
                    <constructor-arg value="50"/>
                    <constructor-arg value="true"/>
                    <constructor-arg value="false"/>
                    <constructor-arg value="3600"/>
                    <constructor-arg value="900"/>
                </bean>
            </property>
        </bean>
    </property>
</bean>
```

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/authentication/cas.html*
