# Performance / 性能

> **Status**: Phase 2+ Available ✅  
> **状态**: 第2阶段+可用 ✅

Nexus is designed for high performance from the ground up.

Nexus 从设计之初就追求高性能。

---

## Performance Goals / 性能目标

| Metric | Target | Status |
|--------|--------|--------|
| **QPS** (simple echo) | 1M+ | 📊 Phase 2 |
| **P99 latency** | < 1ms | 📊 Phase 2 |
| **Memory** (idle) | < 10MB | 📊 Phase 2 |
| **Startup time** | < 50ms | 📊 Phase 2 |

---

## Performance Features / 性能特性

### io-uring / io-uring

- **70% fewer syscalls** vs epoll
- **40% lower latency**
- **Batch I/O operations**

### Thread-per-Core / Thread-per-Core

- **No lock contention**
- **Better cache locality**
- **Linear scalability**

### Zero-Copy I/O / 零拷贝I/O

- **Minimal allocations**
- **Efficient buffer management**
- **Reduced memory pressure**

---

## Benchmarking / 基准测试

See [Benchmarking Guide](../../../benchmarking.md) for detailed performance testing.

详细的性能测试请参阅 [基准测试指南](../../../benchmarking.md)。

---

## Optimization Tips / 优化技巧

1. **Use thread-per-core for I/O-bound** / **I/O密集型使用thread-per-core**
2. **Enable io-uring on Linux 5.1+** / **Linux 5.1+启用io-uring**
3. **Tune queue sizes** / **调整队列大小**
4. **Monitor metrics** / **监控指标**

---

*← [Previous / 上一页](./configuration.md) | [Next / 下一页](./security.md) →*
