# CDS

类数据共享 (CDS) 是一个 JVM 特性，可以帮助减少 Java 应用程序的启动时间和内存占用。

要使用此特性，需要为应用程序的特定类路径创建一个 CDS 归档文件。Spring Framework 提供了一个钩子点，以便于创建归档文件。一旦归档文件可用，用户应通过 JVM 标志选择使用它。

## 创建 CDS 归档

可以在应用程序退出时创建应用程序的 CDS 归档。Spring Framework 提供了一种操作模式，其中 `ApplicationContext` 刷新后进程可以自动退出。在此模式下，所有非延迟初始化的单例都已实例化，并且 `InitializingBean#afterPropertiesSet` 回调已被调用；但生命周期尚未开始，并且 `ContextRefreshedEvent` 尚未发布。

### 创建归档文件

必须指定两个额外的 JVM 标志：

- `-XX:ArchiveClassesAtExit=application.jsa`: 在退出时创建 CDS 归档
- `-Dspring.context.exit=onRefresh`: 启动 Spring 应用程序，然后按照上述描述立即退出

### 创建基础 CDS 归档

要创建 CDS 归档，您的 JDK/JRE 必须具有基础镜像。基础 CDS 归档通常是开箱即用的，但如果需要，也可以通过发出以下命令来创建：

```bash
$ java -Xshare:dump
```

如果缺少基础镜像，您可能会收到如下警告：

```
-XX:ArchiveClassesAtExit is unsupported when base CDS archive is not loaded.
Run with -Xlog:cds for more info.
```

## 使用归档

归档文件可用后，将 `-XX:SharedArchiveFile=application.jsa` 添加到您的启动脚本中以使用它：

```bash
java -XX:SharedArchiveFile=application.jsa -jar your-application.jar
```

### 检查 CDS 有效性

要检查 CDS 缓存是否有效，可以使用 `-Xshare:on`（仅用于测试，不要在生产环境中使用），如果无法启用 CDS，该标志将打印错误消息并退出。

### 启用类加载日志

要了解缓存的有效性，可以通过添加一个额外的属性来启用类加载日志：`-Xlog:class+load:file=cds.log`。这会创建一个 `cds.log` 文件，其中包含每次加载类及其来源的尝试记录：

```
[0.064s][info][class,load] org.springframework.core.env.EnvironmentCapable source: shared objects file (top)
[0.064s][info][class,load] org.springframework.beans.factory.BeanFactory source: shared objects file (top)
[0.064s][info][class,load] org.springframework.beans.factory.ListableBeanFactory source: shared objects file (top)
```

## CDS 使用条件

如果无法启用 CDS 或有大量类未从缓存加载，请确保在创建和使用归档时满足以下条件：

1. **相同的 JVM**: 必须使用完全相同的 JVM 版本和配置
2. **类路径格式**: 类路径必须指定为 JAR 文件列表，避免使用目录和 `*` 通配符
3. **时间戳**: 必须保留 JAR 文件的时间戳
4. **类路径一致性**: 使用归档文件时，类路径必须与创建归档时使用的类路径相同，且顺序一致
5. **额外 JAR**: 可以在末尾指定额外的 JAR 或目录（但不会被缓存）

## 完整示例

### 步骤 1: 运行应用程序并创建 CDS 归档

```bash
java -XX:ArchiveClassesAtExit=application.jsa \
     -Dspring.context.exit=onRefresh \
     -jar your-application.jar
```

### 步骤 2: 使用 CDS 归档启动应用程序

```bash
java -XX:SharedArchiveFile=application.jsa \
     -jar your-application.jar
```

## Spring Boot 中的 CDS 支持

Spring Boot 提供了对 CDS 的额外支持。您可以使用 Spring Boot 的 Maven/Gradle 插件来简化 CDS 归档的创建：

### Maven 配置

```xml
<plugin>
    <groupId>org.springframework.boot</groupId>
    <artifactId>spring-boot-maven-plugin</artifactId>
    <configuration>
        <image>
            <builder>paketobuildpacks/builder:tiny</builder>
            <env>
                <BP_JVM_CDS_ENABLED>true</BP_JVM_CDS_ENABLED>
            </env>
        </image>
    </configuration>
</plugin>
```

## AOT 编译与 CDS

Spring Framework 的 AOT (Ahead-of-Time) 编译功能与 CDS 配合使用效果更佳。AOT 编译可以：

1. 提前分析应用程序上下文
2. 生成优化的 Bean 定义
3. 减少运行时的反射和代理创建

结合 CDS，可以进一步减少启动时间：

```bash
# 1. 使用 AOT 编译生成优化代码
java -Dspring.jmx.enabled=false \
     -Dspring.backgroundpreinitializer.ignore=true \
     -Dspring.instrument.compile-only=true \
     -jar your-application.jar

# 2. 使用 AOT 优化的应用创建 CDS 归档
java -XX:ArchiveClassesAtExit=application.jsa \
     -Dspring.context.exit=onRefresh \
     -jar your-application.jar

# 3. 使用 CDS 归档启动
java -XX:SharedArchiveFile=application.jsa \
     -jar your-application.jar
```

## CDS 的限制

1. **平台限制**: CDS 在某些平台上可能不可用或功能有限
2. **类路径限制**: 所有类必须在类路径上，模块化 Jigsaw 应用程序需要额外配置
3. **动态类**: 动态生成的类不能被缓存
4. **JNI**: 使用 JNI 的库可能无法正确处理 CDS

## CDS 与 CRaC 的比较

| 特性 | CDS | CRaC |
| --- | --- | --- |
| 目标 | 共享类元数据 | 完整 JVM 状态 |
| 启动时间改善 | 中等 | 显著 |
| 内存占用改善 | 是 | 是 |
| 平台支持 | 广泛 | 主要是 Linux |
| 状态恢复 | 无 | 完整状态 |

CDS 和 CRaC 可以结合使用以获得最佳性能。
