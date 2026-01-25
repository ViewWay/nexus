//! # Cache Annotations with Conditions Examples
//! # 带条件的缓存注解示例
//!
//! This example demonstrates advanced caching with condition and unless expressions
//! 本示例演示使用条件表达式的先进缓存功能
//!
//! ## Run Example / 运行示例
//!
//! ```bash
//! cargo run --example cache_with_conditions
//! ```

use nexus_cache::{
    Cache, CacheBuilder, evaluate_cache_condition,
    CacheableOptions, CachePutOptions, CacheEvictOptions
};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::Value as JsonValue;

// ========================================================================
// Domain Models / 领域模型
// ========================================================================

/// User entity
/// 用户实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub age: i32,
    pub active: bool,
    pub role: String,
}

/// Product entity
/// 商品实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub available: bool,
}

// ========================================================================
// Mock Repositories / 模拟仓库
// ========================================================================

/// In-memory user store
/// 内存用户存储
pub struct UserStore {
    users: Arc<RwLock<HashMap<i64, User>>>,
}

impl UserStore {
    pub fn new() -> Self {
        let mut users = HashMap::new();

        // Add sample users
        users.insert(1, User {
            id: 1,
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
            age: 35,
            active: true,
            role: "ADMIN".to_string(),
        });

        users.insert(2, User {
            id: 2,
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            age: 28,
            active: true,
            role: "USER".to_string(),
        });

        users.insert(3, User {
            id: 3,
            username: "bob_inactive".to_string(),
            email: "bob@example.com".to_string(),
            age: 45,
            active: false,
            role: "USER".to_string(),
        });

        Self {
            users: Arc::new(RwLock::new(users)),
        }
    }

    pub async fn find_by_id(&self, id: i64) -> Option<User> {
        self.users.read().await.get(&id).cloned()
    }

    pub async fn save(&self, user: User) -> User {
        let mut users = self.users.write().await;
        users.insert(user.id, user.clone());
        user
    }

    pub async fn delete(&self, id: i64) {
        self.users.write().await.remove(&id);
    }
}

// ========================================================================
// Enhanced Cacheable Functions / 增强的可缓存函数
// ========================================================================

/// Get user with cache - Cache only if user is active
/// 获取用户带缓存 - 仅缓存活跃用户
///
/// Spring Equivalent:
/// ```java
/// @Cacheable(
///     value = "users",
///     key = "#id",
///     condition = "#id > 0"
/// )
/// ```
async fn get_user_cached_condition(
    cache: &Arc<MemoryCache<String, User>>,
    store: &UserStore,
    id: i64,
) -> Option<User> {
    // Build arguments map for condition evaluation
    let mut args = HashMap::new();
    args.insert("id".to_string(), JsonValue::Number(id.into()));

    // Evaluate condition: Cache only if id > 0
    let condition = "#id > 0";
    let should_cache = evaluate_cache_condition(condition, &args, None);

    if !should_cache {
        println!("  ⚠️  Condition failed: {}, skipping cache", condition);
        return store.find_by_id(id).await;
    }

    let key = format!("user:{}", id);

    // Try to get from cache
    if let Some(user) = cache.get(&key).await {
        println!("  ✅ Cache hit for user {}", id);
        return Some(user);
    }

    println!("  💾 Cache miss, fetching from store");

    // Fetch from store
    if let Some(user) = store.find_by_id(id).await {
        // Cache the result
        cache.put(key, user.clone()).await;
        Some(user)
    } else {
        None
    }
}

/// Get user with cache - Don't cache inactive users
/// 获取用户带缓存 - 不缓存非活跃用户
///
/// Spring Equivalent:
/// ```java
/// @Cacheable(
///     value = "users",
///     key = "#id",
///     unless = "#result != null and !#result.active"
/// )
/// ```
async fn get_user_cached_unless(
    cache: &Arc<MemoryCache<String, User>>,
    store: &UserStore,
    id: i64,
) -> Option<User> {
    let key = format!("user:{}", id);

    // Try to get from cache
    if let Some(user) = cache.get(&key).await {
        println!("  ✅ Cache hit for user {}", id);
        return Some(user);
    }

    println!("  💾 Cache miss, fetching from store");

    // Fetch from store
    let user = store.find_by_id(id).await;

    if let Some(ref user) = user {
        // Build arguments for unless evaluation
        let mut args = HashMap::new();
        args.insert("id".to_string(), JsonValue::Number(id.into()));

        // Convert user to JSON for #result evaluation
        let result_json = serde_json::to_value(user).ok();

        // Evaluate unless: Don't cache if user is inactive
        let unless = "#result != null and !#result.active";
        let should_not_cache = evaluate_cache_condition(unless, &args, result_json.as_ref());

        if should_not_cache {
            println!("  ⚠️  Unless condition met, NOT caching inactive user {}", id);
        } else {
            println!("  💾 Caching active user {}", id);
            cache.put(key, user.clone()).await;
        }
    }

    user
}

/// Update user and cache - Only cache if user is active
/// 更新用户并缓存 - 仅缓存活跃用户
///
/// Spring Equivalent:
/// ```java
/// @CachePut(
///     value = "users",
///     key = "#user.id",
///     condition = "#user.active"
/// )
/// ```
async fn update_user_cached_condition(
    cache: &Arc<MemoryCache<String, User>>,
    store: &UserStore,
    user: User,
) -> User {
    // Build arguments for condition evaluation
    let user_json = serde_json::to_value(&user).ok();
    let mut args = HashMap::new();

    if let Some(ref json) = user_json {
        if let Some(id) = json.get("id") {
            args.insert("user.id".to_string(), id.clone());
        }
        if let Some(active) = json.get("active") {
            args.insert("user.active".to_string(), active.clone());
        }
    }

    // Evaluate condition: Only cache if user is active
    let condition = "#user.active == true";
    let should_cache = evaluate_cache_condition(condition, &args, None);

    // Update in store
    let updated_user = store.save(user.clone()).await;

    if should_cache {
        let key = format!("user:{}", updated_user.id);
        cache.put(key, updated_user.clone()).await;
        println!("  💾 Cached updated user {} (condition met)", updated_user.id);
    } else {
        println!("  ⚠️  Condition failed, NOT caching inactive user", updated_user.id);
    }

    updated_user
}

/// Delete user and evict from cache - Only evict if user exists
/// 删除用户并从缓存驱逐 - 仅当用户存在时驱逐
///
/// Spring Equivalent:
/// ```java
/// @CacheEvict(
///     value = "users",
///     key = "#id",
///     condition = "#id > 0"
/// )
/// ```
async fn delete_user_cached_condition(
    cache: &Arc<MemoryCache<String, User>>,
    store: &UserStore,
    id: i64,
) {
    // Build arguments for condition evaluation
    let mut args = HashMap::new();
    args.insert("id".to_string(), JsonValue::Number(id.into()));

    // Evaluate condition: Only evict if id > 0
    let condition = "#id > 0";
    let should_evict = evaluate_cache_condition(condition, &args, None);

    if should_evict {
        // Delete from store
        store.delete(id).await;

        // Evict from cache
        let key = format!("user:{}", id);
        cache.invalidate(&key).await;
        println!("  🗑️  Deleted and evicted user {}", id);
    } else {
        println!("  ⚠️  Condition failed, NOT deleting user {}", id);
    }
}

/// Get all users - Only cache adults (age >= 18)
/// 获取所有用户 - 仅缓存成年人（年龄 >= 18）
///
/// Spring Equivalent:
/// ```java
/// @Cacheable(
///     value = "users",
///     condition = "#minAge >= 18"
/// )
/// ```
async fn get_users_by_min_age(
    cache: &Arc<MemoryCache<String, Vec<User>>>,
    store: &UserStore,
    min_age: i32,
) -> Vec<User> {
    let mut args = HashMap::new();
    args.insert("minAge".to_string(), JsonValue::Number(min_age.into()));

    let condition = "#minAge >= 18";
    let should_cache = evaluate_cache_condition(condition, &args, None);

    if !should_cache {
        println!("  ⚠️  Condition failed: {}, skipping cache", condition);
        // Return all users
        return store.users.read().await.values().cloned().collect();
    }

    let key = format!("users:min_age:{}", min_age);

    // Try to get from cache
    if let Some(users) = cache.get(&key).await {
        println!("  ✅ Cache hit for users with min_age {}", min_age);
        return users;
    }

    println!("  💾 Cache miss, fetching from store");

    // Fetch from store
    let users: Vec<User> = store.users.read().await
        .values()
        .filter(|u| u.age >= min_age)
        .cloned()
        .collect();

    cache.put(key, users.clone()).await;
    users
}

/// Cache alias
pub type MemoryCache<K, V> = dyn Cache<K, V> + Send + Sync;

// ========================================================================
// Examples / 示例
// ========================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║   Cache Annotations with Conditions / 带条件的缓存注解示例      ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Create caches
    let user_cache: Arc<MemoryCache<String, User>> = Arc::new(
        CacheBuilder::new()
            .max_capacity(100)
            .time_to_live_secs(60)
            .build()
    );

    let users_cache: Arc<MemoryCache<String, Vec<User>>> = Arc::new(
        CacheBuilder::new()
            .max_capacity(50)
            .time_to_live_secs(120)
            .build()
    );

    let store = UserStore::new();

    // ========================================================================
    // Example 1: Cache with condition
    // ========================================================================
    println!("📋 Example 1: Cache with condition / 示例 1：带条件的缓存");
    println!("─────────────────────────────────────────────────────────────");
    println!("Condition: #id > 0");
    println!();

    println!("Fetching user ID 2 (should cache):");
    get_user_cached_condition(&user_cache, &store, 2).await;
    println!();

    println!("Fetching user ID 2 again (cache hit):");
    get_user_cached_condition(&user_cache, &store, 2).await;
    println!();

    println!("Fetching user ID -1 (should NOT cache):");
    get_user_cached_condition(&user_cache, &store, -1).await;
    println!();

    // ========================================================================
    // Example 2: Cache with unless
    // ========================================================================
    println!("📋 Example 2: Cache with unless / 示例 2：带 unless 的缓存");
    println!("─────────────────────────────────────────────────────────────");
    println!("Unless: #result != null and !#result.active");
    println!();

    println!("Fetching active user ID 2 (should cache):");
    get_user_cached_unless(&user_cache, &store, 2).await;
    println!();

    println!("Fetching inactive user ID 3 (should NOT cache):");
    get_user_cached_unless(&user_cache, &store, 3).await;
    println!();

    // ========================================================================
    // Example 3: CachePut with condition
    // ========================================================================
    println!("📋 Example 3: CachePut with condition / 示例 3：带条件的 CachePut");
    println!("─────────────────────────────────────────────────────────────");
    println!("Condition: #user.active == true");
    println!();

    let mut user = User {
        id: 2,
        username: "alice_updated".to_string(),
        email: "alice.updated@example.com".to_string(),
        age: 29,
        active: true,
        role: "USER".to_string(),
    };

    println!("Updating active user 2 (should cache):");
    update_user_cached_condition(&user_cache, &store, user.clone()).await;
    println!();

    user.id = 3;
    user.username = "bob_updated".to_string();
    user.active = false;

    println!("Updating inactive user 3 (should NOT cache):");
    update_user_cached_condition(&user_cache, &store, user).await;
    println!();

    // ========================================================================
    // Example 4: CacheEvict with condition
    // ========================================================================
    println!("📋 Example 4: CacheEvict with condition / 示例 4：带条件的 CacheEvict");
    println!("─────────────────────────────────────────────────────────────");
    println!("Condition: #id > 0");
    println!();

    println!("Deleting user ID 1 (should evict):");
    delete_user_cached_condition(&user_cache, &store, 1).await;
    println!();

    println!("Deleting user ID -1 (should NOT evict):");
    delete_user_cached_condition(&user_cache, &store, -1).await;
    println!();

    // ========================================================================
    // Example 5: Complex conditions with AND/OR
    // ========================================================================
    println!("📋 Example 5: Complex conditions / 示例 5：复杂条件");
    println!("─────────────────────────────────────────────────────────────");
    println!("Condition: #minAge >= 18");
    println!();

    println!("Fetching users with min_age 18 (should cache):");
    let users = get_users_by_min_age(&users_cache, &store, 18).await;
    println!("Found {} users", users.len());
    println!();

    println!("Fetching users with min_age 18 again (cache hit):");
    let users = get_users_by_min_age(&users_cache, &store, 18).await;
    println!("Found {} users", users.len());
    println!();

    println!("Fetching users with min_age 16 (should NOT cache):");
    let users = get_users_by_min_age(&users_cache, &store, 16).await;
    println!("Found {} users", users.len());
    println!();

    // ========================================================================
    // Example 6: Advanced condition examples
    // ========================================================================
    println!("📋 Example 6: Advanced condition evaluation / 示例 6：高级条件求值");
    println!("─────────────────────────────────────────────────────────────");
    println!();

    let mut args = HashMap::new();
    args.insert("age".to_string(), JsonValue::Number(25.into()));
    args.insert("active".to_string(), JsonValue::Bool(true));
    args.insert("role".to_string(), JsonValue::String("ADMIN".to_string()));
    args.insert("username".to_string(), JsonValue::String("alice".to_string()));
    args.insert("name".to_string(), JsonValue::String("Bo".to_string()));

    println!("Evaluating: #age > 18 and #active");
    println!("Result: {}", evaluate_cache_condition("#age > 18 and #active", &args, None));
    println!();

    println!("Evaluating: #role == 'ADMIN' or #active");
    println!("Result: {}", evaluate_cache_condition("#role == 'ADMIN' or #active", &args, None));
    println!();

    println!("Evaluating: #username.length() > 3");
    println!("Result: {}", evaluate_cache_condition("#username.length() > 3", &args, None));
    println!();

    println!("Evaluating: #name.length() > 3");
    println!("Result: {}", evaluate_cache_condition("#name.length() > 3", &args, None));
    println!();

    println!("Evaluating: !#active");
    println!("Result: {}", evaluate_cache_condition("!#active", &args, None));
    println!();

    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║              Examples completed! / 示例完成！                      ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    Ok(())
}
