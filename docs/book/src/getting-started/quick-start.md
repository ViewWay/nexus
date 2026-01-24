# Quick Start
# 快速开始

This guide will help you get started with Nexus Runtime in just a few minutes.
本指南将帮助你在几分钟内开始使用 Nexus Runtime。

## Your First Program / 你的第一个程序

Let's start with a simple "Hello, World!" example:
让我们从一个简单的 "Hello, World!" 示例开始：

```rust
use nexus_runtime::Runtime;

fn main() -> std::io::Result<()> {
    // Create a runtime with default configuration
    // 使用默认配置创建运行时
    let mut runtime = Runtime::new()?;
    
    // Run an async block
    // 运行一个异步块
    runtime.block_on(async {
        println!("Hello, Nexus Runtime!");
        println!("你好，Nexus Runtime！");
    });
    
    Ok(())
}
```

Save this as `main.rs` and run:
将其保存为 `main.rs` 并运行：

```bash
cargo run
```

You should see:
你应该看到：

```
Hello, Nexus Runtime!
你好，Nexus Runtime！
```

## Basic Async Operations / 基本异步操作

### Spawning Tasks / 生成任务

You can spawn tasks to run concurrently:
你可以生成任务以并发运行：

```rust
use nexus_runtime::{Runtime, spawn};

fn main() -> std::io::Result<()> {
    let mut runtime = Runtime::new()?;
    
    runtime.block_on(async {
        // Spawn a task / 生成一个任务
        let handle = spawn(async {
            println!("Task 1: Hello from spawned task!");
            println!("任务 1: 来自生成任务的问候！");
            42
        });
        
        // Spawn another task / 生成另一个任务
        let handle2 = spawn(async {
            println!("Task 2: Another task!");
            println!("任务 2: 另一个任务！");
            "done"
        });
        
        // Wait for tasks to complete / 等待任务完成
        let result1 = handle.await.unwrap();
        let result2 = handle2.await.unwrap();
        
        println!("Task 1 returned: {}", result1);
        println!("任务 1 返回: {}", result1);
        println!("Task 2 returned: {}", result2);
        println!("任务 2 返回: {}", result2);
    });
    
    Ok(())
}
```

### Using Channels / 使用通道

Nexus Runtime provides async channels for communication between tasks:
Nexus Runtime 提供异步通道用于任务间通信：

```rust
use nexus_runtime::{Runtime, spawn, bounded};

fn main() -> std::io::Result<()> {
    let mut runtime = Runtime::new()?;
    
    runtime.block_on(async {
        // Create a bounded channel / 创建一个有界通道
        let (tx, rx) = bounded::<i32>(10);
        
        // Spawn a sender task / 生成发送者任务
        let tx_handle = spawn(async move {
            for i in 0..5 {
                tx.send(i).await.unwrap();
                println!("Sent: {}", i);
                println!("发送: {}", i);
            }
        });
        
        // Spawn a receiver task / 生成接收者任务
        let rx_handle = spawn(async move {
            while let Ok(value) = rx.recv().await {
                println!("Received: {}", value);
                println!("接收: {}", value);
            }
        });
        
        // Wait for both tasks / 等待两个任务
        tx_handle.await.unwrap();
        rx_handle.await.unwrap();
    });
    
    Ok(())
}
```

### Timers and Delays / 定时器和延迟

You can use `sleep` to add delays:
你可以使用 `sleep` 来添加延迟：

```rust
use nexus_runtime::{Runtime, sleep, Duration};

fn main() -> std::io::Result<()> {
    let mut runtime = Runtime::new()?;
    
    runtime.block_on(async {
        println!("Starting...");
        println!("开始...");
        
        // Sleep for 1 second / 休眠 1 秒
        sleep(Duration::from_secs(1)).await;
        
        println!("1 second later...");
        println!("1 秒后...");
        
        // Sleep for 500 milliseconds / 休眠 500 毫秒
        sleep(Duration::from_millis(500)).await;
        
        println!("Done!");
        println!("完成！");
    });
    
    Ok(())
}
```

## I/O Operations / I/O 操作

### TCP Client / TCP 客户端

Here's a simple TCP client example:
这是一个简单的 TCP 客户端示例：

```rust
use nexus_runtime::{Runtime, io::TcpStream};

fn main() -> std::io::Result<()> {
    let mut runtime = Runtime::new()?;
    
    runtime.block_on(async {
        // Connect to a server / 连接到服务器
        match TcpStream::connect("127.0.0.1:8080").await {
            Ok(mut stream) => {
                // Write data / 写入数据
                let data = b"Hello, Server!";
                stream.write_all(data).await?;
                
                // Read response / 读取响应
                let mut buf = [0u8; 1024];
                let n = stream.read(&mut buf).await?;
                println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
                println!("接收: {}", String::from_utf8_lossy(&buf[..n]));
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
                eprintln!("连接失败: {}", e);
            }
        }
    })?;
    
    Ok(())
}
```

### TCP Server / TCP 服务器

And a simple TCP server:
以及一个简单的 TCP 服务器：

```rust
use nexus_runtime::{Runtime, io::TcpListener, spawn};

fn main() -> std::io::Result<()> {
    let mut runtime = Runtime::new()?;
    
    runtime.block_on(async {
        // Bind to an address / 绑定到地址
        let listener = TcpListener::bind("127.0.0.1:8080").await?;
        println!("Server listening on 127.0.0.1:8080");
        println!("服务器监听 127.0.0.1:8080");
        
        // Accept connections / 接受连接
        while let Ok((mut stream, addr)) = listener.accept().await {
            println!("Accepted connection from {}", addr);
            println!("接受来自 {} 的连接", addr);
            
            // Spawn a task to handle each connection / 生成任务处理每个连接
            spawn(async move {
                let mut buf = [0u8; 1024];
                if let Ok(n) = stream.read(&mut buf).await {
                    let request = String::from_utf8_lossy(&buf[..n]);
                    println!("Received: {}", request);
                    println!("接收: {}", request);
                    
                    // Echo back / 回显
                    let response = format!("Echo: {}", request);
                    let _ = stream.write_all(response.as_bytes()).await;
                }
            });
        }
    })?;
    
    Ok(())
}
```

## Custom Runtime Configuration / 自定义运行时配置

You can customize the runtime configuration:
你可以自定义运行时配置：

```rust
use nexus_runtime::{Runtime, RuntimeBuilder, driver::DriverType};

fn main() -> std::io::Result<()> {
    // Create a custom runtime / 创建自定义运行时
    let mut runtime = Runtime::builder()
        .worker_threads(4)              // 4 worker threads / 4个工作线程
        .queue_size(512)               // Queue size / 队列大小
        .driver_type(DriverType::Auto) // Auto-detect driver / 自动检测驱动
        .io_entries(256)               // I/O queue depth / I/O队列深度
        .build()?;
    
    runtime.block_on(async {
        println!("Custom runtime is running!");
        println!("自定义运行时正在运行！");
    });
    
    Ok(())
}
```

## Selecting Multiple Futures / 选择多个 Future

You can wait for multiple futures using `select`:
你可以使用 `select` 等待多个 future：

```rust
use nexus_runtime::{Runtime, spawn, select, select_two, sleep, Duration};

fn main() -> std::io::Result<()> {
    let mut runtime = Runtime::new()?;
    
    runtime.block_on(async {
        let task1 = spawn(async {
            sleep(Duration::from_millis(100)).await;
            "Task 1 completed"
        });
        
        let task2 = spawn(async {
            sleep(Duration::from_millis(200)).await;
            "Task 2 completed"
        });
        
        // Wait for the first task to complete / 等待第一个任务完成
        match select_two(task1, task2).await {
            select_two::First(result, _) => {
                println!("First task completed: {}", result.unwrap());
                println!("第一个任务完成: {}", result.unwrap());
            }
            select_two::Second(_, result) => {
                println!("Second task completed: {}", result.unwrap());
                println!("第二个任务完成: {}", result.unwrap());
            }
        }
    });
    
    Ok(())
}
```

## Next Steps / 下一步

Now that you've learned the basics, you can:
现在你已经学习了基础知识，你可以：

- Read the [Runtime Documentation](../core-concepts/runtime.md) for detailed information
  阅读 [Runtime 文档](../core-concepts/runtime.md) 获取详细信息

- Explore the [API Documentation](https://docs.rs/nexus-runtime) for complete API reference
  探索 [API 文档](https://docs.rs/nexus-runtime) 获取完整的 API 参考

- Check out the [Examples](https://github.com/nexus-rs/nexus/tree/main/examples) for more complex use cases
  查看 [示例](https://github.com/nexus-rs/nexus/tree/main/examples) 了解更多复杂用例

---

*← [Previous / 上一页](./installation.md) | [Next / 下一页](../core-concepts/runtime.md) →*
