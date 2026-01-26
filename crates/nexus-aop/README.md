# Nexus AOP

[![Crates.io](https://img.shields.io/crates/v/nexus-aop)](https://crates.io/nexus-aop)
[![Documentation](https://docs.rs/nexus-aop/badge.svg)](https://docs.rs/nexus-aop)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](../../LICENSE)

> Spring AOP style annotations for Nexus framework
>
> Nexus 框架的 Spring AOP 风格注解

---

## 📋 Overview / 概述

`nexus-aop` provides Spring AOP-style procedural macros for aspect-oriented programming in the Nexus framework.

`nexus-aop` 为 Nexus 框架提供 Spring AOP 风格的过程宏，支持面向切面编程。

**Key Features / 核心特性**:

- ✅ **`#[Aspect]`** - Marks a struct as an aspect / 标记切面
- ✅ **`@Before`** - Before advice / 前置通知
- ✅ **`@After`** - After advice / 后置通知
- ✅ **`@Around`** - Around advice / 环绕通知
- ✅ **`@Pointcut`** - Pointcut definitions / 切点定义

---

## 🚀 Quick Start / 快速开始

### Installation / 安装

Add to `Cargo.toml`:

```toml
[dependencies]
nexus-aop = "0.1"
```

### Basic Usage / 基本用法

```rust
use nexus_aop::{Aspect, Before, After, Around};

#[Aspect]
struct LoggingAspect;

impl LoggingAspect {
    #[Before("execution(* com.example..*.*(..))")]
    fn log_before(&self, join_point: &JoinPoint) {
        println!("Entering: {}", join_point.method_name());
    }

    #[After("execution(* com.example..*.*(..))")]
    fn log_after(&self, join_point: &JoinPoint) {
        println!("Exiting: {}", join_point.method_name());
    }

    #[Around("execution(* com.example..*.*(..))")]
    fn log_around(&self, join_point: JoinPoint) -> Result<(), Error> {
        println!("Before: {}", join_point.method_name());
        let result = join_point.proceed()?;
        println!("After: {}", join_point.method_name());
        Ok(result)
    }
}
```

---

## 📖 Available Annotations / 可用注解

### `#[Aspect]`

Marks a struct as an AOP aspect.
将结构体标记为 AOP 切面。

```rust
#[Aspect]
struct LoggingAspect;
```

### `@Before("pointcut")`

Before advice - executes before the method.
前置通知 - 在方法执行前执行。

```rust
#[Before("execution(* com.example..*.*(..))")]
fn log_before(&self, join_point: &JoinPoint) {
    println!("Entering: {}", join_point.method_name());
}
```

### `@After("pointcut")`

After advice - executes after the method (normal or exceptional exit).
后置通知 - 在方法执行后执行（正常或异常退出）。

```rust
#[After("execution(* com.example..*.*(..))")]
fn log_after(&self, join_point: &JoinPoint) {
    println!("Exiting: {}", join_point.method_name());
}
```

### `@Around("pointcut")`

Around advice - wraps the method execution, can control execution flow.
环绕通知 - 包装方法执行，可以控制执行流程。

```rust
#[Around("execution(* com.example..*.*(..))")]
fn log_around(&self, join_point: JoinPoint) -> Result<(), Error> {
    println!("Before: {}", join_point.method_name());
    let result = join_point.proceed()?;
    println!("After: {}", join_point.method_name());
    Ok(result)
}
```

### `@Pointcut("expression")`

Defines a reusable pointcut expression.
定义可重用的切点表达式。

```rust
#[Pointcut("execution(* com.example.service.*.*(..))")]
fn service_layer() -> PointcutExpression {}

// Reference the pointcut
// 引用切点
#[Before("service_layer()")]
fn log_service(&self, join_point: &JoinPoint) {
    println!("Service method called");
}
```

---

## 📚 Pointcut Expressions / 切点表达式

### Execution Pattern / 执行模式

```
execution(modifiers-pattern? return-type-pattern declaring-type-pattern? method-name-pattern(param-pattern) throws-pattern?)
```

**Examples / 示例**:

```rust
// All public methods
// 所有 public 方法
"execution(public * *(..))"

// All methods in a package
// 包中的所有方法
"execution(* com.example..*.*(..))"

// All methods in a class
// 类中的所有方法
"execution(* com.example.Service.*(..))"

// Specific method
// 特定方法
"execution(* com.example.UserService.getUser(..))"

// Methods with specific parameter types
// 带有特定参数类型的方法
"execution(* com.example.UserService.*(String, ..))"

// Methods returning void
// 返回 void 的方法
"execution(void com.example..*.*(..))"
```

### Combining Pointcuts / 组合切点

```rust
// AND - both conditions must match
// AND - 两个条件都必须匹配
"execution(* com.example..*.*(..)) && execution(public * *(..))"

// OR - either condition must match
// OR - 任一条件必须匹配
"execution(* com.example.Service.*(..)) || execution(* com.example.Repository.*(..))"

// NOT - negate the condition
// NOT - 否定条件
"execution(* com.example..*.*(..)) && !execution(* com.example..*.*(..))"
```

### Other Designators / 其他指示符

```rust
// Within a certain type
// 在特定类型内
"within(com.example.service.*)"

// Match bean reference (Spring)
// 匹配 bean 引用（Spring）
"this(com.example.service.UserService)"

// Match target object
// 匹配目标对象
"target(com.example.service.UserService)"

// Match arguments
// 匹配参数
"args(String, ..)"
"args(com.example.User)"

// Methods with specific annotation
// 带有特定注解的方法
"@annotation(com.example.Transactional)"
"@annotation(org.springframework.web.bind.annotation.PostMapping)"
```

---

## 📚 Examples / 示例

### Example 1: Logging Aspect / 日志切面

```rust
use nexus_aop::{Aspect, Before, After};
use tracing::{info, instrument};

#[Aspect]
struct LoggingAspect;

impl LoggingAspect {
    #[Before("execution(* com.example..*.*(..))")]
    fn log_method_entry(&self, join_point: &JoinPoint) {
        info!(
            "Entering method: {} with args: {:?}",
            join_point.method_name(),
            join_point.args()
        );
    }

    #[After("execution(* com.example..*.*(..))")]
    fn log_method_exit(&self, join_point: &JoinPoint) {
        info!(
            "Exiting method: {}",
            join_point.method_name()
        );
    }
}
```

### Example 2: Transaction Management Aspect / 事务管理切面

```rust
use nexus_aop::{Aspect, Around};

#[Aspect]
struct TransactionAspect;

impl TransactionAspect {
    #[Around("execution(* com.example.service.*.*(..)) && @annotation(com.example.Transactional)")]
    fn manage_transaction(&self, join_point: JoinPoint) -> Result<(), Error> {
        // Begin transaction
        // 开始事务
        let tx = Transaction::begin()?;

        match join_point.proceed() {
            Ok(result) => {
                tx.commit()?;
                Ok(result)
            }
            Err(e) => {
                tx.rollback()?;
                Err(e)
            }
        }
    }
}
```

### Example 3: Caching Aspect / 缓存切面

```rust
use nexus_aop::{Aspect, Around};
use std::collections::HashMap;

#[Aspect]
struct CachingAspect {
    cache: HashMap<String, CachedValue>,
}

impl CachingAspect {
    #[Around("execution(* com.example.repository.*.find*(..))")]
    fn cache_result(&self, join_point: JoinPoint) -> Result<Option<Entity>, Error> {
        let cache_key = format!("{:?}", join_point.args());

        // Check cache
        // 检查缓存
        if let Some(value) = self.cache.get(&cache_key) {
            return Ok(value.clone());
        }

        // Execute method
        // 执行方法
        let result = join_point.proceed()?;

        // Cache result
        // 缓存结果
        self.cache.insert(cache_key, result.clone());

        Ok(result)
    }
}
```

### Example 4: Security Aspect / 安全切面

```rust
use nexus_aop::{Aspect, Before};

#[Aspect]
struct SecurityAspect;

impl SecurityAspect {
    #[Before("execution(* com.example.controller.*.*(..))")]
    fn check_authorization(&self, join_point: &JoinPoint) {
        let user = get_current_user();

        if !user.has_permission(join_point.method_name()) {
            panic!("Unauthorized access to {}", join_point.method_name());
        }
    }
}
```

### Example 5: Performance Monitoring / 性能监控

```rust
use nexus_aop::{Aspect, Around};
use std::time::Instant;

#[Aspect]
struct PerformanceMonitoringAspect;

impl PerformanceMonitoringAspect {
    #[Around("execution(* com.example.service.*.*(..))")]
    fn monitor_performance(&self, join_point: JoinPoint) -> Result<(), Error> {
        let start = Instant::now();

        let result = join_point.proceed();

        let duration = start.elapsed();

        if duration.as_millis() > 1000 {
            warn!(
                "Slow method: {} took {}ms",
                join_point.method_name(),
                duration.as_millis()
            );
        }

        result
    }
}
```

---

## 🔀 Annotation vs Plain Rust / 注解版本 vs 原生 Rust

### Logging Aspect Example / 日志切面示例

#### ❌ Without Annotations (Plain Rust) / 不使用注解

```rust
// Must manually call logging before/after each method
// 必须在每个方法前后手动调用日志记录
struct UserService {
    db: Database,
}

impl UserService {
    async fn get_user(&self, id: i64) -> Result<Option<User>, Error> {
        // Manual logging - repetitive and error-prone
        // 手动日志记录 - 重复且容易出错
        println!("Entering: get_user with id={}", id);

        let result = self.db.find_user(id).await;

        println!("Exiting: get_user");
        result
    }

    async fn create_user(&self, user: User) -> Result<User, Error> {
        println!("Entering: create_user with user={:?}", user);

        let result = self.db.insert_user(&user).await?;

        println!("Exiting: create_user");
        Ok(result)
    }

    async fn update_user(&self, id: i64, user: User) -> Result<User, Error> {
        println!("Entering: update_user with id={}, user={:?}", id, user);

        let result = self.db.update_user(id, &user).await?;

        println!("Exiting: update_user");
        Ok(result)
    }

    async fn delete_user(&self, id: i64) -> Result<(), Error> {
        println!("Entering: delete_user with id={}", id);

        let result = self.db.delete_user(id).await;

        println!("Exiting: delete_user");
        result
    }
}

// Problems:
// - Logging code repeated in every method / 日志代码在每个方法中重复
// - Easy to forget logging / 容易忘记记录日志
// - Cannot centrally manage logging / 无法集中管理日志
// - Mixes business logic with cross-cutting concerns / 混合业务逻辑和横切关注点
```

#### ✅ With Annotations (Nexus AOP) / 使用注解

```rust
use nexus_aop::{Aspect, Before, After};

// Define aspect once - applies to all matching methods
// 定义切面一次 - 应用于所有匹配的方法
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

// Clean business logic - no logging code mixed in
// 清晰的业务逻辑 - 没有混合日志代码
struct UserService {
    db: Database,
}

impl UserService {
    async fn get_user(&self, id: i64) -> Result<Option<User>, Error> {
        // Logging is applied automatically by AOP!
        // 日志由 AOP 自动应用！
        self.db.find_user(id).await
    }

    async fn create_user(&self, user: User) -> Result<User, Error> {
        self.db.insert_user(&user).await
    }

    async fn update_user(&self, id: i64, user: User) -> Result<User, Error> {
        self.db.update_user(id, &user).await
    }

    async fn delete_user(&self, id: i64) -> Result<(), Error> {
        self.db.delete_user(id).await
    }
}

// Benefits:
// - Logging separated from business logic / 日志与业务逻辑分离
// - Consistent logging across all methods / 所有方法的日志一致
// - Easy to modify logging in one place / 在一个地方轻松修改日志
// - Business logic remains clean / 业务逻辑保持清晰
```

**Code Reduction / 代码减少**: Eliminates 50%+ repetitive logging code / 消除 50%+ 的重复日志代码

---

### Transaction Management Example / 事务管理示例

#### ❌ Without Annotations / 不使用注解

```rust
impl PaymentService {
    async fn process_payment(&self, from: i64, to: i64, amount: i64) -> Result<(), Error> {
        // Manual transaction management - error-prone
        // 手动事务管理 - 容易出错
        let tx = self.begin_transaction().await?;

        match self.debit(&tx, from, amount).await {
            Ok(_) => match self.credit(&tx, to, amount).await {
                Ok(_) => {
                    tx.commit().await?;
                    Ok(())
                }
                Err(e) => {
                    tx.rollback().await?;
                    Err(e)
                }
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }

    // Must repeat this pattern for every transactional method
    // 必须为每个事务方法重复此模式
    async fn transfer_funds(&self, from: i64, to: i64, amount: i64) -> Result<(), Error> {
        let tx = self.begin_transaction().await?;
        // ... same pattern ...
    }
}
```

#### ✅ With Annotations / 使用注解

```rust
use nexus_aop::{Aspect, Around};
use nexus_data_annotations::Transactional;

#[Aspect]
struct TransactionAspect;

impl TransactionAspect {
    #[Around("execution(* PaymentService.*(..))")]
    async fn manage_transaction(&self, join_point: JoinPoint) -> Result<(), Error> {
        let tx = self.begin_transaction().await?;

        match join_point.proceed().await {
            Ok(_) => {
                tx.commit().await?;
                Ok(())
            }
            Err(e) => {
                tx.rollback().await?;
                Err(e)
            }
        }
    }
}

impl PaymentService {
    // Transaction is managed automatically!
    // 事务自动管理！
    async fn process_payment(&self, from: i64, to: i64, amount: i64) -> Result<(), Error> {
        self.debit(from, amount).await?;
        self.credit(to, amount).await?;
        Ok(())
    }
}
```

---

### Comparison Table / 对比表

| Feature / 功能 | Plain Rust / 原生 Rust | With AOP Annotations / 使用 AOP 注解 |
|----------------|----------------------|----------------------------------|
| **Code Duplication** / 代码重复 | ❌ High / 高 | ✅ Low / 低 |
| **Separation of Concerns** / 关注点分离 | ❌ Mixed / 混合 | ✅ Separated / 分离 |
| **Maintainability** / 可维护性 | ❌ Changes in many places / 多处修改 | ✅ Change in one place / 一处修改 |
| **Business Logic Clarity** / 业务逻辑清晰度 | ❌ Obsured by cross-cutting code / 被横切代码模糊 | ✅ Clear and focused / 清晰专注 |
| **Consistency** / 一致性 | ❌ Easy to miss / 容易遗漏 | ✅ Automatically applied / 自动应用 |
| **Flexibility** / 灵活性 | ❌ Hard to modify behavior | ✅ Easy to change aspect / 易于修改切面 |

---

## 🧪 Testing / 测试

Run tests:

```bash
cargo test --package nexus-aop
```

Run examples:

```bash
cargo run --package nexus-aop --example logging_aspect
```

---

## 📖 Documentation / 文档

- **API Documentation**: [docs.rs/nexus-aop](https://docs.rs/nexus-aop)
- **Spring AOP Documentation**: [https://docs.spring.io/spring-framework/docs/current/reference/html/core.html#aop](https://docs.spring.io/spring-framework/docs/current/reference/html/core.html#aop)

---

## 🔄 Migration from Spring AOP / 从 Spring AOP 迁移

### Java / Spring AOP

```java
@Aspect
@Component
public class LoggingAspect {
    private static final Logger logger = LoggerFactory.getLogger(LoggingAspect.class);

    @Before("execution(* com.example..*.*(..))")
    public void logBefore(JoinPoint joinPoint) {
        logger.info("Entering: {}", joinPoint.getSignature().toShortString());
    }

    @After("execution(* com.example..*.*(..))")
    public void logAfter(JoinPoint joinPoint) {
        logger.info("Exiting: {}", joinPoint.getSignature().toShortString());
    }
}
```

### Rust / Nexus AOP

```rust
use nexus_aop::{Aspect, Before, After};
use tracing::info;

#[Aspect]
struct LoggingAspect;

impl LoggingAspect {
    #[Before("execution(* com.example..*.*(..))")]
    fn log_before(&self, join_point: &JoinPoint) {
        info!("Entering: {}", join_point.method_name());
    }

    #[After("execution(* com.example..*.*(..))")]
    fn log_after(&self, join_point: &JoinPoint) {
        info!("Exiting: {}", join_point.method_name());
    }
}
```

---

## 📝 License / 许可证

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option. / 由您选择。

---

**Built with ❤️ for Java developers transitioning to Rust**

**为从 Java 转向 Rust 的开发者构建 ❤️**
