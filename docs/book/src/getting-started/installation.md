# Installation
# 安装

<<<<<<< Current (Your changes)
Nexus Framework 目前处于 **Phase 1** 阶段，`nexus-runtime` 模块已经可用。本文档介绍如何安装和使用 Nexus Runtime。

Nexus Framework is currently in **Phase 1**, and the `nexus-runtime` module is available. This document explains how to install and use Nexus Runtime.

## Requirements / 系统要求

### Supported Platforms / 支持的平台

| Platform / 平台 | Driver / 驱动 | Status / 状态 |
|-----------------|---------------|---------------|
| Linux (5.1+) | io-uring | ✅ Fully Supported / 完全支持 |
| Linux (older) | epoll | ✅ Fully Supported / 完全支持 |
| macOS | kqueue | ✅ Fully Supported / 完全支持 |
| FreeBSD | kqueue | ✅ Fully Supported / 完全支持 |
| NetBSD | kqueue | ✅ Fully Supported / 完全支持 |
| OpenBSD | kqueue | ✅ Fully Supported / 完全支持 |
| Windows | ❌ | ⚠️ Not Yet Supported / 暂不支持 |

### Rust Version / Rust版本

Nexus requires Rust **1.85 or later**.

Nexus 需要 Rust **1.85 或更高版本**。

Check your Rust version:
检查你的 Rust 版本：

```bash
rustc --version
```

If you need to update Rust, use rustup:
如果需要更新 Rust，使用 rustup：

```bash
rustup update
```

## Installing Nexus Runtime / 安装 Nexus Runtime

### Add to Cargo.toml / 添加到 Cargo.toml

Add `nexus-runtime` to your `Cargo.toml` dependencies:
将 `nexus-runtime` 添加到你的 `Cargo.toml` 依赖中：

```toml
[dependencies]
nexus-runtime = "0.1.0-alpha"
```

### From Git Repository / 从 Git 仓库安装

If you want to use the latest development version:
如果你想使用最新的开发版本：

```toml
[dependencies]
nexus-runtime = { git = "https://github.com/nexus-rs/nexus", package = "nexus-runtime" }
```

### Feature Flags / 功能特性

Nexus Runtime supports optional features:
Nexus Runtime 支持可选功能：

```toml
[dependencies]
nexus-runtime = { version = "0.1.0-alpha", features = ["full"] }
```

Available features:
可用功能：

| Feature / 功能 | Description / 描述 |
|----------------|-------------------|
| `default` | Basic runtime functionality / 基本运行时功能 |
| `full` | All features enabled / 启用所有功能 |

## Verifying Installation / 验证安装

Create a simple test program to verify the installation:
创建一个简单的测试程序来验证安装：

```rust
use nexus_runtime::Runtime;

fn main() -> std::io::Result<()> {
    let mut runtime = Runtime::new()?;
    runtime.block_on(async {
        println!("Nexus Runtime is working!");
        println!("Nexus Runtime 运行正常！");
    });
    Ok(())
}
```

Run it:
运行它：

```bash
cargo run
```

If you see the output, installation is successful!
如果你看到输出，说明安装成功！

## Building from Source / 从源码构建

If you want to build Nexus from source:
如果你想从源码构建 Nexus：

```bash
# Clone the repository / 克隆仓库
git clone https://github.com/nexus-rs/nexus.git
cd nexus

# Build all crates / 构建所有crate
cargo build --workspace

# Run tests / 运行测试
cargo test --workspace

# Build with optimizations / 使用优化构建
cargo build --workspace --release
```

## Troubleshooting / 故障排除

### io-uring Not Available / io-uring 不可用

On Linux systems without io-uring support (kernel < 5.1), Nexus will automatically fall back to epoll. This is transparent and requires no configuration changes.

在没有 io-uring 支持的 Linux 系统上（内核 < 5.1），Nexus 会自动回退到 epoll。这是透明的，不需要配置更改。

### Compilation Errors / 编译错误

If you encounter compilation errors:
如果遇到编译错误：

1. **Check Rust version**: Ensure you're using Rust 1.85+
   **检查 Rust 版本**：确保你使用的是 Rust 1.85+

2. **Update dependencies**: Run `cargo update`
   **更新依赖**：运行 `cargo update`

3. **Clean build**: Run `cargo clean && cargo build`
   **清理构建**：运行 `cargo clean && cargo build`

### Platform-Specific Issues / 平台特定问题

#### Linux

- **io-uring support**: Requires kernel 5.1+ and liburing
  **io-uring 支持**：需要内核 5.1+ 和 liburing

- **epoll fallback**: Automatically used if io-uring unavailable
  **epoll 回退**：如果 io-uring 不可用则自动使用

#### macOS

- **kqueue**: Automatically used (no additional setup needed)
  **kqueue**：自动使用（无需额外设置）

## Next Steps / 下一步

Now that you have Nexus Runtime installed, you can:
现在你已经安装了 Nexus Runtime，你可以：

- Read the [Quick Start Guide](./quick-start.md) to learn basic usage
  阅读 [快速开始指南](./quick-start.md) 学习基本用法

- Explore the [Runtime Documentation](../core-concepts/runtime.md) for detailed information
  探索 [Runtime 文档](../core-concepts/runtime.md) 获取详细信息

- Check out the [API Documentation](https://docs.rs/nexus-runtime) for complete API reference
  查看 [API 文档](https://docs.rs/nexus-runtime) 获取完整的 API 参考
=======
## Requirements / 系统要求

### Rust Toolchain / Rust 工具链

Nexus requires Rust 1.75 or later.
Nexus 需要 Rust 1.75 或更高版本。

```bash
# Install Rust / 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation / 验证安装
rustc --version
cargo --version
```

### Platform Support / 平台支持

| Platform | Status | I/O Driver |
|----------|--------|------------|
| Linux (kernel 5.1+) | ✅ Full support | io-uring |
| Linux (older kernels) | ✅ Supported | epoll |
| macOS | ✅ Supported | kqueue |
| Windows | 🔄 In progress | IOCP |

### Linux: io-uring Requirements / Linux: io-uring 要求

For best performance on Linux, ensure you have kernel 5.1+ with io-uring support:
为在 Linux 上获得最佳性能，请确保您有内核 5.1+ 并支持 io-uring：

```bash
# Check kernel version / 检查内核版本
uname -r

# For Ubuntu/Debian, install liburing-dev (optional)
# 对于 Ubuntu/Debian，安装 liburing-dev（可选）
sudo apt-get install liburing-dev
```

## Adding Nexus to Your Project / 将 Nexus 添加到项目

### Using Cargo / 使用 Cargo

Add Nexus to your `Cargo.toml`:
将 Nexus 添加到您的 `Cargo.toml`：

```toml
[dependencies]
nexus = "0.1.0-alpha"
```

Or use cargo-add:
或使用 cargo-add：

```bash
cargo add nexus
```

### Feature Flags / 功能标志

Nexus provides several optional features:
Nexus 提供多个可选功能：

```toml
[dependencies]
nexus = { version = "0.1.0-alpha", features = ["full"] }
```

| Feature | Description | Default |
|---------|-------------|---------|
| `runtime` | Async runtime | ✅ |
| `http` | HTTP server/client | ✅ |
| `router` | Request routing | ✅ |
| `json` | JSON serialization | ✅ |
| `middleware` | Built-in middleware | ✅ |
| `web3` | Web3/blockchain support | ❌ |
| `full` | All features | ❌ |

### Using Individual Crates / 使用单独的 Crate

You can also use individual crates for more control:
您也可以使用单独的 crate 以获得更多控制：

```toml
[dependencies]
nexus-runtime = "0.1.0-alpha"
nexus-http = "0.1.0-alpha"
nexus-router = "0.1.0-alpha"
nexus-middleware = "0.1.0-alpha"
```

## Building from Source / 从源码构建

```bash
# Clone the repository / 克隆仓库
git clone https://github.com/nexus-framework/nexus.git
cd nexus

# Build all crates / 构建所有 crate
cargo build --workspace

# Build with optimizations / 优化构建
cargo build --workspace --release

# Run tests / 运行测试
cargo test --workspace

# Run examples / 运行示例
cargo run --example hello_world
```

## Verifying Installation / 验证安装

Create a simple test project:
创建一个简单的测试项目：

```bash
cargo new hello-nexus
cd hello-nexus
```

Edit `Cargo.toml`:
编辑 `Cargo.toml`：

```toml
[package]
name = "hello-nexus"
version = "0.1.0"
edition = "2021"

[dependencies]
nexus-runtime = "0.1.0-alpha"
```

Edit `src/main.rs`:
编辑 `src/main.rs`：

```rust
use nexus_runtime::Runtime;

fn main() -> std::io::Result<()> {
    let runtime = Runtime::new()?;
    
    runtime.block_on(async {
        println!("Nexus is working!");
    });
    
    Ok(())
}
```

Run the project:
运行项目：

```bash
cargo run
```

If you see "Nexus is working!", the installation is successful!
如果看到 "Nexus is working!"，安装成功！

## IDE Support / IDE 支持

### VS Code / RustRover

Nexus works with any Rust IDE. Recommended extensions:
Nexus 可与任何 Rust IDE 配合使用。推荐扩展：

- **rust-analyzer**: Language server for Rust
- **Even Better TOML**: TOML file support
- **crates**: Cargo.toml dependency management

## Troubleshooting / 故障排除

### Common Issues / 常见问题

**1. io-uring not available**
```
Error: io-uring requires Linux kernel 5.1+
```
Solution: Nexus will automatically fall back to epoll. No action needed.
解决方案：Nexus 会自动回退到 epoll。无需操作。

**2. Missing liburing**
```
Error: failed to find liburing
```
Solution: Install liburing-dev or let Nexus build it from source.
解决方案：安装 liburing-dev 或让 Nexus 从源码构建。

**3. Compilation errors on Windows**
Windows support is still in development. Use WSL2 for now.
Windows 支持仍在开发中。目前请使用 WSL2。
>>>>>>> Incoming (Background Agent changes)

---

*← [Previous / 上一页](./introduction.md) | [Next / 下一页](./quick-start.md) →*
