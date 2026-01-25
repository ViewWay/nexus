# 提前优化

本章介绍 Spring 的提前 (AOT) 优化。

有关集成测试特定的 AOT 支持，请参阅 测试的提前支持。

## 提前优化介绍

Spring 对 AOT 优化的支持旨在在构建时检查 `ApplicationContext`，并应用通常在运行时发生的决策和发现逻辑。这样做可以构建更直接的应用程序启动安排，主要基于类路径和环境，专注于固定的一组功能。

提早应用此类优化意味着以下限制：

- 类路径在构建时是固定的且完全定义。
- 应用程序中定义的 bean 在运行时不能改变，这意味着：
  - `@Profile`，特别是针对特定配置文件的配置，需要在构建时选择，并在启用 AOT 时在运行时自动启用。
  - 影响 bean 是否存在的 `Environment` 属性 (`@Conditional`) 只在构建时考虑。
- 带有实例供应商（lambda 或方法引用）的 Bean 定义无法提前转换。
- 注册为单例（通常使用 `ConfigurableListableBeanFactory` 的 `registerSingleton`）的 Bean 也无法提前转换。
- 由于我们不能依赖实例，请确保 Bean 类型尽可能精确。

|  |  |
| --- | --- |
|  | 另请参阅最佳实践部分。 |

当存在这些限制时，就可以在构建时执行提前处理并生成额外的资产。经过 Spring AOT 处理的应用程序通常会生成：

- Java 源代码
- 字节码（通常用于动态代理）
- `RuntimeHints` 用于反射、资源加载、序列化和 JDK 代理

|  |  |
| --- | --- |
|  | 目前，AOT 主要用于允许 Spring 应用程序使用 GraalVM 部署为原生镜像。我们打算在未来的版本中支持更多基于 JVM 的用例。 |

## AOT 引擎概述

AOT 引擎处理 `ApplicationContext` 的入口点是 `ApplicationContextAotGenerator`。它基于代表要优化的应用程序的 `GenericApplicationContext` 和一个 `GenerationContext` 来处理以下步骤：

- 刷新 `ApplicationContext` 以进行 AOT 处理。与传统的刷新不同，此版本只创建 bean 定义，而不创建 bean 实例。
- 调用可用的 `BeanFactoryInitializationAotProcessor` 实现，并将其贡献应用于 `GenerationContext`。例如，一个核心实现会遍历所有候选 bean 定义，并生成必要的代码来恢复 `BeanFactory` 的状态。

此过程完成后，`GenerationContext` 将更新生成的代码、资源和应用程序运行所需的类。`RuntimeHints` 实例还可以用于生成相关的 GraalVM 原生镜像配置文件。

`ApplicationContextAotGenerator#processAheadOfTime` 返回 `ApplicationContextInitializer` 入口点的类名，该入口点允许在启用 AOT 优化的情况下启动上下文。

## AOT 处理的刷新

所有 `GenericApplicationContext` 实现都支持 AOP 处理的刷新。可以使用任意数量的入口点创建应用程序上下文，通常是 `@Configuration` 注解的类。

## Bean Factory 初始化 AOT 贡献

想要参与此步骤的组件可以实现 `BeanFactoryInitializationAotProcessor` 接口。每个实现都可以基于 bean 工厂的状态返回一个 AOT 贡献。

## 使用 AOT 优化运行

|  |  |
| --- | --- |
|  | AOT 是将 Spring 应用程序转换为原生可执行文件的一个强制步骤，因此在此模式下运行时会自动启用它。可以通过将 `spring.aot.enabled` 系统属性设置为 `true` 来在 JVM 上使用这些优化。 |

## 最佳实践

AOT 引擎旨在处理尽可能多的用例，而无需更改应用程序代码。但是，请记住，一些优化是在构建时基于 bean 的静态定义进行的。

本节列出了确保应用程序为 AOT 做好准备的最佳实践：

- 编程式 Bean 注册
- 暴露最精确的 Bean 类型
- 避免将复杂数据结构用于构造函数参数和属性
- 避免使用自定义参数创建 Bean
- 避免循环依赖
- FactoryBean
- JPA

## 运行时提示

与常规 JVM 运行时相比，将应用程序作为本地镜像运行需要额外信息。例如，GraalVM 需要事先知道组件是否使用了反射。类似地，除非明确指定，否则类路径资源不会包含在本地镜像中。

`RuntimeHints` API 收集运行时对反射、资源加载、序列化和 JDK 代理的需求。

- `@ImportRuntimeHints`
- `@Reflective`
- `@RegisterReflection`
- 测试运行时提示

---

*来源：https://docs.springframework.org.cn/spring-framework/reference/core/aot.html*
