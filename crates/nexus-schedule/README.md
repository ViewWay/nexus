# nexus-schedule

[![Crates.io](https://img.shields.io/crates/v/nexus-schedule)](https://crates.io/nexus-schedule)
[![Documentation](https://docs.rs/nexus-schedule/badge.svg)](https://docs.rs/nexus-schedule)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](../../LICENSE)

> Task scheduling for Nexus framework
>
> Nexus框架的任务调度

---

## 📋 Overview / 概述

`nexus-schedule` provides task scheduling capabilities with annotation-based scheduling, similar to Spring's `@Scheduled` and `@EnableScheduling`.

`nexus-schedule` 提供任务调度功能，支持基于注解的调度，类似于Spring的`@Scheduled`和`@EnableScheduling`。

**Key Features** / **核心特性**:
- ✅ **Annotation-based** / **基于注解** - `@Scheduled`
- ✅ **Fixed Rate** / **固定速率** - Run at fixed intervals
- ✅ **Fixed Delay** / **固定延迟** - Wait between completions
- ✅ **Cron Support** / **Cron支持** - Cron expression scheduling
- ✅ **Task Scheduler** / **任务调度器** - Centralized task management

---

## ✨ Features / 特性

| Feature | Spring Equivalent | Description | Status |
|---------|------------------|-------------|--------|
| **@Scheduled** | `@Scheduled` | Method scheduling | ✅ |
| **TaskScheduler** | `TaskScheduler` | Task scheduler | ✅ |
| **fixedRate** | `fixedRate` | Fixed rate execution | ✅ |
| **fixedDelay** | `fixedDelay` | Fixed delay execution | ✅ |
| **cron** | `cron` | Cron expression | ✅ |

---

## 🚀 Quick Start / 快速开始

### Installation / 安装

```toml
[dependencies]
nexus-schedule = "0.1.0-alpha"
```

### Basic Usage / 基本用法

```rust
use nexus_schedule::{ScheduledTask, TaskScheduler, schedule_fixed_rate, schedule_fixed_delay};

#[tokio::main]
async fn main() {
    // Create scheduler / 创建调度器
    let scheduler = TaskScheduler::new();
    scheduler.run().await;

    // Fixed rate task / 固定速率任务
    schedule_fixed_rate(5000, || {
        println!("Running every 5 seconds");
    }).await;

    // Fixed delay task / 固定延迟任务
    schedule_fixed_delay(5000, || {
        println!("Running 5 seconds after completion");
    }).await;
}
```

---

## 📖 Scheduling Options / 调度选项

### Fixed Rate / 固定速率

Execute at a fixed interval, regardless of execution time:

以固定间隔执行，不考虑执行时间：

```rust
use nexus_schedule::schedule_fixed_rate;

// Run every 5 seconds / 每5秒运行一次
schedule_fixed_rate(5000, || {
    println!("Fixed rate task");
}).await;
```

**Spring Equivalent / Spring等价物**:
```java
@Scheduled(fixedRate = 5000)
public void task() {
    // Runs every 5 seconds
}
```

---

### Fixed Delay / 固定延迟

Wait a specified delay between the end of one execution and the start of the next:

在一次执行结束和下一次执行开始之间等待指定延迟：

```rust
use nexus_schedule::schedule_fixed_delay;

// Run 5 seconds after completion / 完成后5秒运行
schedule_fixed_delay(5000, || {
    println!("Fixed delay task");
}).await;
```

**Spring Equivalent / Spring等价物**:
```java
@Scheduled(fixedDelay = 5000)
public void task() {
    // Runs 5 seconds after completion
}
```

---

### Initial Delay / 初始延迟

Delay the first execution by a specified amount:

第一次执行前延迟指定时间：

```rust
use nexus_schedule::ScheduledTask;

let task = ScheduledTask::fixed_rate("my-task", 5000)
    .initial_delay(1000);  // Wait 1 second before first run
```

**Spring Equivalent / Spring等价物**:
```java
@Scheduled(fixedRate = 5000, initialDelay = 1000)
public void task() {
    // Runs every 5 seconds, starting after 1 second
}
```

---

### Cron Expression / Cron表达式

Schedule using cron expressions:

使用cron表达式调度：

```rust
use nexus_schedule::ScheduledTask;

let task = ScheduledTask::cron("my-task", "0 0 * * * ?");
```

**Spring Equivalent / Spring等价物**:
```java
@Scheduled(cron = "0 0 * * * ?")
public void task() {
    // Runs every hour
}
```

---

## 🏗️ Task Scheduler / 任务调度器

### TaskScheduler / 任务调度器

Centralized task management:

集中式任务管理：

```rust
use nexus_schedule::{TaskScheduler, ScheduledTask};

let scheduler = TaskScheduler::new();

// Start scheduler / 启动调度器
scheduler.run().await;

// Create tasks / 创建任务
let task1 = ScheduledTask::fixed_rate("task1", 5000);
let task2 = ScheduledTask::fixed_delay("task2", 10000);
let task3 = ScheduledTask::cron("task3", "0 0 * * * ?");
```

---

## 🧪 Testing / 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scheduled_task() {
        let task = ScheduledTask::fixed_rate("test", 1000);
        assert_eq!(task.name, "test");
    }

    #[tokio::test]
    async fn test_task_scheduler() {
        let scheduler = TaskScheduler::new();
        scheduler.run().await;
        // Test scheduler functionality / 测试调度器功能
    }
}
```

---

## 🚦 Roadmap / 路线图

### Phase 3: Core Scheduling ✅ (Completed / 已完成)
- [x] @Scheduled annotation
- [x] TaskScheduler
- [x] Fixed rate scheduling
- [x] Fixed delay scheduling
- [x] Initial delay support

### Phase 4: Advanced Features 📋 (Planned / 计划中)
- [ ] Full cron expression support
- [ ] Async task executor
- [ ] Task pool management
- [ ] Quartz integration

---

## 📚 Documentation / 文档

- **API Documentation**: [docs.rs/nexus-schedule](https://docs.rs/nexus-schedule)
- **Examples**: [examples/schedule_example.rs](../../examples/schedule_example.rs)

---

## 🤝 Contributing / 贡献

We welcome contributions! Please see:

- [CONTRIBUTING.md](../../CONTRIBUTING.md)
- [Design Spec](../../docs/design-spec.md)
- [GitHub Issues](https://github.com/nexus-framework/nexus/issues)

---

## 📄 License / 许可证

Licensed under Apache License 2.0. See [LICENSE](../../LICENSE) for details.

---

## 🙏 Acknowledgments / 致谢

Nexus Schedule is inspired by:

- **[Spring Framework](https://spring.io/projects/spring-framework)** - `@Scheduled`, `TaskScheduler`
- **[Tokio](https://tokio.rs/)** - Async runtime
- **[Sched](https://github.com/mfontanini/sched)** - Cron scheduling

---

**Built with ❤️ for task scheduling**

**为任务调度构建 ❤️**
