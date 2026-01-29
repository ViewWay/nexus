//! Runtime benchmarks
//! 运行时基准测试
//!
//! # Equivalent to Tokio Benchmarks / 等价于 Tokio 基准测试
//!
//! This benchmark suite measures the performance of Nexus runtime
//! compared to the baseline (tokio/async-std).
//!
//! 此基准测试套件测量 Nexus 运行时与基线（tokio/async-std）相比的性能。

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use nexus_runtime::{Runtime, bounded, channel, select_two, spawn};

fn bench_spawn_single(c: &mut Criterion) {
    let mut group = c.benchmark_group("spawn_single");

    group.bench_function("nexus", |b| {
        let mut runtime = Runtime::new().unwrap();
        b.iter(|| {
            let _ = runtime.block_on(async {
                let handle = spawn(async { 42i32 });
                std::hint::black_box(handle.wait().await.unwrap());
            });
        });
    });

    group.finish();
}

fn bench_spawn_many(c: &mut Criterion) {
    let mut group = c.benchmark_group("spawn_many");

    for count in [100usize, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(BenchmarkId::from_parameter(count), count, |b, count| {
            let mut runtime = Runtime::new().unwrap();
            b.iter(|| {
                let _ = runtime.block_on(async {
                    let mut handles = Vec::with_capacity(*count);
                    for _ in 0..*count {
                        handles.push(spawn(async { 42i32 }));
                    }
                    for handle in handles {
                        std::hint::black_box(handle.wait().await.unwrap());
                    }
                });
            });
        });
    }

    group.finish();
}

fn bench_channel_unbounded(c: &mut Criterion) {
    let mut group = c.benchmark_group("channel_unbounded");

    group.bench_function("single_send_recv", |b| {
        let mut runtime = Runtime::new().unwrap();
        b.iter(|| {
            let _ = runtime.block_on(async {
                let (tx, mut rx) = channel::unbounded::<i32>();
                tx.send(42).unwrap();
                std::hint::black_box(rx.recv().await.unwrap());
            });
        });
    });

    for count in [100usize, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*count as u64));
        group.bench_with_input(BenchmarkId::new("bulk_send_recv", count), count, |b, count| {
            let mut runtime = Runtime::new().unwrap();
            let count_val = *count;
            b.iter(|| {
                let _ = runtime.block_on(async {
                    let (tx, mut rx) = channel::unbounded::<i32>();
                    let sender = spawn(async move {
                        for i in 0..count_val {
                            tx.send(i as i32).unwrap();
                        }
                    });
                    let receiver = spawn(async move {
                        let mut sum = 0i32;
                        for _ in 0..count_val {
                            if let Some(v) = rx.recv().await {
                                sum += v;
                            }
                        }
                        sum
                    });
                    std::hint::black_box(sender.wait().await.unwrap());
                    std::hint::black_box(receiver.wait().await.unwrap());
                });
            });
        });
    }

    group.finish();
}

fn bench_channel_bounded(c: &mut Criterion) {
    let mut group = c.benchmark_group("channel_bounded");

    for buffer in [0usize, 1, 10, 100].iter() {
        group.bench_with_input(BenchmarkId::new("single_send_recv", buffer), buffer, |b, buffer| {
            let mut runtime = Runtime::new().unwrap();
            b.iter(|| {
                let _ = runtime.block_on(async {
                    let (tx, mut rx) = bounded::<i32>(*buffer);
                    tx.send(42).unwrap();
                    std::hint::black_box(rx.recv().await.unwrap());
                });
            });
        });
    }

    group.finish();
}

fn bench_select_two(c: &mut Criterion) {
    let mut group = c.benchmark_group("select_two");

    group.bench_function("select_two_async", |b| {
        let mut runtime = Runtime::new().unwrap();
        b.iter(|| {
            let _ = runtime.block_on(async {
                let (tx1, mut rx1) = bounded::<i32>(1);
                let (_tx2, mut rx2) = bounded::<i32>(1);

                let select_task = spawn(async move {
                    std::hint::black_box(select_two(rx1.recv(), rx2.recv()).await);
                });

                // First one completes
                tx1.send(1).unwrap();

                std::hint::black_box(select_task.wait().await.unwrap());
            });
        });
    });

    group.finish();
}

fn bench_scheduler_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("scheduler");

    for tasks in [10usize, 100, 1000].iter() {
        group.throughput(Throughput::Elements(*tasks as u64));
        group.bench_with_input(BenchmarkId::new("spawn_and_await", tasks), tasks, |b, tasks| {
            let mut runtime = Runtime::new().unwrap();
            b.iter(|| {
                let _ = runtime.block_on(async {
                    let mut handles = Vec::new();
                    for i in 0..*tasks {
                        handles.push(spawn(async move {
                            let mut sum = 0i64;
                            for j in 0..100 {
                                sum += i as i64 * j as i64;
                            }
                            sum
                        }));
                    }
                    let mut total = 0i64;
                    for handle in handles {
                        total += handle.wait().await.unwrap();
                    }
                    std::hint::black_box(total);
                });
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_spawn_single,
    bench_spawn_many,
    bench_channel_unbounded,
    bench_channel_bounded,
    bench_select_two,
    bench_scheduler_throughput
);

criterion_main!(benches);
