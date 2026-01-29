//! Actuator 自动配置模块 / Actuator Auto-Configuration Module
//!
//! 自动配置监控端点（健康检查、指标等）。
//! Auto-configures monitoring endpoints (health check, metrics, etc.).

use crate::core::{AutoConfiguration, ApplicationContext};

// ============================================================================
// ActuatorAutoConfiguration / Actuator 自动配置
// ============================================================================

/// Actuator 自动配置
/// Actuator auto-configuration
///
/// 参考 Spring Boot 的 `ActuatorAutoConfiguration`。
/// Based on Spring Boot's `ActuatorAutoConfiguration`.
#[derive(Debug)]
pub struct ActuatorAutoConfiguration {
    /// 是否启用 Actuator
    pub enabled: bool,

    /// 是否启用健康检查端点
    pub health_enabled: bool,

    /// 是否启用指标端点
    pub metrics_enabled: bool,

    /// Actuator 基础路径
    pub base_path: String,
}

impl ActuatorAutoConfiguration {
    /// 创建新的 Actuator 自动配置
    pub fn new() -> Self {
        Self {
            enabled: true,
            health_enabled: true,
            metrics_enabled: true,
            base_path: "/actuator".to_string(),
        }
    }

    /// 从配置创建
    pub fn from_config(ctx: &ApplicationContext) -> Self {
        Self {
            enabled: ctx
                .get_property("actuator.enabled")
                .and_then(|p| p.parse().ok())
                .unwrap_or(true),
            health_enabled: ctx
                .get_property("actuator.health.enabled")
                .and_then(|p| p.parse().ok())
                .unwrap_or(true),
            metrics_enabled: ctx
                .get_property("actuator.metrics.enabled")
                .and_then(|p| p.parse().ok())
                .unwrap_or(true),
            base_path: ctx
                .get_property_or("actuator.base_path", "/actuator"),
        }
    }
}

impl Default for ActuatorAutoConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoConfiguration for ActuatorAutoConfiguration {
    fn name(&self) -> &'static str {
        "ActuatorAutoConfiguration"
    }

    fn order(&self) -> i32 {
        200  // 较低优先级，在业务配置之后
    }

    fn configure(&self, ctx: &mut ApplicationContext) -> anyhow::Result<()> {
        tracing::info!("Configuring Actuator (base path: {})", self.base_path);

        if self.health_enabled {
            tracing::info!("  Health endpoint: {}/health", self.base_path);
            // TODO: 注册健康检查端点
        }

        if self.metrics_enabled {
            tracing::info!("  Metrics endpoint: {}/metrics", self.base_path);
            // TODO: 注册指标端点
        }

        // TODO: 注册其他端点（info, env, beans 等）

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
    fn test_actuator_auto_config() {
        let config = ActuatorAutoConfiguration::new();
        assert!(config.enabled);
        assert!(config.health_enabled);
        assert!(config.metrics_enabled);
        assert_eq!(config.base_path, "/actuator");
    }
}
