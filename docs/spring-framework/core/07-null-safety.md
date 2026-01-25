# 空安全

尽管 Java 的类型系统不允许表达空安全，但 Spring Framework 在 `org.springframework.lang` 包中提供了以下注解，用于声明 API 和字段的可空性：

- `@Nullable`：一个注解，用于指示特定的参数、返回值或字段可以为 `null`。
- `@NonNull`：一个注解，用于指示特定的参数、返回值或字段不能为 `null`（在应用了 `@NonNullApi` 和 `@NonNullFields` 的参数、返回值和字段上则不需要此注解）。
- `@NonNullApi`：一个包级别注解，声明参数和返回值的默认语义为非空。
- `@NonNullFields`：一个包级别注解，声明字段的默认语义为非空。

Spring Framework 本身就使用了这些注解，但它们也可用于任何基于 Spring 的 Java 项目中，以声明空安全的 API 和可选的空安全字段。尚不支持泛型类型参数、可变参数和数组元素的空安全声明。空安全声明预计将在 Spring Framework 的各个版本之间（包括次要版本）进行微调。方法体内部使用的类型的空安全不在本功能的范围之内。

|  |  |
| --- | --- |
|  | 其他常用库（如 Reactor 和 Spring Data）也提供了使用类似空安全安排的空安全 API，为 Spring 应用开发者提供了整体一致的体验。 |

## 用例

除了为 Spring Framework API 的可空性提供显式声明外，这些注解还可以被 IDE（如 IDEA 或 Eclipse）用来提供与空安全相关的有用警告，以避免运行时出现 `NullPointerException`。

它们也被用于使 Spring API 在 Kotlin 项目中实现空安全，因为 Kotlin 原生支持空安全。更多详情请参阅 Kotlin 支持文档。

## JSR-305 元注解

Spring 注解使用 JSR 305 注解进行元注解（一个休眠但广泛使用的 JSR）。JSR-305 元注解允许 IDEA 或 Kotlin 等工具供应商以通用方式提供空安全支持，而无需硬编码对 Spring 注解的支持。

利用 Spring 的空安全 API 时，既不需要也不推荐向项目类路径添加 JSR-305 依赖。只有像在代码库中使用空安全注解的基于 Spring 的库这样的项目，才应在 Gradle 配置中添加 `com.google.code.findbugs:jsr305:3.0.2` 并使用 `compileOnly`，或在 Maven 中使用 `provided` 范围，以避免编译器警告。

---

*来源：https://docs.springframework.org.cn/spring-framework/reference/core/null-safety.html*
