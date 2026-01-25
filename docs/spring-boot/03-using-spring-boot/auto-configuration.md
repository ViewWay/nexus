# Auto-configuration / 自动配置

Source: https://docs.springframework.org.cn/spring-boot/reference/using/auto-configuration.html

---

## English

Spring Boot auto-configuration attempts to automatically configure your Spring application based on the jar dependencies that you have added. For example, if `HSQLDB` is on your classpath and you have not manually configured any database connection beans, Spring Boot automatically configures an in-memory database.

You need to opt-in to auto-configuration by adding the `@EnableAutoConfiguration` or `@SpringBootApplication` annotation to one of your `@Configuration` classes.

|  |  |
| --- | --- |
|  | You should only add one `@SpringBootApplication` or `@EnableAutoConfiguration` annotation. We generally recommend that you add only one to your primary `@Configuration` class. |

## Gradually Replacing Auto-configuration

Auto-configuration is non-invasive. At any point, you can start to define your own configuration to replace specific parts of auto-configuration. For example, if you add your own `DataSource` bean, the default embedded database support backs off.

If you need to find out what auto-configuration is currently being applied and why, start your application with the `--debug` switch. Doing so enables debug logs for a selection of core loggers and logs a conditions report to the console.

## Disabling Specific Auto-configuration Classes

If you find that specific auto-configuration classes are being applied that you do not want, you can use the `exclude` attribute of `@SpringBootApplication` to disable them, as shown in the following example:

- Java
- Kotlin

```java
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.jdbc.autoconfigure.DataSourceAutoConfiguration;

@SpringBootApplication(exclude = { DataSourceAutoConfiguration.class })
public class MyApplication {

}
```

```kotlin
import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.jdbc.autoconfigure.DataSourceAutoConfiguration

@SpringBootApplication(exclude = [DataSourceAutoConfiguration::class])
class MyApplication
```

If the class is not on the classpath, you can use the `excludeName` attribute of the annotation and specify the fully qualified name. If you prefer to use `@EnableAutoConfiguration` rather than `@SpringBootApplication`, `exclude` and `excludeName` are also available. Finally, you can also control the list of auto-configuration classes to exclude by using the `spring.autoconfigure.exclude` property.

|  |  |
| --- | --- |
|  | You can define exclusions both at the annotation level and by using a property. |

|  |  |
| --- | --- |
|  | While auto-configuration classes are `public`, the only aspect of the class that is considered public API is the name of the class which can be used to disable the auto-configuration. The actual content of the classes, such as nested configuration classes or bean methods, is for internal use and we do not recommend using them directly. |

## Auto-configuration Packages

The auto-configuration package is the package that various auto-configuration features default to looking in for entities and Spring Data repositories and more. The `@EnableAutoConfiguration` annotation (either directly or through its presence on `@SpringBootApplication`) determines the default auto-configuration package. Additional packages can be configured by using the `@AutoConfigurationPackage` annotation.

---

## 中文 / Chinese

Spring Boot 自动配置尝试根据您添加的 jar 依赖项自动配置您的 Spring 应用程序。例如，如果您的类路径中存在 `HSQLDB`，并且您尚未手动配置任何数据库连接 bean，则 Spring Boot 会自动配置一个内存数据库。

您需要通过在其中一个 `@Configuration` 类上添加 `@EnableAutoConfiguration` 或 `@SpringBootApplication` 注解来选择启用自动配置。

|  |  |
| --- | --- |
|  | 您应该只添加一个 `@SpringBootApplication` 或 `@EnableAutoConfiguration` 注解。我们通常建议您只将其中一个添加到您的主 `@Configuration` 类中。 |

## 逐步替换自动配置

自动配置是非侵入性的。在任何时候，您都可以开始定义自己的配置来替换自动配置的特定部分。例如，如果您添加自己的 `DataSource` bean，默认的嵌入式数据库支持就会退出。

如果您需要了解当前正在应用哪些自动配置以及原因，请使用 `--debug` 开关启动您的应用程序。这样做会为一部分核心日志记录器启用调试日志，并将条件报告记录到控制台。

## 禁用特定自动配置类

如果您发现正在应用您不希望的特定自动配置类，您可以使用 `@SpringBootApplication` 的 exclude 属性来禁用它们，如以下示例所示：

- Java
- Kotlin

```java
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.jdbc.autoconfigure.DataSourceAutoConfiguration;

@SpringBootApplication(exclude = { DataSourceAutoConfiguration.class })
public class MyApplication {

}
```

```kotlin
import org.springframework.boot.autoconfigure.SpringBootApplication
import org.springframework.boot.jdbc.autoconfigure.DataSourceAutoConfiguration

@SpringBootApplication(exclude = [DataSourceAutoConfiguration::class])
class MyApplication
```

如果该类不在类路径中，您可以使用注解的 `excludeName` 属性并指定完全限定名。如果您更喜欢使用 `@EnableAutoConfiguration` 而不是 `@SpringBootApplication`，则 `exclude` 和 `excludeName` 也可用。最后，您还可以通过使用 `spring.autoconfigure.exclude` 属性来控制要排除的自动配置类列表。

|  |  |
| --- | --- |
|  | 您可以在注解级别和使用属性来定义排除项。 |

|  |  |
| --- | --- |
|  | 尽管自动配置类是 `public` 的，但该类唯一被视为公共 API 的方面是类的名称，可用于禁用自动配置。这些类的实际内容，例如嵌套配置类或 bean 方法，仅供内部使用，我们不建议直接使用它们。 |

## 自动配置包

自动配置包是各种自动配置功能在扫描实体和 Spring Data 仓库等内容时默认查找的包。`@EnableAutoConfiguration` 注解（直接或通过其在 `@SpringBootApplication` 上的存在）确定默认的自动配置包。可以使用 `@AutoConfigurationPackage` 注解配置其他包。
