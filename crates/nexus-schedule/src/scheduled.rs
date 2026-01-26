//! Scheduled task module
//! 定时任务模块
//!
//! # Equivalent to Spring / 等价于 Spring
//!
//! - `@Scheduled` - ScheduledTask
//! - `@EnableScheduling` - TaskScheduler::run()
//! - `fixedRate` - schedule_fixed_rate()
//! - `fixedDelay` - schedule_fixed_delay()
//! - `cron` - schedule_cron()
//! - `initialDelay` - initial_delay parameter

use crate::DEFAULT_INITIAL_DELAY_MS;
use std::time::Duration;
use tokio::time::{interval, sleep};
use tracing::info;

/// Schedule type
/// 调度类型
#[derive(Debug, Clone)]
pub enum ScheduleType {
    /// Fixed rate (runs at fixed intervals)
    /// 固定速率（按固定间隔运行）
    FixedRate(Duration),

    /// Fixed delay (waits specified delay between completion and next start)
    /// 固定延迟（完成和下次开始之间等待指定延迟）
    FixedDelay(Duration),

    /// Cron expression
    /// Cron表达式
    Cron(String),
}

/// Scheduled task
/// 定时任务
///
/// Equivalent to Spring's @Scheduled annotation.
/// 等价于Spring的@Scheduled注解。
///
/// # Spring Equivalent / Spring等价物
///
/// ```java
/// @Scheduled(fixedRate = 5000)
/// public void task() {
///     // Runs every 5 seconds
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ScheduledTask {
    /// Task name
    /// 任务名称
    pub name: String,

    /// Schedule type
    /// 调度类型
    pub schedule_type: ScheduleType,

    /// Initial delay
    /// 初始延迟
    pub initial_delay: Duration,
}

impl ScheduledTask {
    /// Create a new scheduled task with fixed rate
    /// 创建固定速率的定时任务
    pub fn fixed_rate(name: impl Into<String>, interval_ms: u64) -> Self {
        Self {
            name: name.into(),
            schedule_type: ScheduleType::FixedRate(Duration::from_millis(interval_ms)),
            initial_delay: Duration::from_millis(DEFAULT_INITIAL_DELAY_MS),
        }
    }

    /// Create a new scheduled task with fixed delay
    /// 创建固定延迟的定时任务
    pub fn fixed_delay(name: impl Into<String>, delay_ms: u64) -> Self {
        Self {
            name: name.into(),
            schedule_type: ScheduleType::FixedDelay(Duration::from_millis(delay_ms)),
            initial_delay: Duration::from_millis(DEFAULT_INITIAL_DELAY_MS),
        }
    }

    /// Create a new scheduled task with cron expression
    /// 创建Cron表达式的定时任务
    pub fn cron(name: impl Into<String>, cron_expression: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            schedule_type: ScheduleType::Cron(cron_expression.into()),
            initial_delay: Duration::from_millis(DEFAULT_INITIAL_DELAY_MS),
        }
    }

    /// Set initial delay
    /// 设置初始延迟
    pub fn initial_delay(mut self, delay_ms: u64) -> Self {
        self.initial_delay = Duration::from_millis(delay_ms);
        self
    }
}

/// Task scheduler
/// 任务调度器
///
/// Equivalent to Spring's @EnableScheduling.
/// 等价于Spring的@EnableScheduling。
///
/// # Spring Equivalent / Spring等价物
///
/// ```java
/// @SpringBootApplication
/// @EnableScheduling
/// public class MyApp {
///     // Scheduled tasks will be automatically detected
/// }
/// ```
#[derive(Debug)]
pub struct TaskScheduler {
    /// Running state
    /// 运行状态
    running: std::sync::Arc<tokio::sync::RwLock<bool>>,
}

impl TaskScheduler {
    /// Create a new task scheduler
    /// 创建新的任务调度器
    pub fn new() -> Self {
        Self {
            running: std::sync::Arc::new(tokio::sync::RwLock::new(false)),
        }
    }

    /// Run the scheduler
    /// 运行调度器
    pub async fn run(&self) {
        *self.running.write().await = true;
        info!("Task scheduler started");
    }

    /// Shutdown the scheduler
    /// 关闭调度器
    pub async fn shutdown(&self) {
        *self.running.write().await = false;
        info!("Task scheduler shut down");
    }
}

impl Default for TaskScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to run a scheduled task with fixed rate
/// 辅助函数：按固定速率运行定时任务
///
/// # Spring Equivalent / Spring等价物
///
/// ```java
/// @Scheduled(fixedRate = 5000)
/// public void task() { }
/// ```
pub async fn schedule_fixed_rate<F>(interval_ms: u64, mut f: F)
where
    F: FnMut() + Send + 'static,
{
    let mut timer = interval(Duration::from_millis(interval_ms));
    loop {
        f();
        timer.tick().await;
    }
}

/// Helper function to run a scheduled task with fixed delay
/// 辅助函数：按固定延迟运行定时任务
///
/// # Spring Equivalent / Spring等价物
///
/// ```java
/// @Scheduled(fixedDelay = 5000)
/// public void task() { }
/// ```
pub async fn schedule_fixed_delay<F>(delay_ms: u64, mut f: F)
where
    F: FnMut() + Send + 'static,
{
    loop {
        f();
        sleep(Duration::from_millis(delay_ms)).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheduled_task_creation() {
        let task = ScheduledTask::fixed_rate("test", 5000);
        assert_eq!(task.name, "test");

        let task = ScheduledTask::fixed_delay("test", 5000);
        assert_eq!(task.name, "test");

        let task = ScheduledTask::cron("test", "0 0 * * * ?");
        assert_eq!(task.name, "test");
    }

    #[test]
    fn test_task_scheduler() {
        let scheduler = TaskScheduler::new();
        assert!(!*scheduler.running.try_read().unwrap());
    }
}
