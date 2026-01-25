# 校验、数据绑定和类型转换

将校验视为业务逻辑有利有弊，Spring 提供了一种校验和数据绑定的设计，该设计并不排除其中任何一种。具体来说，校验不应与 Web 层绑定，应易于本地化，并且应能够插入任何可用的校验器。考虑到这些问题，Spring 提供了一个既基本又易于在应用程序的每一层中使用的 `Validator` 契约。

数据绑定有助于将用户输入动态绑定到应用程序的领域模型（或您用于处理用户输入的任何对象）。Spring 提供了恰如其名的 `DataBinder` 来实现此功能。`Validator` 和 `DataBinder` 构成了 `validation` 包，该包主要用于但不限于 Web 层。

`BeanWrapper` 是 Spring Framework 中的一个基本概念，并在许多地方使用。但是，您可能不需要直接使用 `BeanWrapper`。然而，由于这是参考文档，我们认为有必要进行一些解释。我们在本章中解释 `BeanWrapper`，因为如果您要使用它，很可能是在尝试将数据绑定到对象时使用。

Spring 的 `DataBinder` 和较低层的 `BeanWrapper` 都使用 `PropertyEditorSupport` 实现来解析和格式化属性值。`PropertyEditor` 和 `PropertyEditorSupport` 类型是 JavaBeans 规范的一部分，本章中也有解释。Spring 的 `core.convert` 包提供了一个通用的类型转换功能，以及一个用于格式化 UI 字段值的高层 `format` 包。您可以使用这些包作为 `PropertyEditorSupport` 实现的更简单替代方案。本章中也讨论了它们。

Spring 通过设置基础结构和对 Spring 自己的 `Validator` 契约的适配器来支持 Java Bean 校验。应用程序可以在全局范围内一次性启用 Bean 校验，如Java Bean 校验中所述，并将其专门用于所有校验需求。在 Web 层，应用程序可以根据每个 `DataBinder` 进一步注册控制器本地的 Spring `Validator` 实例，如配置 `DataBinder`中所述，这对于插入自定义校验逻辑非常有用。

## 章节摘要

- 使用 Spring 的 Validator 接口进行校验
- 数据绑定
- 将代码解析为错误消息
- Spring 类型转换
- Spring 字段格式化
- 配置全局日期和时间格式
- Java Bean 校验

---

*来源：https://docs.springframework.org.cn/spring-framework/reference/core/validation.html*
