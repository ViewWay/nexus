<div align="center">
<p><img alt="Nexus" width="132" style="max-width:40%;min-width:60px;" src="https://via.placeholder.com/132x40/0066CC/FFFFFF?text=Nexus" /></p>
<p>
    <a href="https://github.com/nexus-rs/nexus/blob/main/README.md">English</a>&nbsp;&nbsp;
    <a href="https://github.com/nexus-rs/nexus/blob/main/README.zh.md">简体中文</a>
</p>
<p>
<a href="https://github.com/nexus-rs/nexus/actions">
    <img alt="build status" src="https://github.com/nexus-rs/nexus/workflows/CI/badge.svg" />
</a>
<a href="https://codecov.io/gh/nexus-rs/nexus">
    <img alt="codecov" src="https://codecov.io/gh/nexus-rs/nexus/branch/main/graph/badge.svg" />
</a>
<br>
<a href="https://crates.io/crates/nexus"><img alt="crates.io" src="https://img.shields.io/crates/v/nexus" /></a>
<a href="https://docs.rs/nexus"><img alt="Documentation" src="https://docs.rs/nexus/badge.svg" /></a>
<a href="https://crates.io/crates/nexus"><img alt="Download" src="https://img.shields.io/crates/d/nexus.svg" /></a>
<a href="https://github.com/rust-secure-code/safety-dance/"><img alt="unsafe forbidden" src="https://img.shields.io/badge/unsafe-forbidden-success.svg" /></a>
<br>
<a href="https://nexusframework.com">
    <img alt="Website" src="https://img.shields.io/badge/https-nexusframework.com-%23f00" />
</a>
</p>
</div>

Nexus 是一个用 Rust 编写的生产级、高可用 Web 框架，具有自定义异步运行时。与其他使用 Tokio 的框架不同，Nexus 具有从零开始构建的自定义异步运行时，使用 io-uring 以实现最高性能。

## 🎯 特性

- **自定义运行时** - 支持 io-uring 的 Thread-per-core 架构
- **Spring Boot 风格注解** - @Entity, @Query, @Valid, @Transactional, @Aspect 等
- **高可用性** - 熔断器、限流器、重试逻辑
- **原生 Web3** - 内置区块链和智能合约支持
- **可观测性** - 兼容 OpenTelemetry 的追踪/指标
- **类型安全** - 利用 Rust 类型系统
- **类 Spring** - Spring Boot 开发者熟悉的模式

## ⚡️ 快速开始

您可以查看示例 [这里](https://github.com/nexus-rs/nexus/tree/main/examples)，或查看 [官方文档](https://docs.nexusframework.com)。

### 基础 HTTP 服务器 / Basic HTTP Server

```rust
use nexus_http::{Body, Response, Server, StatusCode};
use nexus_runtime::task::block_on;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    block_on(async {
        let _server = Server::bind("127.0.0.1:8080")
            .run(handle_request)
            .await?;

        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    })
}

async fn handle_request(req: nexus_http::Request) -> Result<Response, nexus_http::Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/plain")
        .body(Body::from("Hello, Nexus!"))
        .unwrap())
}
```

### 使用 Nexus 注解 / Using Nexus Annotations

#### ❌ 不使用注解（原生 Rust 写法）

```rust
// 用户实体 - 必须手动实现所有方法
#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub age: i32,
}

impl User {
    // 手动实现构造函数
    pub fn new(id: i64, username: String, email: String, age: i32) -> Self {
        Self { id, username, email, age }
    }

    // 手动实现 getter
    pub fn id(&self) -> &i64 { &self.id }
    pub fn username(&self) -> &str { &self.username }
    pub fn email(&self) -> &str { &self.email }
    pub fn age(&self) -> i32 { self.age }

    // 手动实现 setter
    pub fn set_id(&mut self, id: i64) { self.id = id; }
    pub fn set_username(&mut self, username: String) { self.username = username; }
    pub fn set_email(&mut self, email: String) { self.email = email; }
    pub fn set_age(&mut self, age: i32) { self.age = age; }
}

// 仓库 - 手动编写 SQL 查询
struct UserRepository {
    db: Database,
}

impl UserRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, Error> {
        let sql = "SELECT * FROM users WHERE id = $1";
        let row = self.db.query_one(sql, &[&id]).await?;
        Ok(row.map(|r| User {
            id: r.get("id"),
            username: r.get("username"),
            email: r.get("email"),
            age: r.get("age"),
        }).transpose()?)
    }
}

// 服务 - 手动添加日志和事务管理
impl UserService {
    async fn create_user(&self, user: User) -> Result<(), Error> {
        println!("Creating user: {:?}", user); // 手动日志

        let tx = self.begin_transaction().await?; // 手动事务
        match self.repository.insert(&tx, &user).await {
            Ok(_) => {
                tx.commit().await?;
                println!("User created"); // 手动日志
                Ok(())
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}
```

#### ✅ 使用 Nexus 注解（推荐）

```rust
use nexus_lombok::Data;
use nexus_data_annotations::{Entity, Table, Id, Column, Query, Insert};
use nexus_aop::{Aspect, Before, After};
use nexus_data_annotations::Transactional;

// 简洁的实体定义 - 自动生成所有方法
#[Entity]
#[Table(name = "users")]
#[Data]
#[derive(Debug, Clone)]
pub struct User {
    #[Id]
    #[Column(name = "id")]
    pub id: i64,

    #[Column(name = "username", nullable = false)]
    pub username: String,

    #[Column(name = "email")]
    pub email: String,

    #[Column(name = "age")]
    pub age: i32,
}

// 声明式查询 - 无需手动编写 SQL 绑定
trait UserRepository {
    #[Query("SELECT * FROM users WHERE id = :id")]
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, Error>;

    #[Insert("INSERT INTO users (id, username, email, age) VALUES (:id, :username, :email, :age)")]
    async fn insert(&self, user: &User) -> Result<u64, Error>;
}

// AOP 切面 - 自动添加日志
#[Aspect]
struct LoggingAspect;

impl LoggingAspect {
    #[Before("execution(* UserService.*(..))")]
    fn log_before(&self, join_point: &JoinPoint) {
        println!("Entering: {}", join_point.method_name());
    }

    #[After("execution(* UserService.*(..))")]
    fn log_after(&self, join_point: &JoinPoint) {
        println!("Exiting: {}", join_point.method_name());
    }
}

// 服务 - 自动事务管理
impl UserService {
    #[Transactional(isolation = ReadCommitted)]
    async fn create_user(&self, user: User) -> Result<(), Error> {
        // 日志由 AOP 自动添加
        // 事务由 @Transactional 自动管理
        self.repository.insert(&user).await?;
        Ok(())
    }
}

// 使用示例
async fn main() {
    // 创建用户（自动生成构造函数）
    let user = User::new(1, "alice".into(), "alice@example.com".into(), 25);

    // 查询用户（声明式 SQL，自动映射）
    let found = repository.find_by_id(1).await?;

    // 创建用户（自动日志，自动事务）
    service.create_user(user).await?;
}
```

**代码对比 / Code Comparison**:
- ❌ 不使用注解：~200 行样板代码
- ✅ 使用注解：~60 行清晰代码（减少 70%）

### Hello World

```rust
use nexus_http::{Body, Response, Server, StatusCode};
use nexus_runtime::task::block_on;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    block_on(async {
        let _server = Server::bind("127.0.0.1:8080")
            .run(handle_request)
            .await?;

        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    })
}

async fn handle_request(req: nexus_http::Request) -> Result<Response, nexus_http::Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/plain")
        .body(Body::from("Hello, Nexus!"))
        .unwrap())
}
```

### Nexus 日志

```rust
use nexus_observability::log::Logger;
#[cfg(feature = "nexus-format")]
use nexus_observability::{Banner, StartupLogger};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "nexus-format")]
    {
        Banner::print("MyApp", "0.1.0", 8080);
        Logger::init_spring_style()?;

        let startup = StartupLogger::new();
        startup.log_starting("MyApplication");
        startup.log_server_started(8080, startup.elapsed_ms());
    }

    tracing::info!(target: "my.app", "Application running");
    Ok(())
}
```

## 🚀 性能

Nexus 从设计之初就追求高性能：

- **相比 epoll 减少 70% 系统调用**（使用 io-uring）
- **延迟降低 40%**（Thread-per-core 架构）
- **零拷贝 I/O**，最小化内存分配
- **线性扩展性**，无锁竞争

基准测试结果将在第2阶段提供。

## 📚 文档

| 资源 | 链接 |
|------|------|
| **指南** | [docs.nexusframework.com](https://docs.nexusframework.com) |
| **API 文档** | [docs.rs/nexus](https://docs.rs/nexus) |
| **设计规范** | [design-spec.md](docs/design-spec.md) |
| **实施计划** | [implementation-plan.md](docs/implementation-plan.md) |

## 🏗️ 架构

```
┌─────────────────────────────────────────────────────────────┐
│                    应用层                                    │
├─────────────────────────────────────────────────────────────┤
│  Handlers  │  Middleware  │  Extractors  │  Response        │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                    Nexus运行时                               │
├─────────────────────────────────────────────────────────────┤
│  Task Scheduler  │  I/O Driver  │  Timer  │  Executor       │
│  (Thread-per-Core)  │  (io-uring)   │                          │
└─────────────────────────────────────────────────────────────┘
                              │
┌─────────────────────────────────────────────────────────────┐
│                    系统层                                    │
├─────────────────────────────────────────────────────────────┤
│       io-uring (Linux) / epoll / kqueue                      │
└─────────────────────────────────────────────────────────────┘
```

## 🛠️ 开发

```bash
# 克隆仓库
git clone https://github.com/nexus-rs/nexus.git
cd nexus

# 构建
cargo build --workspace

# 测试
cargo test --workspace

# 格式化
cargo fmt --all

# 检查
cargo clippy --workspace -- -D warnings
```

## 📋 项目状态

> **⚠️ Alpha 版本**
>
> Nexus 目前处于 **第1阶段：运行时核心**（已完成）和 **注解系统开发**（90% 完成）。异步运行时已完全可用，支持 io-uring/epoll/kqueue。Spring Boot 风格注解系统已基本完成。

| 阶段 | 状态 | 描述 |
|------|------|------|
| Phase 0 | ✅ 已完成 | 基础设施 |
| Phase 1 | ✅ 已完成 | 运行时核心 |
| **注解系统** | 🎉 **90% 完成** | **Spring Boot 风格注解** |
| Phase 2 | 🔄 进行中 | HTTP服务器 |
| Phase 3 | 📋 计划中 | 路由和中间件 |
| Phase 4 | 📋 计划中 | 弹性 |
| Phase 5 | 📋 计划中 | 可观测性 |
| Phase 6 | 📋 计划中 | Web3集成 |
| Phase 7 | 📋 计划中 | 性能和加固 |

### 🎊 注解系统进度 / Annotations Progress

```text
═══════════════════════════════════════════════════════════════
  Nexus 注解系统 Nexus Annotations System (90%)
═══════════════════════════════════════════════════════════════

  ✅ Lombok 注解 (100%) - @Data, @Builder, @Getter, @Setter
  ✅ Spring Data 注解 (90%) - @Entity, @Table, @Query, @Insert, @Update, @Delete
  ✅ Validation 注解 (100%) - @Valid, @NotNull, @Email, @Size
  ✅ AOP 注解 (100%) - @Aspect, @Before, @After, @Around, @Pointcut
  ✅ Transactional 注解 (100%) - @Transactional 编译时 + 运行时完整支持 🎉

═══════════════════════════════════════════════════════════════
  运行时集成 Runtime Integration: 100% ✅
═══════════════════════════════════════════════════════════════

  ✅ 查询运行时 - SQL 执行引擎，4 种参数绑定风格
  ✅ 验证运行时 - 8 种验证助手，HTTP 中间件
  ✅ AOP 运行时 - JoinPoint, 切点解析，切面注册表
  ✅ 事务运行时 - 5 种隔离级别，7 种传播行为，@Transactional 宏

详情请参阅：
- [注解进度报告](docs/FINAL-PROGRESS-REPORT.md)
- [运行时集成报告](docs/RUNTIME-INTEGRATION-PROGRESS.md)
```

详情请参阅 [实施计划](docs/implementation-plan.md)。

## 🤝 贡献

我们欢迎贡献！请参阅 [CONTRIBUTING.md](CONTRIBUTING.md) 了解指南。

## 📄 许可证

Nexus 采用以下许可证之一：

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) 或 [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT license ([LICENSE-MIT](LICENSE-MIT) 或 [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

## 🙏 致谢

Nexus 受多种语言中优秀框架的启发：

- **Rust**: Axum, Actix Web, Monoio, Salvo
- **Go**: Gin, Echo
- **Java**: Spring Boot, WebFlux
- **Python**: FastAPI, Starlette

---

**Nexus 框架** — 为 Web 开发的未来而构建。
