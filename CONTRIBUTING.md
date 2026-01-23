# Contributing to Nexus Framework
# 为 Nexus 框架做贡献

Thank you for your interest in contributing to Nexus Framework! This document provides guidelines for contributing.
感谢您有兴趣为 Nexus 框架做贡献！本文档提供了贡献指南。

## Table of Contents / 目录

- [Code of Conduct / 行为准则](#code-of-conduct--行为准则)
- [Getting Started / 入门](#getting-started--入门)
- [Development Workflow / 开发工作流](#development-workflow--开发工作流)
- [Coding Standards / 编码标准](#coding-standards--编码标准)
- [Testing / 测试](#testing--测试)
- [Documentation / 文档](#documentation--文档)
- [Submitting Changes / 提交更改](#submitting-changes--提交更改)

---

## Code of Conduct / 行为准则

### Our Pledge / 我们的承诺

We are committed to providing a welcoming and inclusive environment for all contributors. We pledge to:
我们致力于为所有贡献者提供热情和包容的环境。我们承诺：

- Use welcoming and inclusive language / 使用热情和包容的语言
- Be respectful of differing viewpoints and experiences / 尊重不同的观点和经验
- Gracefully accept constructive criticism / 优雅地接受建设性批评
- Focus on what is best for the community / 专注于对社区最有利的事情
- Show empathy towards other community members / 对其他社区成员表示同情

### Our Standards / 我们的标准

Examples of behavior that contributes to a positive environment:
有助于营造积极环境的行为示例：

- Using welcoming and inclusive language / 使用热情和包容的语言
- Being respectful of differing viewpoints and experiences / 尊重不同的观点和经验
- Gracefully accepting constructive criticism / 优雅地接受建设性批评
- Focusing on what is best for the community / 专注于对社区最有利的事情
- Showing empathy towards other community members / 对其他社区成员表示同情

---

## Getting Started / 入门

### Prerequisites / 先决条件

- Rust 1.75 or later / Rust 1.75 或更高版本
- Git / Git
- A GitHub account / GitHub 账户

### Setting Up Development Environment / 设置开发环境

```bash
# Clone the repository / 克隆仓库
git clone https://github.com/nexus-framework/nexus.git
cd nexus

# Install Rust toolchain / 安装 Rust 工具链
rustup install stable
rustup default stable

# Install development tools / 安装开发工具
cargo install cargo-watch
cargo install cargo-edit
```

### Building the Project / 构建项目

```bash
# Build all crates / 构建所有 crate
cargo build --workspace

# Run tests / 运行测试
cargo test --workspace

# Check formatting / 检查格式化
cargo fmt --all -- --check

# Run linter / 运行 linter
cargo clippy --workspace --all-targets -- -D warnings
```

---

## Development Workflow / 开发工作流

### 1. Find an Issue / 查找问题

Look for issues labeled `good first issue` or `help wanted` in our [issue tracker](https://github.com/nexus-framework/nexus/issues).
在我们的[问题跟踪器](https://github.com/nexus-framework/nexus/issues)中查找标记为 `good first issue` 或 `help wanted` 的问题。

### 2. Create a Branch / 创建分支

```bash
# From main branch / 从 main 分支
git checkout main
git pull origin main

# Create a feature branch / 创建功能分支
git checkout -b feature/your-feature-name
# or / 或者
git checkout -b fix/your-bug-fix
```

### 3. Make Changes / 进行更改

- Write code following our [Coding Standards](#coding-standards--编码标准) / 遵循我们的[编码标准]编写代码
- Add tests for your changes / 为您的更改添加测试
- Update documentation as needed / 根据需要更新文档
- Ensure all tests pass / 确保所有测试通过

### 4. Commit Your Changes / 提交您的更改

```bash
git add .
git commit -m "feat: add xyz feature"
```

#### Commit Message Format / 提交消息格式

We follow [Conventional Commits](https://www.conventionalcommits.org/):
我们遵循 [Conventional Commits](https://www.conventionalcommits.org/)：

```
<type>: <subject>

<body>

<footer>
```

**Types / 类型:**
- `feat`: New feature / 新功能
- `fix`: Bug fix / 错误修复
- `docs`: Documentation changes / 文档更改
- `style`: Code style changes (formatting) / 代码风格更改（格式化）
- `refactor`: Code refactoring / 代码重构
- `test`: Adding or updating tests / 添加或更新测试
- `chore`: Maintenance tasks / 维护任务

**Example / 示例:**
```
feat(runtime): add io-uring support for Linux

Implement io-uring-based I/O driver for Linux systems
with fallback to epoll for older kernels.

# 实现基于 io-uring 的 Linux 系统 I/O 驱动程序
# 对于较旧的内核回退到 epoll

Closes #123
```

### 5. Push and Create PR / 推送并创建 PR

```bash
git push origin feature/your-feature-name
```

Then create a pull request on GitHub with:
然后在 GitHub 上创建拉取请求：

- Clear description of changes / 清晰的更改描述
- Reference to related issues / 对相关问题的引用
- Checklist of items completed / 已完成项目的检查清单

---

## Coding Standards / 编码标准

### Language / 语言

- All public APIs must have bilingual documentation (English and Chinese) / 所有公共 API 必须有双语文档（英文和中文）
- Use English for code and variable names / 代码和变量名使用英文
- Use Chinese for user-facing messages where appropriate / 用户面向的消息在适当情况下使用中文

### Formatting / 格式化

```bash
# Format code / 格式化代码
cargo fmt --all
```

Our `rustfmt.toml` configuration enforces consistent formatting.
我们的 `rustfmt.toml` 配置强制执行一致的格式化。

### Linting / 检查

```bash
# Run clippy / 运行 clippy
cargo clippy --workspace --all-targets -- -D warnings
```

We treat clippy warnings as errors. Fix all warnings before submitting.
我们将 clippy 警告视为错误。在提交之前修复所有警告。

### Documentation Comments / 文档注释

```rust
//! Module level documentation / 模块级文档
//!
//! This module provides... / 本模块提供...

/// Function summary / 函数摘要
/// 函数摘要（中文）
///
/// # Arguments / 参数
///
/// * `arg1` - Description / 描述
///
/// # Returns / 返回值
///
/// Description of return value / 返回值描述
///
/// # Examples / 示例
///
/// ```
/// let result = function(arg1);
/// assert_eq!(result, expected);
/// ```
pub fn function(arg1: Type) -> ReturnType {
    // Implementation / 实现
}
```

---

## Testing / 测试

### Test Requirements / 测试要求

- Unit tests for all public functions / 所有公共函数的单元测试
- Integration tests for complex features / 复杂功能的集成测试
- Minimum 80% code coverage / 最低 80% 代码覆盖率

### Running Tests / 运行测试

```bash
# Run all tests / 运行所有测试
cargo test --workspace

# Run tests with output / 运行测试并显示输出
cargo test --workspace -- --nocapture

# Run specific test / 运行特定测试
cargo test test_name

# Run tests in a specific crate / 在特定 crate 中运行测试
cargo test -p nexus-runtime
```

### Writing Tests / 编写测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let result = function_under_test();
        assert_eq!(result, expected);
    }
}
```

---

## Documentation / 文档

### Code Documentation / 代码文档

- All public items must have documentation comments / 所有公共项必须有文档注释
- Include examples for complex APIs / 为复杂的 API 包含示例
- Run `cargo doc` to check documentation / 运行 `cargo doc` 检查文档

### Book Documentation / 书籍文档

The project uses [mdBook](https://rust-lang.github.io/mdBook/) for documentation.
项目使用 [mdBook](https://rust-lang.github.io/mdBook/) 进行文档编写。

```bash
# Serve book locally / 在本地提供书籍
mdbook serve docs/book

# Build book / 构建书籍
mdbook build docs/book
```

---

## Submitting Changes / 提交更改

### Pull Request Checklist / 拉取请求检查清单

Before submitting your PR, ensure:
在提交您的 PR 之前，请确保：

- [ ] All tests pass / 所有测试通过
- [ ] Code is formatted (`cargo fmt`) / 代码已格式化
- [ ] No clippy warnings / 没有 clippy 警告
- [ ] Documentation is updated / 文档已更新
- [ ] Commit messages follow conventions / 提交消息遵循约定
- [ ] PR description is clear / PR 描述清晰

### Review Process / 审查流程

1. Automated checks must pass / 自动检查必须通过
2. At least one maintainer approval / 至少一位维护者批准
3. All review comments addressed / 所有审查意见已处理
4. CI/CD pipeline passes / CI/CD 流水线通过

---

## Getting Help / 获取帮助

- **GitHub Issues**: [Report bugs or request features](https://github.com/nexus-framework/nexus/issues) / [报告错误或请求功能]
- **Discussions**: [Ask questions or discuss ideas](https://github.com/nexus-framework/nexus/discussions) / [提问或讨论想法]
- **Discord**: Join our community server / 加入我们的社区服务器

---

Thank you for contributing to Nexus Framework! 🎉
感谢您为 Nexus 框架做出贡献！🎉
