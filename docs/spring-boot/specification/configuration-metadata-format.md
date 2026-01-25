# Configuration Metadata Format / 配置元数据格式

Source: https://docs.springframework.org.cn/spring-boot/specification/configuration-metadata/format.html

---

## English

Configuration metadata files are located in JAR files under `META-INF/spring-configuration-metadata.json`. They use a JSON format with items categorized under "groups" or "properties", additional value hints categorized under "hints", and ignored items categorized under "ignored", as shown in the following example:

```json
{
    "groups": [
        {
            "name": "server",
            "type": "org.springframework.boot.web.server.autoconfigure.ServerProperties",
            "sourceType": "org.springframework.boot.web.server.autoconfigure.ServerProperties"
        },
        {
            "name": "spring.jpa.hibernate.",
            "type": "org.springframework.boot.jpa.autoconfigure.JpaProperties$Hibernate",
            "sourceType": "org.springframework.boot.jpa.autoconfigure.JpaProperties",
            "sourceMethod": "getHibernate()"
        }
        ...
    ],
    "properties": [
        {
            "name": "server.port",
            "type": "java.lang.Integer",
            "sourceType": "org.springframework.boot.web.server.autoconfigure.ServerProperties"
        },
        {
            "name": "server.address",
            "type": "java.net.InetAddress",
            "sourceType": "org.springframework.boot.web.server.autoconfigure.ServerProperties"
        },
        {
            "name": "spring.jpa.hibernate.ddl-auto",
            "type": "java.lang.String",
            "description": "DDL mode. This is actually a shortcut for the \"hibernate.hbm2ddl.auto\" property.",
            "sourceType": "org.springframework.boot.jpa.autoconfigure.JpaProperties$Hibernate"
        }
        ...
    ],
    "hints": [
        {
            "name": "spring.jpa.hibernate.ddl-auto",
            "values": [
                {
                    "value": "none",
                    "description": "Disable DDL handling."
                },
                {
                    "value": "validate",
                    "description": "Validate the schema, make no changes to the database."
                },
                {
                    "value": "update",
                    "description": "Update the schema if necessary."
                },
                {
                    "value": "create",
                    "description": "Create the schema and destroy previous data."
                },
                {
                    "value": "create-drop",
                    "description": "Create and then destroy the schema at the end of the session."
                }
            ]
        }
        ...
    ],
    "ignored": {
        "properties": [
            {
                "name": "server.ignored"
            }
            ...
        ]
    }
}
```

Each "property" is a configuration item that you specify a value for. For example, `server.port` and `server.address` might be specified in your `application.properties`/`application.yaml` as follows:

- Properties
- YAML

```properties
server.port=9090
server.address=127.0.0.1
```

```yaml
server:
  port: 9090
  address: 127.0.0.1
```

A "group" is a higher-level item that does not itself specify a value, but provides a contextual grouping for properties. For example, the `server.port` and `server.address` properties are part of the `server` group.

|  |  |
| --- | --- |
|  | Not every "property" needs to have a "group". Some properties may exist independently. |

A "hint" is additional information used to help the user configure a given property. For example, when a developer configures the `spring.jpa.hibernate.ddl-auto` property, tools can use the hints to provide auto-completion assistance for the `none`, `validate`, `update`, `create`, and `create-drop` values.

Finally, "ignored" is used for items that have been deliberately ignored. The content of this section typically comes from additional metadata.

The JSON objects contained in the `properties` array can include the attributes shown in the following table:

| Name | Type | Purpose |
|------|------|---------|
| `name` | String | The full name of the property. Names are in lowercase period-separated form (for example, `server.address`). This attribute is mandatory. |
| `type` | String | The full signature of the data type of the property (for example, `String`), but can also be a full generic type (for example, `java.util.Map<java.lang.String,com.example.MyEnum>`). You can use this attribute to guide the user as to the type of value they should enter. For consistency, primitive types are specified by their wrapper counterparts (for example, `boolean` becomes `Boolean`). Note that this type can be a complex type that is converted from `String` when binding the value. If the type is unknown, it can be omitted. |
| `description` | String | A short description of the property that can be displayed to the user. If there is no description, it can be omitted. We recommend that descriptions be short paragraphs, with the first line providing a concise summary. The last line of the description should end with a period (`.`). |
| `sourceType` | String | The class name of the source that provides this property. For example, if the property comes from a class annotated with `@ConfigurationProperties`, this attribute will contain the fully qualified name of that class. If the source type is unknown, it can be omitted. |
| `defaultValue` | Object | The default value that will be used if the property is not specified. If the property type is an array, it can be an array of values. If the default value is unknown, it can be omitted. |
| `deprecation` | Deprecation | Specifies whether the property is deprecated. If the field is not deprecated or the information is unknown, it can be omitted. The following table provides more information about the `deprecation` attribute. |

The JSON objects contained in the `deprecation` attribute of each `properties` element can include the following attributes:

| Name | Type | Purpose |
|------|------|---------|
| `level` | String | The deprecation level, which can be `warning` (the default) or `error`. When a property has a `warning` deprecation level, it should still be bound in the environment. However, when it has an `error` deprecation level, the property is no longer managed and is not bound. |
| `reason` | String | A short description of why the property is deprecated. If there is no reason, it can be omitted. We recommend that descriptions be short paragraphs, with the first line providing a concise summary. The last line of the description should end with a period (`.`). |
| `replacement` | String | The full name of the property that replaces this deprecated property. If this property has no replacement, it can be omitted. |
| `since` | String | The version in which the property was deprecated. Can be omitted. |

|  |  |
| --- | --- |
|  | Prior to Spring Boot 1.3, a single `deprecated` boolean property could be used instead of the `deprecation` element. This is still supported in a deprecated fashion and should not be used anymore. If there is no reason and replacement, an empty `deprecation` object should be set. |

Deprecation can also be specified declaratively in code by adding the `@DeprecatedConfigurationProperty` annotation to the getter that exposes the deprecated property. For example, assume the `my.app.target` property was confusing and has been renamed to `my.app.name`. The following example shows how to handle that situation:

- Java
- Kotlin

```java
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.boot.context.properties.DeprecatedConfigurationProperty;

@ConfigurationProperties("my.app")
public class MyProperties {

    private String name;

    public String getName() {
        return this.name;
    }

    public void setName(String name) {
        this.name = name;
    }

    @Deprecated
    @DeprecatedConfigurationProperty(replacement = "my.app.name", since = "1.2.0")
    public String getTarget() {
        return this.name;
    }

    @Deprecated
    public void setTarget(String target) {
        this.name = target;
    }

}
```

```kotlin
import org.springframework.boot.context.properties.ConfigurationProperties
import org.springframework.boot.context.properties.DeprecatedConfigurationProperty

@ConfigurationProperties("my.app")
class MyProperties(val name: String?) {

    var target: String? = null
        @Deprecated("") @DeprecatedConfigurationProperty(replacement = "my.app.name", since = "1.2.0") get
        @Deprecated("") set

}
```

|  |  |
| --- | --- |
|  | There is no way to set a `level`. It is always assumed to be `warning` because the code is still handling the property. |

The preceding code ensures that the deprecated property still works (delegating to the `name` property behind the scenes). Once the `getTarget` and `setTarget` methods can be removed from your public API, the automatic deprecation hints in the metadata will disappear as well. If you want to keep the hint, adding manual metadata with an `error` deprecation level ensures that users are still aware of the property. This is particularly useful when a `replacement` is provided.

---

## 中文 / Chinese

配置元数据文件位于 JAR 包中 `META-INF/spring-configuration-metadata.json` 路径下。它们使用 JSON 格式，其中包含按"组"或"属性"分类的项，按"提示"分类的附加值提示，以及按"忽略"分类的被忽略项，如下例所示：

```json
{
    "groups": [
        {
            "name": "server",
            "type": "org.springframework.boot.web.server.autoconfigure.ServerProperties",
            "sourceType": "org.springframework.boot.web.server.autoconfigure.ServerProperties"
        },
        {
            "name": "spring.jpa.hibernate.",
            "type": "org.springframework.boot.jpa.autoconfigure.JpaProperties$Hibernate",
            "sourceType": "org.springframework.boot.jpa.autoconfigure.JpaProperties",
            "sourceMethod": "getHibernate()"
        }
        ...
    ],
    "properties": [
        {
            "name": "server.port",
            "type": "java.lang.Integer",
            "sourceType": "org.springframework.boot.web.server.autoconfigure.ServerProperties"
        },
        {
            "name": "server.address",
            "type": "java.net.InetAddress",
            "sourceType": "org.springframework.boot.web.server.autoconfigure.ServerProperties"
        },
        {
            "name": "spring.jpa.hibernate.ddl-auto",
            "type": "java.lang.String",
            "description": "DDL mode. This is actually a shortcut for the \"hibernate.hbm2ddl.auto\" property.",
            "sourceType": "org.springframework.boot.jpa.autoconfigure.JpaProperties$Hibernate"
        }
        ...
    ],
    "hints": [
        {
            "name": "spring.jpa.hibernate.ddl-auto",
            "values": [
                {
                    "value": "none",
                    "description": "Disable DDL handling."
                },
                {
                    "value": "validate",
                    "description": "Validate the schema, make no changes to the database."
                },
                {
                    "value": "update",
                    "description": "Update the schema if necessary."
                },
                {
                    "value": "create",
                    "description": "Create the schema and destroy previous data."
                },
                {
                    "value": "create-drop",
                    "description": "Create and then destroy the schema at the end of the session."
                }
            ]
        }
        ...
    ],
    "ignored": {
        "properties": [
            {
                "name": "server.ignored"
            }
            ...
        ]
    }
}
```

每个"属性"是用户指定值的配置项。例如，`server.port` 和 `server.address` 可能在您的 `application.properties`/`application.yaml` 中指定，如下所示：

- 属性
- YAML

```properties
server.port=9090
server.address=127.0.0.1
```

```yaml
server:
  port: 9090
  address: 127.0.0.1
```

"组"是更高级别的项，它们本身不指定值，而是为属性提供上下文分组。例如，`server.port` 和 `server.address` 属性是 `server` 组的一部分。

|  |  |
| --- | --- |
|  | 并非每个"属性"都必须有一个"组"。有些属性可能独立存在。 |

"提示"是用于帮助用户配置给定属性的附加信息。例如，当开发人员配置 `spring.jpa.hibernate.ddl-auto` 属性时，工具可以使用提示为 `none`、`validate`、`update`、`create` 和 `create-drop` 值提供自动补全帮助。

最后，"忽略"用于已被特意忽略的项。此部分的内容通常来自附加元数据。

`properties` 数组中包含的 JSON 对象可以包含下表所示的属性：

| 名称 | 类型 | 目的 |
|------|------|------|
| `name` | 字符串 | 属性的完整名称。名称采用小写句点分隔形式（例如，`server.address`）。此属性是强制性的。 |
| `type` | 字符串 | 属性数据类型的完整签名（例如，`String`），但也包括完整的泛型类型（例如 `java.util.Map<java.lang.String,com.example.MyEnum>`）。您可以使用此属性来指导用户输入值的类型。为了一致性，原始类型通过其包装器对应项指定（例如，`boolean` 变为 `Boolean`）。请注意，此类型可能是一个复杂类型，在绑定值时会从 `String` 转换而来。如果类型未知，则可以省略。 |
| `description` | 字符串 | 可以向用户显示的属性的简短描述。如果没有描述，可以省略。建议描述为简短段落，第一行提供简洁摘要。描述的最后一行应以句号（`.`）结尾。 |
| `sourceType` | 字符串 | 提供此属性的源的类名。例如，如果属性来自使用 `@ConfigurationProperties` 注解的类，则此属性将包含该类的完全限定名。如果源类型未知，则可以省略。 |
| `defaultValue` | 对象 | 默认值，如果未指定属性，则使用此值。如果属性类型是数组，则可以是值的数组。如果默认值未知，则可以省略。 |
| `deprecation` | 废弃 | 指定属性是否已废弃。如果字段未废弃或该信息未知，则可以省略。下表提供了有关 `deprecation` 属性的更多详细信息。 |

每个 `properties` 元素的 `deprecation` 属性中包含的 JSON 对象可以包含以下属性：

| 名称 | 类型 | 目的 |
|------|------|------|
| `level` | 字符串 | 废弃级别，可以是 `warning`（默认值）或 `error`。当属性具有 `warning` 废弃级别时，它仍应在环境中绑定。但是，当它具有 `error` 废弃级别时，该属性不再受管理且未绑定。 |
| `reason` | 字符串 | 属性被废弃的简短原因描述。如果没有原因，可以省略。建议描述为简短段落，第一行提供简洁摘要。描述的最后一行应以句号（`.`）结尾。 |
| `replacement` | 字符串 | 替换此废弃属性的属性的完整名称。如果此属性没有替换项，则可以省略。 |
| `since` | 字符串 | 属性被废弃的版本。可以省略。 |

|  |  |
| --- | --- |
|  | 在 Spring Boot 1.3 之前，可以使用单个 `deprecated` 布尔属性而不是 `deprecation` 元素。这仍然以废弃的方式受支持，不应再使用。如果没有原因和替换，应设置一个空的 `deprecation` 对象。 |

废弃也可以通过向公开废弃属性的 getter 添加 `@DeprecatedConfigurationProperty` 注解在代码中声明性地指定。例如，假设 `my.app.target` 属性令人困惑，并已重命名为 `my.app.name`。以下示例展示了如何处理这种情况：

- Java
- Kotlin

```java
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.boot.context.properties.DeprecatedConfigurationProperty;

@ConfigurationProperties("my.app")
public class MyProperties {

    private String name;

    public String getName() {
        return this.name;
    }

    public void setName(String name) {
        this.name = name;
    }

    @Deprecated
    @DeprecatedConfigurationProperty(replacement = "my.app.name", since = "1.2.0")
    public String getTarget() {
        return this.name;
    }

    @Deprecated
    public void setTarget(String target) {
        this.name = target;
    }

}
```

```kotlin
import org.springframework.boot.context.properties.ConfigurationProperties
import org.springframework.boot.context.properties.DeprecatedConfigurationProperty

@ConfigurationProperties("my.app")
class MyProperties(val name: String?) {

    var target: String? = null
        @Deprecated("") @DeprecatedConfigurationProperty(replacement = "my.app.name", since = "1.2.0") get
        @Deprecated("") set

}
```

|  |  |
| --- | --- |
|  | 无法设置 `level`。始终假定为 `warning`，因为代码仍在处理该属性。 |

前面的代码确保废弃的属性仍然有效（在幕后委托给 `name` 属性）。一旦 `getTarget` 和 `setTarget` 方法可以从您的公共 API 中删除，元数据中的自动废弃提示也会消失。如果您想保留提示，添加带有 `error` 废弃级别的手动元数据可确保用户仍然了解该属性。当提供了 `replacement` 时，这样做特别有用。
