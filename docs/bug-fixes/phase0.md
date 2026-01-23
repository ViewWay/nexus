# Bug Fix Log - Phase 0
# Bug 修复日志 - 第0阶段

This document records all compilation errors and fixes encountered during Phase 0 implementation.
本文档记录了第0阶段实施期间遇到的所有编译错误和修复。

---

## Bug #001: `panic = "abort"` in workspace profile
## Bug #001: 工作区配置文件中的 `panic = "abort"`

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: `panic` may not be specified in the `[profile.*]` section of the workspace manifest
```

**Location / 位置**: `Cargo.toml` (root workspace)

**Cause / 原因**: The `panic = "abort"` setting cannot be specified at workspace level in `Cargo.toml`.

**Fix / 修复**: Removed `panic = "abort"` from `[profile.release]` in workspace root `Cargo.toml`.

**Files Modified / 修改的文件**:
- `/Cargo.toml`

---

## Bug #002: `alloy` optional workspace dependency
## Bug #002: `alloy` 可选工作区依赖

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: optional dependencies in workspaces are not allowed
```

**Location / 位置**: `Cargo.toml` (root workspace)

**Cause / 原因**: Optional workspace dependencies are not supported by Cargo.

**Fix / 修复**: Removed `alloy` from workspace `[dependencies]` section and defined it directly in `nexus-web3/Cargo.toml` with optional feature.

**Files Modified / 修改的文件**:
- `/Cargo.toml`
- `/crates/nexus-web3/Cargo.toml`

---

## Bug #003: Binary target name conflict in examples
## Bug #003: 示例中的二进制目标名称冲突

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: default `lib` targets are conflicting
```

**Location / 位置**: `examples/Cargo.toml`

**Cause / 原因**: Package name was same as binary target name.

**Fix / 修复**: Changed package name from `"examples"` to `"nexus-examples"` and defined explicit binary targets.

**Files Modified / 修改的文件**:
- `/examples/Cargo.toml`

---

## Bug #004: Missing benchmark files
## Bug #004: 缺失的基准测试文件

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: failed to read bench file
```

**Location / 位置**: Multiple crates

**Cause / 原因**: `[[bench]]` sections declared without corresponding files.

**Fix / 修复**: Removed `[[bench]]` sections from all `Cargo.toml` files. Added comments that benchmarks will be added in appropriate phases.

**Files Modified / 修改的文件**:
- `/crates/nexus-runtime/Cargo.toml`
- `/crates/nexus-core/Cargo.toml`
- `/crates/nexus-http/Cargo.toml`

---

## Bug #005: `path-prefix` dependency not found
## Bug #005: 找不到 `path-prefix` 依赖

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: failed to select a version for `path-prefix`
```

**Location / 位置**: `crates/nexus-router/Cargo.toml`

**Cause / 原因**: `path-prefix` crate does not exist in the registry.

**Fix / 修复**: Removed `path-prefix = "0.1"` from dependencies.

**Files Modified / 修改的文件**:
- `/crates/nexus-router/Cargo.toml`

---

## Bug #006: Missing module files
## Bug #006: 缺失的模块文件

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: failed to find module files
```

**Location / 位置**: Multiple crates

**Cause / 原因**: Module files declared in `lib.rs` did not exist.

**Fix / 修复**: Created all placeholder module files with TODO comments indicating which phase they will be implemented.

**Files Created / 创建的文件**:
- `/crates/nexus-runtime/src/driver.rs`
- `/crates/nexus-runtime/src/io.rs`
- `/crates/nexus-runtime/src/task.rs`
- `/crates/nexus-runtime/src/time.rs`
- `/crates/nexus-core/src/error.rs`
- `/crates/nexus-core/src/extension.rs`
- `/crates/nexus-http/src/body.rs`
- `/crates/nexus-http/src/server.rs`
- `/crates/nexus-router/src/router.rs`
- `/crates/nexus-router/src/params.rs`
- `/crates/nexus-extractors/src/*.rs` (all extractor modules)
- `/crates/nexus-response/src/*.rs` (all response modules)
- `/crates/nexus-middleware/src/*.rs` (all middleware modules)
- `/crates/nexus-resilience/src/*.rs` (all resilience modules)
- `/crates/nexus-observability/src/*.rs` (all observability modules)
- `/crates/nexus-web3/src/*.rs` (all web3 modules)
- `/crates/nexus-macros/src/*.rs` (all macro modules)

---

## Bug #007: `await` is a reserved keyword
## Bug #007: `await` 是保留关键字

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: expected identifier, found keyword `await`
```

**Location / 位置**: `crates/nexus-runtime/src/task.rs`

**Cause / 原因**: `await` is a reserved Rust keyword and cannot be used as a method name.

**Fix / 修复**: Renamed `JoinHandle::await()` method to `JoinHandle::wait()`.

**Files Modified / 修改的文件**:
- `/crates/nexus-runtime/src/task.rs`

---

## Bug #008: Duplicate `Driver` definition
## Bug #008: 重复的 `Driver` 定义

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: duplicate definitions
```

**Location / 位置**: `crates/nexus-runtime/src/driver.rs`

**Cause / 原因**: Both a trait and type alias with the same name `Driver` were defined.

**Fix / 修复**: Removed duplicate type alias and made trait public directly.

**Files Modified / 修改的文件**:
- `/crates/nexus-runtime/src/driver.rs`

---

## Bug #009: Doc comment format errors (multiple files)
## Bug #009: 文档注释格式错误（多个文件）

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: expected item after doc comment
```

**Location / 位置**:
- `/crates/nexus-observability/src/log.rs`
- `/crates/nexus-resilience/src/timeout.rs`
- `/crates/nexus-resilience/src/discovery.rs`
- `/crates/nexus-macros/src/handler.rs`

**Cause / 原因**: Module-level doc comments used `///` instead of `//!`.

**Fix / 修复**: Changed `///` to `//!` for module-level documentation.

**Files Modified / 修改的文件**:
- `/crates/nexus-observability/src/log.rs`
- `/crates/nexus-resilience/src/timeout.rs`
- `/crates/nexus-resilience/src/discovery.rs`
- `/crates/nexus-macros/src/handler.rs`

---

## Bug #010: `Request` missing generic parameter
## Bug #010: `Request` 缺少泛型参数

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: missing generics for struct `http::Request`
```

**Location / 位置**: `crates/nexus-middleware/src/middleware.rs`

**Cause / 原因**: `http::Request` requires a generic parameter for the body type.

**Fix / 修复**: Changed `Request` to `Request<()>` in the `Middleware::call` method signature.

**Files Modified / 修改的文件**:
- `/crates/nexus-middleware/src/middleware.rs`

---

## Bug #011: Proc-macro crate structure violations
## Bug #011: Proc-macro crate 结构违规

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: `proc-macro` crate types currently cannot export any items other than functions
error: functions tagged with `#[proc_macro_derive]` must currently reside in the root of the crate
```

**Location / 位置**: `crates/nexus-macros/src/`

**Cause / 原因**: Proc-macro crates have strict requirements - only macro functions can be exported, and they must be at the crate root.

**Fix / 修复**: Consolidated all macro functions directly into `lib.rs` and removed the `handler.rs` and `derive.rs` module files.

**Files Modified / 修改的文件**:
- `/crates/nexus-macros/src/lib.rs`
- `/crates/nexus-macros/src/handler.rs` (removed)
- `/crates/nexus-macros/src/derive.rs` (removed)

---

## Bug #012: Missing `Path` and `Query` types in extractors
## Bug #012: Extractors 中缺失 `Path` 和 `Query` 类型

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: unresolved imports `path::Path`, `query::Query`
```

**Location / 位置**: `crates/nexus-extractors/src/lib.rs`

**Cause / 原因**: The `path.rs` and `query.rs` files were empty and didn't define the expected types.

**Fix / 修复**: Added placeholder `Path<T>` and `Query<T>` struct definitions with `PhantomData`.

**Files Modified / 修改的文件**:
- `/crates/nexus-extractors/src/path.rs`
- `/crates/nexus-extractors/src/query.rs`

---

## Bug #013: Missing `Transaction` and `TransactionBuilder` types
## Bug #013: 缺失 `Transaction` 和 `TransactionBuilder` 类型

**Date / 日期**: 2025-01-23

**Error / 错误**:
```
error: unresolved imports `tx::Transaction`, `tx::TransactionBuilder`
```

**Location / 位置**: `crates/nexus-web3/src/lib.rs`

**Cause / 原因**: The `tx.rs` module only defined `TxHash` but `lib.rs` tried to export additional types.

**Fix / 修复**: Added placeholder `Transaction` and `TransactionBuilder` struct definitions in `tx.rs`.

**Files Modified / 修改的文件**:
- `/crates/nexus-web3/src/tx.rs`

---

## Summary / 总结

**Total Bugs Fixed / 总修复 Bug 数**: 13

**Categories / 类别**:
- Configuration errors: 5 (配置错误)
- Missing files: 6 (缺失文件)
- Syntax errors: 2 (语法错误)

**Workspace Status / 工作区状态**: ✅ Compiling successfully with only minor warnings / ✅ 仅带有轻微警告的编译成功
