//! Nexus Cache - Caching abstraction module
//! Nexus缓存 - 缓存抽象模块
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - `@Cacheable` - Cacheable
//! - `@CacheEvict` - CacheEvict
//! - `@CachePut` - CachePut
//! - `@Caching` - Caching (multi-operation)
//! - `@CacheConfig` - CacheConfig
//! - CacheManager - CacheManager
//! - `@EnableCaching` - EnableCaching
//!
//! # Example / 示例
//!
//! ```rust,no_run,ignore
//! use nexus_cache::{Cache, Cacheable, CachePut, CacheEvict};
//! use std::sync::Arc;
//!
//! struct UserService {
//!     cache: Arc<Cache<String, User>>,
//! }
//!
//! impl UserService {
//!     // Equivalent to @Cacheable("users")
//!     #[cacheable("users")]
//!     async fn get_user(&self, id: &str) -> Option<User> {
//!         // Implementation
//!         None
//!     }
//!
//!     // Equivalent to @CachePut("users")
//!     #[cache_put("users")]
//!     async fn update_user(&self, user: User) -> User {
//!         // Implementation
//!         user
//!     }
//!
//!     // Equivalent to @CacheEvict("users")
//!     #[cache_evict("users")]
//!     async fn delete_user(&self, id: &str) {
//!         // Implementation
//!     }
//! }
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]

mod cache;
mod cacheable;
mod cache_config;
mod cache_evict;
mod cache_manager;
mod cache_put;
mod key_generator;
mod resolver;

pub use cache::{Cache, CacheBuilder, CacheConfig, CacheStats, MemoryCache};
pub use cacheable::{Cacheable, Cached};
pub use cache_config::CacheConfig as CacheSettings;
pub use cache_evict::{CacheEvict, EvictPolicy};
pub use cache_manager::{CacheManager, CacheManagerBuilder};
pub use cache_put::CachePut;
pub use key_generator::{KeyGenerator, DefaultKeyGenerator, HashKeyGenerator};
pub use resolver::{CacheResolver, SimpleCacheResolver};

/// Re-exports of commonly used types
/// 常用类型的重新导出
pub mod prelude {
    pub use super::{
        Cache, CacheBuilder, CacheConfig, CacheManager, CacheManagerBuilder,
        Cacheable, CacheEvict, CachePut, Cached,
    };
}

/// Version of the cache module
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default cache name
/// 默认缓存名称
pub const DEFAULT_CACHE: &str = "default";

/// Default TTL (time to live) in seconds
/// 默认TTL（生存时间），单位秒
pub const DEFAULT_TTL_SECS: u64 = 600; // 10 minutes

/// Default maximum cache size
/// 默认最大缓存大小
pub const DEFAULT_MAX_CAPACITY: usize = 10_000;
