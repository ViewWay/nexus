# Runtime
# 运行时

The Nexus runtime (`nexus-runtime`) is a high-performance async runtime built from scratch, designed specifically for web server workloads. Unlike Tokio-based frameworks, Nexus uses a custom runtime optimized for maximum throughput and minimal latency.

Nexus 运行时（`nexus-runtime`）是一个从零构建的高性能异步运行时，专为 Web 服务器工作负载设计。与基于 Tokio 的框架不同，Nexus 使用自定义运行时以实现最大吞吐量和最低延迟。

## Overview / 概述

```
┌─────────────────────────────────────────────────────────────┐
│                     Nexus Runtime                            │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐         │
│  │   Task      │  │   Timer     │  │   Channel   │         │
│  │  Scheduler  │  │   Wheel     │  │   (MPSC)    │         │
│  └──────┬──────┘  └──────┬──────┘  └─────────────┘         │
│         │                │                                  │
│  ┌──────┴────────────────┴──────┐                          │
│  │         I/O Driver           │                          │
│  │  io-uring / epoll / kqueue   │                          │
│  └──────────────────────────────┘                          │
└─────────────────────────────────────────────────────────────┘
```

## Key Features / 核心特性

### 1. I/O Drivers / I/O 驱动器

Nexus automatically selects the best I/O driver for your platform:

Nexus 自动为您的平台选择最佳 I/O 驱动器：

| Platform | Primary Driver | Fallback |
|----------|---------------|----------|
| Linux 5.1+ | io-uring | epoll |
| Linux < 5.1 | epoll | - |
| macOS/BSD | kqueue | - |
| Windows | IOCP (planned) | - |

```rust
use nexus_runtime::{Runtime, DriverType};

// Auto-select best driver / 自动选择最佳驱动
let runtime = Runtime::new()?;

// Or specify driver explicitly / 或明确指定驱动
let runtime = Runtime::builder()
    .driver_type(DriverType::IoUring)
    .build()?;
```

### 2. Thread-per-Core Architecture / Thread-per-Core 架构

By default, Nexus uses a thread-per-core scheduler where each CPU core has its own task queue. This eliminates cross-thread synchronization overhead.

默认情况下，Nexus 使用 thread-per-core 调度器，每个 CPU 核心都有自己的任务队列。这消除了跨线程同步开销。

```rust
use nexus_runtime::{Runtime, SchedulerConfig};

// Thread-per-core (default) / Thread-per-core（默认）
let runtime = Runtime::builder()
    .scheduler(SchedulerConfig::ThreadPerCore)
    .build()?;

// Work-stealing for better load balancing / Work-stealing 以实现更好的负载均衡
let runtime = Runtime::builder()
    .scheduler(SchedulerConfig::WorkStealing)
    .build()?;
```

### 3. Timer Wheel / 时间轮

Nexus uses a hierarchical timer wheel with 4 levels for efficient timer management:

Nexus 使用 4 层层次化时间轮进行高效的定时器管理：

| Level | Resolution | Range |
|-------|------------|-------|
| 1 | 1ms | 256ms |
| 2 | 256ms | 65s |
| 3 | 65s | 4.6h |
| 4 | 4.6h | 49d |

```rust
use nexus_runtime::{sleep, Duration};

async fn example() {
    // Sleep for 100ms / 休眠 100ms
    sleep(Duration::from_millis(100)).await;
    
    // Sleep until specific time / 休眠到指定时间
    let deadline = Instant::now() + Duration::from_secs(5);
    sleep_until(deadline).await;
}
```

## Basic Usage / 基础用法

### Creating a Runtime / 创建运行时

```rust
use nexus_runtime::Runtime;

fn main() -> std::io::Result<()> {
    // Create runtime with default settings / 使用默认设置创建运行时
    let runtime = Runtime::new()?;
    
    // Run async code / 运行异步代码
    runtime.block_on(async {
        println!("Hello from Nexus runtime!");
    });
    
    Ok(())
}
```

### Spawning Tasks / 生成任务

```rust
use nexus_runtime::{spawn, JoinHandle};

async fn main_task() {
    // Spawn a background task / 生成后台任务
    let handle: JoinHandle<i32> = spawn(async {
        // Do some work / 执行一些工作
        42
    });
    
    // Wait for result / 等待结果
    let result = handle.await.unwrap();
    println!("Task returned: {}", result);
}
```

### Using Channels / 使用通道

```rust
use nexus_runtime::channel::{bounded, unbounded};

async fn channel_example() {
    // Bounded channel (backpressure) / 有界通道（背压）
    let (tx, rx) = bounded::<i32>(100);
    
    // Unbounded channel / 无界通道
    let (tx2, rx2) = unbounded::<String>();
    
    // Send and receive / 发送和接收
    tx.send(42).await.unwrap();
    let value = rx.recv().await.unwrap();
}
```

### Select on Multiple Futures / 在多个 Future 上 Select

```rust
use nexus_runtime::{select_two, SelectTwoOutput, sleep, Duration};

async fn timeout_example() {
    let operation = async { 
        // Long running operation / 长时间运行的操作
        sleep(Duration::from_secs(10)).await;
        "completed"
    };
    
    let timeout = sleep(Duration::from_secs(5));
    
    match select_two(operation, timeout).await {
        SelectTwoOutput::First(result) => {
            println!("Operation completed: {}", result);
        }
        SelectTwoOutput::Second(_) => {
            println!("Operation timed out!");
        }
    }
}
```

## Advanced Configuration / 高级配置

### RuntimeBuilder / 运行时构建器

```rust
use nexus_runtime::{Runtime, RuntimeBuilder, DriverType};

let runtime = RuntimeBuilder::new()
    // I/O driver configuration / I/O 驱动配置
    .driver_type(DriverType::IoUring)
    .io_uring_entries(1024)  // io-uring queue size / io-uring 队列大小
    
    // Scheduler configuration / 调度器配置
    .worker_threads(4)       // Number of worker threads / 工作线程数
    .thread_name("nexus-worker")
    
    // Timer configuration / 定时器配置
    .timer_tick_ms(1)        // Timer resolution / 定时器精度
    
    .build()?;
```

### Driver Configuration / 驱动配置

```rust
use nexus_runtime::driver::{DriverConfig, DriverConfigBuilder};

let driver_config = DriverConfigBuilder::new()
    .io_uring_entries(2048)      // Queue depth / 队列深度
    .io_uring_sqpoll(true)       // SQ polling mode / SQ 轮询模式
    .build();
```

## Performance Tips / 性能提示

1. **Use thread-per-core for CPU-bound workloads**
   对于 CPU 密集型工作负载使用 thread-per-core

2. **Use work-stealing for mixed workloads**
   对于混合工作负载使用 work-stealing

3. **Tune io-uring queue size based on connection count**
   根据连接数调整 io-uring 队列大小

4. **Prefer bounded channels for backpressure**
   优先使用有界通道以实现背压

## Comparison with Tokio / 与 Tokio 对比

| Feature | Nexus | Tokio |
|---------|-------|-------|
| io-uring support | Native | Via tokio-uring |
| Thread-per-core | Default | Optional |
| Timer resolution | 1ms | ~1ms |
| Work-stealing | Optional | Default |
| Zero-copy I/O | Yes | Partial |

---

*← [Previous / 上一页](../getting-started/quick-start.md) | [Next / 下一页](./http.md) →*
