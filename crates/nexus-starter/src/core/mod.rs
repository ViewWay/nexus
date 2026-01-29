//! 核心模块 / Core Module
//!
//! 包含自动配置、IoC 容器、组件扫描等核心功能。
//! Contains auto-configuration, IoC container, component scanning, etc.

pub mod autoconfig;
pub mod container;
pub mod scanner;
pub mod condition;
pub mod config;
pub mod loader;
pub mod logging;

// 重新导出常用类型
// Re-export commonly used types
pub use autoconfig::{
    AutoConfiguration,
    AutoConfigurationMetadata,
    order,
};

pub use container::{
    ApplicationContext,
    BeanDefinition,
    ComponentRegistry,
};

pub use scanner::ComponentScanner;
pub use condition::{
    Conditional,
    ConditionalOnProperty,
    ConditionalOnMissingBean,
};

pub use config::CoreAutoConfiguration;

pub use loader::{
    AutoConfigurationLoader,
    AutoConfigurationRegistry,
};
