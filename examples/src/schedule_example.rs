// Scheduled Tasks Example / 定时任务示例
//
// Demonstrates Nexus's scheduling capabilities:
// 演示 Nexus 的调度能力：
// - Cron-based scheduling / 基于 Cron 的调度
// - Fixed rate scheduling / 固定速率调度
// - Fixed delay scheduling / 固定延迟调度
// - One-time delayed tasks / 一次性延迟任务
//
// Equivalent to: Spring @Scheduled, Quartz Scheduler
// 等价于：Spring @Scheduled, Quartz Scheduler

use nexus_schedule::{
    scheduler::{ScheduledTask, TaskScheduler},
    cron::CronExpression,
};
use std::time::Duration;
use tokio::time::sleep;

/// Simple scheduled task / 简单的定时任务
async fn hello_world_task() {
    println!("[Scheduled Task] Hello, World! - Time: {}", chrono::Utc::now().format("%H:%M:%S"));
}

/// Data cleanup task / 数据清理任务
async fn cleanup_expired_data_task() {
    println!("[Cleanup Task] Cleaning expired data...");
    // Simulate cleanup work / 模拟清理工作
    sleep(Duration::from_millis(100)).await;
    println!("[Cleanup Task] Cleanup completed!");
}

/// Report generation task / 报告生成任务
async fn generate_daily_report_task() {
    println!("[Report Task] Generating daily report...");
    // Simulate report generation / 模拟报告生成
    sleep(Duration::from_millis(200)).await;
    println!("[Report Task] Report generated successfully!");
}

/// Health check task / 健康检查任务
async fn health_check_task() {
    println!("[Health Check] Checking system health...");
    // Simulate health check / 模拟健康检查
    println!("[Health Check] All systems operational!");
}

/// Data synchronization task / 数据同步任务
async fn data_sync_task() {
    println!("[Sync Task] Syncing data with remote services...");
    // Simulate data sync / 模拟数据同步
    sleep(Duration::from_millis(150)).await;
    println!("[Sync Task] Data synchronized!");
}

/// Cron scheduling example / Cron 调度示例
#[tokio::main]
async fn cron_scheduling_example() {
    println!("\n=== Cron Scheduling Example / Cron 调度示例 ===\n");

    let scheduler = TaskScheduler::new();

    // Schedule task every 5 seconds / 每5秒调度任务
    let task1 = ScheduledTask::cron(
        "hello-task",
        hello_world_task,
        CronExpression::new("0/5 * * * * *").unwrap(), // Every 5 seconds
    );
    scheduler.schedule(task1).await;

    // Schedule cleanup task every 30 seconds / 每30秒调度清理任务
    let task2 = ScheduledTask::cron(
        "cleanup-task",
        cleanup_expired_data_task,
        CronExpression::new("0/30 * * * * *").unwrap(), // Every 30 seconds
    );
    scheduler.schedule(task2).await;

    println!("Cron tasks scheduled:");
    println!("  - hello-task: Every 5 seconds");
    println!("  - cleanup-task: Every 30 seconds");

    // Run for 20 seconds / 运行20秒
    println!("\nRunning for 20 seconds...\n");
    sleep(Duration::from_secs(20)).await;

    // Stop scheduler / 停止调度器
    scheduler.shutdown().await;
    println!("\nScheduler stopped.\n");
}

/// Fixed rate scheduling example / 固定速率调度示例
#[tokio::main]
async fn fixed_rate_scheduling_example() {
    println!("\n=== Fixed Rate Scheduling Example / 固定速率调度示例 ===\n");

    let scheduler = TaskScheduler::new();

    // Schedule task every 3 seconds / 每3秒调度任务
    let task = ScheduledTask::fixed_rate(
        "report-task",
        generate_daily_report_task,
        Duration::from_secs(3),
    );
    scheduler.schedule(task).await;

    println!("Fixed rate task scheduled:");
    println!("  - report-task: Every 3 seconds");

    println!("\nRunning for 15 seconds...\n");
    sleep(Duration::from_secs(15)).await;

    scheduler.shutdown().await;
    println!("\nScheduler stopped.\n");
}

/// Fixed delay scheduling example / 固定延迟调度示例
#[tokio::main]
async fn fixed_delay_scheduling_example() {
    println!("\n=== Fixed Delay Scheduling Example / 固定延迟调度示例 ===\n");

    let scheduler = TaskScheduler::new();

    // Schedule task with 2 second delay between executions / 调度任务，执行间隔2秒
    let task = ScheduledTask::fixed_delay(
        "sync-task",
        data_sync_task,
        Duration::from_secs(2),
    );
    scheduler.schedule(task).await;

    println!("Fixed delay task scheduled:");
    println!("  - sync-task: 2 seconds delay between executions");

    println!("\nRunning for 12 seconds...\n");
    sleep(Duration::from_secs(12)).await;

    scheduler.shutdown().await;
    println!("\nScheduler stopped.\n");
}

/// One-time delayed task example / 一次性延迟任务示例
#[tokio::main]
async fn one_time_task_example() {
    println!("\n=== One-Time Task Example / 一次性任务示例 ===\n");

    let scheduler = TaskScheduler::new();

    // Schedule task to run after 3 seconds / 3秒后调度任务
    let task = ScheduledTask::one_time_delayed(
        "health-check",
        health_check_task,
        Duration::from_secs(3),
    );
    scheduler.schedule(task).await;

    println!("One-time task scheduled to run in 3 seconds");

    println!("Waiting...\n");
    sleep(Duration::from_secs(5)).await;

    scheduler.shutdown().await;
    println!("\nScheduler stopped.\n");
}

/// Complex scheduling example / 复杂调度示例
#[tokio::main]
async fn complex_scheduling_example() {
    println!("\n=== Complex Scheduling Example / 复杂调度示例 ===\n");

    let scheduler = TaskScheduler::new();

    // Task 1: Every 10 seconds / 任务1：每10秒
    let task1 = ScheduledTask::fixed_rate(
        "task1-metrics",
        || async {
            println!("[Metrics] Collecting system metrics...");
        },
        Duration::from_secs(10),
    );
    scheduler.schedule(task1).await;

    // Task 2: Every minute at second 0 / 任务2：每分钟的第0秒
    let task2 = ScheduledTask::cron(
        "task2-logrotate",
        || async {
            println!("[LogRotate] Rotating logs...");
        },
        CronExpression::new("0 * * * * *").unwrap(),
    );
    scheduler.schedule(task2).await;

    // Task 3: Every hour at minute 0 / 任务3：每小时的第0分钟
    let task3 = ScheduledTask::cron(
        "task3-backup",
        || async {
            println!("[Backup] Creating backup...");
        },
        CronExpression::new("0 0 * * * *").unwrap(),
    );
    scheduler.schedule(task3).await;

    // Task 4: Every day at 2 AM / 任务4：每天凌晨2点
    let task4 = ScheduledTask::cron(
        "task4-cleanup",
        || async {
            println!("[Cleanup] Daily cleanup job...");
        },
        CronExpression::new("0 0 2 * * *").unwrap(),
    );
    scheduler.schedule(task4).await;

    println!("Complex scheduling configured:");
    println!("  - task1-metrics: Every 10 seconds");
    println!("  - task2-logrotate: Every minute");
    println!("  - task3-backup: Every hour");
    println!("  - task4-cleanup: Every day at 2 AM");

    println!("\nRunning for 25 seconds...\n");
    sleep(Duration::from_secs(25)).await;

    scheduler.shutdown().await;
    println!("\nScheduler stopped.\n");
}

/// Scheduled HTTP endpoints / 定时HTTP端点
async fn scheduled_endpoints_example() {
    println!("\n=== Scheduled HTTP Endpoints Example / 定时HTTP端点示例 ===\n");

    let scheduler = TaskScheduler::new();

    // Background task: Cleanup expired sessions every hour
    // 后台任务：每小时清理过期会话
    let cleanup_task = ScheduledTask::cron(
        "cleanup-sessions",
        || async {
            println!("[Background] Cleaning up expired sessions...");
        },
        CronExpression::new("0 0 * * * *").unwrap(), // Every hour
    );
    scheduler.schedule(cleanup_task).await;

    // Background task: Send email notifications every 6 hours
    // 后台任务：每6小时发送邮件通知
    let notification_task = ScheduledTask::cron(
        "send-notifications",
        || async {
            println!("[Background] Sending notification emails...");
        },
        CronExpression::new("0 0 */6 * * *").unwrap(), // Every 6 hours
    );
    scheduler.schedule(notification_task).await;

    // Background task: Generate reports daily at midnight
    // 后台任务：每天午夜生成报告
    let report_task = ScheduledTask::cron(
        "generate-reports",
        || async {
            println!("[Background] Generating daily reports...");
        },
        CronExpression::new("0 0 0 * * *").unwrap(), // Daily at midnight
    );
    scheduler.schedule(report_task).await;

    println!("Background tasks configured:");
    println!("  - cleanup-sessions: Every hour");
    println!("  - send-notifications: Every 6 hours");
    println!("  - generate-reports: Daily at midnight");

    println!("\nAPI server ready with background tasks!");
    println!("Tasks are running in the background...\n");

    // Keep scheduler running / 保持调度器运行
    sleep(Duration::from_secs(10)).await;
    scheduler.shutdown().await;
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║   Nexus Scheduled Tasks Example / 定时任务示例                ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    println!("\nScheduling Features:");
    println!("  ✓ Cron-based scheduling");
    println!("  ✓ Fixed rate scheduling");
    println!("  ✓ Fixed delay scheduling");
    println!("  ✓ One-time delayed tasks");
    println!("  ✓ Complex scheduling patterns");

    // Run all examples / 运行所有示例
    cron_scheduling_example();
    fixed_rate_scheduling_example();
    fixed_delay_scheduling_example();
    one_time_task_example();
    complex_scheduling_example();
    scheduled_endpoints_example();

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║   All scheduling examples completed!                         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}
