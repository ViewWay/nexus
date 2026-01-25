# JVM 检查点/恢复

Spring Framework 与 Project CRaC 实现的检查点/恢复功能集成，以便通过 JVM 减少基于 Spring 的 Java 应用的启动和预热时间。

使用此功能需要满足以下条件：

- 支持检查点/恢复的 JVM（目前仅限 Linux）
- 类路径中存在 `org.crac:crac` 库（支持 1.4.0 及以上版本）
- 指定所需的 Java 命令行参数，例如 `-XX:CRaCCheckpointTo=PATH` 或 `-XX:CRaCRestoreFrom=PATH`

> 当请求创建检查点时，在 `-XX:CRaCCheckpointTo=PATH` 指定的路径中生成的文件包含运行中 JVM 内存的表示，其中可能包含秘密或其他敏感数据。使用此功能时，应假设 JVM "看到" 的任何值（例如来自环境的配置属性）都将存储在这些 CRaC 文件中。因此，应仔细评估这些文件生成、存储和访问位置与方式的安全影响。

从概念上讲，检查点和恢复与单个 Bean 的 Spring `Lifecycle` 契约一致。

## 运行中应用的按需检查点/恢复

可以按需创建检查点，例如使用 `jcmd application.jar JDK.checkpoint` 等命令。创建检查点之前，Spring 会停止所有正在运行的 Bean，通过实现 `Lifecycle.stop` 使它们有机会在需要时关闭资源。恢复后，相同的 Bean 会重新启动，通过 `Lifecycle.start` 允许 Bean 在相关时重新打开资源。

对于不依赖 Spring 的库，可以通过实现 `org.crac.Resource` 并注册相关实例来提供自定义的检查点/恢复集成。

> 利用运行中应用的检查点/恢复通常需要额外的生命周期管理，以便优雅地停止和开始使用文件或套接字等资源，并停止活动线程。

> 请注意，当以固定速率定义调度任务时（例如使用 `@Scheduled(fixedRate = 5000)` 注解），通过按需检查点/恢复恢复 JVM 后，检查点和恢复之间所有错过的执行都将执行。如果这不是您想要的行为，建议以固定延迟（例如使用 `@Scheduled(fixedDelay = 5000)`）或 cron 表达式调度任务。

> 如果在已预热的 JVM 上创建检查点，恢复的 JVM 也将同样预热，从而可能立即达到峰值性能。此方法通常需要访问远程服务，因此需要一定程度的平台集成。

## 启动时的自动检查点/恢复

设置 `-Dspring.context.checkpoint=onRefresh` JVM 系统属性后，将在启动时 `LifecycleProcessor.onRefresh` 阶段自动创建检查点。此阶段完成后，所有非延迟初始化的单例都已实例化，并且已调用 `InitializingBean#afterPropertiesSet` 回调；但生命周期尚未开始，`ContextRefreshedEvent` 尚未发布。

出于测试目的，也可以利用 `-Dspring.context.exit=onRefresh` JVM 系统属性，它会触发类似的行为，但不是创建检查点，而是在相同的生命周期阶段退出 Spring 应用，无需 Project CraC 依赖/JVM 或 Linux。

> 如上所述，特别是在 CRaC 文件作为可部署 artifact（例如容器镜像）一部分交付的使用场景中，应假设 JVM "看到" 的任何敏感数据最终都会存入 CRaC 文件中，并仔细评估相关的安全影响。

> 自动检查点/恢复是一种将应用启动"快进"到应用上下文即将启动阶段的方式，但它不能实现完全预热的 JVM。

## 配置示例

### Maven 依赖

```xml
<dependency>
    <groupId>org.crac</groupId>
    <artifactId>crac</artifactId>
    <version>1.4.0</version>
</dependency>
```

### 创建检查点

```bash
java -XX:CRaCCheckpointTo=./checkpoint -jar your-application.jar
# 应用程序启动后，在另一个终端执行：
jcmd your-application.jar JDK.checkpoint
```

### 从检查点恢复

```bash
java -XX:CRaCRestoreFrom=./checkpoint
```

### 自动检查点配置

```bash
java -Dspring.context.checkpoint=onRefresh \
     -XX:CRaCCheckpointTo=./checkpoint \
     -jar your-application.jar
```

## 自定义资源处理

如果您的应用程序使用需要特殊处理的资源（如文件句柄、套接字连接等），可以实现 `org.crac.Resource` 接口：

```java
import org.crac.Resource;
import org.crac.Context;
import org.springframework.stereotype.Component;

@Component
public class CustomCracResource implements Resource {

    @Override
    public void beforeCheckpoint(Context<? extends Resource> context) {
        // 在创建检查点之前关闭资源
    }

    @Override
    public void afterRestore(Context<? extends Resource> context) {
        // 在恢复后重新打开资源
    }
}
```

注册资源：

```java
import org.crac.Core;
import org.springframework.context.annotation.Configuration;

import jakarta.annotation.PostConstruct;

@Configuration
public class CracConfiguration {

    @PostConstruct
    public void registerResources() {
        Core.getGlobalContext().register(new CustomCracResource());
    }
}
```

## 与 Spring 生命周期集成

Spring Framework 会自动将 CRaC 与 Spring 的生命周期集成：

- 在创建检查点之前调用 `Lifecycle.stop()`
- 在恢复之后调用 `Lifecycle.start()`

这意味着您的 Spring Bean 可以通过实现 `Lifecycle` 或 `SmartLifecycle` 接口来参与检查点/恢复过程：

```java
import org.springframework.context.SmartLifecycle;
import org.springframework.stereotype.Component;

@Component
public class CustomLifecycleBean implements SmartLifecycle {

    private boolean isRunning = false;

    @Override
    public void start() {
        // 恢复后或正常启动时调用
        isRunning = true;
    }

    @Override
    public void stop() {
        // 创建检查点之前或正常关闭时调用
        isRunning = false;
    }

    @Override
    public boolean isRunning() {
        return isRunning;
    }

    @Override
    public boolean isAutoStartup() {
        return true;
    }

    @Override
    public void stop(Runnable callback) {
        stop();
        callback.run();
    }

    @Override
    public int getPhase() {
        return 0;
    }
}
```
