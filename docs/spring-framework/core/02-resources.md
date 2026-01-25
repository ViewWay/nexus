# 资源

## 简介

不幸的是，Java 标准的 `java.net.URL` 类以及用于各种 URL 前缀的标准处理程序对于访问所有低级资源来说并不完全足够。例如，没有可用于访问需要从类路径或相对于 `ServletContext` 获取的资源的标准化 `URL` 实现。虽然可以为专门的 `URL` 前缀注册新的处理程序（类似于用于 `http:` 等前缀的现有处理程序），但这通常相当复杂，并且 `URL` 接口仍然缺少一些需要的功能，例如用于检查所指向资源是否存在的方法。

## `Resource` 接口

Spring 的 `Resource` 接口位于 `org.springframework.core.io.` 包中，旨在成为一个更强大的接口，用于抽象访问低级资源。以下列表提供了 `Resource` 接口的概述。有关更多详细信息，请参阅 `Resource` 的 Javadoc。

```java
public interface Resource extends InputStreamSource {

    boolean exists();

    boolean isReadable();

    boolean isOpen();

    boolean isFile();

    URL getURL() throws IOException;

    URI getURI() throws IOException;

    File getFile() throws IOException;

    ReadableByteChannel readableChannel() throws IOException;

    long contentLength() throws IOException;

    long lastModified() throws IOException;

    Resource createRelative(String relativePath) throws IOException;

    String getFilename();

    String getDescription();
}
```

如 `Resource` 接口的定义所示，它扩展了 `InputStreamSource` 接口。以下列表显示了 `InputStreamSource` 接口的定义：

```java
public interface InputStreamSource {

    InputStream getInputStream() throws IOException;
}
```

`Resource` 接口中一些最重要的方法包括：

- `getInputStream()`：定位并打开资源，返回一个 `InputStream` 用于从资源读取。每次调用都应该返回一个新的 `InputStream`。调用者有责任关闭流。
- `exists()`：返回一个 `boolean` 值，指示此资源是否实际物理存在。
- `isOpen()`：返回一个 `boolean` 值，指示此资源是否表示一个带有打开流的句柄。如果为 `true`，则 `InputStream` 不能多次读取，只能读取一次，然后必须关闭以避免资源泄漏。除了 `InputStreamResource` 之外，所有常规资源实现都返回 `false`。
- `getDescription()`：返回此资源的描述，用于处理资源时的错误输出。这通常是资源的完全限定文件名或实际 URL。

其他方法允许您获取表示资源的实际 `URL` 或 `File` 对象（如果底层实现兼容并支持该功能）。

Spring 本身广泛使用 `Resource` 抽象，在许多需要资源的方法签名中将其作为参数类型。某些 Spring API 中的其他方法（例如各种 `ApplicationContext` 实现的构造函数）接受一个 `String` 参数，该参数在未加修饰或简单形式下用于创建适合该上下文实现的 `Resource`，或者通过 `String` 路径上的特殊前缀，让调用者指定必须创建和使用特定的 `Resource` 实现。

虽然 `Resource` 接口在 Spring 中被广泛使用，但它本身作为一个通用工具类在您的代码中访问资源也非常方便，即使您的代码不知道或不关心 Spring 的任何其他部分。虽然这会将您的代码与 Spring 耦合，但它实际上只与这小组工具类耦合，这些工具类可以作为 `URL` 的更强大的替代品，可以被认为是您用于此目的的任何其他库的等价物。

> `Resource` 抽象不会替换功能。它在可能的情况下进行包装。例如，`UrlResource` 包装一个 URL，并使用被包装的 `URL` 来完成其工作。

## 内置的 `Resource` 实现

Spring 包含几种内置的 `Resource` 实现：

- `UrlResource`
- `ClassPathResource`
- `FileSystemResource`
- `PathResource`
- `ServletContextResource`
- `InputStreamResource`
- `ByteArrayResource`

有关 Spring 中所有可用的 `Resource` 实现的完整列表，请查阅 `Resource` 的 Javadoc 文档中的"所有已知实现类"部分。

### `UrlResource`

`UrlResource` 包装了 `java.net.URL`，可用于访问通常可以通过 URL 访问的任何对象，例如文件、HTTPS 目标、FTP 目标等。所有 URL 都有标准化的 `String` 表示形式，以便使用适当的标准化前缀来区分不同的 URL 类型。这包括用于访问文件系统路径的 `file:`，通过 HTTPS 协议访问资源的 `https:`，通过 FTP 访问资源的 `ftp:` 等。

`UrlResource` 是通过 Java 代码显式使用 `UrlResource` 构造函数创建的，但在调用接受表示路径的 `String` 参数的 API 方法时，通常是隐式创建的。在后一种情况下，JavaBeans `PropertyEditor` 最终决定创建哪种类型的 `Resource`。如果路径字符串包含一个众所周知（对属性编辑器而言）的前缀（例如 `classpath:`），它会为该前缀创建一个适当的专用 `Resource`。但是，如果它不识别该前缀，它会假定该字符串是一个标准的 URL 字符串，并创建一个 `UrlResource`。

### `ClassPathResource`

此类表示应从类路径获取的资源。它使用线程上下文类加载器、给定的类加载器或给定的类来加载资源。

如果类路径资源位于文件系统中，则此 `Resource` 实现支持解析为 `java.io.File`，但对于驻留在 jar 中且未被（由 servlet 引擎或任何环境）展开到文件系统的类路径资源则不支持。为了解决此问题，各种 `Resource` 实现始终支持解析为 `java.net.URL`。

`ClassPathResource` 是通过 Java 代码显式使用 `ClassPathResource` 构造函数创建的，但在调用接受表示路径的 `String` 参数的 API 方法时，通常是隐式创建的。在后一种情况下，JavaBeans `PropertyEditor` 会识别字符串路径上的特殊前缀 `classpath:`，并在该情况下创建一个 `ClassPathResource`。

### `FileSystemResource`

这是一个用于 `java.io.File` 句柄的 `Resource` 实现。它还支持 `java.nio.file.Path` 句柄，应用 Spring 标准的基于 String 的路径转换，但所有操作都通过 `java.nio.file.Files` API 执行。对于纯粹基于 `java.nio.path.Path` 的支持，请改用 `PathResource`。`FileSystemResource` 支持解析为 `File` 和 `URL`。

### `PathResource`

这是一个用于 `java.nio.file.Path` 句柄的 `Resource` 实现，所有操作和转换都通过 `Path` API 执行。它支持解析为 `File` 和 `URL`，并且还实现了扩展的 `WritableResource` 接口。`PathResource` 实际上是 `FileSystemResource` 的一个纯粹基于 `java.nio.path.Path` 的替代品，其 `createRelative` 行为不同。

### `ServletContextResource`

这是一个用于 `ServletContext` 资源的 `Resource` 实现，它解释相关 Web 应用根目录中的相对路径。

它始终支持流访问和 URL 访问，但只有当 Web 应用归档文件被解压且资源实际位于文件系统上时，才允许 `java.io.File` 访问。它是否被解压并在文件系统上，或者直接从 JAR 或其他地方（例如数据库，这是可以想象的）访问，实际上取决于 Servlet 容器。

### `InputStreamResource`

`InputStreamResource` 是给定 `InputStream` 的 `Resource` 实现。只有在没有特定的 `Resource` 实现适用的情况下才应使用它。特别是，在可能的情况下，优先使用 `ByteArrayResource` 或任何基于文件的 `Resource` 实现。

与其他 `Resource` 实现不同，这是一个已打开资源的描述符。因此，它从 `isOpen()` 返回 `true`。如果您需要在某处保留资源描述符，或者需要多次读取流，请勿使用它。

### `ByteArrayResource`

这是一个给定字节数组的 `Resource` 实现。它为给定的字节数组创建一个 `ByteArrayInputStream`。

它对于从任何给定的字节数组加载内容非常有用，而无需依赖单次使用的 `InputStreamResource`。

## `ResourceLoader` 接口

`ResourceLoader` 接口旨在由可以返回（即加载）`Resource` 实例的对象实现。以下列表显示了 `ResourceLoader` 接口的定义：

```java
public interface ResourceLoader {

    Resource getResource(String location);

    ClassLoader getClassLoader();
}
```

所有应用上下文都实现了 `ResourceLoader` 接口。因此，所有应用上下文都可以用于获取 `Resource` 实例。

当您在特定的应用上下文上调用 `getResource()`，并且指定的 location path 没有特定的前缀时，您将获得一个适合该特定应用上下文的 `Resource` 类型。例如，假设以下代码片段针对 `ClassPathXmlApplicationContext` 实例运行：

```java
Resource template = ctx.getResource("some/resource/path/myTemplate.txt");
```

对于 `ClassPathXmlApplicationContext`，该代码返回一个 `ClassPathResource`。如果对 `FileSystemXmlApplicationContext` 实例运行相同的方法，它将返回一个 `FileSystemResource`。对于 `WebApplicationContext`，它将返回一个 `ServletContextResource`。它会类似地为每个上下文返回适当的对象。

因此，您可以以适合特定应用上下文的方式加载资源。

另一方面，您也可以通过指定特殊的 `classpath:` 前缀来强制使用 `ClassPathResource`，无论应用上下文类型如何，如下例所示：

```java
Resource template = ctx.getResource("classpath:some/resource/path/myTemplate.txt");
```

类似地，您可以通过指定任何标准的 `java.net.URL` 前缀来强制使用 `UrlResource`。以下示例使用 `file` 和 `https` 前缀：

```java
Resource template = ctx.getResource("file:///some/resource/path/myTemplate.txt");
```

```java
Resource template = ctx.getResource("https://myhost.com/resource/path/myTemplate.txt");
```

下表总结了将 `String` 对象转换为 `Resource` 对象的策略：

**表 1. 资源字符串**

| 前缀 | 示例 | 说明 |
| --- | --- | --- |
| classpath | `classpath:com/myapp/config.xml` | 从类路径加载。 |
| file | `file:///data/config.xml` | 从文件系统加载为 `URL`。另请参阅 `FileSystemResource` 注意事项。 |
| https | `https://myserver/logo.png` | 加载为 `URL`。 |
| (无) | `/data/config.xml` | 取决于底层的 `ApplicationContext`。 |

## `ResourcePatternResolver` 接口

`ResourcePatternResolver` 接口是 `ResourceLoader` 接口的扩展，它定义了一种策略，用于将位置模式（例如，Ant 风格路径模式）解析为 `Resource` 对象。

```java
public interface ResourcePatternResolver extends ResourceLoader {

    String CLASSPATH_ALL_URL_PREFIX = "classpath*:";

    Resource[] getResources(String locationPattern) throws IOException;
}
```

如上所示，此接口还为类路径中的所有匹配资源定义了一个特殊的 `classpath*:` 资源前缀。请注意，在这种情况下，资源位置应该是一个没有占位符的路径——例如，`classpath*:/config/beans.xml`。类路径中的 JAR 文件或不同目录可以包含具有相同路径和相同名称的多个文件。有关使用 `classpath*:` 资源前缀的通配符支持的更多详细信息，请参见 应用上下文构造函数资源路径中的通配符 及其子节。

可以检查传入的 `ResourceLoader`（例如，通过 `ResourceLoaderAware` 语义提供的加载器）是否也实现了此扩展接口。

`PathMatchingResourcePatternResolver` 是一个独立的实现，可在 `ApplicationContext` 外部使用，并且还被 `ResourceArrayPropertyEditor` 用于填充 `Resource[]` Bean 属性。`PathMatchingResourcePatternResolver` 能够将指定的资源位置路径解析为一个或多个匹配的 `Resource` 对象。源路径可以是与目标 `Resource` 具有一对一映射的简单路径，也可以包含特殊的 `classpath*:` 前缀和/或内部的 Ant 风格正则表达式（使用 Spring 的 `org.springframework.util.AntPathMatcher` 工具进行匹配）。后两者实际上都是通配符。

> 任何标准 `ApplicationContext` 中的默认 `ResourceLoader` 实际上是 `PathMatchingResourcePatternResolver` 的一个实例，它实现了 `ResourcePatternResolver` 接口。`ApplicationContext` 实例本身也是如此，它也实现了 `ResourcePatternResolver` 接口并委托给默认的 `PathMatchingResourcePatternResolver`。

## `ResourceLoaderAware` 接口

`ResourceLoaderAware` 接口是一个特殊的 callback 接口，它标识那些期望获得 `ResourceLoader` 引用的组件。以下清单显示了 `ResourceLoaderAware` 接口的定义：

```java
public interface ResourceLoaderAware {

    void setResourceLoader(ResourceLoader resourceLoader);
}
```

当一个类实现 `ResourceLoaderAware` 并被部署到应用上下文（作为 Spring 管理的 Bean）中时，应用上下文会将其识别为 `ResourceLoaderAware`。然后应用上下文会调用 `setResourceLoader(ResourceLoader)` 方法，将自身作为参数提供（记住，Spring 中的所有应用上下文都实现了 `ResourceLoader` 接口）。

由于 `ApplicationContext` 也是一个 `ResourceLoader`，因此 Bean 也可以实现 `ApplicationContextAware` 接口并直接使用提供的应用上下文来加载资源。然而，通常情况下，如果只需要资源加载功能，最好使用专门的 `ResourceLoader` 接口。这样代码只会耦合到资源加载接口（可以被视为一个工具接口），而不会耦合到整个 Spring `ApplicationContext` 接口。

在应用组件中，你也可以依靠 `ResourceLoader` 的自动注入作为实现 `ResourceLoaderAware` 接口的替代方案。_传统的_ `constructor` 和 `byType` 自动注入模式（如 自动注入协作者 中所述）能够分别为构造函数参数或 setter 方法参数提供 `ResourceLoader`。为了获得更大的灵活性（包括自动注入字段和多参数方法的能力），请考虑使用基于注解的自动注入特性。在这种情况下，只要相关的字段、构造函数或方法带有 `@Autowired` 注解，`ResourceLoader` 就会被自动注入到期望 `ResourceLoader` 类型的字段、构造函数参数或方法参数中。更多信息请参见 使用 `@Autowired`。

> 对于包含通配符或使用特殊 `classpath*:` 资源前缀的资源路径，如果需要加载一个或多个 `Resource` 对象，可以考虑将 `ResourcePatternResolver` 的实例自动注入到你的应用组件中，而不是 `ResourceLoader`。

## 作为依赖的资源

如果 Bean 本身要通过某种动态过程来确定和提供资源路径，那么 Bean 使用 `ResourceLoader` 或 `ResourcePatternResolver` 接口来加载资源可能是合理的。例如，考虑加载某种模板，其中所需的特定资源取决于用户的角色。如果资源是静态的，那么完全取消使用 `ResourceLoader` 接口（或 `ResourcePatternResolver` 接口）是合理的，让 Bean 暴露其所需的 `Resource` 属性，并期望它们被注入进来。

使得这些属性的注入变得简单的是，所有应用上下文都注册并使用一个特殊的 JavaBeans `PropertyEditor`（属性编辑器），它可以将 `String` 路径转换为 `Resource` 对象。例如，以下 `MyBean` 类有一个类型为 `Resource` 的 `template` 属性。

```java
public class MyBean {

    private Resource template;

    public setTemplate(Resource template) {
        this.template = template;
    }

    // ...
}
```

在 XML 配置文件中，`template` 属性可以使用该资源的简单字符串进行配置，如下例所示：

```xml
<bean id="myBean" class="example.MyBean">
    <property name="template" value="some/resource/path/myTemplate.txt"/>
</bean>
```

请注意，资源路径没有前缀。因此，由于应用上下文本身将被用作 `ResourceLoader`，资源将通过 `ClassPathResource`、`FileSystemResource` 或 `ServletContextResource` 进行加载，具体取决于应用上下文的确切类型。

如果你需要强制使用特定的 `Resource` 类型，可以使用前缀。以下两个示例展示了如何强制使用 `ClassPathResource` 和 `UrlResource`（后者用于访问文件系统中的文件）：

```xml
<property name="template" value="classpath:some/resource/path/myTemplate.txt">
```

```xml
<property name="template" value="file:///some/resource/path/myTemplate.txt"/>
```

如果将 `MyBean` 类重构为与注解驱动的配置一起使用，则 `myTemplate.txt` 的路径可以存储在名为 `template.path` 的键下——例如，在提供给 Spring `Environment` 的属性文件中（参见 环境抽象）。然后可以通过 `@Value` 注解使用属性占位符来引用模板路径（参见 使用 `@Value`）。Spring 将检索模板路径的值作为字符串，并且一个特殊的 `PropertyEditor`（属性编辑器）将把该字符串转换为 `Resource` 对象，注入到 `MyBean` 的构造函数中。以下示例展示了如何实现这一点。

```java
@Component
public class MyBean {

    private final Resource template;

    public MyBean(@Value("${template.path}") Resource template) {
        this.template = template;
    }

    // ...
}
```

如果我们想支持在类路径中多个位置（例如，类路径中的多个 jar 包）的同一路径下发现多个模板，我们可以使用特殊的 `classpath*:` 前缀和通配符来将 `templates.path` 键定义为 `classpath*:/config/templates/*.txt`。如果我们按照如下方式重新定义 `MyBean` 类，Spring 会将模板路径模式转换为 `Resource` 对象的数组，该数组可以注入到 `MyBean` 的构造函数中。

```java
@Component
public class MyBean {

    private final Resource[] templates;

    public MyBean(@Value("${templates.path}") Resource[] templates) {
        this.templates = templates;
    }

    // ...
}
```

## 应用上下文和资源路径

本节介绍如何使用资源创建应用上下文，包括适用于 XML 的快捷方式、如何使用通配符以及其他详细信息。

### 构造应用上下文

应用上下文构造函数（针对特定的应用上下文类型）通常接受一个字符串或字符串数组作为资源的路径位置，例如构成上下文定义的 XML 文件。

当此类位置路径没有前缀时，根据该路径构建并用于加载 Bean 定义的特定 `Resource` 类型取决于具体的应用上下文，并且与该上下文相适应。例如，考虑以下示例，它创建了一个 `ClassPathXmlApplicationContext`：

```java
ApplicationContext ctx = new ClassPathXmlApplicationContext("conf/appContext.xml");
```

Bean 定义从类路径加载，因为使用了 `ClassPathResource`。然而，考虑以下示例，它创建了一个 `FileSystemXmlApplicationContext`：

```java
ApplicationContext ctx =
    new FileSystemXmlApplicationContext("conf/appContext.xml");
```

现在 Bean 定义从文件系统位置加载（在这种情况下，相对于当前工作目录）。

请注意，在位置路径上使用特殊的 `classpath` 前缀或标准 URL 前缀会覆盖用于加载 Bean 定义的默认 `Resource` 类型。考虑以下示例：

```java
ApplicationContext ctx =
    new FileSystemXmlApplicationContext("classpath:conf/appContext.xml");
```

使用 `FileSystemXmlApplicationContext` 从类路径加载 Bean 定义。然而，它仍然是一个 `FileSystemXmlApplicationContext`。如果随后将其用作 `ResourceLoader`，任何没有前缀的路径仍将被视为文件系统路径。

#### 构造 `ClassPathXmlApplicationContext` 实例 — 快捷方式

`ClassPathXmlApplicationContext` 公开了多个构造函数以方便实例化。基本思想是，你只需提供一个只包含 XML 文件名本身（不带前导路径信息）的字符串数组，并提供一个 `Class`。然后 `ClassPathXmlApplicationContext` 会从提供的类派生路径信息。

考虑以下目录布局：

```
com/
  example/
    services.xml
    repositories.xml
    MessengerService.class
```

以下示例展示了如何实例化一个由名为 `services.xml` 和 `repositories.xml` 文件（它们位于类路径中）中定义的 Bean 组成的 `ClassPathXmlApplicationContext` 实例：

```java
ApplicationContext ctx = new ClassPathXmlApplicationContext(
    new String[] {"services.xml", "repositories.xml"}, MessengerService.class);
```

### 应用上下文构造函数资源路径中的通配符

应用上下文构造函数值中的资源路径可以是简单的路径（如前所示），每个路径与目标 `Resource` 具有一对一的映射，或者，可以包含特殊的 `classpath*:` 前缀或内部的 Ant 风格模式（通过使用 Spring 的 `PathMatcher` 工具进行匹配）。后两者实际上都是通配符。

这种机制的一个用途是当你需要进行组件式应用程序组装时。所有组件都可以将上下文定义片段_发布_到已知的位置路径，并且当使用带有 `classpath*:` 前缀的相同路径创建最终的应用上下文时，所有组件片段都会自动被拾取。

请注意，这种通配符使用仅限于应用上下文构造函数中的资源路径（或者当你直接使用 `PathMatcher` 工具类层次结构时），并在构造时解析。它与 `Resource` 类型本身无关。你不能使用 `classpath*:` 前缀来构造一个实际的 `Resource`，因为一个资源一次只指向一个资源。

#### Ant 风格模式

路径位置可以包含 Ant 风格模式，如下例所示：

```
/WEB-INF/*-context.xml
com/mycompany/**/applicationContext.xml
file:C:/some/path/*-context.xml
classpath:com/mycompany/**/applicationContext.xml
```

当路径位置包含 Ant 风格模式时，解析器会遵循一个更复杂的程序来尝试解析通配符。它会为直到最后一个非通配符段的路径生成一个 `Resource` 并从中获取一个 URL。如果此 URL 不是 `jar:` URL 或容器特定的变体（例如 WebLogic 中的 `zip:`，WebSphere 中的 `wsjar` 等），则会从中获取 `java.io.File` 并通过遍历文件系统来解析通配符。对于 jar URL，解析器要么从中获取 `java.net.JarURLConnection`，要么手动解析 jar URL，然后遍历 jar 文件的内容来解析通配符。

##### 对可移植性的影响

如果指定的路径已经是 `file` URL（无论是隐式因为基础 `ResourceLoader` 是文件系统类型的，还是显式指定），通配符的使用可以保证以完全可移植的方式工作。

如果指定的路径是 `classpath` 位置，解析器必须通过调用 `Classloader.getResource()` 来获取最后一个非通配符路径段的 URL。由于这只是路径的一个节点（而不是末尾的文件），在这种情况下返回的 URL 类型实际上是未定义的（在 `ClassLoader` 的 javadoc 中）。实际上，它总是代表目录的 `java.io.File`（当类路径资源解析到文件系统位置时）或某种 jar URL（当类路径资源解析到 jar 位置时）。尽管如此，此操作仍存在可移植性问题。

如果最后一个非通配符段获得了 jar URL，解析器必须能够从中获取 `java.net.JarURLConnection` 或手动解析 jar URL，以便遍历 jar 的内容并解析通配符。这在大多数环境中有效，但在其他环境中会失败，我们强烈建议在使用来自 jar 的资源的通配符解析之前，在你的特定环境中进行彻底测试。

#### `classpath*:` 前缀

构造基于 XML 的应用上下文时，位置字符串可以使用特殊的 `classpath*:` 前缀，如下例所示：

```java
ApplicationContext ctx =
    new ClassPathXmlApplicationContext("classpath*:conf/appContext.xml");
```

这个特殊前缀指定必须获取所有匹配给定名称的类路径资源（在内部，这主要通过调用 `ClassLoader.getResources(…​)` 来实现），然后合并它们以形成最终的应用上下文定义。

> 通配符类路径依赖于底层 `ClassLoader` 的 `getResources()` 方法。由于如今大多数应用服务器提供了自己的 `ClassLoader` 实现，行为可能会有所不同，尤其是在处理 jar 文件时。一个简单的测试来检查 `classpath*` 是否工作是使用 `ClassLoader` 从类路径中的 jar 包内加载文件：`getClass().getClassLoader().getResources("<someFileInsideTheJar>")`。用具有相同名称但位于两个不同位置的文件进行此测试——例如，在类路径中的不同 jar 包内具有相同名称和相同路径的文件。如果返回了不恰当的结果，请查阅应用服务器文档中可能影响 `ClassLoader` 行为的设置。

你也可以将 `classpath*:` 前缀与位置路径的其余部分中的 `PathMatcher` 模式结合使用（例如，`classpath*:META-INF/*-beans.xml`）。在这种情况下，解析策略非常简单：对最后一个非通配符路径段使用 `ClassLoader.getResources()` 调用，以获取类加载器层次结构中的所有匹配资源，然后，对每个资源，使用前面描述的相同 `PathMatcher` 解析策略来处理通配符子路径。

#### 与通配符相关的其他注意事项

请注意，`classpath*:` 与 Ant 风格模式结合使用时，除非实际目标文件位于文件系统中，否则只有在模式开始之前至少有一个根目录时才能可靠工作。这意味着 `classpath*:*.xml` 这样的模式可能无法从 jar 文件的根目录中检索文件，而只能从展开目录的根目录中检索文件。

Spring 检索类路径条目的能力源于 JDK 的 `ClassLoader.getResources()` 方法，该方法只为（表示潜在搜索根的）空字符串返回文件系统位置。Spring 还会评估 `URLClassLoader` 运行时配置和 jar 文件中的 `java.class.path` manifest，但这并不能保证带来可移植的行为。

如果如果要搜索的根包在多个类路径位置可用，带有 `classpath:` 资源的 Ant 风格模式不保证能找到匹配的资源。考虑以下资源位置示例：

```
com/mycompany/package1/service-context.xml
```

现在考虑某人可能用来尝试查找该文件的 Ant 风格路径：

```
classpath:com/mycompany/**/service-context.xml
```

此类资源可能仅存在于类路径中的一个位置，但是当使用上述示例中的路径尝试解析它时，解析器会基于 `getResource("com/mycompany");` 返回的（第一个）URL 进行工作。如果此基本包节点存在于多个 `ClassLoader` 位置，则所需资源可能不存在于找到的第一个位置中。因此，在这种情况下，你应该优先使用带有相同 Ant 风格模式的 `classpath*:`，它会搜索所有包含 `com.mycompany` 基本包的类路径位置：`classpath*:com/mycompany/**/service-context.xml`。

### `FileSystemResource` 注意事项

未附加到 `FileSystemApplicationContext` 的 `FileSystemResource`（即 `FileSystemApplicationContext` 不是实际的 `ResourceLoader` 时），会如你预期地处理绝对路径和相对路径。相对路径相对于当前工作目录，而绝对路径相对于文件系统的根目录。

然而，出于向后兼容性（历史）原因，当 `FileSystemApplicationContext` 是 `ResourceLoader` 时，情况发生了变化。`FileSystemApplicationContext` 会强制所有附加的 `FileSystemResource` 实例将所有位置路径都视为相对路径，无论它们是否以斜杠开头。实际上，这意味着以下示例是等效的：

```java
ApplicationContext ctx =
    new FileSystemXmlApplicationContext("conf/context.xml");
```

```java
ApplicationContext ctx =
    new FileSystemXmlApplicationContext("/conf/context.xml");
```

以下示例也是等效的（尽管从语义上讲它们应该不同，因为一个情况是相对路径而另一个是绝对路径）：

```java
FileSystemXmlApplicationContext ctx = ...;
ctx.getResource("some/resource/path/myTemplate.txt");
```

```java
FileSystemXmlApplicationContext ctx = ...;
ctx.getResource("/some/resource/path/myTemplate.txt");
```

实际上，如果你需要真正的绝对文件系统路径，你应该避免使用 `FileSystemResource` 或 `FileSystemXmlApplicationContext` 的绝对路径，并通过使用 `file:` URL 前缀来强制使用 `UrlResource`。以下示例展示了如何这样做：

```java
// actual context type doesn't matter, the Resource will always be UrlResource
ctx.getResource("file:///some/resource/path/myTemplate.txt");
```

```java
// force this FileSystemXmlApplicationContext to load its definition via a UrlResource
ApplicationContext ctx =
    new FileSystemXmlApplicationContext("file:///conf/context.xml");
```

---

*来源：https://docs.springframework.org.cn/spring-framework/reference/core/resources.html*
