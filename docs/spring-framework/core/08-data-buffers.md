# 数据缓冲区和编解码器

Java NIO 提供了 `ByteBuffer`，但许多库在此基础上构建了自己的字节缓冲区 API，特别是对于网络操作，重用缓冲区和/或使用直接缓冲区对性能有利。例如，Netty 有 `ByteBuf` 层次结构，Undertow 使用 XNIO，Jetty 使用带释放回调的池化字节缓冲区，等等。`spring-core` 模块提供了一组抽象来处理各种字节缓冲区 API，如下所示：

- `DataBufferFactory` 抽象了数据缓冲区的创建。
- `DataBuffer` 表示一个字节缓冲区，它可以是池化的。
- `DataBufferUtils` 为数据缓冲区提供了工具方法。
- 编解码器 将数据缓冲区流解码或编码为更高级别的对象。

## `DataBufferFactory`

`DataBufferFactory` 用于通过以下两种方式之一创建数据缓冲区：

1. 分配一个新的数据缓冲区，如果容量已知，可以选择预先指定，这样效率更高，尽管 `DataBuffer` 的实现可以按需增长和收缩。
2. 包装现有的 `byte[]` 或 `java.nio.ByteBuffer`，这将使用 `DataBuffer` 实现装饰给定数据，且不涉及分配。

注意，WebFlux 应用不直接创建 `DataBufferFactory`，而是通过 `ServerHttpResponse` 或客户端的 `ClientHttpRequest` 访问它。工厂的类型取决于底层的客户端或服务器，例如，Reactor Netty 使用 `NettyDataBufferFactory`，其他则使用 `DefaultDataBufferFactory`。

## `DataBuffer`

`DataBuffer` 接口提供与 `java.nio.ByteBuffer` 类似的操作，但也带来了一些额外的优势，其中一些受到了 Netty `ByteBuf` 的启发。以下是部分优势列表：

- 具有独立的读写位置，即在读写之间切换时不需要调用 `flip()`。
- 容量按需扩展，类似于 `java.lang.StringBuilder`。
- 通过 `PooledDataBuffer` 进行池化缓冲区和引用计数。
- 将缓冲区视为 `java.nio.ByteBuffer`、`InputStream` 或 `OutputStream`。
- 确定给定字节的索引或最后一个索引。

## `PooledDataBuffer`

正如 ByteBuffer 的 Javadoc 中所解释的，字节缓冲区可以是直接的或非直接的。直接缓冲区可能驻留在 Java 堆之外，这消除了原生 I/O 操作中的复制需求。这使得直接缓冲区特别适合通过 socket 接收和发送数据，但它们的创建和释放成本也更高，从而产生了池化缓冲区的想法。

`PooledDataBuffer` 是 `DataBuffer` 的一个扩展，它有助于引用计数，这对于字节缓冲区池化至关重要。它是如何工作的？当分配一个 `PooledDataBuffer` 时，引用计数为 1。调用 `retain()` 会增加计数，而调用 `release()` 会减少计数。只要计数大于 0，缓冲区就保证不会被释放。当计数减少到 0 时，池化缓冲区可以被释放，这实际上意味着为缓冲区保留的内存被返还给内存池。

注意，在大多数情况下，最好不要直接操作 `PooledDataBuffer`，而是使用 `DataBufferUtils` 中的便利方法，这些方法仅当 `DataBuffer` 是 `PooledDataBuffer` 的实例时才对其应用 release 或 retain 操作。

## `DataBufferUtils`

`DataBufferUtils` 提供了许多操作数据缓冲区的工具方法：

- 将数据缓冲区流连接成一个单独的缓冲区，如果底层字节缓冲区 API 支持，可能实现零复制，例如通过组合缓冲区。
- 将 `InputStream` 或 NIO `Channel` 转换为 `Flux<DataBuffer>`，反之，将 `Publisher<DataBuffer>` 转换为 `OutputStream` 或 NIO `Channel`。
- 如果缓冲区是 `PooledDataBuffer` 的实例，则释放或 retain `DataBuffer` 的方法。
- 从字节流中跳过或获取指定字节数。

## 编解码器

`org.springframework.core.codec` 包提供以下策略接口：

- `Encoder` 用于将 `Publisher<T>` 编码为数据缓冲区流。
- `Decoder` 用于将 `Publisher<DataBuffer>` 解码为更高级别的对象流。

`spring-core` 模块提供了 `byte[]`、`ByteBuffer`、`DataBuffer`、`Resource` 和 `String` 的编码器和解码器实现。`spring-web` 模块增加了 Jackson JSON、Jackson Smile、JAXB2、Protocol Buffers 以及其他编码器和解码器。详见 WebFlux 部分的编解码器。

## 使用 `DataBuffer`

使用数据缓冲区时，必须特别注意确保缓冲区被释放，因为它们可能是池化的。我们将使用编解码器来说明这一点，但这些概念更具普遍性。让我们看看编解码器内部必须如何管理数据缓冲区。

`Decoder` 是最后一个读取输入数据缓冲区的，在创建更高级别对象之前，因此它必须如下释放它们：

1. 如果 `Decoder` 只是简单地读取每个输入缓冲区并准备立即释放它，它可以通过 `DataBufferUtils.release(dataBuffer)` 来做到。
2. 如果 `Decoder` 使用 `Flux` 或 `Mono` 运算符，如 `flatMap`、`reduce` 等，这些运算符在内部预取和缓存数据项，或者使用 `filter`、`skip` 等丢弃某些项的运算符，则必须在组合链中添加 `doOnDiscard(DataBuffer.class, DataBufferUtils::release)`，以确保这些缓冲区在被丢弃之前被释放，即使是由于错误或取消信号导致。
3. 如果 `Decoder` 以任何其他方式持有一个或多个数据缓冲区，它必须确保在完全读取后释放它们，或者在缓存的数据缓冲区被读取和释放之前发生错误或取消信号时释放它们。

注意，`DataBufferUtils#join` 提供了一种安全高效的方式将数据缓冲区流聚合成一个单独的数据缓冲区。类似地，`skipUntilByteCount` 和 `takeUntilByteCount` 是供解码器使用的其他安全方法。

`Encoder` 分配数据缓冲区供其他人读取（和释放）。因此 `Encoder` 不需要做太多工作。但是，如果在用数据填充缓冲区时发生序列化错误，`Encoder` 必须注意释放该数据缓冲区。例如：

```java
DataBuffer buffer = factory.allocateBuffer();
boolean release = true;
try {
    // serialize and populate buffer..
    release = false;
}
finally {
    if (release) {
        DataBufferUtils.release(buffer);
    }
}
return buffer;
```

`Encoder` 的消费者负责释放其接收的数据缓冲区。在 WebFlux 应用中，`Encoder` 的输出用于写入 HTTP 服务器响应或客户端 HTTP 请求，在这种情况下，释放数据缓冲区的责任在于写入服务器响应或客户端请求的代码。

注意，在 Netty 上运行时，有一些用于排查缓冲区泄露的调试选项。

---

*来源：https://docs.springframework.org.cn/spring-framework/reference/core/databuffer-codec.html*
