//! Cache 自动配置模块 / Cache Auto-Configuration Module
//!
//! 自动配置缓存功能。
//! Auto-configures caching functionality.

use crate::core::{AutoConfiguration, ApplicationContext};

// ============================================================================
// CacheAutoConfiguration / 缓存自动配置
// ============================================================================

/// 缓存自动配置
/// Cache auto-configuration
///
/// 参考 Spring Boot 的 `CacheAutoConfiguration`。
/// Based on Spring Boot's `CacheAutoConfiguration`.
#[derive(Debug)]
pub struct CacheAutoConfiguration {
    /// 是否启用缓存
    pub enabled: bool,

    /// 缓存 TTL（秒）
    pub ttl: u64,

    /// 最大缓存条目数
    pub max_size: usize,
}

impl CacheAutoConfiguration {
    /// 创建新的缓存自动配置
    pub fn new() -> Self {
        Self {
            enabled: false,
            ttl: 600,
            max_size: 10000,
        }
    }

    /// 从配置创建
    pub fn from_config(ctx: &ApplicationContext) -> Self {
        Self {
            enabled: ctx
                .get_property("cache.enabled")
                .and_then(|p| p.parse().ok())
                .unwrap_or(false),
            ttl: ctx
                .get_property("cache.ttl")
                .and_then(|p| p.parse().ok())
                .unwrap_or(600),
            max_size: ctx
                .get_property("cache.max_size")
                .and_then(|p| p.parse().ok())
                .unwrap_or(10000),
        }
    }
}

impl Default for CacheAutoConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoConfiguration for CacheAutoConfiguration {
    fn name(&self) -> &'static str {
        "CacheAutoConfiguration"
    }

    fn order(&self) -> i32 {
        100  // 在核心配置之后
    }

    fn condition(&self) -> bool {
        self.enabled
    }

    fn configure(&self, ctx: &mut ApplicationContext) -> anyhow::Result<()> {
        tracing::info!("Configuring Cache (TTL: {}s, Max size: {})", self.ttl, self.max_size);

        // TODO: 创建并注册 CacheManager
        // let cache = MemoryCache::new(self.ttl, self.max_size);
        // ctx.register_bean(cache);

        Ok(())
    }
}

// ============================================================================
// 测试 / Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_auto_config() {
        let config = CacheAutoConfiguration::new();
        assert!(!config.enabled);
        assert_eq!(config.ttl, 600);
        assert_eq!(config.max_size, 10000);
    }
}
