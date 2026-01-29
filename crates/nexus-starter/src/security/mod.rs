//! Security 自动配置模块 / Security Auto-Configuration Module
//!
//! 自动配置安全功能（认证、授权、JWT）。
//! Auto-configures security features (authentication, authorization, JWT).

use crate::core::{AutoConfiguration, ApplicationContext};

// ============================================================================
// SecurityAutoConfiguration / 安全自动配置
// ============================================================================

/// Security 自动配置
/// Security auto-configuration
///
/// 参考 Spring Boot 的 `SecurityAutoConfiguration`。
/// Based on Spring Boot's `SecurityAutoConfiguration`.
#[derive(Debug)]
pub struct SecurityAutoConfiguration {
    /// 是否启用安全
    pub enabled: bool,

    /// JWT 密钥
    pub jwt_secret: Option<String>,

    /// JWT 过期时间（秒）
    pub jwt_expiration: u64,
}

impl SecurityAutoConfiguration {
    /// 创建新的安全自动配置
    pub fn new() -> Self {
        Self {
            enabled: true,
            jwt_secret: None,
            jwt_expiration: 3600,
        }
    }

    /// 从配置创建
    pub fn from_config(ctx: &ApplicationContext) -> Self {
        Self {
            enabled: ctx
                .get_property("security.enabled")
                .and_then(|p| p.parse().ok())
                .unwrap_or(true),
            jwt_secret: ctx.get_property("security.jwt_secret"),
            jwt_expiration: ctx
                .get_property("security.jwt_expiration")
                .and_then(|p| p.parse().ok())
                .unwrap_or(3600),
        }
    }
}

impl Default for SecurityAutoConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoConfiguration for SecurityAutoConfiguration {
    fn name(&self) -> &'static str {
        "SecurityAutoConfiguration"
    }

    fn order(&self) -> i32 {
        50  // 在服务器配置之后
    }

    fn condition(&self) -> bool {
        self.enabled
    }

    fn configure(&self, ctx: &mut ApplicationContext) -> anyhow::Result<()> {
        tracing::info!("Configuring Security");

        if let Some(ref secret) = self.jwt_secret {
            tracing::info!("JWT authentication enabled (expiration: {}s)", self.jwt_expiration);

            // TODO: 创建并注册 JwtTokenProvider
            // ctx.register_bean(JwtTokenProvider::new(secret));
        } else {
            tracing::warn!("JWT secret not configured, using default (insecure!)");
        }

        Ok(())
    }
}

// ============================================================================
// JwtAutoConfiguration / JWT 自动配置
// ============================================================================

/// JWT 自动配置
/// JWT auto-configuration
///
/// 配置 JWT 相关的 Bean。
/// Configures JWT-related beans.
#[derive(Debug)]
pub struct JwtAutoConfiguration;

impl AutoConfiguration for JwtAutoConfiguration {
    fn name(&self) -> &'static str {
        "JwtAutoConfiguration"
    }

    fn order(&self) -> i32 {
        60  // 在 SecurityAutoConfiguration 之后
    }

    fn configure(&self, ctx: &mut ApplicationContext) -> anyhow::Result<()> {
        tracing::info!("Configuring JWT");

        // TODO: 配置 JWT 验证、编码等
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
    fn test_security_auto_config() {
        let config = SecurityAutoConfiguration::new();
        assert!(config.enabled);
        assert_eq!(config.jwt_expiration, 3600);
    }
}
