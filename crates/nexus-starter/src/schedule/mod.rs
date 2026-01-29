//! Schedule 自动配置模块 / Schedule Auto-Configuration Module
//!
//! 自动配置定时任务功能。
//! Auto-configures scheduled task functionality.

use crate::core::{AutoConfiguration, ApplicationContext};

// ============================================================================
// ScheduleAutoConfiguration / 定时任务自动配置
// ============================================================================

/// 定时任务自动配置
/// Schedule auto-configuration
///
/// 参考 Spring Boot 的 `TaskSchedulingAutoConfiguration`。
/// Based on Spring Boot's `TaskSchedulingAutoConfiguration`.
#[derive(Debug)]
pub struct ScheduleAutoConfiguration {
    /// 是否启用定时任务
    pub enabled: bool,

    /// 线程池大小
    pub pool_size: usize,
}

impl ScheduleAutoConfiguration {
    /// 创建新的定时任务自动配置
    pub fn new() -> Self {
        Self {
            enabled: false,
            pool_size: 4,
        }
    }

    /// 从配置创建
    pub fn from_config(ctx: &ApplicationContext) -> Self {
        Self {
            enabled: ctx
                .get_property("schedule.enabled")
                .and_then(|p| p.parse().ok())
                .unwrap_or(false),
            pool_size: ctx
                .get_property("schedule.pool_size")
                .and_then(|p| p.parse().ok())
                .unwrap_or(4),
        }
    }
}

impl Default for ScheduleAutoConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoConfiguration for ScheduleAutoConfiguration {
    fn name(&self) -> &'static str {
        "ScheduleAutoConfiguration"
    }

    fn order(&self) -> i32 {
        100  // 在核心配置之后
    }

    fn condition(&self) -> bool {
        self.enabled
    }

    fn configure(&self, ctx: &mut ApplicationContext) -> anyhow::Result<()> {
        tracing::info!("Configuring Scheduled Tasks (Pool size: {})", self.pool_size);

        // TODO: 创建并注册 TaskScheduler
        // let scheduler = TaskScheduler::new(self.pool_size);
        // ctx.register_bean(scheduler);

        // 扫描 @Scheduled 注解的方法
        // 扫描 @cron、@fixed_rate、@fixed_delay 注解
        // 注册定时任务

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
    fn test_schedule_auto_config() {
        let config = ScheduleAutoConfiguration::new();
        assert!(!config.enabled);
        assert_eq!(config.pool_size, 4);
    }
}
