//! Nexus Observability - Tracing, metrics, and logging
//! Nexus可观测性 - 追踪、指标和日志
//!
//! # Overview / 概述
//!
//! `nexus-observability` provides distributed tracing, metrics collection,
//! and structured logging for the Nexus framework.
//!
//! `nexus-observability` 为Nexus框架提供分布式追踪、指标收集和结构化日志。

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod trace;
pub mod metrics;
pub mod log;

pub use trace::{Tracer, Span, TraceContext, TraceId, SpanId};
pub use metrics::{MetricsRegistry, Counter, Gauge, Histogram};
pub use log::{
    Logger, LoggerConfig, LoggerFactory, LoggerHandle, LogLevel, LogFormat, LogRotation,
};

/// Re-export tracing for convenience
/// 重新导出 tracing 以便使用
pub use tracing::{self, error, warn, info, debug, trace};
