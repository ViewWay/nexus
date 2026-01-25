# 任务执行与调度

Spring Framework 分别通过 `TaskExecutor` 和 `TaskScheduler` 接口提供了任务异步执行和调度的抽象。Spring 还提供了这些接口的实现，支持线程池或在应用服务器环境中委托给 CommonJ。最终，在通用接口后使用这些实现抽象了 Java SE 和 Jakarta EE 环境之间的差异。

Spring 还提供了集成类，支持使用 Quartz Scheduler 进行调度。

## Spring `TaskExecutor` 抽象

Executors 是 JDK 对线程池概念的命名。"executor" 的命名是因为无法保证底层实现确实是线程池。Executor 可以是单线程的，甚至是同步的。Spring 的抽象隐藏了 Java SE 和 Jakarta EE 环境之间的实现细节。

Spring 的 `TaskExecutor` 接口与 `java.util.concurrent.Executor` 接口相同。事实上，最初它存在的主要原因是为了在使用线程池时抽象掉对 Java 5 的依赖。该接口有一个方法 (`execute(Runnable task)`)，根据线程池的语义和配置接受一个任务来执行。

`TaskExecutor` 最初创建的目的是为了给其他 Spring 组件提供线程池抽象。诸如 `ApplicationEventMulticaster`、JMS 的 `AbstractMessageListenerContainer` 和 Quartz 集成等组件都使用 `TaskExecutor` 抽象来管理线程池。但是，如果您的 Bean 需要线程池行为，您也可以使用此抽象来满足您的需求。

### `TaskExecutor` 类型

Spring 包含了一些预构建的 `TaskExecutor` 实现。在大多数情况下，您应该不需要自己实现。Spring 提供的变体如下：

- `SyncTaskExecutor`: 此实现不会异步运行调用。相反，每次调用都在调用线程中发生。它主要用于不需要多线程的情况，例如简单的测试用例。
- `SimpleAsyncTaskExecutor`: 此实现不重用任何线程。相反，它为每次调用启动一个新线程。但是，它确实支持一个并发限制，超出限制的调用会被阻塞，直到有空闲槽位。如果您需要真正的线程池，请参阅列表后面的 `ThreadPoolTaskExecutor`。当启用 "virtualThreads" 选项时，它将使用 JDK 21 的虚拟线程。
- `ConcurrentTaskExecutor`: 此实现是 `java.util.concurrent.Executor` 实例的适配器。
- `ThreadPoolTaskExecutor`: 此实现是最常用的。它暴露了配置 `java.util.concurrent.ThreadPoolExecutor` 的 bean 属性，并将其封装在 `TaskExecutor` 中。
- `DefaultManagedTaskExecutor`: 此实现在 JSR-236 兼容的运行时环境（例如 Jakarta EE 应用服务器）中使用通过 JNDI 获取的 `ManagedExecutorService`。

### 使用 `TaskExecutor`

Spring 的 `TaskExecutor` 实现通常与依赖注入一起使用：

```java
public class TaskExecutorExample {

    private class MessagePrinterTask implements Runnable {

        private String message;

        public MessagePrinterTask(String message) {
            this.message = message;
        }

        public void run() {
            System.out.println(message);
        }
    }

    private TaskExecutor taskExecutor;

    public TaskExecutorExample(TaskExecutor taskExecutor) {
        this.taskExecutor = taskExecutor;
    }

    public void printMessages() {
        for(int i = 0; i < 25; i++) {
            taskExecutor.execute(new MessagePrinterTask("Message" + i));
        }
    }
}
```

要配置 `TaskExecutor` 使用的规则：

```java
@Bean
ThreadPoolTaskExecutor taskExecutor() {
    ThreadPoolTaskExecutor taskExecutor = new ThreadPoolTaskExecutor();
    taskExecutor.setCorePoolSize(5);
    taskExecutor.setMaxPoolSize(10);
    taskExecutor.setQueueCapacity(25);
    return taskExecutor;
}

@Bean
TaskExecutorExample taskExecutorExample(ThreadPoolTaskExecutor taskExecutor) {
    return new TaskExecutorExample(taskExecutor);
}
```

大多数 `TaskExecutor` 实现提供了一种使用 `TaskDecorator` 自动包装提交的任务的方法：

```java
import org.springframework.core.task.TaskDecorator;

public class LoggingTaskDecorator implements TaskDecorator {

    @Override
    public Runnable decorate(Runnable runnable) {
        return () -> {
            System.out.println("Before execution of " + runnable);
            runnable.run();
            System.out.println("After execution of " + runnable);
        };
    }
}
```

## Spring `TaskScheduler` 抽象

除了 `TaskExecutor` 抽象外，Spring 还有一个 `TaskScheduler` SPI，提供了多种方法用于调度任务在将来某个时间点运行。

```java
public interface TaskScheduler {

    Clock getClock();

    ScheduledFuture<?> schedule(Runnable task, Trigger trigger);

    ScheduledFuture<?> schedule(Runnable task, Instant startTime);

    ScheduledFuture<?> scheduleAtFixedRate(Runnable task, Instant startTime, Duration period);

    ScheduledFuture<?> scheduleAtFixedRate(Runnable task, Duration period);

    ScheduledFuture<?> scheduleWithFixedDelay(Runnable task, Instant startTime, Duration delay);

    ScheduledFuture<?> scheduleWithFixedDelay(Runnable task, Duration delay);
}
```

### `Trigger` 接口

`Trigger` 接口本质上受到 JSR-236 的启发：

```java
public interface Trigger {

    Instant nextExecution(TriggerContext triggerContext);
}
```

Spring 提供了 `CronTrigger`，它基于 cron 表达式启用任务调度：

```java
scheduler.schedule(task, new CronTrigger("0 15 9-17 * * MON-FRI"));
```

## 调度和异步执行的注解支持

Spring 为任务调度和异步方法执行提供了注解支持。

### 启用调度注解

```java
@Configuration
@EnableAsync
@EnableScheduling
public class SchedulingConfiguration {
}
```

### `@Scheduled` 注解

您可以将 `@Scheduled` 注解添加到方法中，并附带触发器元数据：

```java
@Scheduled(fixedDelay = 5000)
public void doSomething() {
    // something that should run periodically
}
```

使用不同时间单位：

```java
@Scheduled(fixedDelay = 5, timeUnit = TimeUnit.SECONDS)
public void doSomething() {
    // something that should run periodically
}
```

固定速率执行：

```java
@Scheduled(fixedRate = 5, timeUnit = TimeUnit.SECONDS)
public void doSomething() {
    // something that should run periodically
}
```

带初始延迟：

```java
@Scheduled(initialDelay = 1000, fixedRate = 5000)
public void doSomething() {
    // something that should run periodically
}
```

使用 cron 表达式：

```java
@Scheduled(cron="*/5 * * * * MON-FRI")
public void doSomething() {
    // something that should run on weekdays only
}
```

### `@Async` 注解

您可以在方法上提供 `@Async` 注解，以便该方法的调用异步发生：

```java
@Async
void doSomething() {
    // this will be run asynchronously
}
```

带参数的方法：

```java
@Async
void doSomething(String s) {
    // this will be run asynchronously
}
```

返回值的方法：

```java
@Async
Future<String> returnSomething(int i) {
    // this will be run asynchronously
}
```

### 使用 `@Async` 进行执行器限定

```java
@Async("otherExecutor")
void doSomething(String s) {
    // this will be run asynchronously by "otherExecutor"
}
```

### 使用 `@Async` 进行异常管理

```java
public class MyAsyncUncaughtExceptionHandler implements AsyncUncaughtExceptionHandler {

    @Override
    public void handleUncaughtException(Throwable ex, Method method, Object... params) {
        // handle exception
    }
}
```

## Cron 表达式

所有 Spring cron 表达式都必须符合相同的格式：

```
 ┌───────────── second (0-59)
 │ ┌───────────── minute (0 - 59)
 │ │ ┌───────────── hour (0 - 23)
 │ │ │ ┌───────────── day of the month (1 - 31)
 │ │ │ │ ┌───────────── month (1 - 12) (or JAN-DEC)
 │ │ │ │ │ ┌───────────── day of the week (0 - 7)
 │ │ │ │ │ │          (0 or 7 is Sunday, or MON-SUN)
 │ │ │ │ │ │
 * * * * * *
```

### Cron 表达式示例

| Cron 表达式 | 含义 |
| --- | --- |
| `0 0 * * * *` | 每天每小时的顶部 |
| `*/10 * * * * *` | 每十秒钟 |
| `0 0 8-10 * * *` | 每天的 8、9 和 10 点 |
| `0 0 6,19 * * *` | 每天的上午 6:00 和下午 7:00 |
| `0 0/30 8-10 * * *` | 每天的 8:00、8:30、9:00、9:30、10:00 和 10:30 |
| `0 0 9-17 * * MON-FRI` | 工作日九点到五点每小时的顶部 |
| `0 0 0 25 DEC ?` | 每年圣诞节午夜 |
| `0 0 0 L * *` | 每月最后一天午夜 |
| `0 0 0 ? * 5#2` | 每月第二个星期五午夜 |

### 宏（Macros）

Spring 支持以下宏，它们代表常用序列：

| 宏 | 含义 |
| --- | --- |
| `@yearly` (或 `@annually`) | 每年一次 (`0 0 0 1 1 *`) |
| `@monthly` | 每月一次 (`0 0 0 1 * *`) |
| `@weekly` | 每周一次 (`0 0 0 * * 0`) |
| `@daily` (或 `@midnight`) | 每天一次 (`0 0 0 * * *`) |
| `@hourly` | 每小时一次 (`0 0 * * * *`) |

## 使用 Quartz 调度器

Quartz 使用 `Trigger`、`Job` 和 `JobDetail` 对象来实现各种作业的调度。

### 使用 `JobDetailFactoryBean`

```xml
<bean name="exampleJob" class="org.springframework.scheduling.quartz.JobDetailFactoryBean">
    <property name="jobClass" value="example.ExampleJob"/>
    <property name="jobDataAsMap">
        <map>
            <entry key="timeout" value="5"/>
        </map>
    </property>
</bean>
```

### 使用 `MethodInvokingJobDetailFactoryBean`

```xml
<bean id="jobDetail" class="org.springframework.scheduling.quartz.MethodInvokingJobDetailFactoryBean">
    <property name="targetObject" ref="exampleBusinessObject"/>
    <property name="targetMethod" value="doIt"/>
</bean>
```

### 使用触发器和 `SchedulerFactoryBean` 连接作业

```xml
<bean id="simpleTrigger" class="org.springframework.scheduling.quartz.SimpleTriggerFactoryBean">
    <property name="jobDetail" ref="jobDetail"/>
    <property name="startDelay" value="10000"/>
    <property name="repeatInterval" value="50000"/>
</bean>

<bean id="cronTrigger" class="org.springframework.scheduling.quartz.CronTriggerFactoryBean">
    <property name="jobDetail" ref="exampleJob"/>
    <property name="cronExpression" value="0 0 6 * * ?"/>
</bean>

<bean class="org.springframework.scheduling.quartz.SchedulerFactoryBean">
    <property name="triggers">
        <list>
            <ref bean="cronTrigger"/>
            <ref bean="simpleTrigger"/>
        </list>
    </property>
</bean>
```
