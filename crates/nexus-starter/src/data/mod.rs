//! Data 自动配置模块 / Data Auto-Configuration Module
//!
//! 自动配置数据源和事务管理。
//! Auto-configures data source and transaction management.

use crate::core::{AutoConfiguration, ApplicationContext};

// ============================================================================
// DataSourceAutoConfiguration / 数据源自动配置
// ============================================================================

/// 数据源自动配置
/// Data source auto-configuration
///
/// 参考 Spring Boot 的 `DataSourceAutoConfiguration`。
/// Based on Spring Boot's `DataSourceAutoConfiguration`.
#[derive(Debug)]
pub struct DataSourceAutoConfiguration {
    /// 数据源 URL
    pub url: Option<String>,

    /// 用户名
    pub username: Option<String>,

    /// 密码
    pub password: Option<String>,

    /// 最大连接数
    pub max_connections: u32,
}

impl DataSourceAutoConfiguration {
    /// 创建新的数据源自动配置
    pub fn new() -> Self {
        Self {
            url: None,
            username: None,
            password: None,
            max_connections: 10,
        }
    }

    /// 从配置创建
    pub fn from_config(ctx: &ApplicationContext) -> Self {
        Self {
            url: ctx.get_property("datasource.url"),
            username: ctx.get_property("datasource.username"),
            password: ctx.get_property("datasource.password"),
            max_connections: ctx
                .get_property("datasource.max_connections")
                .and_then(|p| p.parse().ok())
                .unwrap_or(10),
        }
    }
}

impl Default for DataSourceAutoConfiguration {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoConfiguration for DataSourceAutoConfiguration {
    fn name(&self) -> &'static str {
        "DataSourceAutoConfiguration"
    }

    fn order(&self) -> i32 {
        -50  // 较高优先级，在其他配置之前
    }

    fn condition(&self) -> bool {
        self.url.is_some()
    }

    fn configure(&self, ctx: &mut ApplicationContext) -> anyhow::Result<()> {
        tracing::info!("Configuring DataSource");

        if let Some(ref url) = self.url {
            tracing::info!("  URL: {}", url);
            tracing::info!("  Max connections: {}", self.max_connections);

            // TODO: 创建并注册 DataSource Bean
            // let datasource = DataSource::new(url, username, password);
            // ctx.register_bean(datasource);
        }

        Ok(())
    }
}

// ============================================================================
// TransactionAutoConfiguration / 事务自动配置
// ============================================================================

/// 事务自动配置
/// Transaction auto-configuration
///
/// 配置事务管理器。
/// Configures transaction manager.
#[derive(Debug)]
pub struct TransactionAutoConfiguration;

impl AutoConfiguration for TransactionAutoConfiguration {
    fn name(&self) -> &'static str {
        "TransactionAutoConfiguration"
    }

    fn order(&self) -> i32 {
        50  // 在数据源配置之后
    }

    fn configure(&self, ctx: &mut ApplicationContext) -> anyhow::Result<()> {
        tracing::info!("Configuring Transaction Manager");

        // TODO: 创建并注册 TransactionManager
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
    fn test_data_source_auto_config() {
        let config = DataSourceAutoConfiguration::new();
        assert!(config.url.is_none());
        assert_eq!(config.max_connections, 10);
    }
}
