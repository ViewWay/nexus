# Nexus Framework - Benchmarking Guide
# Nexus框架 - 性能基准测试指南

**Version**: 0.1.0-alpha
**Date**: 2026-01-24
**Status**: Phase 1 Completed ✅, Phase 2 Benchmarks Completed ✅
**状态**: 第1阶段已完成 ✅，第2阶段基准测试已完成 ✅

---

## Table of Contents / 目录

1. [Overview / 概览](#1-overview-概览)
2. [Benchmarking Strategy / 基准测试策略](#2-benchmarking-strategy-基准测试策略)
3. [Phase 1: Runtime Benchmarks / 第1阶段：运行时基准测试](#3-phase-1-runtime-benchmarks-第1阶段运行时基准测试)
4. [Phase 2: HTTP Benchmarks / 第2阶段：HTTP基准测试](#4-phase-2-http-benchmarks-第2阶段http基准测试)
5. [Tools & Environment / 工具与环境](#5-tools--environment-工具与环境)
6. [Running Benchmarks / 运行基准测试](#6-running-benchmarks-运行基准测试)
7. [Performance Regression Detection / 性能回归检测](#7-performance-regression-detection-性能回归检测)
8. [Benchmark Results Archive / 基准测试结果归档](#8-benchmark-results-archive-基准测试结果归档)

---

## 1. Overview / 概览

### 1.1 Purpose / 目的

This document provides comprehensive guidelines for benchmarking the Nexus framework to:

本文档提供了对Nexus框架进行基准测试的全面指南，以便：

- **Validate performance goals** / **验证性能目标** - Ensure Nexus meets target QPS, latency, and memory usage
- **Compare with existing solutions** / **与现有解决方案比较** - Benchmark against Tokio, Actix Web, Axum
- **Detect regressions** / **检测性能回归** - Identify performance degradation across commits
- **Guide optimizations** / **指导优化** - Profile and identify bottlenecks

### 1.2 Performance Goals / 性能目标

| Metric / 指标 | Target / 目标 | Baseline (Tokio) | Status / 状态 |
|--------------|---------------|------------------|--------------|
| **QPS** (simple echo) | 1M+ | ~800K | 📊 Phase 2 |
| **P99 latency** (no middleware) | < 1ms | ~1.5ms | 📊 Phase 2 |
| **P999 latency** | < 5ms | ~8ms | 📊 Phase 2 |
| **Memory** (idle) | < 10MB | ~16MB | 📊 Phase 2 |
| **Memory** (10K connections) | < 200MB | ~280MB | 📊 Phase 2 |
| **CPU efficiency** | 95%+ | ~85% | 📊 Phase 2 |
| **Startup time** | < 50ms | ~80ms | 📊 Phase 2 |
| **Syscalls per request** | < 3 | ~8 | 📊 Phase 2 |

### 1.3 Comparison Matrix / 对比矩阵

| Framework | Runtime | I/O Backend | Scheduler | Our Goal |
|-----------|---------|-------------|-----------|----------|
| **Nexus** | Custom | io-uring (Linux) | Thread-per-core | **Baseline** |
| **Actix Web** | Tokio | epoll/kqueue | Work-stealing | +20% QPS |
| **Axum** | Tokio | epoll/kqueue | Work-stealing | +20% QPS |
| **Rocket** | Tokio | epoll/kqueue | Work-stealing | +30% QPS |
| **Hyper** | Tokio | epoll/kqueue | Work-stealing | +15% QPS |
| **Monoio** | Custom | io-uring | Thread-per-core | Comparable |

---

## 2. Benchmarking Strategy / 基准测试策略

### 2.1 Benchmark Types / 基准测试类型

```
┌────────────────────────────────────────────────────────────┐
│                   Benchmark Pyramid                        │
│                   基准测试金字塔                            │
├────────────────────────────────────────────────────────────┤
│                                                             │
│                   ┌─────────────────┐                      │
│                   │  Integration    │   10%               │
│                   │  集成基准测试    │   - E2E scenarios   │
│                   │  (TechEmpower)  │   - Full stack      │
│                   └─────────────────┘                      │
│                                                             │
│              ┌──────────────────────────┐                  │
│              │     Component            │   30%            │
│              │     组件基准测试          │   - HTTP parser  │
│              │  (Criterion benches)    │   - Router       │
│              └──────────────────────────┘   - Middleware  │
│                                                             │
│         ┌──────────────────────────────────────┐          │
│         │          Micro                        │   60%    │
│         │          微基准测试                   │   - spawn│
│         │      (inline benchmarks)             │   - I/O  │
│         └──────────────────────────────────────┘   - Timer│
│                                                             │
└────────────────────────────────────────────────────────────┘
```

#### 2.1.1 Micro Benchmarks / 微基准测试

**Purpose** / **目的**: Measure individual operations in isolation.

**Tools** / **工具**: `criterion` (Rust), custom harness

**Examples** / **示例**:
- Task spawn latency / 任务生成延迟
- Channel send/recv throughput / 通道发送/接收吞吐量
- Timer wheel operations / 时间轮操作
- Driver submit/wait cycles / 驱动提交/等待周期

#### 2.1.2 Component Benchmarks / 组件基准测试

**Purpose** / **目的**: Measure subsystem performance.

**Tools** / **工具**: `criterion` (Rust)

**Examples** / **示例**:
- HTTP request parsing / HTTP请求解析
- Router matching / 路由匹配
- Middleware chain execution / 中间件链执行
- Response serialization / 响应序列化

#### 2.1.3 Integration Benchmarks / 集成基准测试

**Purpose** / **目的**: Measure real-world performance.

**Tools** / **工具**: `wrk`, `hey`, `oha`, TechEmpower

**Examples** / **示例**:
- Simple echo server / 简单回显服务器
- JSON API server / JSON API服务器
- Database query benchmark / 数据库查询基准测试
- Full-stack web application / 全栈Web应用

---

## 3. Phase 1: Runtime Benchmarks / 第1阶段：运行时基准测试

### 3.1 Setup / 设置

#### 3.1.1 Create Benchmark Directory / 创建基准测试目录

```bash
# Project structure / 项目结构
nexus/
├── benches/                    # Benchmark suite / 基准测试套件
│   ├── runtime_bench.rs        # Runtime core benchmarks
│   ├── scheduler_bench.rs      # Scheduler benchmarks
│   ├── io_bench.rs             # I/O driver benchmarks
│   ├── timer_bench.rs          # Timer wheel benchmarks
│   ├── channel_bench.rs        # Channel benchmarks
│   └── support/                # Shared utilities
│       └── mod.rs
└── Cargo.toml                  # Add [[bench]] sections
```

#### 3.1.2 Cargo.toml Configuration / Cargo.toml配置

```toml
# nexus-runtime/Cargo.toml

[dev-dependencies]
criterion = { version = "0.8", features = ["html_reports"] }
tokio = { version = "1.43", features = ["full"] }  # For comparison
rand = "0.9"

[[bench]]
name = "runtime"
harness = false

[[bench]]
name = "scheduler"
harness = false

[[bench]]
name = "io"
harness = false

[[bench]]
name = "timer"
harness = false

[[bench]]
name = "channel"
harness = false
```

### 3.2 Runtime Core Benchmarks / 运行时核心基准测试

#### 3.2.1 Task Spawn Latency / 任务生成延迟

**Goal** / **目标**: < 10µs per spawn

```rust
// benches/runtime_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nexus_runtime::{Runtime, spawn};

fn bench_spawn_latency(c: &mut Criterion) {
    let mut runtime = Runtime::new().unwrap();
    
    c.bench_function("spawn_latency", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let handle = spawn(async {
                    black_box(42)
                });
                handle.wait().await.unwrap()
            }).unwrap();
        });
    });
}

criterion_group!(benches, bench_spawn_latency);
criterion_main!(benches);
```

#### 3.2.2 Channel Throughput / 通道吞吐量

**Goal** / **目标**: > 10M ops/sec (bounded), > 5M ops/sec (unbounded)

```rust
// benches/channel_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use nexus_runtime::{Runtime, bounded, unbounded};

fn bench_bounded_channel(c: &mut Criterion) {
    let mut runtime = Runtime::new().unwrap();
    let mut group = c.benchmark_group("bounded_channel");
    
    for size in [1, 10, 100, 1000] {
        group.throughput(Throughput::Elements(size as u64));
        group.bench_with_input(format!("size_{}", size), &size, |b, &size| {
            b.iter(|| {
                runtime.block_on(async {
                    let (tx, rx) = bounded::<i32>(size);
                    
                    for i in 0..size {
                        tx.send(i).await.unwrap();
                    }
                    
                    for _ in 0..size {
                        black_box(rx.recv().await.unwrap());
                    }
                }).unwrap();
            });
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_bounded_channel);
criterion_main!(benches);
```

#### 3.2.3 Timer Wheel Operations / 时间轮操作

**Goal** / **目标**: O(1) insertion, < 1µs overhead

```rust
// benches/timer_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nexus_runtime::{Runtime, sleep};
use std::time::Duration;

fn bench_timer_insert(c: &mut Criterion) {
    let mut runtime = Runtime::new().unwrap();
    
    c.bench_function("timer_insert", |b| {
        b.iter(|| {
            runtime.block_on(async {
                // Insert timer without waiting
                let _ = sleep(Duration::from_millis(black_box(100)));
            }).unwrap();
        });
    });
}

fn bench_timer_fire(c: &mut Criterion) {
    let mut runtime = Runtime::new().unwrap();
    
    c.bench_function("timer_fire", |b| {
        b.iter(|| {
            runtime.block_on(async {
                sleep(Duration::from_millis(1)).await;
            }).unwrap();
        });
    });
}

criterion_group!(benches, bench_timer_insert, bench_timer_fire);
criterion_main!(benches);
```

### 3.3 Scheduler Benchmarks / 调度器基准测试

#### 3.3.1 Task Queue Operations / 任务队列操作

**Goal** / **目标**: > 10M tasks/sec throughput

```rust
// benches/scheduler_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nexus_runtime::{Runtime, spawn};

fn bench_scheduler_throughput(c: &mut Criterion) {
    let mut runtime = Runtime::new().unwrap();
    
    c.bench_function("scheduler_throughput_1000_tasks", |b| {
        b.iter(|| {
            runtime.block_on(async {
                let mut handles = Vec::with_capacity(1000);
                
                for i in 0..1000 {
                    let handle = spawn(async move {
                        black_box(i * 2)
                    });
                    handles.push(handle);
                }
                
                for handle in handles {
                    black_box(handle.wait().await.unwrap());
                }
            }).unwrap();
        });
    });
}

criterion_group!(benches, bench_scheduler_throughput);
criterion_main!(benches);
```

### 3.4 I/O Driver Benchmarks / I/O驱动基准测试

#### 3.4.1 TCP Echo Throughput / TCP回显吞吐量

**Goal** / **目标**: > 1M requests/sec (single connection)

```rust
// benches/io_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use nexus_runtime::{Runtime, io::{TcpListener, TcpStream}};

fn bench_tcp_echo(c: &mut Criterion) {
    let mut runtime = Runtime::new().unwrap();
    let mut group = c.benchmark_group("tcp_echo");
    group.throughput(Throughput::Bytes(1024));
    
    group.bench_function("1kb_payload", |b| {
        b.iter(|| {
            runtime.block_on(async {
                // TODO: Implement TCP echo benchmark
                // 待实现TCP回显基准测试
            }).unwrap();
        });
    });
    
    group.finish();
}

criterion_group!(benches, bench_tcp_echo);
criterion_main!(benches);
```

### 3.5 Comparison with Tokio / 与Tokio对比

```rust
// benches/runtime_bench.rs (continued)

fn bench_spawn_latency_tokio(c: &mut Criterion) {
    c.bench_function("spawn_latency_tokio", |b| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        
        b.iter(|| {
            runtime.block_on(async {
                let handle = tokio::spawn(async {
                    black_box(42)
                });
                handle.await.unwrap()
            });
        });
    });
}

criterion_group!(
    benches,
    bench_spawn_latency,        // Nexus
    bench_spawn_latency_tokio   // Tokio
);
criterion_main!(benches);
```

---

## 4. Phase 2: HTTP Benchmarks / 第2阶段：HTTP基准测试

> **Status** / **状态**: ✅ Completed | Benchmark infrastructure set up with Criterion
> **状态**：已完成 | 使用Criterion设置了基准测试基础设施

> **Date** / **日期**: 2026-01-24

### 4.0 Benchmark Results Summary / 基准测试结果摘要

#### HTTP Server Benchmarks / HTTP服务器基准测试

| Benchmark | Time | Throughput | Notes |
|-----------|------|------------|-------|
| parse_simple_get | 170 ns | - | Simple GET request parsing |
| parse_get_with_headers | 215 ns | - | GET with multiple headers |
| parse_post_json | 617 ns | - | POST with JSON body |
| encode_response | 121 ns | - | Response serialization |
| encode_response_large | 403 ns | - | Large response (~10KB) |
| request_creation | 145 ns | - | Building HTTP request |
| response_creation | 5.1 ns | - | Building HTTP response |

**Throughput Results:**
- 64B POST: 124 MiB/s
- 256B POST: 488 MiB/s
- 1KB POST: 1.80 GiB/s
- 4KB POST: 6.80 GiB/s

#### Router Benchmarks / 路由器基准测试

| Benchmark | Time | Notes |
|-----------|------|-------|
| route_registration | 10.4 µs | 100 routes registration |
| large_router_creation | 11.4 µs | Large router with params |
| static_route_registration | 418 ns | Static routes (5 routes) |
| param_route_registration | 589 ns | Routes with path params |
| request_creation | 69 ns | Request building |
| response_creation | 5.5 ns | Response building |

### 4.1 TechEmpower Benchmarks / TechEmpower基准测试

#### 4.1.1 Test Types / 测试类型

| Test / 测试 | Description / 描述 | Nexus Target | Actix (Current) |
|------------|-------------------|--------------|-----------------|
| **JSON Serialization** | Return JSON response / 返回JSON响应 | Top 20 | #8 |
| **Single Query** | Database SELECT / 数据库查询 | Top 30 | #15 |
| **Multiple Queries** | N x SELECT / N次查询 | Top 30 | #18 |
| **Fortunes** | Template rendering / 模板渲染 | Top 40 | #25 |
| **Updates** | Database UPDATE / 数据库更新 | Top 40 | #22 |
| **Plaintext** | Raw throughput / 原始吞吐量 | Top 10 | #5 |

#### 4.1.2 Setup Instructions / 设置说明

```bash
# Clone TechEmpower Framework Benchmarks / 克隆TechEmpower框架基准测试
git clone https://github.com/TechEmpower/FrameworkBenchmarks.git
cd FrameworkBenchmarks

# Create Nexus benchmark / 创建Nexus基准测试
mkdir -p frameworks/Rust/nexus
cp -r frameworks/Rust/actix-web/* frameworks/Rust/nexus/

# Edit config.toml to add nexus / 编辑config.toml添加nexus
# ...

# Run benchmark / 运行基准测试
./tfb --mode verify --test nexus
./tfb --mode benchmark --test nexus
```

### 4.2 HTTP Parser Benchmarks / HTTP解析器基准测试

**Goal** / **目标**: > 1GB/s parsing throughput

```rust
// benches/http_parser_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

fn bench_http_request_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("http_parser");
    
    let request = b"GET /hello HTTP/1.1\r\n\
                     Host: example.com\r\n\
                     User-Agent: benchmark\r\n\
                     Accept: */*\r\n\
                     \r\n";
    
    group.throughput(Throughput::Bytes(request.len() as u64));
    
    group.bench_function("parse_request", |b| {
        b.iter(|| {
            // Parse HTTP request / 解析HTTP请求
            // TODO: Implement parser benchmark
            black_box(request)
        });
    });
    
    group.finish();
}

criterion_group!(benches, bench_http_request_parsing);
criterion_main!(benches);
```

### 4.3 Router Benchmarks / 路由器基准测试

**Goal** / **目标**: < 100ns per route match

```rust
// benches/router_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use nexus_router::Router;

fn bench_router_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("router");
    
    // Create router with N routes / 创建有N个路由的路由器
    for route_count in [10, 100, 1000, 10000] {
        let mut router = Router::new();
        for i in 0..route_count {
            router = router.get(&format!("/route{}", i), || async { "ok" });
        }
        
        group.bench_with_input(
            BenchmarkId::new("match", route_count),
            &route_count,
            |b, _| {
                b.iter(|| {
                    // Match route / 匹配路由
                    black_box(router.match_route("/route999"));
                });
            },
        );
    }
    
    group.finish();
}

criterion_group!(benches, bench_router_match);
criterion_main!(benches);
```

---

## 5. Tools & Environment / 工具与环境

### 5.1 Benchmark Tools / 基准测试工具

| Tool / 工具 | Purpose / 用途 | Install / 安装 |
|------------|---------------|---------------|
| **Criterion** | Micro benchmarks / 微基准测试 | `cargo add --dev criterion` |
| **wrk** | HTTP load testing / HTTP负载测试 | `brew install wrk` (macOS) |
| **hey** | HTTP load generator / HTTP负载生成器 | `go install github.com/rakyll/hey@latest` |
| **oha** | Modern HTTP load tester / 现代HTTP负载测试器 | `cargo install oha` |
| **hyperfine** | Command-line benchmarking / 命令行基准测试 | `cargo install hyperfine` |
| **perf** (Linux) | CPU profiling / CPU性能分析 | `apt install linux-tools-generic` |
| **Instruments** (macOS) | CPU/memory profiling / CPU/内存分析 | Built-in / 内置 |

### 5.2 Test Environment / 测试环境

#### 5.2.1 Recommended Hardware / 推荐硬件

```yaml
Minimum Specification / 最低规格:
  CPU: 4 cores / 4核心
  RAM: 16GB
  Storage: SSD
  Network: 1Gbps

Recommended Specification / 推荐规格:
  CPU: 8+ cores (Intel Xeon or AMD EPYC) / 8+核心
  RAM: 32GB+
  Storage: NVMe SSD
  Network: 10Gbps

TechEmpower Official / TechEmpower官方:
  CPU: Dell R440 - Intel Xeon Gold 5120 (14 cores)
  RAM: 32GB ECC
  Storage: SSD
  Network: 10Gbps dedicated
```

#### 5.2.2 OS Configuration / 操作系统配置

```bash
# Linux kernel tuning / Linux内核调优
sudo sysctl -w net.core.somaxconn=65535
sudo sysctl -w net.ipv4.tcp_max_syn_backlog=8192
sudo sysctl -w net.ipv4.ip_local_port_range="1024 65535"
sudo sysctl -w net.core.netdev_max_backlog=65535
sudo sysctl -w fs.file-max=2097152

# Increase file descriptor limits / 增加文件描述符限制
ulimit -n 65535

# Disable CPU frequency scaling / 禁用CPU频率缩放
sudo cpupower frequency-set --governor performance

# Enable io-uring (Linux 5.1+) / 启用io-uring
# Verify: cat /proc/sys/kernel/io_uring_disabled (should be 0)
```

### 5.3 Profiling Tools / 性能分析工具

#### 5.3.1 CPU Profiling / CPU性能分析

```bash
# Linux perf / Linux性能分析
perf record --call-graph=dwarf ./target/release/nexus-example
perf report

# Flamegraph generation / 火焰图生成
cargo install flamegraph
cargo flamegraph --bin nexus-example

# macOS Instruments / macOS性能分析
instruments -t "Time Profiler" ./target/release/nexus-example
```

#### 5.3.2 Memory Profiling / 内存分析

```bash
# Valgrind (Linux) / Valgrind内存分析
valgrind --tool=massif ./target/release/nexus-example
massif-visualizer massif.out.*

# Heaptrack (Linux) / Heaptrack内存追踪
heaptrack ./target/release/nexus-example
heaptrack_gui heaptrack.nexus-example.*
```

---

## 6. Running Benchmarks / 运行基准测试

### 6.1 Micro Benchmarks / 微基准测试

```bash
# Run all benchmarks / 运行所有基准测试
cargo bench

# Run specific benchmark / 运行特定基准测试
cargo bench --bench runtime_bench

# Save baseline for comparison / 保存基线用于比较
cargo bench --bench runtime_bench -- --save-baseline before-optimization

# Compare with baseline / 与基线比较
cargo bench --bench runtime_bench -- --baseline before-optimization

# Generate HTML reports / 生成HTML报告
cargo bench
open target/criterion/report/index.html
```

### 6.2 HTTP Load Testing / HTTP负载测试

#### 6.2.1 Using wrk / 使用wrk

```bash
# Simple GET request / 简单GET请求
wrk -t4 -c100 -d30s http://127.0.0.1:3000/

# With script (POST JSON) / 使用脚本（POST JSON）
wrk -t4 -c100 -d30s -s scripts/post.lua http://127.0.0.1:3000/api/users

# Latency percentiles / 延迟百分位
wrk -t4 -c100 -d30s --latency http://127.0.0.1:3000/
```

#### 6.2.2 Using hey / 使用hey

```bash
# 10K requests, 100 concurrent / 10K请求，100并发
hey -n 10000 -c 100 http://127.0.0.1:3000/

# POST with JSON body / POST带JSON体
hey -n 10000 -c 100 -m POST -H "Content-Type: application/json" \
    -d '{"name":"test"}' http://127.0.0.1:3000/api/users

# Save results / 保存结果
hey -n 10000 -c 100 -o csv http://127.0.0.1:3000/ > results.csv
```

#### 6.2.3 Using oha / 使用oha

```bash
# Modern output with histogram / 现代化输出带直方图
oha -n 10000 -c 100 http://127.0.0.1:3000/

# HTTP/2 testing / HTTP/2测试
oha -n 10000 -c 100 --http2 https://127.0.0.1:3443/

# Save JSON results / 保存JSON结果
oha -n 10000 -c 100 --json http://127.0.0.1:3000/ > results.json
```

### 6.3 Comparative Benchmarks / 对比基准测试

```bash
# Create test servers for comparison / 创建用于比较的测试服务器

# 1. Nexus server / Nexus服务器
cargo run --release --bin nexus-echo-server &
NEXUS_PID=$!

# 2. Actix server / Actix服务器
cargo run --release --bin actix-echo-server &
ACTIX_PID=$!

# Run benchmarks / 运行基准测试
echo "Benchmarking Nexus..."
wrk -t4 -c100 -d30s http://127.0.0.1:3000/ > nexus_results.txt

echo "Benchmarking Actix..."
wrk -t4 -c100 -d30s http://127.0.0.1:3001/ > actix_results.txt

# Cleanup / 清理
kill $NEXUS_PID $ACTIX_PID

# Compare results / 比较结果
python scripts/compare_bench.py nexus_results.txt actix_results.txt
```

---

## 7. Performance Regression Detection / 性能回归检测

### 7.1 CI/CD Integration / CI/CD集成

```yaml
# .github/workflows/bench.yml
name: Benchmark

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust / 安装Rust
        uses: actions-rs/toolchain@v1
        with:
          profile: minimal
          toolchain: stable
      
      - name: Run benchmarks / 运行基准测试
        run: cargo bench --workspace -- --save-baseline current
      
      - name: Compare with main / 与main分支比较
        if: github.event_name == 'pull_request'
        run: |
          git fetch origin main
          git checkout origin/main
          cargo bench --workspace -- --save-baseline main
          git checkout -
          cargo bench --workspace -- --baseline main
      
      - name: Upload results / 上传结果
        uses: actions/upload-artifact@v4
        with:
          name: benchmark-results
          path: target/criterion/
```

### 7.2 Regression Thresholds / 回归阈值

```rust
// benches/support/regression.rs

/// Acceptable performance degradation / 可接受的性能下降
const MAX_REGRESSION: f64 = 0.05; // 5%

/// Detect regression from baseline / 从基线检测回归
pub fn check_regression(baseline: Duration, current: Duration) -> Result<(), String> {
    let baseline_ns = baseline.as_nanos() as f64;
    let current_ns = current.as_nanos() as f64;
    
    let regression = (current_ns - baseline_ns) / baseline_ns;
    
    if regression > MAX_REGRESSION {
        Err(format!(
            "Performance regression detected: {:.1}% slower (max: {:.1}%)",
            regression * 100.0,
            MAX_REGRESSION * 100.0
        ))
    } else {
        Ok(())
    }
}
```

---

## 8. Benchmark Results Archive / 基准测试结果归档

### 8.1 Phase 1 Results (Pending) / 第1阶段结果（待补充）

> **Note** / **注意**: Comprehensive benchmark results will be added once Phase 2 HTTP server is complete.
> **注意**：一旦第2阶段HTTP服务器完成，将添加全面的基准测试结果。

```
Phase 1 Runtime Benchmarks / 第1阶段运行时基准测试
Expected completion: Phase 2 end (Month 7)
预计完成时间：第2阶段结束（第7个月）

Planned tests / 计划的测试:
- Task spawn latency: < 10µs
- Channel throughput: > 10M ops/sec
- Timer operations: O(1), < 1µs
- TCP echo: > 1M req/sec
```

### 8.2 Historical Data / 历史数据

```
docs/benchmarks/
├── phase1-runtime.md          # Phase 1 runtime results / 第1阶段运行时结果
├── phase2-http.md             # Phase 2 HTTP results / 第2阶段HTTP结果
├── phase3-middleware.md       # Phase 3 middleware results / 第3阶段中间件结果
└── comparisons/               # vs Tokio/Actix/Axum
    ├── vs-tokio.md
    ├── vs-actix.md
    └── vs-axum.md
```

---

## 9. Best Practices / 最佳实践

### 9.1 Benchmark Design / 基准测试设计

✅ **DO** / **应该做**:
- Use `black_box()` to prevent compiler optimizations / 使用`black_box()`防止编译器优化
- Run warm-up iterations / 运行预热迭代
- Test with realistic data sizes / 使用真实数据大小测试
- Measure multiple metrics (throughput, latency, memory) / 测量多个指标（吞吐量、延迟、内存）
- Document test environment / 记录测试环境

❌ **DON'T** / **不应该做**:
- Benchmark in debug mode / 在调试模式下基准测试
- Ignore cold start effects / 忽略冷启动效应
- Cherry-pick best results / 挑选最好的结果
- Run on battery power (laptops) / 使用电池供电运行（笔记本电脑）
- Ignore variance / 忽略方差

### 9.2 Interpreting Results / 解释结果

```
Example Criterion output / Criterion输出示例:

spawn_latency           time:   [8.234 µs 8.456 µs 8.701 µs]
                        change: [-2.3% -0.8% +0.7%] (p = 0.18 > 0.05)
                        No change in performance detected.
                        未检测到性能变化。

Key metrics / 关键指标:
- time: [lower_bound estimate upper_bound] / 时间：[下界 估计值 上界]
- change: Performance change from baseline / 与基线的性能变化
- p-value: Statistical significance / 统计显著性
```

### 9.3 Optimization Workflow / 优化工作流

```
1. Measure / 测量
   ↓
2. Profile (find bottleneck) / 分析（找到瓶颈）
   ↓
3. Optimize (fix bottleneck) / 优化（修复瓶颈）
   ↓
4. Measure again (verify improvement) / 再次测量（验证改进）
   ↓
5. Repeat / 重复
```

---

## 10. Troubleshooting / 故障排查

### 10.1 Common Issues / 常见问题

**Issue** / **问题**: High variance in results / 结果方差大

**Solution** / **解决方案**:
```bash
# Disable CPU frequency scaling / 禁用CPU频率缩放
sudo cpupower frequency-set --governor performance

# Disable turbo boost / 禁用睿频
echo 1 | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo

# Pin to specific cores / 固定到特定核心
taskset -c 0-3 cargo bench
```

**Issue** / **问题**: I/O bottleneck / I/O瓶颈

**Solution** / **解决方案**:
```bash
# Check I/O scheduler / 检查I/O调度器
cat /sys/block/sda/queue/scheduler

# Use none or mq-deadline for NVMe / NVMe使用none或mq-deadline
echo none | sudo tee /sys/block/nvme0n1/queue/scheduler
```

**Issue** / **问题**: Network saturation / 网络饱和

**Solution** / **解决方案**:
```bash
# Use loopback (127.0.0.1) for benchmarks / 基准测试使用回环地址
# Verify no packet drops / 验证无丢包
netstat -s | grep -i drop
```

---

## Appendix A: Benchmark Scripts / 附录A：基准测试脚本

### A.1 Full Benchmark Suite / 完整基准测试套件

```bash
#!/bin/bash
# scripts/run_all_benchmarks.sh

set -e

echo "🚀 Running Nexus Framework Benchmarks"
echo "======================================"

# 1. Micro benchmarks / 微基准测试
echo "📊 Phase 1: Micro benchmarks..."
cargo bench --bench runtime_bench
cargo bench --bench scheduler_bench
cargo bench --bench channel_bench
cargo bench --bench timer_bench

# 2. HTTP benchmarks (Phase 2) / HTTP基准测试（第2阶段）
echo "📊 Phase 2: HTTP benchmarks..."
# TODO: Add HTTP benchmarks

# 3. Generate report / 生成报告
echo "📄 Generating report..."
python scripts/generate_report.py

echo "✅ Benchmarks complete!"
echo "View results: open target/criterion/report/index.html"
```

---

## Appendix B: Reference Results / 附录B：参考结果

### B.1 Target vs Actual (Template) / 目标vs实际（模板）

```markdown
| Benchmark | Target | Actual | Status | Notes |
|-----------|--------|--------|--------|-------|
| Task spawn | < 10µs | TBD | 📊 | Phase 2 |
| Channel (bounded) | > 10M ops/s | TBD | 📊 | Phase 2 |
| Timer insert | < 1µs | TBD | 📊 | Phase 2 |
| TCP echo | > 1M req/s | TBD | 📊 | Phase 2 |
| HTTP parse | > 1GB/s | TBD | 📊 | Phase 2 |
| Router match | < 100ns | TBD | 📊 | Phase 2 |
```

---

**Last Updated** / **最后更新**: 2026-01-24  
**Next Review** / **下次审查**: Phase 2 completion (Month 7)

---

For questions or contributions, see [CONTRIBUTING.md](../CONTRIBUTING.md).
