# Runtime
# 运行时

Nexus Runtime is a high-performance async runtime built from scratch, designed for maximum scalability and performance.

Nexus Runtime 是一个从头构建的高性能异步运行时，旨在实现最大的可扩展性和性能。

## Overview / 概述

The Nexus runtime provides:
Nexus 运行时提供：

- **Thread-per-core architecture**: Each CPU core runs its own scheduler, eliminating work-stealing overhead
  **Thread-per-core 架构**：每个 CPU 核心运行自己的调度器，消除工作窃取开销

- **io-uring based I/O**: Zero-copy operations on Linux 5.1+
  **基于 io-uring 的 I/O**：在 Linux 5.1+ 上实现零拷贝操作

- **Hierarchical timer wheel**: O(1) timer operations
  **分层时间轮**：O(1) 定时器操作

- **Ownership-based buffers**: Safe buffer management without reference counting
  **基于所有权的缓冲区**：无需引用计数的安全缓冲区管理

## Architecture / 架构

```
┌─────────────────────────────────────────┐
│           Runtime                       │
│           运行时                         │
├─────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌──────┐ │
│  │Scheduler │  │  Driver  │  │Timer │ │
│  │调度器    │  │  驱动    │  │定时器│ │
│  └──────────┘  └──────────┘  └──────┘ │
└─────────────────────────────────────────┘
           │         │         │
           ▼         ▼         ▼
    ┌──────────┐ ┌──────┐ ┌────────┐
    │Task Queue│ │io-uring│ │Wheel 0-3│
    │任务队列  │ │epoll  │ │时间轮   │
    │          │ │kqueue │ │        │
    └──────────┘ └──────┘ └────────┘
```

## Creating a Runtime / 创建运行时

### Default Runtime / 默认运行时

The simplest way to create a runtime:
创建运行时的最简单方法：

```rust
use nexus_runtime::Runtime;

let mut runtime = Runtime::new()?;
```

This creates a runtime with:
这将创建一个具有以下配置的运行时：

- Auto-detected driver (io-uring on Linux, kqueue on macOS)
  自动检测的驱动（Linux 上为 io-uring，macOS 上为 kqueue）

- Default scheduler configuration
  默认调度器配置

- Default I/O queue depth (256 entries)
  默认 I/O 队列深度（256 个条目）

### Custom Runtime / 自定义运行时

Use `RuntimeBuilder` for custom configuration:
使用 `RuntimeBuilder` 进行自定义配置：

```rust
use nexus_runtime::{Runtime, RuntimeBuilder, driver::DriverType};

let mut runtime = Runtime::builder()
    .worker_threads(4)              // 4 worker threads / 4个工作线程
    .queue_size(512)                 // Task queue size / 任务队列大小
    .driver_type(DriverType::IOUring) // Force io-uring / 强制使用io-uring
    .io_entries(512)                 // I/O queue depth / I/O队列深度
    .enable_parking(true)            // Enable thread parking / 启用线程休眠
    .park_timeout(Duration::from_millis(100)) // Park timeout / 休眠超时
    .build()?;
```

## Running Async Code / 运行异步代码

### block_on / 阻塞运行

The `block_on` method runs a future to completion:
`block_on` 方法运行一个 future 直到完成：

```rust
runtime.block_on(async {
    println!("Hello from async!");
    println!("来自异步的问候！");
});
```

### Spawning Tasks / 生成任务

Tasks can be spawned to run concurrently:
可以生成任务以并发运行：

```rust
use nexus_runtime::{Runtime, spawn};

runtime.block_on(async {
    let handle = spawn(async {
        println!("Task running!");
        println!("任务运行中！");
        42
    });
    
    let result = handle.await.unwrap();
    println!("Task returned: {}", result);
    println!("任务返回: {}", result);
});
```

## I/O Drivers / I/O 驱动

Nexus Runtime supports multiple I/O drivers:
Nexus Runtime 支持多个 I/O 驱动：

### io-uring (Linux)

The fastest driver, available on Linux 5.1+:
最快的驱动，在 Linux 5.1+ 上可用：

```rust
use nexus_runtime::{Runtime, RuntimeBuilder, driver::DriverType};

let runtime = Runtime::builder()
    .driver_type(DriverType::IOUring)
    .build()?;
```

**Features / 特性**:
- Zero-copy operations / 零拷贝操作
- Batched I/O submissions / 批量 I/O 提交
- Async I/O completion / 异步 I/O 完成

### epoll (Linux fallback)

Automatic fallback for older Linux kernels:
旧版 Linux 内核的自动回退：

```rust
let runtime = Runtime::builder()
    .driver_type(DriverType::Epoll)
    .build()?;
```

### kqueue (macOS/BSD)

Used automatically on macOS and BSD systems:
在 macOS 和 BSD 系统上自动使用：

```rust
let runtime = Runtime::builder()
    .driver_type(DriverType::Kqueue)
    .build()?;
```

### Auto Detection / 自动检测

The default `Auto` type selects the best available driver:
默认的 `Auto` 类型选择最佳可用驱动：

```rust
let runtime = Runtime::builder()
    .driver_type(DriverType::Auto) // Default / 默认
    .build()?;
```

## Task Scheduling / 任务调度

### Thread-per-Core / 每核心一线程

Each CPU core runs its own scheduler:
每个 CPU 核心运行自己的调度器：

```rust
let runtime = Runtime::builder()
    .worker_threads(num_cpus::get()) // One per core / 每个核心一个
    .build()?;
```

**Benefits / 优势**:
- No work-stealing overhead / 无工作窃取开销
- Better cache locality / 更好的缓存局部性
- Linear scalability / 线性可扩展性

### Work-Stealing (Optional) / 工作窃取（可选）

For workloads with uneven distribution:
对于分布不均匀的工作负载：

```rust
use nexus_runtime::scheduler::{WorkStealingScheduler, WorkStealingConfig};

let config = WorkStealingConfig {
    queue_size: 512,
    // ... other settings / 其他设置
};
```

## Channels / 通道

### Unbounded Channels / 无界通道

For high-throughput scenarios:
用于高吞吐量场景：

```rust
use nexus_runtime::{spawn, unbounded};

let (tx, mut rx) = unbounded();

spawn(async move {
    for i in 0..100 {
        tx.send(i).await.unwrap();
    }
});

while let Some(value) = rx.recv().await {
    println!("Received: {}", value);
    println!("接收: {}", value);
}
```

### Bounded Channels / 有界通道

For backpressure control:
用于背压控制：

```rust
use nexus_runtime::{spawn, bounded};

let (tx, mut rx) = bounded::<i32>(10); // Buffer size 10 / 缓冲区大小10

spawn(async move {
    for i in 0..100 {
        // Will block when buffer is full / 缓冲区满时会阻塞
        tx.send(i).await.unwrap();
    }
});
```

## Timers / 定时器

### Sleep / 休眠

Simple delay:
简单延迟：

```rust
use nexus_runtime::{sleep, Duration};

sleep(Duration::from_secs(1)).await;
println!("1 second later");
println!("1秒后");
```

### Sleep Until / 休眠直到

Sleep until a specific time:
休眠到特定时间：

```rust
use nexus_runtime::{sleep_until, Instant, Duration};

let deadline = Instant::now() + Duration::from_secs(5);
sleep_until(deadline).await;
println!("5 seconds elapsed");
println!("5秒已过");
```

### Timer Wheel / 时间轮

The runtime uses a hierarchical timer wheel:
运行时使用分层时间轮：

- **Wheel 0**: 1ms resolution, 256 slots (256ms range)
  **轮0**：1ms 分辨率，256 个槽（256ms 范围）

- **Wheel 1**: 256ms resolution, 64 slots (16.384s range)
  **轮1**：256ms 分辨率，64 个槽（16.384s 范围）

- **Wheel 2**: 16.384s resolution, 64 slots (1048.576s range)
  **轮2**：16.384s 分辨率，64 个槽（1048.576s 范围）

- **Wheel 3**: 1048.576s resolution, 64 slots (67108.864s range)
  **轮3**：1048.576s 分辨率，64 个槽（67108.864s 范围）

This provides O(1) insertion and O(1) advancement.
这提供了 O(1) 插入和 O(1) 推进。

## I/O Operations / I/O 操作

### TCP Stream / TCP 流

```rust
use nexus_runtime::io::TcpStream;

// Connect / 连接
let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

// Write / 写入
stream.write_all(b"Hello, Server!").await?;

// Read / 读取
let mut buf = [0u8; 1024];
let n = stream.read(&mut buf).await?;
```

### TCP Listener / TCP 监听器

```rust
use nexus_runtime::{io::TcpListener, spawn};

let listener = TcpListener::bind("127.0.0.1:8080").await?;

while let Ok((stream, addr)) = listener.accept().await {
    spawn(async move {
        // Handle connection / 处理连接
        let mut buf = [0u8; 1024];
        let _ = stream.read(&mut buf).await;
    });
}
```

### UDP Socket / UDP 套接字

```rust
use nexus_runtime::io::UdpSocket;

let socket = UdpSocket::bind("127.0.0.1:0").await?;
socket.send_to(b"Hello", "127.0.0.1:8080").await?;

let mut buf = [0u8; 1024];
let (n, addr) = socket.recv_from(&mut buf).await?;
```

## Selecting Futures / 选择 Future

### select_two / 选择两个

Wait for the first of two futures:
等待两个 future 中的第一个：

```rust
use nexus_runtime::{select_two, spawn, sleep, Duration};

let task1 = spawn(async {
    sleep(Duration::from_millis(100)).await;
    "Task 1"
});

let task2 = spawn(async {
    sleep(Duration::from_millis(200)).await;
    "Task 2"
});

match select_two(task1, task2).await {
    select_two::First(result, _) => {
        println!("First completed: {}", result.unwrap());
    }
    select_two::Second(_, result) => {
        println!("Second completed: {}", result.unwrap());
    }
}
```

### select_multiple / 选择多个

Wait for the first of multiple futures:
等待多个 future 中的第一个：

```rust
use nexus_runtime::{select_multiple, SelectMultiple, spawn, sleep, Duration};

let futures = vec![
    spawn(async { sleep(Duration::from_millis(100)).await; 1 }),
    spawn(async { sleep(Duration::from_millis(200)).await; 2 }),
    spawn(async { sleep(Duration::from_millis(300)).await; 3 }),
];

let mut select = SelectMultiple::new(futures);
let (index, result) = select.await;
println!("Future {} completed: {}", index, result.unwrap());
```

## Performance Considerations / 性能考虑

### Driver Selection / 驱动选择

- **io-uring**: Best performance on Linux 5.1+
  **io-uring**：在 Linux 5.1+ 上性能最佳

- **epoll**: Good performance, wider compatibility
  **epoll**：性能良好，兼容性更广

- **kqueue**: Best option on macOS/BSD
  **kqueue**：在 macOS/BSD 上的最佳选择

### Queue Sizes / 队列大小

Larger queues reduce contention but use more memory:
更大的队列减少竞争但使用更多内存：

```rust
let runtime = Runtime::builder()
    .queue_size(1024)    // Larger queue / 更大的队列
    .io_entries(512)     // More I/O entries / 更多I/O条目
    .build()?;
```

### Thread Configuration / 线程配置

Match worker threads to CPU cores:
将工作线程与 CPU 核心匹配：

```rust
let runtime = Runtime::builder()
    .worker_threads(num_cpus::get()) // One per core / 每个核心一个
    .build()?;
```

## Error Handling / 错误处理

Runtime operations return `io::Result`:
运行时操作返回 `io::Result`：

```rust
use nexus_runtime::Runtime;

match Runtime::new() {
    Ok(mut runtime) => {
        runtime.block_on(async {
            // Use runtime / 使用运行时
        });
    }
    Err(e) => {
        eprintln!("Failed to create runtime: {}", e);
        eprintln!("创建运行时失败: {}", e);
    }
}
```

## Best Practices / 最佳实践

1. **Use Auto driver type**: Let the runtime choose the best driver
   **使用 Auto 驱动类型**：让运行时选择最佳驱动

2. **Match threads to cores**: One worker thread per CPU core
   **线程与核心匹配**：每个 CPU 核心一个工作线程

3. **Use bounded channels**: For backpressure control
   **使用有界通道**：用于背压控制

4. **Avoid blocking**: Use `spawn_blocking` for CPU-intensive work
   **避免阻塞**：对 CPU 密集型工作使用 `spawn_blocking`

5. **Profile first**: Measure before optimizing
   **先分析**：优化前先测量

## Next Steps / 下一步

- Explore the [API Documentation](https://docs.rs/nexus-runtime) for complete reference
  探索 [API 文档](https://docs.rs/nexus-runtime) 获取完整参考

- Check out [HTTP Server](../core-concepts/http.md) documentation (coming in Phase 2)
  查看 [HTTP 服务器](../core-concepts/http.md) 文档（将在 Phase 2 提供）

- See [Examples](https://github.com/nexus-rs/nexus/tree/main/examples) for more use cases
  查看 [示例](https://github.com/nexus-rs/nexus/tree/main/examples) 了解更多用例

---

*← [Previous / 上一页](../getting-started/quick-start.md) | [Next / 下一页](./http.md) →*
