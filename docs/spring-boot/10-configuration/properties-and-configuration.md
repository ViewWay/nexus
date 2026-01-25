# Properties and Configuration / 属性与配置

Source: https://docs.springframework.org.cn/spring-boot/how-to/properties-and-configuration.html

---

## English

This section contains topics on setting and reading properties and configuration settings and how they interact with Spring Boot Applications.

## Expanding Properties at Build Time

Rather than hardcoding certain properties that are also specified in your project's build configuration, you can use the existing build configuration to automatically expand them. This is possible in both Maven and Gradle.

### Using Maven to Expand Properties

You can automatically expand properties in a Maven project by using resource filtering. If you use the `spring-boot-starter-parent`, you can refer to your Maven 'project properties' with `@..@` placeholders, for example in your `application.properties`:

- Properties
- YAML

```properties
[email protected]@
[email protected]@
```

```yaml
app:
  encoding: "@project.build.sourceEncoding@"
  java:
    version: "@java.version@"
```

|  |  |
| --- | --- |
|  | Only production configurations are filtered in this way (in other words, no filtering is applied on `src/test/resources`). |
|  |  |
| --- | --- |
|  | If you enable the `addResources` flag, the `spring-boot:run` goal can add `src/main/resources` directly to the classpath (for hot reloading). Doing so circumvents resource filtering and this feature. You can use the `exec:java` goal instead or custom plugin configuration. See the plugin usage page for more details. |

If you do not use the starter parent, you need to include the following element inside the `<build/>` element of your `pom.xml`:

```xml
<resources>
    <resource>
        <directory>src/main/resources</directory>
        <filtering>true</filtering>
    </resource>
</resources>
```

You also need to include the following element inside `<plugins/>`:

```xml
<plugin>
    <groupId>org.apache.maven.plugins</groupId>
    <artifactId>maven-resources-plugin</artifactId>
    <version>2.7</version>
    <configuration>
        <delimiters>
            <delimiter>@</delimiter>
        </delimiters>
        <useDefaultDelimiters>false</useDefaultDelimiters>
    </configuration>
</plugin>
```

|  |  |
| --- | --- |
|  | The `useDefaultDelimiters` property is important if you use standard Spring placeholders in your configuration (such as `${placeholder}`). If you do not set the property to `false`, these might be expanded by the build. |

### Using Gradle to Expand Properties

You can automatically expand properties in a Gradle project by configuring the Java plugin's `processResources` task to do so, as follows:

```groovy
tasks.named('processResources') {
    expand(project.properties)
}
```

You can then refer to your Gradle project's properties by using placeholders, as follows:

- Properties
- YAML

```properties
app.name=${name}
app.description=${description}
```

```yaml
app:
  name: "${name}"
  description: "${description}"
```

|  |  |
| --- | --- |
|  | Gradle's `expand` method uses Groovy's `SimpleTemplateEngine` which transforms `${..}` tokens. The `${..}` style conflicts with Spring's own property placeholder mechanism. To use both Spring property placeholders and automatic expansion, escape the Spring property placeholders as follows: `\\${..}`. |

## Externalizing the Configuration of SpringApplication

A `SpringApplication` has bean property setters, so you can use its Java API to modify its behavior as you create the application. Alternatively, you can externalize the configuration by setting properties in `spring.main.*`. For example, in `application.properties`, you might have the following settings:

- Properties
- YAML

```properties
spring.main.web-application-type=none
spring.main.banner-mode=off
```

```yaml
spring:
  main:
    web-application-type: "none"
    banner-mode: "off"
```

This way, the Spring Boot banner will not be printed on startup and the application will not start an embedded web server.

Properties defined in external configuration override and replace values specified using the Java API, with the notable exception of the primary sources. The primary sources are those provided to the `SpringApplication` constructor

- Java
- Kotlin

```java
import org.springframework.boot.Banner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class MyApplication {

    public static void main(String[] args) {
        SpringApplication application = new SpringApplication(MyApplication.class);
        application.setBannerMode(Banner.Mode.OFF);
        application.run(args);
    }

}
```

```kotlin
import org.springframework.boot.Banner
import org.springframework.boot.SpringApplication
import org.springframework.boot.autoconfigure.SpringBootApplication

@SpringBootApplication
object MyApplication {

    @JvmStatic
    fun main(args: Array<String>) {
        val application = SpringApplication(MyApplication::class.java)
        application.setBannerMode(Banner.Mode.OFF)
        application.run(*args)
    }

}
```

Or to the `sources(...)` method of `SpringApplicationBuilder`

- Java
- Kotlin

```java
import org.springframework.boot.Banner;
import org.springframework.boot.builder.SpringApplicationBuilder;

public class MyApplication {

    public static void main(String[] args) {
        new SpringApplicationBuilder()
            .bannerMode(Banner.Mode.OFF)
            .sources(MyApplication.class)
            .run(args);
    }

}
```

```kotlin
import org.springframework.boot.Banner
import org.springframework.boot.builder.SpringApplicationBuilder
object MyApplication {

    @JvmStatic
    fun main(args: Array<String>) {
        SpringApplicationBuilder()
            .bannerMode(Banner.Mode.OFF)
            .sources(MyApplication::class.java)
            .run(*args)
    }

}
```

Given the example above, if we have the following configuration:

- Properties
- YAML

```properties
spring.main.sources=com.example.MyDatabaseConfig,com.example.MyJmsConfig
spring.main.banner-mode=console
```

```yaml
spring:
  main:
    sources: "com.example.MyDatabaseConfig,com.example.MyJmsConfig"
    banner-mode: "console"
```

The actual application will display the banner (overridden by the configuration) and use three sources for the `ApplicationContext`. The application sources are:

1. `MyApplication` (from the code)
2. `MyDatabaseConfig` (from external configuration)
3. `MyJmsConfig` (from external configuration)

## Changing the Location of External Properties of an Application

By default, properties from different sources are added to the Spring `Environment` in a defined order (see "Spring Boot Features" section, Externalized Configuration for the exact order).

You can also provide the following system properties (or environment variables) to change the behavior:

- `spring.config.name` (`SPRING_CONFIG_NAME`): Defaults to `application` as the root of the file name.
- `spring.config.location` (`SPRING_CONFIG_LOCATION`): File to load (for example a classpath resource or a URL). A separate `Environment` property source is set up for this document, which can be overridden by system properties, environment variables or the command line.

Spring Boot always loads `application.properties` as described above, whatever the value of `spring.config.name`. If you use YAML, files with the extensions '.yaml' and '.yml' are also added to the list by default.

|  |  |
| --- | --- |
|  | If you want detailed information on what files are being loaded, you can set the debug level of the logging for `org.springframework.boot.context.config` to `trace`. |

## Using 'Short' Command Line Arguments

Some people like to use (for example) `--port=9000` instead of `--server.port=9000` to set configuration properties on the command line. You can enable this behavior by using placeholders in `application.properties`, as follows:

- Properties
- YAML

```properties
server.port=${port:8080}
```

```yaml
server:
  port: "${port:8080}"
```

|  |  |
| --- | --- |
|  | If you inherit from the `spring-boot-starter-parent` POM, the default filtering token of the `maven-resources-plugins` has been changed from `${*}` to `@` (that is `@maven.token@` instead of `${maven.token}`) to prevent conflicts with Spring-style placeholders. If you have enabled Maven filtering directly on the `application.properties`, you may also need to change the default filter token to a different delimiter. |
|  |  |
| --- | --- |
|  | In this specific case, the port binding works in PaaS environments such as Heroku or Cloud Foundry. In both of these platforms, the `PORT` environment variable is set automatically and Spring can bind to capitalized synonyms of `Environment` properties. |

## Using YAML for External Properties

YAML is a superset of JSON and, as such, is a convenient syntax for storing external properties in a hierarchical format, as follows:

```yaml
spring:
  application:
    name: "cruncher"
  datasource:
    driver-class-name: "com.mysql.jdbc.Driver"
    url: "jdbc:mysql:///test"
server:
  port: 9000
```

Create a file called `application.yaml` and put it at the root of your classpath. Then add `snakeyaml` to your dependencies (Maven coordinates `org.yaml:snakeyaml`, already included if you use the `spring-boot-starter`). The YAML file is parsed to a Java `Map<String,Object>` (like a JSON object), and Spring Boot flattens the map so that it is one level deep and has dotted keys, as many people are used to with `Properties` files in Java.

The preceding YAML example corresponds to the following `application.properties` file:

```properties
spring.application.name=cruncher
spring.datasource.driver-class-name=com.mysql.jdbc.Driver
spring.datasource.url=jdbc:mysql:///test
server.port=9000
```

For more on YAML, see "Spring Boot Features" section, Using YAML.

## Setting the Active Spring Profiles

The Spring `Environment` has an API for this, but you would typically set a system property (`spring.profiles.active`) or an OS environment variable (`SPRING_PROFILES_ACTIVE`). Also, you can launch your application with a `-D` argument (remember to put it before the main class or jar archive), as follows:

```bash
$ java -jar -Dspring.profiles.active=production demo-0.0.1-SNAPSHOT.jar
```

In Spring Boot, you can also set the active profile in `application.properties`, as follows:

- Properties
- YAML

```properties
spring.profiles.active=production
```

```yaml
spring:
  profiles:
    active: "production"
```

Values set in this way are replaced by system properties or environment variable settings but not by the `SpringApplicationBuilder.profiles()` method. Thus the latter Java API can be used to augment the profiles without changing the defaults.

For more information, see "Spring Boot Features" section, Profiles.

## Setting the Default Profile Name

The default profile is the profile that is enabled when no other profile is active. By default, the name of the default profile is `default`, but it can be changed using a system property (`spring.profiles.default`) or an OS environment variable (`SPRING_PROFILES_DEFAULT`).

In Spring Boot, you can also set the default profile name in `application.properties`, as follows:

- Properties
- YAML

```properties
spring.profiles.default=dev
```

```yaml
spring:
  profiles:
    default: "dev"
```

For more information, see "Spring Boot Features" section, Profiles.

## Changing Configuration Depending on the Environment

Spring Boot supports multi-document YAML and Properties files (see Using Multi-Document Files for details) which can be conditionally activated based on the active profiles.

If a document contains a `spring.config.activate.on-profile` key, the profile value (a comma-separated list of profile names or a profile expression) is passed to the Spring's `Environment.acceptsProfiles()` method. If the profile expression matches then the document is included in the final merge, otherwise it is not, as follows:

- Properties
- YAML

```properties
server.port=9000
#---
spring.config.activate.on-profile=development
server.port=9001
#---
spring.config.activate.on-profile=production
server.port=0
```

```yaml
server:
  port: 9000
---
spring:
  config:
    activate:
      on-profile: "development"
server:
  port: 9001
---
spring:
  config:
    activate:
      on-profile: "production"
server:
  port: 0
```

In the preceding example, the default port is 9000. However, if the Spring profile named "development" is active, the port is 9001. If "production" is active, the port is 0.

|  |  |
| --- | --- |
|  | Documents are merged in the order they are encountered. Later values override earlier values. |

## Discovering Built-in Options for External Properties

Spring Boot at runtime binds external properties from `application.properties` (or YAML files and other locations) to your application. There is no (technically it cannot be) complete list of all supported properties in a single location because contributions can come from additional jar files on your classpath.

A running application with the Actuator feature has a `configprops` endpoint that shows all the bound and bindable properties that are available through `@ConfigurationProperties`.

The appendix includes an `application.properties` example with a list of the most common properties supported by Spring Boot. The exact list can be obtained by searching the source code for `@ConfigurationProperties` and `@Value` annotations and occasionally by using the `Binder`. See externalized configuration for the exact order in which properties are loaded.

---

## 中文 / Chinese

本节包含关于设置和读取属性与配置设置及其与 Spring Boot 应用交互的主题。

## 在构建时自动展开属性

与其硬编码项目构建配置中已指定的某些属性，不如使用现有的构建配置来自动展开它们。这在 Maven 和 Gradle 中都是可能的。

### 使用 Maven 自动展开属性

你可以使用资源过滤来在 Maven 项目中自动展开属性。如果你使用 `spring-boot-starter-parent`，那么你可以通过 `@..@` 占位符引用你的 Maven '项目属性'，示例如下：

- 属性
- YAML

```properties
[email protected]@
[email protected]@
```

```yaml
app:
  encoding: "@project.build.sourceEncoding@"
  java:
    version: "@java.version@"
```

|  |  |
| --- | --- |
|  | 只有生产配置会以这种方式被过滤（换句话说，`src/test/resources` 不会应用过滤）。 |
|  |  |
| --- | --- |
|  | 如果你启用 `addResources` 标志，`spring-boot:run` goal 可以直接将 `src/main/resources` 添加到 classpath（用于热重载）。这样做会绕过资源过滤和此功能。你可以转而使用 `exec:java` goal 或自定义插件配置。更多详情请参阅插件使用页面。 |

如果你不使用 starter parent，你需要在 `pom.xml` 的 `<build/>` 元素内包含以下元素：

```xml
<resources>
    <resource>
        <directory>src/main/resources</directory>
        <filtering>true</filtering>
    </resource>
</resources>
```

你还需要在 `<plugins/>` 内包含以下元素：

```xml
<plugin>
    <groupId>org.apache.maven.plugins</groupId>
    <artifactId>maven-resources-plugin</artifactId>
    <version>2.7</version>
    <configuration>
        <delimiters>
            <delimiter>@</delimiter>
        </delimiters>
        <useDefaultDelimiters>false</useDefaultDelimiters>
    </configuration>
</plugin>
```

|  |  |
| --- | --- |
|  | 如果你在配置中使用标准的 Spring 占位符（例如 `${placeholder}`），`useDefaultDelimiters` 属性很重要。如果该属性未设置为 `false`，这些占位符可能会被构建过程展开。 |

### 使用 Gradle 自动展开属性

你可以通过配置 Java 插件的 `processResources` 任务来自动展开 Gradle 项目中的属性，示例如下：

```groovy
tasks.named('processResources') {
    expand(project.properties)
}
```

然后你可以通过使用占位符引用你的 Gradle 项目属性，示例如下：

- 属性
- YAML

```properties
app.name=${name}
app.description=${description}
```

```yaml
app:
  name: "${name}"
  description: "${description}"
```

|  |  |
| --- | --- |
|  | Gradle 的 `expand` 方法使用 Groovy 的 `SimpleTemplateEngine`，它会转换 `${..}` 标记。`${..}` 样式与 Spring 自身的属性占位符机制冲突。要同时使用 Spring 属性占位符和自动展开，请按如下方式转义 Spring 属性占位符：`\\${..}`。 |

## 外部化 SpringApplication 的配置

一个 `SpringApplication` 具有 bean 属性 setter，因此你可以在创建应用时使用其 Java API 来修改其行为。或者，你可以通过在 `spring.main.*` 中设置属性来外部化配置。例如，在 `application.properties` 中，你可能有以下设置：

- 属性
- YAML

```properties
spring.main.web-application-type=none
spring.main.banner-mode=off
```

```yaml
spring:
  main:
    web-application-type: "none"
    banner-mode: "off"
```

这样，Spring Boot banner 在启动时将不会打印，并且应用不会启动嵌入式 Web 服务器。

外部配置中定义的属性会覆盖和替换使用 Java API 指定的值，但主要源（primary sources）是值得注意的例外。主要源是提供给 `SpringApplication` 构造函数的那些源：

- Java
- Kotlin

```java
import org.springframework.boot.Banner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;

@SpringBootApplication
public class MyApplication {

    public static void main(String[] args) {
        SpringApplication application = new SpringApplication(MyApplication.class);
        application.setBannerMode(Banner.Mode.OFF);
        application.run(args);
    }

}
```

```kotlin
import org.springframework.boot.Banner
import org.springframework.boot.SpringApplication
import org.springframework.boot.autoconfigure.SpringBootApplication

@SpringBootApplication
object MyApplication {

    @JvmStatic
    fun main(args: Array<String>) {
        val application = SpringApplication(MyApplication::class.java)
        application.setBannerMode(Banner.Mode.OFF)
        application.run(*args)
    }

}
```

或者提供给 `SpringApplicationBuilder` 的 `sources(...)` 方法：

- Java
- Kotlin

```java
import org.springframework.boot.Banner;
import org.springframework.boot.builder.SpringApplicationBuilder;

public class MyApplication {

    public static void main(String[] args) {
        new SpringApplicationBuilder()
            .bannerMode(Banner.Mode.OFF)
            .sources(MyApplication.class)
            .run(args);
    }

}
```

```kotlin
import org.springframework.boot.Banner
import org.springframework.boot.builder.SpringApplicationBuilder
object MyApplication {

    @JvmStatic
    fun main(args: Array<String>) {
        SpringApplicationBuilder()
            .bannerMode(Banner.Mode.OFF)
            .sources(MyApplication::class.java)
            .run(*args)
    }

}
```

考虑到上面的示例，如果我们的配置如下：

- 属性
- YAML

```properties
spring.main.sources=com.example.MyDatabaseConfig,com.example.MyJmsConfig
spring.main.banner-mode=console
```

```yaml
spring:
  main:
    sources: "com.example.MyDatabaseConfig,com.example.MyJmsConfig"
    banner-mode: "console"
```

实际应用将显示 banner（被配置覆盖），并为 `ApplicationContext` 使用三个源。应用源是：

1. `MyApplication`（来自代码）
2. `MyDatabaseConfig`（来自外部配置）
3. `MyJmsConfig`（来自外部配置）

## 更改应用外部属性的位置

默认情况下，来自不同源的属性会按照定义的顺序添加到 Spring `Environment` 中（确切顺序请参阅"Spring Boot 特性"部分中的外部化配置）。

你也可以提供以下系统属性（或环境变量）来改变行为：

- `spring.config.name` (`SPRING_CONFIG_NAME`)：默认为 `application` 作为文件名的根。
- `spring.config.location` (`SPRING_CONFIG_LOCATION`)：要加载的文件（例如 classpath 资源或 URL）。会为此文档设置一个单独的 `Environment` 属性源，它可以被系统属性、环境变量或命令行覆盖。

无论你在环境中设置了什么，Spring Boot 总是会像上面描述的那样加载 `application.properties`。默认情况下，如果使用 YAML，那么扩展名为 '.yaml' 和 '.yml' 的文件也会被添加到列表中。

|  |  |
| --- | --- |
|  | 如果你想获取关于正在加载文件的详细信息，你可以将 `org.springframework.boot.context.config` 的日志级别设置为 `trace`。 |

## 使用'简短'命令行参数

有些人喜欢使用（例如）`--port=9000` 而不是 `--server.port=9000` 来在命令行上设置配置属性。你可以通过在 `application.properties` 中使用占位符来启用此行为，示例如下：

- 属性
- YAML

```properties
server.port=${port:8080}
```

```yaml
server:
  port: "${port:8080}"
```

|  |  |
| --- | --- |
|  | 如果你继承了 `spring-boot-starter-parent` POM，则 `maven-resources-plugins` 的默认过滤标记已从 `${*}` 更改为 `@`（即 `@maven.token@` 而不是 `${maven.token}`），以防止与 Spring 样式的占位符冲突。如果你直接为 `application.properties` 启用了 Maven 过滤，你可能还需要更改默认过滤标记以使用其他分隔符。 |
|  |  |
| --- | --- |
|  | 在这种特定情况下，端口绑定在 Heroku 或 Cloud Foundry 等 PaaS 环境中是有效的。在这两个平台上，`PORT` 环境变量会自动设置，并且 Spring 可以绑定到 `Environment` 属性的大写同义词。 |

## 使用 YAML 作为外部属性

YAML 是 JSON 的超集，因此，它是一种方便的语法，用于以层次结构格式存储外部属性，示例如下：

```yaml
spring:
  application:
    name: "cruncher"
  datasource:
    driver-class-name: "com.mysql.jdbc.Driver"
    url: "jdbc:mysql:///test"
server:
  port: 9000
```

创建一个名为 `application.yaml` 的文件，并将其放在 classpath 的根目录。然后将 `snakeyaml` 添加到你的依赖项中（Maven 坐标为 `org.yaml:snakeyaml`，如果你使用 `spring-boot-starter` 则已包含）。YAML 文件会被解析为 Java `Map<String,Object>`（类似于 JSON 对象），Spring Boot 会展平该 Map，使其深度为一层，并使用句点分隔的键，就像许多人在 Java 中习惯使用 `Properties` 文件一样。

前面的 YAML 示例对应于以下 `application.properties` 文件：

```properties
spring.application.name=cruncher
spring.datasource.driver-class-name=com.mysql.jdbc.Driver
spring.datasource.url=jdbc:mysql:///test
server.port=9000
```

有关 YAML 的更多信息，请参阅"Spring Boot 特性"部分中的使用 YAML。

## 设置活动的 Spring Profile

Spring `Environment` 有一个 API 用于此，但你通常会设置一个系统属性（`spring.profiles.active`）或一个操作系统环境变量（`SPRING_PROFILES_ACTIVE`）。此外，你还可以使用 `-D` 参数启动你的应用（记住将其放在主类或 jar 包之前），如下所示：

```bash
$ java -jar -Dspring.profiles.active=production demo-0.0.1-SNAPSHOT.jar
```

在 Spring Boot 中，你也可以在 `application.properties` 中设置活动的 profile，示例如下：

- 属性
- YAML

```properties
spring.profiles.active=production
```

```yaml
spring:
  profiles:
    active: "production"
```

以这种方式设置的值会被系统属性或环境变量设置替换，但不会被 `SpringApplicationBuilder.profiles()` 方法替换。因此后者 Java API 可用于增加 profile，而无需更改默认设置。

更多信息请参阅"Spring Boot 特性"部分中的 Profile。

## 设置默认 Profile 名称

默认 profile 是在没有其他 profile 活动时启用的 profile。默认情况下，默认 profile 的名称是 `default`，但可以使用系统属性（`spring.profiles.default`）或操作系统环境变量（`SPRING_PROFILES_DEFAULT`）进行更改。

在 Spring Boot 中，你也可以在 `application.properties` 中设置默认 profile 名称，示例如下：

- 属性
- YAML

```properties
spring.profiles.default=dev
```

```yaml
spring:
  profiles:
    default: "dev"
```

更多信息请参阅"Spring Boot 特性"部分中的 Profile。

## 根据环境更改配置

Spring Boot 支持多文档 YAML 和 Properties 文件（详情请参阅使用多文档文件），它们可以根据活动的 profile 有条件地激活。

如果文档包含 `spring.config.activate.on-profile` 键，则 profile 值（逗号分隔的 profile 列表或 profile 表达式）将被传递给 Spring 的 `Environment.acceptsProfiles()` 方法。如果 profile 表达式匹配，则该文档将被包含在最终合并中（否则不会），示例如下：

- 属性
- YAML

```properties
server.port=9000
#---
spring.config.activate.on-profile=development
server.port=9001
#---
spring.config.activate.on-profile=production
server.port=0
```

```yaml
server:
  port: 9000
---
spring:
  config:
    activate:
      on-profile: "development"
server:
  port: 9001
---
spring:
  config:
    activate:
      on-profile: "production"
server:
  port: 0
```

在前面的示例中，默认端口是 9000。但是，如果名为"development"的 Spring profile 活动，则端口为 9001。如果"production"活动，则端口为 0。

|  |  |
| --- | --- |
|  | 文档按照遇到的顺序合并。后面的值会覆盖前面的值。 |

## 发现外部属性的内置选项

Spring Boot 在运行时将来自 `application.properties`（或 YAML 文件及其他位置）的外部属性绑定到应用中。没有（技术上也不可能有）在单一位置列出所有支持属性的完整列表，因为贡献可能来自 classpath 上的额外 jar 文件。

具有 Actuator 特性的正在运行的应用有一个 `configprops` 端点，该端点显示所有通过 `@ConfigurationProperties` 可用的已绑定和可绑定属性。

附录包含一个 `application.properties` 示例，其中列出了 Spring Boot 支持的大部分常用属性。确切的列表可以通过搜索源代码中的 `@ConfigurationProperties` 和 `@Value` 注解以及偶尔使用 `Binder` 来获取。有关加载属性的确切顺序，请参阅外部化配置。
