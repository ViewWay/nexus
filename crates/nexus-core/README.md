# nexus-core

[![Crates.io](https://img.shields.io/crates/v/nexus-core)](https://crates.io/crates/nexus-core)
[![Documentation](https://docs.rs/nexus-core/badge.svg)](https://docs.rs/nexus-core)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](../../LICENSE)

> Core types and IoC container for Nexus framework
> 
> Nexus框架的核心类型和IoC容器

---

## 📋 Overview / 概述

`nexus-core` provides the foundation for the Nexus framework, featuring:

`nexus-core` 为Nexus框架提供基础，具有：

- **IoC Container** / **IoC容器** - Dependency injection and bean management
- **Bean lifecycle** / **Bean生命周期** - Initialization, destruction, and scopes
- **Reflection system** / **反射系统** - Runtime type information
- **Context management** / **上下文管理** - Application and request contexts
- **Extension system** / **扩展系统** - Plugin architecture

---

## ✨ Key Features / 核心特性

| Feature / 特性 | Status / 状态 | Description / 描述 |
|---------------|--------------|-------------------|
| **IoC Container** | ✅ Phase 1 | Dependency injection |
| **Bean management** | ✅ Phase 1 | Singleton, prototype, request scopes |
| **Reflection** | ✅ Phase 1 | Runtime type info |
| **Context** | ✅ Phase 1 | Application and request contexts |
| **Extensions** | ✅ Phase 1 | Plugin system |
| **Auto-wiring** | 🔄 Phase 2 | Automatic dependency resolution |
| **AOP** | 📋 Future | Aspect-oriented programming |

---

## 🚀 Quick Start / 快速开始

### Installation / 安装

```toml
[dependencies]
nexus-core = "0.1.0-alpha"
```

### IoC Container Example / IoC容器示例

```rust
use nexus_core::{Container, Bean, Scope};

// Define a bean / 定义一个bean
#[derive(Clone)]
struct Database {
    url: String,
}

impl Database {
    fn new(url: String) -> Self {
        Self { url }
    }
}

// Register beans / 注册bean
let mut container = Container::new();

container.register_bean(
    "database",
    Bean::new(Database::new("postgres://localhost".to_string()))
        .with_scope(Scope::Singleton)
);

// Get bean / 获取bean
let db = container.get::<Database>("database").unwrap();
println!("Connected to: {}", db.url);
```

### Dependency Injection / 依赖注入

```rust
use nexus_core::{Container, Injectable};

#[derive(Clone)]
struct UserService {
    database: Arc<Database>,
}

impl Injectable for UserService {
    fn inject(container: &Container) -> Self {
        Self {
            database: container.get("database").unwrap(),
        }
    }
}

// Register with auto-injection / 使用自动注入注册
container.register_injectable::<UserService>("user_service");

// Use the service / 使用服务
let service = container.get::<UserService>("user_service").unwrap();
```

---

## 🏗️ Architecture / 架构

```
┌─────────────────────────────────────────────────────────────┐
│                  nexus-core Architecture                     │
│                  nexus-core 架构                             │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │               Application Context                       │ │
│  │               应用上下文                                 │ │
│  ├────────────────────────────────────────────────────────┤ │
│  │  Environment  │  Configuration  │  Extensions          │ │
│  └────────────────────────────────────────────────────────┘ │
│                             │                                │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                 IoC Container                           │ │
│  │                 IoC容器                                  │ │
│  ├────────────────────────────────────────────────────────┤ │
│  │  Bean Registry  │  Dependency Graph  │  Lifecycle      │ │
│  └────────────────────────────────────────────────────────┘ │
│                             │                                │
│  ┌────────────────────────────────────────────────────────┐ │
│  │              Reflection System                          │ │
│  │              反射系统                                    │ │
│  ├────────────────────────────────────────────────────────┤ │
│  │  Type Info  │  Method Calls  │  Property Access        │ │
│  └────────────────────────────────────────────────────────┘ │
│                             │                                │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                Error Handling                           │ │
│  │                错误处理                                  │ │
│  ├────────────────────────────────────────────────────────┤ │
│  │  ErrorKind  │  Context  │  Backtrace                   │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Module Structure / 模块结构

```
nexus-core/
├── container.rs          # IoC container
├── bean.rs               # Bean definition and lifecycle
├── context.rs            # Application and request contexts
├── extension.rs          # Extension system
├── reflect.rs            # Reflection system
├── error.rs              # Error types
└── lib.rs
```

---

## 📖 Core Concepts / 核心概念

### IoC Container / IoC容器

```rust
use nexus_core::Container;

// Create container / 创建容器
let mut container = Container::new();

// Register singleton / 注册单例
container.register_singleton("database", Database::new());

// Register prototype (new instance each time) / 注册原型（每次新实例）
container.register_prototype("user", || User::default());

// Register factory / 注册工厂
container.register_factory("connection", |container| {
    let db = container.get::<Database>("database")?;
    Ok(Connection::new(db))
});

// Get bean / 获取bean
let db = container.get::<Database>("database")?;
```

### Bean Scopes / Bean作用域

```rust
use nexus_core::{Bean, Scope};

// Singleton: One instance per container / 单例：每个容器一个实例
let bean = Bean::new(service).with_scope(Scope::Singleton);

// Prototype: New instance each time / 原型：每次新实例
let bean = Bean::new(service).with_scope(Scope::Prototype);

// Request: One instance per request / 请求：每个请求一个实例
let bean = Bean::new(service).with_scope(Scope::Request);

// Session: One instance per session / 会话：每个会话一个实例
let bean = Bean::new(service).with_scope(Scope::Session);

// Application: Global singleton / 应用：全局单例
let bean = Bean::new(service).with_scope(Scope::Application);
```

### Bean Lifecycle / Bean生命周期

```rust
use nexus_core::{Bean, BeanLifecycle};

struct MyService;

impl BeanLifecycle for MyService {
    fn post_construct(&mut self) {
        // Called after bean creation / bean创建后调用
        println!("Initializing service...");
    }
    
    fn pre_destroy(&mut self) {
        // Called before bean destruction / bean销毁前调用
        println!("Cleaning up service...");
    }
}

container.register_bean(
    "my_service",
    Bean::new(MyService)
        .with_lifecycle()
);
```

### Context / 上下文

```rust
use nexus_core::Context;

// Application context / 应用上下文
let app_context = Context::application();
app_context.set("app_name", "MyApp");
app_context.set("version", "1.0.0");

// Request context / 请求上下文
async fn handler(req: Request) {
    let ctx = req.context();
    ctx.set("request_id", generate_id());
    ctx.set("user", current_user());
}
```

---

## 🎯 Dependency Injection / 依赖注入

### Manual Injection / 手动注入

```rust
use nexus_core::Container;

// Define services / 定义服务
struct Database { /* ... */ }
struct UserRepository {
    db: Arc<Database>,
}
struct UserService {
    repo: Arc<UserRepository>,
}

// Register beans / 注册bean
container.register_singleton("database", Database::new());

container.register_factory("user_repository", |c| {
    let db = c.get::<Database>("database")?;
    Ok(UserRepository { db })
});

container.register_factory("user_service", |c| {
    let repo = c.get::<UserRepository>("user_repository")?;
    Ok(UserService { repo })
});
```

### Auto-wiring (Phase 2) / 自动装配（第2阶段）

```rust
use nexus_core::{Injectable, Autowired};

#[derive(Injectable)]
struct UserService {
    #[autowired]
    database: Arc<Database>,
    
    #[autowired]
    cache: Arc<Cache>,
    
    #[value("app.name")]
    app_name: String,
}

// Auto-register with dependencies / 自动注册依赖
container.auto_register::<UserService>();
```

### Constructor Injection / 构造函数注入

```rust
use nexus_core::Injectable;

impl Injectable for UserService {
    fn inject(container: &Container) -> Result<Self, Error> {
        Ok(Self {
            database: container.get("database")?,
            cache: container.get("cache")?,
            config: container.get("config")?,
        })
    }
}

container.register_injectable::<UserService>("user_service");
```

---

## 🔧 Extension System / 扩展系统

```rust
use nexus_core::{Extension, ExtensionContext};

// Define extension / 定义扩展
struct LoggingExtension;

impl Extension for LoggingExtension {
    fn name(&self) -> &str {
        "logging"
    }
    
    fn initialize(&mut self, ctx: &ExtensionContext) -> Result<(), Error> {
        println!("Initializing logging extension");
        // Setup logging / 设置日志
        Ok(())
    }
    
    fn shutdown(&mut self) {
        println!("Shutting down logging extension");
    }
}

// Register extension / 注册扩展
container.add_extension(LoggingExtension);
```

---

## 🪞 Reflection System / 反射系统

```rust
use nexus_core::reflect::{Type, TypeInfo};

// Get type information / 获取类型信息
let type_info = TypeInfo::of::<User>();

println!("Type name: {}", type_info.name());
println!("Type size: {}", type_info.size());
println!("Fields: {:?}", type_info.fields());

// Dynamic method call / 动态方法调用
let user = User::new("Alice");
let result = type_info.call_method(&user, "get_name", &[])?;
```

### Derive Macro / 派生宏

```rust
use nexus_core::Reflect;

#[derive(Reflect)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// Reflection available at runtime / 运行时可用反射
let user = User::new(1, "Alice", "alice@example.com");
let type_info = user.type_info();

for field in type_info.fields() {
    println!("Field: {} = {:?}", field.name(), field.get(&user));
}
```

---

## 🚨 Error Handling / 错误处理

```rust
use nexus_core::{Error, ErrorKind, Result};

// Define errors / 定义错误
fn find_user(id: u64) -> Result<User> {
    if id == 0 {
        return Err(Error::new(ErrorKind::BadRequest)
            .with_message("Invalid user ID"));
    }
    
    database.find_user(id)
        .map_err(|e| Error::new(ErrorKind::Internal)
            .with_source(e)
            .with_context("user_id", id))
}

// Error context / 错误上下文
let err = Error::not_found("User not found")
    .with_context("user_id", 123)
    .with_context("searched_in", "database");

// Convert to HTTP response / 转换为HTTP响应
let response = err.into_response();
```

---

## ⚡ Performance / 性能

### Container Lookup / 容器查找

```
Bean lookup performance / Bean查找性能:
- Singleton: O(1) - Direct Arc clone / 直接Arc克隆
- Prototype: O(1) - Factory call / 工厂调用
- Lazy: O(1) amortized - First call creates / 摊销O(1)

Memory overhead / 内存开销:
- Singleton: 1 instance / 1个实例
- Prototype: N instances / N个实例
- Request: 1 per request / 每个请求1个
```

### Reflection Overhead / 反射开销

```
Reflection performance / 反射性能:
- Type info lookup: O(1) - Cached / 缓存
- Method call: ~100ns overhead vs direct / 比直接调用慢约100ns
- Field access: ~50ns overhead vs direct / 比直接访问慢约50ns
```

---

## 🧪 Testing / 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use nexus_core::Container;

    #[test]
    fn test_singleton_scope() {
        let mut container = Container::new();
        
        container.register_singleton("value", 42);
        
        let v1 = container.get::<i32>("value").unwrap();
        let v2 = container.get::<i32>("value").unwrap();
        
        assert_eq!(Arc::ptr_eq(&v1, &v2), true);
    }

    #[test]
    fn test_dependency_injection() {
        let mut container = Container::new();
        
        container.register_singleton("db", Database::new());
        container.register_injectable::<UserService>("service");
        
        let service = container.get::<UserService>("service").unwrap();
        assert!(service.database.is_some());
    }
}
```

---

## 🚦 Roadmap / 路线图

### Phase 1: Core Foundation ✅ (Completed / 已完成)
- [x] IoC container
- [x] Bean lifecycle
- [x] Context management
- [x] Extension system
- [x] Basic reflection

### Phase 2: Advanced DI 🔄 (In Progress / 进行中)
- [ ] Auto-wiring
- [ ] Qualifier annotations
- [ ] Bean profiles
- [ ] Conditional beans
- [ ] Property injection

### Phase 3: AOP 📋 (Planned / 计划中)
- [ ] Method interception
- [ ] @Before/@After/@Around
- [ ] Aspect composition
- [ ] Performance profiling

---

## 📚 Documentation / 文档

- **API Documentation**: [docs.rs/nexus-core](https://docs.rs/nexus-core)
- **Book**: [Core Concepts](../../docs/book/)
- **Examples**: [examples/ioc_container_example.rs](../../examples/ioc_container_example.rs)

---

## 🤝 Contributing / 贡献

We welcome contributions! Please see:

- [CONTRIBUTING.md](../../CONTRIBUTING.md)
- [Design Spec](../../docs/design-spec.md)
- [GitHub Issues](https://github.com/nexus-framework/nexus/issues)

---

## 📄 License / 许可证

Licensed under Apache License 2.0. See [LICENSE](../../LICENSE) for details.

---

## 🙏 Acknowledgments / 致谢

Nexus Core is inspired by:

- **[Spring Framework](https://spring.io/)** - IoC container design
- **[Guice](https://github.com/google/guice)** - Dependency injection patterns
- **[Dagger](https://dagger.dev/)** - Compile-time DI
- **[bevy_reflect](https://github.com/bevyengine/bevy)** - Rust reflection system

---

**Built with ❤️ for dependency injection**

**为依赖注入构建 ❤️**
