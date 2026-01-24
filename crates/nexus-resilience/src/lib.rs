//! Nexus Resilience - High availability patterns
//! Nexus弹性 - 高可用模式
//!
//! # Overview / 概述
//!
//! `nexus-resilience` provides high availability patterns such as circuit breakers,
//! rate limiters, retry policies, and service discovery.
//!
//! `nexus-resilience` 提供高可用模式，如熔断器、限流器、重试策略和服务发现。

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod circuit;
pub mod rate_limit;
pub mod retry;
pub mod timeout;
pub mod discovery;

pub use circuit::{
    CircuitBreaker, CircuitState, CircuitBreakerConfig,
    CircuitBreakerError, CircuitBreakerRegistry, CircuitMetrics
};
pub use rate_limit::{
    RateLimiter, RateLimiterType, RateLimiterConfig,
    RateLimitError, RateLimiterMetrics, RateLimiterRegistry
};
pub use retry::{
    RetryPolicy, BackoffType, retry, retry_with_predicate,
    RetryError, RetryState, ShouldRetry, RetryAll, RetryErrors
};
pub use discovery::{
    ServiceInstance, InstanceStatus, ServiceRegistry,
    SimpleServiceRegistry, ServiceDiscovery, LoadBalanceStrategy,
    DiscoveryError
};
