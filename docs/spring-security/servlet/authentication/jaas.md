# JAAS Authentication Provider / JAAS 认证提供者

## Overview / 概述

Spring Security provides a package to delegate authentication requests to the Java Authentication and Authorization Service (JAAS). This section discusses that package.

Spring Security 提供了一个包来将认证请求委托给 Java 认证和授权服务 (JAAS)。本节讨论该包。

## AbstractJaasAuthenticationProvider

The `AbstractJaasAuthenticationProvider` class is the base for the provided JAAS `AuthenticationProvider` implementations. Subclasses must implement a method that creates a `LoginContext`.

`AbstractJaasAuthenticationProvider` 类是提供的 JAAS `AuthenticationProvider` 实现的基础。子类必须实现一个创建 `LoginContext` 的方法。

### JAAS Callback Handlers / JAAS 回调处理器

Most JAAS `LoginModule` implementations require some form of callback. These callbacks are typically used to obtain the username and password from the user.

大多数 JAAS `LoginModule` 实例都需要某种形式的回调。这些回调通常用于从用户那里获取用户名和密码。

Spring Security provides two default callback handlers:
- **JaasNameCallbackHandler** - Handles name callbacks
- **JaasPasswordCallbackHandler** - Handles password callbacks

Spring Security 提供两个默认回调处理器：
- **JaasNameCallbackHandler** - 处理名称回调
- **JaasPasswordCallbackHandler** - 处理密码回调

### JAAS Authority Granters / JAAS 权限授予者

JAAS uses principals. Spring Security uses `Authentication` objects containing `GrantedAuthority` instances. To facilitate mapping between these concepts, Spring Security's JAAS package includes an `AuthorityGranter` interface.

JAAS 使用主体。Spring Security 使用包含 `GrantedAuthority` 实例的 `Authentication` 对象。为了促进这些不同概念之间的映射，Spring Security 的 JAAS 包包含一个 `AuthorityGranter` 接口。

```java
public interface AuthorityGranter {
    String[] grant(Authentication authentication);
}
```

## DefaultJaasAuthenticationProvider

The `DefaultJaasAuthenticationProvider` allows a JAAS `Configuration` object to be injected as a dependency. It then uses the injected JAAS `Configuration` to create a `LoginContext`.

`DefaultJaasAuthenticationProvider` 允许将 JAAS `Configuration` 对象作为依赖项注入其中。然后，它使用注入的 JAAS `Configuration` 创建一个 `LoginContext`。

### InMemoryConfiguration / 内存配置

For convenience in injecting a `Configuration` into `DefaultJaasAuthenticationProvider`, a default in-memory implementation called `InMemoryConfiguration` is provided.

为了方便将 `Configuration` 注入到 `DefaultJaasAuthenticationProvider` 中，提供了一个名为 `InMemoryConfiguration` 的默认内存实现。

### Example Configuration / 示例配置

```xml
<bean id="jaasAuthProvider"
    class="org.springframework.security.authentication.jaas.DefaultJaasAuthenticationProvider">
    <property name="configuration">
        <bean class="org.springframework.security.authentication.jaas.memory.InMemoryConfiguration">
            <constructor-arg>
                <map>
                    <entry key="SPRINGSECURITY">
                        <array>
                            <bean class="javax.security.auth.login.AppConfigurationEntry">
                                <constructor-arg value="sample.SampleLoginModule" />
                                <constructor-arg>
                                    <util:constant static-field=
                                        "javax.security.auth.login.AppConfigurationEntry$LoginModuleControlFlag.REQUIRED"/>
                                </constructor-arg>
                                <constructor-arg>
                                    <map></map>
                                </constructor-arg>
                            </bean>
                        </array>
                    </entry>
                </map>
            </constructor-arg>
        </bean>
    </property>
    <property name="authorityGranters">
        <list>
            <!-- You will need to write your own implementation of AuthorityGranter -->
            <bean class="org.springframework.security.authentication.jaas.TestAuthorityGranter"/>
        </list>
    </property>
</bean>
```

## JaasAuthenticationProvider

The `JaasAuthenticationProvider` assumes the default `Configuration` is an instance of `ConfigFile`. It attempts to update the `Configuration` and then uses the default `Configuration` to create a `LoginContext`.

`JaasAuthenticationProvider` 假设默认的 `Configuration` 是 `ConfigFile` 的一个实例。它尝试更新 `Configuration`，然后使用默认的 `Configuration` 来创建 `LoginContext`。

### Login Configuration File / 登录配置文件

```conf
JAASTest {
    sample.SampleLoginModule required;
};
```

### Example Configuration / 示例配置

```xml
<bean id="jaasAuthenticationProvider"
    class="org.springframework.security.authentication.jaas.JaasAuthenticationProvider">
    <property name="loginConfig" value="/WEB-INF/login.conf"/>
    <property name="loginContextName" value="JAASTest"/>
    <property name="callbackHandlers">
        <list>
            <bean class="org.springframework.security.authentication.jaas.JaasNameCallbackHandler"/>
            <bean class="org.springframework.security.authentication.jaas.JaasPasswordCallbackHandler"/>
        </list>
    </property>
    <property name="authorityGranters">
        <list>
            <bean class="org.springframework.security.authentication.jaas.TestAuthorityGranter"/>
        </list>
    </property>
</bean>
```

## Running as a Subject / 以 Subject 身份运行

If configured, `JaasApiIntegrationFilter` attempts to run as the `Subject` on the `JaasAuthenticationToken`. This means the `Subject` can be accessed using:

如果已配置，`JaasApiIntegrationFilter` 会尝试以 `JaasAuthenticationToken` 上的 `Subject` 身份运行。这意味着可以使用以下方式访问 `Subject`：

```java
Subject subject = Subject.getSubject(AccessController.getContext());
```

This feature is useful when integrating with legacy or external APIs that depend on a populated JAAS Subject.

当与依赖于已填充的 JAAS Subject 的遗留或外部 API 集成时，此功能非常有用。

*Source: https://docs.springframework.org.cn/spring-security/reference/servlet/authentication/jaas.html*
