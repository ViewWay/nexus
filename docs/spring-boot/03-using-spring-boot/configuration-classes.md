# Configuration Classes / 配置类

Source: https://docs.springframework.org.cn/spring-boot/reference/using/configuration-classes.html

---

## English

Spring Boot favors Java-based configuration. Although it is possible to use XML sources with `SpringApplication`, we generally recommend that your primary source be a single `@Configuration` class. Generally, the class that defines the `main` method is a good candidate as the primary `@Configuration`.

|  |  |
| --- | --- |
|  | Many Spring configuration examples available on the Internet use XML configuration. If possible, always try to use the equivalent Java-based configuration. Searching for `Enable*` annotations is a good starting point. |

## Importing Additional Configuration Classes

You need not put all your `@Configuration` into a single class. The `@Import` annotation can be used to import additional configuration classes. Alternatively, you can use `@ComponentScan` to automatically pick up all Spring components, including `@Configuration` classes.

## Importing XML Configuration

If you absolutely must use XML-based configuration, we recommend that you still start with a `@Configuration` class. You can then use an `@ImportResource` annotation to load XML configuration files.

---

## 中文 / Chinese

Spring Boot 偏向于基于 Java 的配置。虽然可以使用 XML 源与 `SpringApplication` 一起使用，但我们通常建议您的主要源是一个单一的 `@Configuration` 类。通常，定义 `main` 方法的类是作为主要 `@Configuration` 的一个很好的候选者。

|  |  |
| --- | --- |
|  | 互联网上发布了许多使用 XML 配置的 Spring 配置示例。如果可能，请始终尝试使用等效的基于 Java 的配置。搜索 `Enable*` 注解是一个不错的起点。 |

## 导入附加配置类

您无需将所有 `@Configuration` 都放在一个类中。可以使用 `@Import` 注解导入其他配置类。或者，您可以使用 `@ComponentScan` 自动获取所有 Spring 组件，包括 `@Configuration` 类。

## 导入 XML 配置

如果您绝对必须使用基于 XML 的配置，我们建议您仍然从 `@Configuration` 类开始。然后，您可以使用 `@ImportResource` 注解加载 XML 配置文件。
