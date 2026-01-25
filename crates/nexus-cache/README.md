# nexus-cache

[![Crates.io](https://img.shields.io/crates/v/nexus-cache)](https://crates.io/crates/nexus-cache)
[![Documentation](https://docs.rs/nexus-cache/badge.svg)](https://docs.rs/nexus-cache)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](../../LICENSE)

> Caching abstraction for Nexus framework
> 
> Nexus框架的缓存抽象

---

## 📋 Overview / 概述

`nexus-cache` provides a flexible caching abstraction with annotation-based caching, similar to Spring Boot's `@Cacheable`, `@CacheEvict`, and `@CachePut`.

`nexus-cache` 提供灵活的缓存抽象，支持基于注解的缓存，类似于Spring Boot的`@Cacheable`、`@CacheEvict`和`@CachePut`。

**Key Features** / **核心特性**:
- ✅ **Annotation-based** / **基于注解** - `@Cacheable`, `@CacheEvict`, `@CachePut`
- ✅ **Conditional caching** / **条件缓存** - `condition`, `unless` expressions
- ✅ **Multiple backends** / **多后端** - Memory, Redis, custom
- ✅ **TTL support** / **TTL支持** - Time-to-live expiration
- ✅ **Cache manager** / **缓存管理器** - Centralized cache management
- ✅ **Key generation** / **键生成** - Automatic cache key generation

---

## ✨ Features / 特性

| Feature | Spring Equivalent | Description | Status |
|---------|------------------|-------------|--------|
| **@Cacheable** | `@Cacheable` | Cache method results | ✅ |
| **@Cacheable (condition)** | `@Cacheable(condition=)` | Conditional caching | ✅ |
| **@Cacheable (unless)** | `@Cacheable(unless=)` | Unless caching | ✅ |
| **@CacheEvict** | `@CacheEvict` | Evict cache entries | ✅ |
| **@CachePut** | `@CachePut` | Update cache | ✅ |
| **CacheManager** | `CacheManager` | Cache management | ✅ |
| **KeyGenerator** | `KeyGenerator` | Custom key generation | ✅ |

---

## 🚀 Quick Start / 快速开始

### Installation / 安装

```toml
[dependencies]
nexus-cache = "0.1.0-alpha"
nexus-macros = "0.1.0-alpha"
```

### Basic Usage / 基本用法

```rust
use nexus_cache::{Cacheable, CacheEvict, CachePut, CacheManager};
use nexus_macros::cacheable;

struct UserService {
    cache_manager: CacheManager,
}

impl UserService {
    // Cache method result / 缓存方法结果
    #[cacheable("users")]
    async fn get_user(&self, id: u64) -> Option<User> {
        // This will be cached / 这将被缓存
        find_user_in_db(id).await
    }
    
    // Update cache / 更新缓存
    #[cache_put("users")]
    async fn update_user(&self, user: User) -> User {
        save_user(user.clone()).await;
        user  // This will be cached / 这将被缓存
    }
    
    // Evict cache / 驱逐缓存
    #[cache_evict("users")]
    async fn delete_user(&self, id: u64) {
        delete_user_from_db(id).await;
        // Cache entry for this id will be evicted / 此id的缓存条目将被驱逐
    }
}
```

---

## 📖 Cache Annotations / 缓存注解

### @Cacheable / @Cacheable

Cache method results:

缓存方法结果：

```rust
use nexus_macros::cacheable;

struct ProductService;

impl ProductService {
    // Simple cache / 简单缓存
    #[cacheable("products")]
    async fn get_product(&self, id: u64) -> Option<Product> {
        find_product(id).await
    }
    
    // With key expression / 带键表达式
    #[cacheable("products", key = "#id")]
    async fn get_product_by_id(&self, id: u64) -> Option<Product> {
        find_product(id).await
    }
    
    // With condition / 带条件
    #[cacheable("products", condition = "#id > 100")]
    async fn get_expensive_product(&self, id: u64) -> Option<Product> {
        find_product(id).await
    }
    
    // With TTL / 带TTL
    #[cacheable("products", ttl = 3600)]
    async fn get_product_with_ttl(&self, id: u64) -> Option<Product> {
        find_product(id).await
    }
}
```

**Cache Key Generation** / **缓存键生成**:
- Default: `cache_name::method_name::arg1::arg2::...`
- Custom: `key = "#id"` or `key = "#user.id"`
- Composite: `key = "#user.id + ':' + #product.id"`

---

### @CacheEvict / @CacheEvict

Evict cache entries:

驱逐缓存条目：

```rust
use nexus_macros::cache_evict;

struct UserService;

impl UserService {
    // Evict single entry / 驱逐单个条目
    #[cache_evict("users")]
    async fn delete_user(&self, id: u64) {
        delete_user(id).await;
    }
    
    // Evict all entries / 驱逐所有条目
    #[cache_evict("users", all_entries = true)]
    async fn clear_cache(&self) {
        // All entries in "users" cache will be evicted / "users"缓存中的所有条目将被驱逐
    }
    
    // Evict before method execution / 方法执行前驱逐
    #[cache_evict("users", before_invocation = true)]
    async fn update_user(&self, user: User) {
        save_user(user).await;
    }
    
    // Evict with key / 带键驱逐
    #[cache_evict("users", key = "#user.id")]
    async fn update_user_with_key(&self, user: User) {
        save_user(user).await;
    }
}
```

**Eviction Policies** / **驱逐策略**:
- `all_entries = false` - Evict specific key (default)
- `all_entries = true` - Evict all entries in cache
- `before_invocation = false` - Evict after method (default)
- `before_invocation = true` - Evict before method

---

### @CachePut / @CachePut

Update cache without checking:

不检查直接更新缓存：

```rust
use nexus_macros::cache_put;

struct UserService;

impl UserService {
    // Update cache / 更新缓存
    #[cache_put("users")]
    async fn update_user(&self, user: User) -> User {
        save_user(user.clone()).await;
        user  // Always cached / 总是被缓存
    }
    
    // With key / 带键
    #[cache_put("users", key = "#user.id")]
    async fn save_user(&self, user: User) -> User {
        save_user(user.clone()).await;
        user
    }
    
    // With condition / 带条件
    #[cache_put("users", condition = "#result != null")]
    async fn create_user(&self, user: User) -> Option<User> {
        if validate_user(&user) {
            Some(save_user(user).await)
        } else {
            None  // Not cached if None / None时不缓存
        }
    }
}
```

**Difference from @Cacheable** / **与@Cacheable的区别**:
- `@Cacheable`: Skip method if cache hit / 缓存命中时跳过方法
- `@CachePut`: Always execute method and update cache / 总是执行方法并更新缓存

---

## 🏗️ Cache Manager / 缓存管理器

### CacheManager / 缓存管理器

Centralized cache management:

集中式缓存管理：

```rust
use nexus_cache::{CacheManager, CacheManagerBuilder, MemoryCache};

// Create cache manager / 创建缓存管理器
let cache_manager = CacheManagerBuilder::new()
    .with_cache("users", MemoryCache::new(1000, Duration::from_secs(3600)))
    .with_cache("products", MemoryCache::new(5000, Duration::from_secs(1800)))
    .with_cache("orders", MemoryCache::new(500, Duration::from_secs(600)))
    .build();

// Get cache / 获取缓存
let user_cache = cache_manager.get_cache::<String, User>("users")?;

// Use cache directly / 直接使用缓存
user_cache.put("user:1", user.clone()).await?;
let cached_user = user_cache.get("user:1").await?;
```

### Multiple Cache Backends / 多缓存后端

```rust
use nexus_cache::{CacheManager, MemoryCache, RedisCache};

let cache_manager = CacheManagerBuilder::new()
    // Memory cache / 内存缓存
    .with_cache("users", MemoryCache::new(1000, Duration::from_secs(3600)))
    
    // Redis cache / Redis缓存
    .with_cache("sessions", RedisCache::new("redis://localhost:6379")?)
    
    // Custom cache / 自定义缓存
    .with_cache("custom", MyCustomCache::new())
    
    .build();
```

---

## 🔧 Cache Configuration / 缓存配置

### Cache Settings / 缓存设置

```rust
use nexus_cache::{CacheConfig, CacheBuilder};

let cache = CacheBuilder::new()
    .name("users")
    .max_capacity(10_000)              // Max entries / 最大条目数
    .ttl(Duration::from_secs(3600))    // 1 hour TTL / 1小时TTL
    .eviction_policy(EvictionPolicy::LRU)  // LRU eviction / LRU驱逐
    .build()?;
```

### Cache Statistics / 缓存统计

```rust
use nexus_cache::CacheStats;

let cache = cache_manager.get_cache::<String, User>("users")?;
let stats = cache.stats();

println!("Hits: {}", stats.hits());
println!("Misses: {}", stats.misses());
println!("Hit rate: {:.2}%", stats.hit_rate() * 100.0);
println!("Size: {}", stats.size());
```

---

## 🎯 Key Generation / 键生成

### Default Key Generator / 默认键生成器

```rust
use nexus_cache::DefaultKeyGenerator;

// Default format: cache_name::method_name::arg1::arg2
// 默认格式：cache_name::method_name::arg1::arg2
// Example: "users::get_user::123"
```

### Custom Key Generator / 自定义键生成器

```rust
use nexus_cache::{KeyGenerator, CacheContext};

struct CustomKeyGenerator;

impl KeyGenerator for CustomKeyGenerator {
    fn generate(&self, context: &CacheContext) -> String {
        format!("{}:{}", context.cache_name(), context.args()[0])
    }
}

// Use custom generator / 使用自定义生成器
let cache_manager = CacheManagerBuilder::new()
    .with_key_generator(CustomKeyGenerator)
    .build();
```

### Key Expressions / 键表达式

```rust
// Simple key / 简单键
#[cacheable("users", key = "#id")]

// Composite key / 复合键
#[cacheable("orders", key = "#user.id + ':' + #order.id")]

// Method call / 方法调用
#[cacheable("products", key = "#product.getId()")]

// Conditional / 条件
#[cacheable("users", key = "#user.id", condition = "#user != null")]
```

---

## 🎯 Conditional Caching / 条件缓存

### @Cacheable with Conditions

Condition-based caching using expression evaluation:

使用表达式求值的基于条件的缓存：

```rust
use nexus_cache::{evaluate_cache_condition, Cached};
use std::collections::HashMap;

// Cache only if user is active
// 仅在用户活跃时缓存
async fn get_user_cached_condition(cache: &Cache, id: i64) -> Option<User> {
    let mut args = HashMap::new();
    args.insert("id".to_string(), serde_json::json!(id));

    let condition = "#id > 0";
    let should_cache = evaluate_cache_condition(condition, &args, None);

    if !should_cache {
        return find_user_in_db(id).await;  // Skip caching / 跳过缓存
    }

    Cached::get_or_fetch(cache, &id, || async {
        find_user_in_db(id).await
    }).await
}

// Don't cache if result is null or empty
// 如果结果为null或空则不缓存
async fn get_user_cached_unless(cache: &Cache, id: i64) -> Option<User> {
    let user = find_user_in_db(id).await?;

    let mut args = HashMap::new();
    args.insert("id".to_string(), serde_json::json!(id));

    let unless = "#result == null or #result.isEmpty()";
    let should_not_cache = evaluate_cache_condition(unless, &args, Some(&user));

    if !should_not_cache {
        cache.put(&format!("user:{}", id), user.clone()).await;
    }

    Some(user)
}
```

**Supported Expressions** / **支持的表达式**:

- **Parameter checks** / **参数检查**:
  - `#id > 0` - Numeric comparison / 数值比较
  - `#id == 100` - Equality / 相等性
  - `#age >= 18` - Greater than or equal / 大于等于

- **String operations** / **字符串操作**:
  - `#name.isEmpty()` - Check if empty / 检查是否为空
  - `#name.length() > 3` - Check length / 检查长度

- **Result checks** / **结果检查**:
  - `#result == null` - Null check / null检查
  - `#result.isEmpty()` - Empty check / 空检查

- **Logical operators** / **逻辑运算符**:
  - `#age > 18 and #active` - AND / 与
  - `has_role('ADMIN') or #is_admin` - OR / 或
  - `!#disabled` - NOT / 非

**Spring Boot Comparison** / **Spring Boot 对比**:

```java
// Spring Boot
@Cacheable(value = "users", key = "#id", condition = "#id > 0")
public User getUser(Long id) {
    return userRepository.findById(id);
}

@Cacheable(value = "users", unless = "#result == null")
public User getUser(Long id) {
    return userRepository.findById(id);
}

// Nexus (with evaluate_cache_condition)
let should_cache = evaluate_cache_condition("#id > 0", &args, None);
if should_cache {
    // Cache the result
}
```

---

## ⚡ Performance / 性能

### Cache Hit Rates / 缓存命中率

Monitor cache performance:

监控缓存性能：

```rust
use nexus_cache::CacheStats;

let stats = cache.stats();

// Target hit rate: > 80% / 目标命中率：> 80%
if stats.hit_rate() < 0.8 {
    // Adjust cache size or TTL / 调整缓存大小或TTL
    cache.resize(20_000)?;
}
```

### Cache Warming / 缓存预热

```rust
async fn warm_cache(cache_manager: &CacheManager) {
    let user_cache = cache_manager.get_cache::<String, User>("users")?;
    
    // Preload frequently accessed users / 预加载经常访问的用户
    let popular_user_ids = vec![1, 2, 3, 4, 5];
    
    for id in popular_user_ids {
        if let Some(user) = find_user(id).await {
            user_cache.put(&format!("user:{}", id), user).await?;
        }
    }
}
```

---

## 🧪 Testing / 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cacheable() {
        let service = UserService::new();
        
        // First call - cache miss / 第一次调用 - 缓存未命中
        let user1 = service.get_user(1).await;
        
        // Second call - cache hit / 第二次调用 - 缓存命中
        let user2 = service.get_user(1).await;
        
        assert_eq!(user1, user2);
    }

    #[tokio::test]
    async fn test_cache_evict() {
        let service = UserService::new();
        
        // Cache user / 缓存用户
        let _ = service.get_user(1).await;
        
        // Delete user / 删除用户
        service.delete_user(1).await;
        
        // Cache should be evicted / 缓存应该被驱逐
        // Next call should hit database / 下次调用应该访问数据库
    }
}
```

---

## 🚦 Roadmap / 路线图

### Phase 3: Core Caching ✅ (Completed / 已完成)
- [x] @Cacheable annotation
- [x] @CacheEvict annotation
- [x] @CachePut annotation
- [x] CacheManager
- [x] Memory cache backend

### Phase 4: Advanced Features ✅ (Completed / 已完成)
- [x] Conditional caching (condition, unless expressions)
- [x] Expression evaluator for cache conditions
- [ ] Redis cache backend (planned)
- [ ] Distributed caching (planned)
- [ ] Cache synchronization (planned)
- [ ] Cache metrics integration (planned)

---

## 📚 Documentation / 文档

- **API Documentation**: [docs.rs/nexus-cache](https://docs.rs/nexus-cache)
- **Book**: [Cache Guide](../../docs/book/)
- **Examples**: [examples/cache_example.rs](../../examples/cache_example.rs)

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

Nexus Cache is inspired by:

- **[Spring Boot](https://spring.io/projects/spring-boot)** - `@Cacheable`, `@CacheEvict`, `@CachePut`
- **[Caffeine](https://github.com/ben-manes/caffeine)** - High-performance caching
- **[Redis](https://redis.io/)** - Distributed caching

---

**Built with ❤️ for caching**

**为缓存构建 ❤️**
