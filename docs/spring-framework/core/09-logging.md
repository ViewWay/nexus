# 日志

Spring 附带了其自己的 Commons Logging 桥接，实现在 `spring-jcl` 模块中。该实现会检查 classpath 中是否存在 Log4j 2.x API 和 SLF4J 1.7 API，并使用找到的第一个作为日志实现，如果两者都不可用，则回退到 Java 平台的核心日志设施（也称为 _JUL_ 或 `java.util.logging`）。

将 Log4j 2.x 或 Logback（或其他 SLF4J 提供程序）放入您的 classpath，无需任何额外的桥接，框架将自动适应您的选择。更多信息请参阅 Spring Boot 日志参考文档。

|  |  |
| --- | --- |
|  | Spring 的 Commons Logging 变体仅用于核心框架和扩展中的基础设施日志目的。对于应用程序代码中的日志需求，建议直接使用 Log4j 2.x、SLF4J 或 JUL。 |

可以通过 `org.apache.commons.logging.LogFactory` 获取 `Log` 实现，示例如下：

```java
public class MyBean {
    private final Log log = LogFactory.getLog(getClass());
    // ...
}
```

---

*来源：https://docs.springframework.org.cn/spring-framework/reference/core/spring-jcl.html*
