# Nexus Starter 自动装配系统设计
# Nexus Starter Auto-Configuration System Design

> 参考 Spring Boot 源码自动配置机制实现
> Based on Spring Boot source code auto-configuration mechanism

---

## 概述 / Overview

### 问题陈述

在复杂项目中，依赖管理是关键环节。手动管理依赖并不理想：

> "你在依赖管理上花费的时间越多，投入到实际开发中的时间就越少。"

Nexus Starter 正是为了解决这个问题——提供一组预定义的依赖项集合，一站式获取所有需要的组件。

### 目标

| 目标 | 描述 |
|------|------|
| **一键启动** | `#[nexus_main]` 宏自动配置所有组件 |
| **自动扫描** | 自动发现和注册 `@Component`、`@Service`、`@Controller` |
| **智能配置** | 基于条件注解的智能自动装配 |
| **开箱即用** | 生产就绪的默认配置 |

---

## 1. 自动配置核心机制 / Core Auto-Configuration Mechanism

### 1.1 自动配置流程（参考 Spring Boot）

```
┌─────────────────────────────────────────────────────────────────┐
│                     #[nexus_main] 宏展开                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  1. 加载自动配置元数据                                             │
│     └── META-INF/nexus/autoconfiguration.imports                 │
│                                                                 │
│  2. 条件评估 / Conditional Evaluation                            │
│     ├── @ConditionalOnFeature    - feature 是否启用               │
│     ├── @ConditionalOnProperty   - 属性是否存在                  │
│     ├── @ConditionalOnMissing    - Bean 是否缺失                 │
│     └── @ConditionalOnConfig     - 配置是否满足                  │
│                                                                 │
│  3. 按优先级排序 / Priority Ordering                              │
│     ├── @AutoConfigureOrder(i32)  - 配置顺序                     │
│     ├── @AutoConfigureAfter      - 在某配置之后                  │
│     └── @AutoConfigureBefore     - 在某配置之前                  │
│                                                                 │
│  4. 注册 Bean / Bean Registration                                │
│     ├── 扫描 @Component、@Service、@Repository                   │
│     ├── 处理 @Bean 定义方法                                      │
│     └── 依赖注入 / Dependency Injection                          │
│                                                                 │
│  5. 启动应用 / Application Startup                                │
│     ├── 启动 HTTP 服务器                                         │
│     ├── 初始化定时任务                                           │
│     └── 注册 shutdown hook                                       │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

### 1.2 自动配置元数据文件

**META-INF/nexus/autoconfiguration.imports**

```rust
// nexus-starter/src/resources/META-INF/nexus/autoconfiguration.imports

nexus_starter::core::CoreAutoConfiguration
nexus_starter::web::WebServerAutoConfiguration
nexus_starter::web::RouterAutoConfiguration
nexus_starter::security::SecurityAutoConfiguration
nexus_starter::security::JwtAutoConfiguration
nexus_starter::data::DataSourceAutoConfiguration
nexus_starter::data::TransactionAutoConfiguration
nexus_starter::cache::CacheAutoConfiguration
nexus_starter::schedule::ScheduleAutoConfiguration
nexus_starter::actuator::ActuatorAutoConfiguration
```

---

## 2. 条件注解系统 / Conditional Annotation System

### 2.1 核心条件注解

| 注解 | Spring 等价 | 功能 |
|------|-------------|------|
| `#[conditional_on_feature("web")]` | `@ConditionalOnClass` | feature 是否启用 |
| `#[conditional_on_property("app.cache.enabled")]` | `@ConditionalOnProperty` | 属性是否存在 |
| `#[conditional_on_missing_bean("DataSource")]` | `@ConditionalOnMissingBean` | Bean 是否缺失 |
| `#[conditional_on_config("server.port")]` | `@ConditionalOnProperty` | 配置是否存在 |
| `#[conditional_on_web_app]` | `@ConditionalOnWebApplication` | 是否为 Web 应用 |

### 2.2 条件注解使用示例

```rust
//! nexus-starter/src/cache/autoconfig.rs

use nexus_starter::{
    AutoConfiguration, AutoConfigureOrder, ConditionalOnProperty,
    ConditionalOnMissingBean, EnableConfigurationProperties,
};

#[derive(AutoConfiguration)]
#[auto_configure_order(100)]
#[conditional_on_property("app.cache.enabled")]
#[enable_configuration_properties(CacheProperties)]
pub struct CacheAutoConfiguration {
    props: CacheProperties,
}

impl CacheAutoConfiguration {
    /// 创建缓存 Bean（仅当容器中没有时）
    #[conditional_on_missing_bean("MemoryCache")]
    fn configure_cache(&self) -> MemoryCache {
        MemoryCache::builder()
            .ttl(self.props.ttl)
            .max_size(self.props.max_size)
            .build()
    }
}

#[derive(EnableConfigurationProperties)]
#[config(prefix = "app.cache")]
pub struct CacheProperties {
    #[config(default = "600")]
    ttl: u64,

    #[config(default = "10000")]
    max_size: usize,
}
```

### 2.3 组合条件

```rust
//! 嵌套条件 - 所有条件都满足

#[all(
    conditional_on_property("app.security.enabled"),
    conditional_on_feature("jwt"),
    conditional_on_missing_bean("JwtTokenProvider")
)]
pub struct JwtAutoConfiguration;

//! 或条件 - 任一条件满足

#[any(
    conditional_on_property("app.security.type", value = "jwt"),
    conditional_on_property("app.security.type", value = "oauth2")
)]
pub struct SecurityAutoConfiguration;

//! 非条件

#[not(conditional_on_feature("dev"))]
pub struct ProductionAutoConfiguration;
```

---

## 3. Starter Crate 结构 / Structure

```
crates/
└── nexus-starter/                      # 统一 Starter
    ├── src/
    │   ├── lib.rs                      # 顶层入口
    │   ├── prelude.rs                  # 预导入模块
    │   │
    │   ├── core/                       # 核心自动配置
    │   │   ├── mod.rs
    │   │   ├── autoconfig.rs           # AutoConfiguration trait
    │   │   ├── condition.rs            # 条件注解实现
    │   │   ├── scanner.rs              # 组件扫描器
    │   │   └── container.rs            # DI 容器
    │   │
    │   ├── config/                     # 配置管理
    │   │   ├── mod.rs
    │   │   ├── loader.rs               # 配置加载器
    │   │   └── properties.rs           # ConfigurationProperties
    │   │
    │   ├── web/                        # Web 自动配置
    │   │   ├── mod.rs
    │   │   ├── server.rs               # ServerAutoConfiguration
    │   │   └── router.rs               # RouterAutoConfiguration
    │   │
    │   ├── security/                   # Security 自动配置
    │   │   ├── mod.rs
    │   │   ├── auth.rs                 # SecurityAutoConfiguration
    │   │   └── jwt.rs                  # JwtAutoConfiguration
    │   │
    │   ├── data/                       # Data 自动配置
    │   │   ├── mod.rs
    │   │   ├── datasource.rs           # DataSourceAutoConfiguration
    │   │   └── transaction.rs          # TransactionAutoConfiguration
    │   │
    │   ├── cache/                      # Cache 自动配置
    │   │   ├── mod.rs
    │   │   └── memory.rs               # CacheAutoConfiguration
    │   │
    │   ├── schedule/                   # Schedule 自动配置
    │   │   ├── mod.rs
    │   │   └── scheduler.rs            # ScheduleAutoConfiguration
    │   │
    │   └── actuator/                   # Actuator 自动配置
    │       ├── mod.rs
    │       └── endpoints.rs            # ActuatorAutoConfiguration
    │
    ├── resources/
    │   └── META-INF/
    │       └── nexus/
    │           └── autoconfiguration.imports
    │
    └── Cargo.toml
```

### 3.1 Cargo.toml Feature 设计

```toml
[package]
name = "nexus-starter"
version = "0.1.0"
edition = "2024"

[features]
default = ["core", "web"]

# 核心功能（始终启用）
core = ["nexus-macros", "nexus-config"]

# Web 服务器
web = ["nexus-http", "nexus-router", "nexus-middleware"]

# 安全
security = ["nexus-security", "web"]

# 数据访问
data = ["nexus-data-rdbc", "nexus-tx"]

# 缓存
cache = ["nexus-cache"]

# 定时任务
schedule = ["nexus-schedule"]

# 监控端点
actuator = ["nexus-actuator"]

# 全功能
full = ["web", "security", "data", "cache", "schedule", "actuator"]

# 测试
test = ["full", "nexus-test"]

[dependencies]
# 框架核心
nexus = { path = "../nexus", default-features = false }
nexus-macros = { path = "../nexus-macros" }
nexus-config = { path = "../nexus-config", optional = true }

# Web
nexus-http = { path = "../nexus-http", optional = true }
nexus-router = { path = "../nexus-router", optional = true }
nexus-middleware = { path = "../nexus-middleware", optional = true }

# 其他模块...
nexus-security = { path = "../nexus-security", optional = true }
nexus-data-rdbc = { path = "../nexus-data-rdbc", optional = true }
nexus-cache = { path = "../nexus-cache", optional = true }
nexus-schedule = { path = "../nexus-schedule", optional = true }
nexus-actuator = { path = "../nexus-actuator", optional = true }

# 外部依赖
serde = { workspace = true }
serde_json = { workspace = true }
tokio = { workspace = true }
```

---

## 4. AutoConfiguration Trait 设计

### 4.1 核心 Trait

```rust
//! nexus-starter/src/core/autoconfig.rs

/// 自动配置 trait
/// Auto-configuration trait
///
/// 参考 Spring Boot 的 @AutoConfiguration 注解
/// Based on Spring Boot's @AutoConfiguration annotation
pub trait AutoConfiguration: Send + Sync {
    /// 配置名称（用于日志和调试）
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }

    /// 配置优先级（数字越小优先级越高）
    fn order(&self) -> i32 {
        0
    }

    /// 条件检查（返回 false 则跳过此配置）
    fn condition(&self) -> bool {
        true
    }

    /// 执行自动配置
    fn configure(&self, ctx: &mut ApplicationContext) -> Result<(), Error>;

    /// 应该在哪些配置之后执行
    fn after(&self) -> &[&'static str] {
        &[]
    }

    /// 应该在哪些配置之前执行
    fn before(&self) -> &[&'static str] {
        &[]
    }
}

/// 自动配置宏（派生实现）
#[proc_macro_derive(AutoConfiguration, attributes(
    auto_configure_order,
    conditional_on_property,
    conditional_on_missing_bean,
    conditional_on_feature,
    enable_configuration_properties,
))]
pub fn derive_auto_configuration(input: TokenStream) -> TokenStream {
    // 实现 AutoConfiguration trait
}
```

### 4.2 应用上下文

```rust
//! nexus-starter/src/core/container.rs

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

/// 应用上下文（类似 Spring ApplicationContext）
/// Application context (similar to Spring ApplicationContext)
pub struct ApplicationContext {
    /// 单例 Bean 容器
    singletons: HashMap<TypeId, Box<dyn Any + Send + Sync>>,

    /// 命名 Bean 容器
    named_beans: HashMap<String, Box<dyn Any + Send + Sync>>,

    /// 配置属性
    properties: ConfigurationProperties,

    /// 自动配置注册表
    auto_configurations: Vec<Box<dyn AutoConfiguration>>,
}

impl ApplicationContext {
    /// 获取 Bean（按类型）
    pub fn get_bean<T: 'static>(&self) -> Option<Arc<T>> {
        self.singletons
            .get(&TypeId::of::<T>())
            .and_then(|b| b.downcast_ref::<T>())
            .map(|b| Arc::new(b.clone()))
    }

    /// 获取 Bean（按名称）
    pub fn get_bean_by_name<T: 'static>(&self, name: &str) -> Option<Arc<T>> {
        self.named_beans
            .get(name)
            .and_then(|b| b.downcast_ref::<T>())
            .map(|b| Arc::new(b.clone()))
    }

    /// 注册 Bean
    pub fn register_bean<T: 'static + Send + Sync>(&mut self, bean: T) {
        self.singletons.insert(TypeId::of::<T>(), Box::new(bean));
    }

    /// 注册命名 Bean
    pub fn register_named_bean<T: 'static + Send + Sync>(
        &mut self,
        name: String,
        bean: T,
    ) {
        self.named_beans.insert(name, Box::new(bean));
    }

    /// 检查 Bean 是否存在
    pub fn contains_bean<T: 'static>(&self) -> bool {
        self.singletons.contains_key(&TypeId::of::<T>())
    }

    /// 检查命名 Bean 是否存在
    pub fn contains_named_bean(&self, name: &str) -> bool {
        self.named_beans.contains_key(name)
    }

    /// 获取配置属性
    pub fn get_property(&self, key: &str) -> Option<String> {
        self.properties.get(key)
    }
}
```

---

## 5. 配置属性系统 / Configuration Properties

### 5.1 配置加载顺序

```
1. application.toml / application.yml        # 默认配置
2. application-{profile}.toml                 # 环境配置
3. 环境变量 (NEXUS_*, APP_*)                   # 环境变量覆盖
4. 命令行参数 (--server.port=9090)            # 命令行覆盖
```

### 5.2 配置属性注解

```rust
//! nexus-starter/src/config/properties.rs

/// 配置属性 trait
/// Configuration properties trait
pub trait ConfigurationProperties: Send + Sync {
    fn from_map(map: &HashMap<String, String>) -> Result<Self, Error>
    where
        Self: Sized;
}

/// 配置属性派生宏
#[proc_macro_derive(ConfigurationProperties, attributes(prefix, config))]
pub fn derive_configuration_properties(input: TokenStream) -> TokenStream {
    // 自动实现从配置加载
}
```

### 5.3 配置示例

```rust
//! 应用配置示例

#[derive(ConfigurationProperties, Deserialize)]
#[config(prefix = "server")]
pub struct ServerProperties {
    #[config(default = "8080")]
    pub port: u16,

    #[config(default = "127.0.0.1")]
    pub host: String,

    #[config(default = "10")]
    pub worker_threads: usize,
}

#[derive(ConfigurationProperties, Deserialize)]
#[config(prefix = "app.datasource")]
pub struct DataSourceProperties {
    #[config(required = true)]
    pub url: String,

    #[config(default = "postgres")]
    pub username: String,

    #[config(default = "")]
    pub password: String,

    #[config(default = "5")]
    pub max_connections: u32,
}
```

---

## 6. 使用示例 / Usage Examples

### 6.1 最简单的 Web 应用

```rust
// Cargo.toml
[dependencies]
nexus-starter = { version = "0.1", features = ["web"] }

// src/main.rs
use nexus_starter::prelude::*;

#[nexus_main]
struct MyApp;

#[controller]
struct HelloController;

#[get("/")]
fn hello() -> &'static str {
    "Hello, Nexus!"
}

#[get("/users/:id")]
fn get_user(id: u64) -> Json<User> {
    Json(User { id, name: "Alice".into() })
}
```

### 6.2 完整业务应用

```rust
// Cargo.toml
[dependencies]
nexus-starter = { version = "0.1", features = ["full"] }

// src/main.rs
use nexus_starter::prelude::*;
use std::sync::Arc;

#[nexus_main]
#[component_scan]  // 自动扫描所有组件
struct Application;

// ==================== Controller ====================

#[controller]
struct UserController {
    #[autowired]
    user_service: Arc<UserService>,
}

#[get("/users")]
async fn list_users(
    controller: &UserController,
) -> Json<Vec<User>> {
    controller.user_service.list_all().await.into()
}

#[get("/users/:id")]
async fn get_user(
    controller: &UserController,
    id: u64,
) -> Json<User> {
    controller.user_service.find_by_id(id).await.into()
}

#[post("/users")]
async fn create_user(
    controller: &UserController,
    #[validated] user: CreateUserRequest,
) -> Json<User> {
    controller.user_service.create(user).await.into()
}

// ==================== Service ====================

#[service]
struct UserService {
    #[autowired]
    repository: Arc<UserRepository>,
}

impl UserService {
    #[cacheable("users")]
    async fn find_by_id(&self, id: u64) -> User {
        self.repository.find_by_id(id).await.unwrap()
    }

    #[transactional]
    async fn create(&self, req: CreateUserRequest) -> User {
        self.repository.insert(req).await.unwrap()
    }

    #[scheduled(cron = "0 0 * * * *")]  // 每小时清理缓存
    async fn cleanup_cache(&self) {
        // 清理逻辑
    }
}

// ==================== Repository ====================

#[repository]
trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: u64) -> Option<User>;
    async fn insert(&self, user: CreateUserRequest) -> User;
}
```

### 6.3 配置文件

```toml
# application.toml

[server]
port = 8080
host = "0.0.0.0"
worker_threads = 10

[app]
name = "My Nexus App"
cache.enabled = true

[app.datasource]
url = "postgresql://localhost:5432/mydb"
username = "user"
password = "pass"
max_connections = 20

[app.cache]
ttl = 600
max_size = 10000

[app.security]
jwt_secret = "your-secret-key"
jwt_expiration = 3600

[logging]
level = "info"
format = "json"
```

---

## 7. 与 Spring Boot 对照 / Comparison

| Spring Boot | Nexus | 说明 |
|-------------|-------|------|
| `@SpringBootApplication` | `#[nexus_main]` | 主应用注解 |
| `@RestController` | `#[controller]` | REST 控制器 |
| `@Service` | `#[service]` | 业务服务 |
| `@Repository` | `#[repository]` | 数据访问层 |
| `@Component` | `#[component]` | 通用组件 |
| `@Autowired` | `#[autowired]` | 依赖注入 |
| `@ComponentScan` | `#[component_scan]` | 组件扫描 |
| `@Configuration` | `#[configuration]` | 配置类 |
| `@Bean` | `#[bean]` | Bean 定义 |
| `@ConfigurationProperties` | `#[config]` | 配置属性 |
| `@ConditionalOnClass` | `#[conditional_on_feature]` | 条件装配 |
| `@ConditionalOnProperty` | `#[conditional_on_property]` | 属性条件 |
| `@ConditionalOnMissingBean` | `#[conditional_on_missing_bean]` | Bean 缺失条件 |
| `@AutoConfigureOrder` | `#[auto_configure_order]` | 配置顺序 |
| `@EnableConfigurationProperties` | `#[enable_configuration_properties]` | 启用配置属性 |
| `@Cacheable` | `#[cacheable]` | 缓存 |
| `@Transactional` | `#[transactional]` | 事务 |
| `@Scheduled` | `#[scheduled]` | 定时任务 |
| `@Validated` | `#[validated]` | 验证 |
| `@GetMapping` | `#[get]` | GET 路由 |
| `@PostMapping` | `#[post]` | POST 路由 |

### Starter 对照

| Spring Boot Starter | Nexus Starter Feature |
|--------------------|----------------------|
| `spring-boot-starter-web` | `web` |
| `spring-boot-starter-security` | `security` |
| `spring-boot-starter-data-jpa` | `data` |
| `spring-boot-starter-cache` | `cache` |
| `spring-boot-starter-mail` | 待实现 |
| `spring-boot-starter-test` | `test` |

---

## 8. 实施计划 / Implementation Plan

### Phase 1: 核心框架 (2周)
- [ ] 创建 `nexus-starter` crate 结构
- [ ] 实现 `AutoConfiguration` trait
- [ ] 实现条件注解系统
- [ ] 实现 `ApplicationContext` DI 容器
- [ ] 实现配置加载器
- [ ] 实现组件扫描器

### Phase 2: Web 自动配置 (1周)
- [ ] 实现 `WebServerAutoConfiguration`
- [ ] 实现 `RouterAutoConfiguration`
- [ ] 路由自动注册
- [ ] 中间件自动配置

### Phase 3: Security 自动配置 (1周)
- [ ] 实现 `SecurityAutoConfiguration`
- [ ] 实现 `JwtAutoConfiguration`
- [ ] 安全配置属性

### Phase 4: Data 自动配置 (1周)
- [ ] 实现 `DataSourceAutoConfiguration`
- [ ] 实现 `TransactionAutoConfiguration`
- [ ] 数据源配置属性

### Phase 5: Cache & Schedule (1周)
- [ ] 实现 `CacheAutoConfiguration`
- [ ] 实现 `ScheduleAutoConfiguration`

### Phase 6: Actuator (1周)
- [ ] 实现 `ActuatorAutoConfiguration`
- [ ] 健康检查端点
- [ ] 指标端点

---

## 9. 设计原则 / Design Principles

### 9.1 约定优于配置

```rust
// 最少配置即可运行
#[nexus_main]
struct MyApp;

// 自动使用默认值
// - server.port: 8080
// - server.host: 127.0.0.1
// - logging.level: info
```

### 9.2 智能默认值

```rust
#[derive(ConfigurationProperties)]
#[config(prefix = "server")]
struct ServerProperties {
    #[config(default = "8080")]  // 智能默认
    port: u16,
}
```

### 9.3 可覆盖性

```
默认配置 < 环境配置 < 环境变量 < 命令行参数
```

### 9.4 条件装配

```rust
// 仅在需要时才装配
#[conditional_on_property("app.cache.enabled")]
pub struct CacheAutoConfiguration;
```

---

## 10. 总结 / Summary

### 使用 Nexus Starter 的好处

1. **减少依赖管理** - 一个依赖包含所有需要的模块
2. **生产就绪** - 经过测试的默认配置
3. **减少配置时间** - 智能自动装配
4. **版本统一** - 统一管理所有模块版本
5. **开箱即用** - 添加依赖即可运行

### 下一步

开始实施 Phase 1，创建 `nexus-starter` crate 的核心框架。

---

## 11. 实现状态 / Implementation Status

> 最后更新：2026-01-29
> Last updated: 2026-01-29

### 已完成 / Completed ✅

#### Phase 1: 核心框架 / Core Framework

- [x] `AutoConfiguration` trait 定义
- [x] `ApplicationContext` IoC 容器
- [x] `BeanDefinition` 和 `ComponentRegistry`
- [x] `ComponentScanner` 组件扫描器
- [x] 条件注解系统 (`Conditional`, `ConditionalOnProperty`, `ConditionalOnMissingBean`)

#### Phase 2: 自动配置加载器 / Auto-Configuration Loader

- [x] `AutoConfigurationLoader` - 从 META-INF/nexus/autoconfiguration.imports 加载
- [x] `AutoConfigurationRegistry` - 配置注册表管理
- [x] 优先级排序 (`order()` 方法)
- [x] 元数据文件格式定义

#### Phase 3: 核心自动配置 / Core Auto-Configurations

- [x] `CoreAutoConfiguration` (优先级: -100)
  - [x] 日志系统初始化 (tracing-subscriber)
  - [x] 应用名称和版本管理
  - [x] 工作线程配置

#### Phase 4: Web 自动配置 / Web Auto-Configurations

- [x] `WebServerAutoConfiguration` (优先级: 0)
  - [x] 端口配置 (server.port, 默认: 8080)
  - [x] 主机地址配置 (server.host, 默认: 127.0.0.1)
  - [x] 工作线程配置 (server.worker_threads)
  - [x] HTTP/2 支持 (server.http2.enabled)
  - [x] 请求超时配置 (server.request_timeout_secs)
  - [x] 最大连接数配置 (server.max_connections)

- [x] `RouterAutoConfiguration` (优先级: 10)
  - [x] 基础路径配置
  - [x] CORS 支持

- [x] `MiddlewareAutoConfiguration` (优先级: 20)
  - [x] CORS 中间件
  - [x] 压缩中间件
  - [x] 日志中间件
  - [x] 超时中间件
  - [x] 速率限制中间件

#### Phase 5: 主宏 / Main Macro

- [x] `#[nexus_main]` 宏实现
  - [x] ApplicationContext 创建
  - [x] 自动配置加载
  - [x] 优先级排序
  - [x] 条件评估
  - [x] Bean 注册

#### Phase 6: 示例应用 / Example Application

- [x] `starter_example.rs` 示例
- [x] META-INF/nexus/autoconfiguration.imports 元数据文件
- [x] 测试用例

### 进行中 / In Progress 🚧

- [ ] HTTP 服务器集成（等待 nexus-http 模块完善）

### 待实现 / Pending 📋

#### Security 自动配置
- [ ] `SecurityAutoConfiguration`
- [ ] `JwtAutoConfiguration`

#### Data 自动配置
- [ ] `DataSourceAutoConfiguration`
- [ ] `TransactionAutoConfiguration`

#### Cache 自动配置
- [ ] `CacheAutoConfiguration`

#### Schedule 自动配置
- [ ] `ScheduleAutoConfiguration`

#### Actuator 自动配置
- [ ] `ActuatorAutoConfiguration`

### 文件结构 / File Structure

```
crates/nexus-starter/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── prelude.rs
│   ├── core/
│   │   ├── mod.rs
│   │   ├── autoconfig.rs       # AutoConfiguration trait
│   │   ├── container.rs        # ApplicationContext
│   │   ├── scanner.rs          # ComponentScanner
│   │   ├── condition.rs        # 条件注解
│   │   ├── config.rs           # CoreAutoConfiguration
│   │   └── loader.rs           # AutoConfigurationLoader
│   ├── web/
│   │   └── mod.rs              # Web 自动配置
│   ├── config/
│   │   └── properties.rs       # 配置属性
│   ├── security/
│   ├── data/
│   ├── cache/
│   ├── schedule/
│   └── actuator/
examples/
├── META-INF/nexus/
│   └── autoconfiguration.imports
└── src/
    └── starter_example.rs
```

### 使用示例 / Usage Example

```rust,no_run,ignore
use nexus_macros::nexus_main;

#[nexus_main]
struct Application;

fn main() -> anyhow::Result<()> {
    Application::run()
}
```

### 运行示例 / Running the Example

```bash
# 编译
cargo build --bin starter_example

# 运行
cargo run --bin starter_example
```

预期输出：
```
=== Starting Nexus Application ===
Debug mode: false
Worker threads: 14
Core configuration completed

Running: CoreAutoConfiguration
Running: WebServerAutoConfiguration
=== Configuring Web Server ===
  Bind address: 127.0.0.1:8080
  Worker threads: 14
  HTTP/2: false
  Request timeout: 30s
  Max connections: 10000
Web server configuration completed

Running: RouterAutoConfiguration
=== Configuring Router ===
  Base path: /
  CORS: false
Router configuration completed

Running: MiddlewareAutoConfiguration
=== Configuring Middleware ===
  Enabled: Logging, Timeout
Middleware configuration completed

=== Application Started ===
```

---
