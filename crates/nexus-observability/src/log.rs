//! Spring Boot Style Logging Module
//! Spring Boot 风格日志模块
//!
//! # Overview / 概述
//!
//! This module provides Spring Boot-style structured logging functionality.
//! 本模块提供 Spring Boot 风格的结构化日志功能。
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - SLF4J + Logback → tracing + tracing-subscriber
//! - Logger → tracing::info/warn/error/debug/trace!
//! - @Slf4j → #[nexus_observability::logger] macro
//!
//! # Example / 示例
//!
//! ```rust,no_run,ignore
//! use nexus_observability::log::{Logger, LoggerFactory};
//!
//! // Get logger (equivalent to LoggerFactory.getLogger(MyClass.class))
//! let log = LoggerFactory::get("my::module");
//!
//! // Log at different levels
//! log.error("Something went wrong: {}", error);
//! log.warn("Warning: deprecated API used");
//! log.info("Application started");
//! log.debug("Debugging: variable = {}", value);
//! log.trace("Detailed trace information");
//! ```
//!
//! # Configuration / 配置
//!
//! Environment variables:
//! - `NEXUS_LOG_LEVEL`: Global log level (TRACE, DEBUG, INFO, WARN, ERROR)
//! - `NEXUS_LOG_FORMAT`: log format (pretty, json, compact)
//! - `NEXUS_LOG_FILE`: Log file path (e.g., logs/application.log)
//! - `NEXUS_LOG_MAX_FILES`: Maximum number of log files to keep
//! - `NEXUS_LOG_MAX_SIZE`: Maximum size of each log file (in MB)

#![warn(missing_docs)]
#![warn(unreachable_pub)]

use std::sync::OnceLock;
use tracing::Level;
use tracing_subscriber::{
    fmt::{
        format::FmtSpan, self
    },
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Registry,
};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

#[cfg(feature = "nexus-format")]
use crate::nexus_format::{NexusFormatter, Banner};

/// Spring Boot log levels
/// Spring Boot 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// Trace level - most detailed
    /// TRACE 级别 - 最详细
    /// TRACE: 追踪信息, 比DEBUG更细粒度的信息事件(除非有特殊用意，否则请使用DEBUG级别替代)
    Trace = 0,
    /// Debug level
    /// DEBUG 级别
    /// DEBUG: 调试信息, 需要调试时候的关键信息打印
    Debug = 1,
    /// Info level (default)
    /// INFO 级别（默认）
    /// INFO: 普通信息, 用于记录应用程序正常运行时的一些信息, 例如系统启动完成、请求处理完成等
    Info = 2,
    /// Warning level
    /// WARN 级别
    /// WARN: 警告信息, 不影响使用, 但需要注意的问题
    Warn = 3,
    /// Error level
    /// ERROR 级别
    /// ERROR: 错误信息, 级别较高的错误日志信息, 但仍然不影响系统的继续运行
    Error = 4,
    /// Off - no logging
    /// OFF - 不记录日志
    /// OFF: 不记录日志
    Off = 5,
}

impl LogLevel {
    /// Parse log level from string
    /// 从字符串解析日志级别
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "TRACE" => Some(LogLevel::Trace),
            "DEBUG" => Some(LogLevel::Debug),
            "INFO" => Some(LogLevel::Info),
            "WARN" | "WARNING" => Some(LogLevel::Warn),
            "ERROR" | "ERR" => Some(LogLevel::Error),
            "OFF" => Some(LogLevel::Off),
            _ => None,
        }
    }

    /// Convert to tracing Level
    /// 转换为 tracing Level
    pub fn to_tracing_level(self) -> Option<Level> {
        match self {
            LogLevel::Trace => Some(Level::TRACE),
            LogLevel::Debug => Some(Level::DEBUG),
            LogLevel::Info => Some(Level::INFO),
            LogLevel::Warn => Some(Level::WARN),
            LogLevel::Error => Some(Level::ERROR),
            LogLevel::Off => None,
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Off => write!(f, "OFF"),
        }
    }
}

/// Log format style
/// 日志格式样式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    /// Pretty printed with colors (Spring Boot default console style)
    /// 美化打印带颜色（Spring Boot 默认控制台样式）
    Pretty,
    /// Compact single line format
    /// 紧凑单行格式
    Compact,
    /// JSON format (for log aggregation systems)
    /// JSON 格式（用于日志聚合系统）
    Json,
}

impl LogFormat {
    /// Parse from string
    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pretty" => Some(LogFormat::Pretty),
            "compact" => Some(LogFormat::Compact),
            "json" => Some(LogFormat::Json),
            _ => None,
        }
    }
}

/// Logger configuration
/// 日志配置器
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// Global log level
    /// 全局日志级别
    pub level: LogLevel,
    /// Log format style
    /// 日志格式样式
    pub format: LogFormat,
    /// Log file path (None = console only)
    /// 日志文件路径（None = 仅控制台）
    pub file_path: Option<String>,
    /// Whether to include thread ID in logs
    /// 是否在日志中包含线程ID
    pub with_thread: bool,
    /// Whether to include file and line number
    /// 是否包含文件和行号
    pub with_file: bool,
    /// Whether to include target (module path)
    /// 是否包含目标（模块路径）
    pub with_target: bool,
    /// Log rotation (DAILY, HOURLY, NEVER)
    /// 日志轮转（DAILY, HOURLY, NEVER）
    pub rotation: LogRotation,
    /// Maximum number of log files to keep
    /// 保留的最大日志文件数
    pub max_files: usize,
}

/// Log rotation policy
/// 日志轮转策略
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogRotation {
    /// Never rotate
    /// 从不轮转
    Never,
    /// Rotate daily (Spring Boot default)
    /// 每天轮转（Spring Boot 默认）
    Daily,
    /// Rotate hourly
    /// 每小时轮转
    Hourly,
    /// Rotate every minute (for testing)
    /// 每分钟轮转（用于测试）
    Minutely,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        let level = std::env::var("NEXUS_LOG_LEVEL")
            .ok()
            .and_then(|s| LogLevel::from_str(&s))
            .unwrap_or(LogLevel::Info);

        let format = std::env::var("NEXUS_LOG_FORMAT")
            .ok()
            .and_then(|s| LogFormat::from_str(&s))
            .unwrap_or(LogFormat::Pretty);

        let file_path = std::env::var("NEXUS_LOG_FILE").ok();

        let rotation = match std::env::var("NEXUS_LOG_ROTATION")
            .unwrap_or_default()
            .to_lowercase().as_str()
        {
            "daily" => LogRotation::Daily,
            "hourly" => LogRotation::Hourly,
            "minutely" => LogRotation::Minutely,
            _ => LogRotation::Daily,
        };

        let max_files = std::env::var("NEXUS_LOG_MAX_FILES")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(7);

        LoggerConfig {
            level,
            format,
            file_path,
            with_thread: true,
            with_file: false, // Spring Boot default: false
            with_target: true,
            rotation,
            max_files,
        }
    }
}

/// Global logger initializer
/// 全局日志初始化器
pub struct Logger;

impl Logger {
    /// Initialize the global logger with default configuration
    /// 使用默认配置初始化全局日志
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// use nexus_observability::log::Logger;
    ///
    /// Logger::init().unwrap();
    /// ```
    pub fn init() -> Result<(), Box<dyn std::error::Error>> {
        Logger::init_with_config(LoggerConfig::default())
    }

    /// Initialize the global logger with custom configuration
    /// 使用自定义配置初始化全局日志
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// use nexus_observability::log::{Logger, LoggerConfig, LogLevel, LogFormat};
    ///
    /// let config = LoggerConfig {
    ///     level: LogLevel::Debug,
    ///     format: LogFormat::Pretty,
    ///     file_path: Some("logs/application.log".to_string()),
    ///     ..Default::default()
    /// };
    ///
    /// Logger::init_with_config(config).unwrap();
    /// ```
    pub fn init_with_config(config: LoggerConfig) -> Result<(), Box<dyn std::error::Error>> {
        let env_filter = create_env_filter(config.level);

        match config.format {
            LogFormat::Pretty => {
                #[cfg(feature = "nexus-format")]
                {
                    // Use NexusFormatter (formerly SpringBootFormatter)
                    // 使用 NexusFormatter (原 SpringBootFormatter)
                    let mut fmt_layer = fmt::layer()
                        .with_file(config.with_file)
                        .with_line_number(config.with_file)
                        .with_target(config.with_target)
                        .event_format(NexusFormatter::new());
                    fmt_layer.set_span_events(FmtSpan::CLOSE);

                    if let Some(ref path) = config.file_path {
                        let file_appender = create_file_appender(path, config.rotation)?;
                        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

                        let file_layer = fmt::layer()
                            .with_file(config.with_file)
                            .with_line_number(config.with_file)
                            .with_target(config.with_target)
                            .with_writer(non_blocking)
                            .compact();

                        Registry::default()
                            .with(env_filter)
                            .with(fmt_layer)
                            .with(file_layer)
                            .try_init()?;
                    } else {
                        Registry::default()
                            .with(env_filter)
                            .with(fmt_layer)
                            .try_init()?;
                    }
                }
                #[cfg(not(feature = "nexus-format"))]
                {
                    // Fallback to compact format
                    // 回退到紧凑格式
                    let mut fmt_layer = fmt::layer()
                        .with_file(config.with_file)
                        .with_line_number(config.with_file)
                        .with_target(config.with_target)
                        .compact();
                    fmt_layer.set_span_events(FmtSpan::CLOSE);

                    if let Some(ref path) = config.file_path {
                        let file_appender = create_file_appender(path, config.rotation)?;
                        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

                        let file_layer = fmt::layer()
                            .with_file(config.with_file)
                            .with_line_number(config.with_file)
                            .with_target(config.with_target)
                            .with_writer(non_blocking)
                            .compact();

                        Registry::default()
                            .with(env_filter)
                            .with(fmt_layer)
                            .with(file_layer)
                            .try_init()?;
                    } else {
                        Registry::default()
                            .with(env_filter)
                            .with(fmt_layer)
                            .try_init()?;
                    }
                }
            }
            LogFormat::Compact => {
                let mut fmt_layer = fmt::layer()
                    .with_file(config.with_file)
                    .with_line_number(config.with_file)
                    .with_target(config.with_target)
                    .compact();
                fmt_layer.set_span_events(FmtSpan::CLOSE);

                if let Some(ref path) = config.file_path {
                    let file_appender = create_file_appender(path, config.rotation)?;

                    let file_layer = fmt::layer()
                                                .with_file(config.with_file)
                        .with_line_number(config.with_file)
                        .with_target(config.with_target)
                        .with_writer(file_appender)
                        .compact();

                    Registry::default()
                        .with(env_filter)
                        .with(fmt_layer)
                        .with(file_layer)
                        .try_init()?;
                } else {
                    Registry::default()
                        .with(env_filter)
                        .with(fmt_layer)
                        .try_init()?;
                }
            }
            LogFormat::Json => {
                let mut fmt_layer = fmt::layer()
                    .json()
                    .with_file(config.with_file)
                    .with_line_number(config.with_file)
                    .with_target(config.with_target)
                    .with_current_span(false);
                fmt_layer.set_span_events(FmtSpan::CLOSE);

                if let Some(ref path) = config.file_path {
                    let file_appender = create_file_appender(path, config.rotation)?;

                    let file_layer = fmt::layer()
                        .json()
                                                .with_file(config.with_file)
                        .with_line_number(config.with_file)
                        .with_target(config.with_target)
                        .with_writer(file_appender);

                    Registry::default()
                        .with(env_filter)
                        .with(fmt_layer)
                        .with(file_layer)
                        .try_init()?;
                } else {
                    Registry::default()
                        .with(env_filter)
                        .with(fmt_layer)
                        .try_init()?;
                }
            }
        }

        Ok(())
    }

    /// Initialize with Spring Boot style properties and banner
    /// 使用 Spring Boot 风格属性和横幅初始化
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// use nexus_observability::log::Logger;
    ///
    /// // These can be set via environment or application.properties:
    /// // logging.level.root=INFO
    /// // logging.level.com.example=DEBUG
    /// // logging.file.name=logs/application.log
    /// // logging.pattern.console=%d{yyyy-MM-dd HH:mm:ss} [%thread] %-5level %logger{36} - %msg%n
    ///
    /// Logger::init_spring_style().unwrap();
    /// ```
    pub fn init_spring_style() -> Result<(), Box<dyn std::error::Error>> {
        #[cfg(feature = "nexus-format")]
        {
            // Print banner (uses default values)
            Banner::print("nexus", env!("CARGO_PKG_VERSION"), 8080);
        }

        let mut config = LoggerConfig::default();

        // Spring Boot: logging.level.root
        if let Ok(level) = std::env::var("LOGGING_LEVEL_ROOT") {
            if let Some(lvl) = LogLevel::from_str(&level) {
                config.level = lvl;
            }
        }

        // Spring Boot: logging.file.name
        if let Ok(file) = std::env::var("LOGGING_FILE_NAME") {
            config.file_path = Some(file);
        }

        // Spring Boot: logging.pattern.console
        // TODO: Parse custom pattern (future enhancement)
        // TODO: 解析自定义模式（未来增强）

        Logger::init_with_config(config)
    }
}

/// Create environment filter for log level
/// 创建日志级别的环境过滤器
fn create_env_filter(default_level: LogLevel) -> EnvFilter {
    let base_filter = if let Some(level) = default_level.to_tracing_level() {
        EnvFilter::builder()
            .with_default_directive(level.into())
            .from_env_lossy()
    } else {
        EnvFilter::builder()
            .with_default_directive(Level::INFO.into())
            .from_env_lossy()
    };

    // Support Spring Boot style: logging.level.<package>=<LEVEL>
    // 支持Spring Boot风格：logging.level.<package>=<LEVEL>
    let filter = if let Ok(level_str) = std::env::var("LOGGING_LEVEL") {
        let parts: Vec<&str> = level_str.split('=').collect();
        if parts.len() == 2 {
            let target = parts[0];
            let level = parts[1];
            if let Some(lvl) = LogLevel::from_str(level).and_then(|l| l.to_tracing_level()) {
                base_filter.add_directive(format!("{}={}", target, lvl).parse().unwrap_or_else(|_| lvl.into()))
            } else {
                base_filter
            }
        } else {
            base_filter
        }
    } else {
        base_filter
    };

    filter
}

/// Create rolling file appender
/// 创建滚动文件附加器
fn create_file_appender(
    path: &str,
    rotation: LogRotation,
) -> Result<RollingFileAppender, std::io::Error> {
    let (directory, prefix) = if path.contains('/') {
        let parts: Vec<&str> = path.rsplitn(2, '/').collect();
        (parts[1], parts[0])
    } else {
        (".", path)
    };

    let rotation = match rotation {
        LogRotation::Never => Rotation::NEVER,
        LogRotation::Daily => Rotation::DAILY,
        LogRotation::Hourly => Rotation::HOURLY,
        LogRotation::Minutely => Rotation::MINUTELY,
    };

    Ok(RollingFileAppender::new(rotation, directory, prefix))
}

/// Logger Factory (equivalent to SLF4J's LoggerFactory)
/// 日志工厂（等价于 SLF4J 的 LoggerFactory）
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_observability::log::LoggerFactory;
///
/// // Equivalent to: private static final Logger log = LoggerFactory.getLogger(MyClass.class);
/// let log = LoggerFactory::get("my::module");
/// let log2 = LoggerFactory::get_for::<MyStruct>();
/// ```
pub struct LoggerFactory;

impl LoggerFactory {
    /// Get a logger for the given name
    /// 获取给定名称的日志记录器
    ///
    /// This follows SLF4J's LoggerFactory.getLogger(String name) pattern.
    /// 这遵循 SLF4J 的 LoggerFactory.getLogger(String name) 模式。
    pub fn get(name: &str) -> LoggerHandle {
        LoggerHandle {
            name: name.to_string(),
        }
    }

    /// Get a logger for the given type
    /// 获取给定类型的日志记录器
    ///
    /// This follows SLF4J's LoggerFactory.getLogger(Class.class) pattern.
    /// 这遵循 SLF4J 的 LoggerFactory.getLogger(Class.class) 模式。
    pub fn get_for<T>() -> LoggerHandle {
        LoggerHandle {
            name: std::any::type_name::<T>().to_string(),
        }
    }

    /// Get a logger for the current module
    /// 获取当前模块的日志记录器
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// use nexus_observability::log::LoggerFactory;
    ///
    /// let log = LoggerFactory::current_module!();
    /// ```
    #[inline]
    pub fn current_module() -> LoggerHandle {
        LoggerHandle {
            name: module_path!().to_string(),
        }
    }
}

/// Logger handle (equivalent to SLF4J's Logger)
/// 日志记录器句柄（等价于 SLF4J 的 Logger）
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_observability::log::LoggerFactory;
///
/// let log = LoggerFactory::get("my::module");
///
/// log.error("Error occurred: {}", err);
/// log.warn("Warning message");
/// log.info("Info message");
/// log.debug("Debug message");
/// log.trace("Trace message");
/// ```
#[derive(Debug, Clone)]
pub struct LoggerHandle {
    name: String,
}

impl LoggerHandle {
    /// Log an ERROR message
    /// 记录 ERROR 消息
    pub fn error(&self, message: std::fmt::Arguments) {
        self.error_args(&[], message)
    }

    /// Log an ERROR message with fields
    /// 记录带字段的 ERROR 消息
    pub fn error_args(&self, _fields: &[(&str, String)], message: std::fmt::Arguments) {
        // Note: We include the logger name in the message since tracing macros require constant target
        // 注意：我们在消息中包含日志记录器名称，因为tracing宏需要常量target
        tracing::error!(target: "nexus", "[{}] {}", self.name, message);
    }

    /// Log a WARN message
    /// 记录 WARN 消息
    pub fn warn(&self, message: std::fmt::Arguments) {
        self.warn_args(&[], message)
    }

    /// Log a WARN message with fields
    /// 记录带字段的 WARN 消息
    pub fn warn_args(&self, _fields: &[(&str, String)], message: std::fmt::Arguments) {
        tracing::warn!(target: "nexus", "[{}] {}", self.name, message);
    }

    /// Log an INFO message
    /// 记录 INFO 消息
    pub fn info(&self, message: std::fmt::Arguments) {
        self.info_args(&[], message)
    }

    /// Log an INFO message with fields
    /// 记录带字段的 INFO 消息
    pub fn info_args(&self, _fields: &[(&str, String)], message: std::fmt::Arguments) {
        tracing::info!(target: "nexus", "[{}] {}", self.name, message);
    }

    /// Log a DEBUG message
    /// 记录 DEBUG 消息
    pub fn debug(&self, message: std::fmt::Arguments) {
        self.debug_args(&[], message)
    }

    /// Log a DEBUG message with fields
    /// 记录带字段的 DEBUG 消息
    pub fn debug_args(&self, _fields: &[(&str, String)], message: std::fmt::Arguments) {
        tracing::debug!(target: "nexus", "[{}] {}", self.name, message);
    }

    /// Log a TRACE message
    /// 记录 TRACE 消息
    pub fn trace(&self, message: std::fmt::Arguments) {
        self.trace_args(&[], message)
    }

    /// Log a TRACE message with fields
    /// 记录带字段的 TRACE 消息
    pub fn trace_args(&self, _fields: &[(&str, String)], message: std::fmt::Arguments) {
        tracing::trace!(target: "nexus", "[{}] {}", self.name, message);
    }

    /// Get the logger name
    /// 获取日志记录器名称
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Global logger instance holder (used by macros)
/// 全局日志记录器实例持有者（由宏使用）
static GLOBAL_LOGGER: OnceLock<LoggerHandle> = OnceLock::new();

/// Initialize global logger for current module
/// 初始化当前模块的全局日志记录器
pub fn init_global_logger() -> LoggerHandle {
    let logger = LoggerFactory::current_module();
    GLOBAL_LOGGER.get_or_init(|| logger.clone()).clone()
}

/// Get global logger instance
/// 获取全局日志记录器实例
pub fn global_logger() -> Option<&'static LoggerHandle> {
    GLOBAL_LOGGER.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_parse() {
        assert_eq!(LogLevel::from_str("TRACE"), Some(LogLevel::Trace));
        assert_eq!(LogLevel::from_str("DEBUG"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("INFO"), Some(LogLevel::Info));
        assert_eq!(LogLevel::from_str("WARN"), Some(LogLevel::Warn));
        assert_eq!(LogLevel::from_str("ERROR"), Some(LogLevel::Error));
        assert_eq!(LogLevel::from_str("OFF"), Some(LogLevel::Off));
        assert_eq!(LogLevel::from_str("invalid"), None);
    }

    #[test]
    fn test_log_format_parse() {
        assert_eq!(LogFormat::from_str("pretty"), Some(LogFormat::Pretty));
        assert_eq!(LogFormat::from_str("compact"), Some(LogFormat::Compact));
        assert_eq!(LogFormat::from_str("json"), Some(LogFormat::Json));
        assert_eq!(LogFormat::from_str("invalid"), None);
    }

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
        assert!(LogLevel::Error < LogLevel::Off);
    }

    #[test]
    fn test_logger_factory() {
        let log = LoggerFactory::get("test::module");
        assert_eq!(log.name(), "test::module");

        let log2 = LoggerFactory::get_for::<String>();
        assert_eq!(log2.name(), std::any::type_name::<String>());
    }
}
