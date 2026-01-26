//! Nexus AOP Examples / Nexus AOP 示例
//!
//! This example demonstrates AOP (Aspect-Oriented Programming) usage in Nexus
//! 此示例演示了 Nexus 中 AOP（面向切面编程）的使用

use nexus_aop::{After, Around, Aspect, Before, Pointcut};

// ============================================================================
// Example 1: Basic Logging Aspect / 基础日志切面
// ============================================================================

/// Simple logging aspect
/// 简单的日志切面
#[Aspect]
struct LoggingAspect;

impl LoggingAspect {
    /// Log before method execution
    /// 在方法执行前记录日志
    #[Before("execution(* com.example..*.*(..))")]
    fn log_before(&self, join_point: &JoinPoint) {
        println!("✨ Entering: {}", join_point.method_name());
    }

    /// Log after method execution
    /// 在方法执行后记录日志
    #[After("execution(* com.example..*.*(..))")]
    fn log_after(&self, join_point: &JoinPoint) {
        println!("👋 Exiting: {}", join_point.method_name());
    }
}

// ============================================================================
// Example 2: Transaction Management Aspect / 事务管理切面
// ============================================================================

/// Transaction management aspect
/// 事务管理切面
#[Aspect]
struct TransactionAspect;

impl TransactionAspect {
    /// Manage transactions around service methods
    /// 在服务方法周围管理事务
    #[Around("execution(* com.example.service.*.*(..))")]
    fn manage_transaction(&self, join_point: JoinPoint) -> Result<(), Error> {
        println!("🔒 Beginning transaction");

        match join_point.proceed() {
            Ok(result) => {
                println!("✅ Committing transaction");
                Ok(result)
            },
            Err(e) => {
                println!("❌ Rolling back transaction: {}", e);
                Err(e)
            },
        }
    }
}

// ============================================================================
// Example 3: Caching Aspect / 缓存切面
// ============================================================================

/// Caching aspect for repository methods
/// 仓库方法的缓存切面
#[Aspect]
struct CachingAspect;

impl CachingAspect {
    /// Cache results from repository methods
    /// 缓存仓库方法的结果
    #[Around("execution(* com.example.repository.*.find*(..))")]
    fn cache_result(&self, join_point: JoinPoint) -> Result<Data, Error> {
        let cache_key = format!("{:?}", join_point.args());

        println!("🔍 Checking cache for key: {}", cache_key);

        // Simulate cache miss
        // 模拟缓存未命中
        println!("⚠️ Cache miss, executing method");

        let result = join_point.proceed()?;

        println!("💾 Caching result for key: {}", cache_key);

        Ok(result)
    }
}

// ============================================================================
// Example 4: Security Aspect / 安全切面
// ============================================================================

/// Security aspect for authorization checks
/// 授权检查的安全切面
#[Aspect]
struct SecurityAspect;

impl SecurityAspect {
    /// Check authorization before controller methods
    /// 在控制器方法前检查授权
    #[Before("execution(* com.example.controller.*.*(..))")]
    fn check_authorization(&self, join_point: &JoinPoint) {
        println!("🔐 Checking authorization for: {}", join_point.method_name());

        let user = get_current_user();

        if !user.has_permission(join_point.method_name()) {
            panic!("❌ Unauthorized access to {}", join_point.method_name());
        }

        println!("✅ Authorized user: {}", user.username());
    }
}

// ============================================================================
// Example 5: Performance Monitoring Aspect / 性能监控切面
// ============================================================================

/// Performance monitoring aspect
/// 性能监控切面
#[Aspect]
struct PerformanceMonitoringAspect;

impl PerformanceMonitoringAspect {
    /// Monitor method execution time
    /// 监控方法执行时间
    #[Around("execution(* com.example.service.*.*(..))")]
    fn monitor_performance(&self, join_point: JoinPoint) -> Result<(), Error> {
        let start = std::time::Instant::now();

        let result = join_point.proceed();

        let duration = start.elapsed();

        if duration.as_millis() > 100 {
            println!(
                "⚠️ Slow method: {} took {}ms",
                join_point.method_name(),
                duration.as_millis()
            );
        } else {
            println!("⏱️ Method: {} took {}ms", join_point.method_name(), duration.as_millis());
        }

        result
    }
}

// ============================================================================
// Example 6: Reusable Pointcuts / 可重用切点
// ============================================================================

/// Aspect with reusable pointcuts
/// 带有可重用切点的切面
#[Aspect]
struct ReusablePointcutAspect;

impl ReusablePointcutAspect {
    /// Define a pointcut for service layer
    /// 定义服务层的切点
    #[Pointcut("execution(* com.example.service.*.*(..))")]
    fn service_layer() -> PointcutExpression {}

    /// Define a pointcut for repository layer
    /// 定义仓库层的切点
    #[Pointcut("execution(* com.example.repository.*.*(..))")]
    fn repository_layer() -> PointcutExpression {}

    /// Use the service layer pointcut
    /// 使用服务层切点
    #[Before("service_layer()")]
    fn log_service_entry(&self, join_point: &JoinPoint) {
        println!("🏢 Service layer method: {}", join_point.method_name());
    }

    /// Use the repository layer pointcut
    /// 使用仓库层切点
    #[Before("repository_layer()")]
    fn log_repository_entry(&self, join_point: &JoinPoint) {
        println!("🗄️ Repository layer method: {}", join_point.method_name());
    }

    /// Combine multiple pointcuts with AND
    /// 使用 AND 组合多个切点
    #[Before("service_layer() && execution(* save*(..))")]
    fn log_save_operations(&self, join_point: &JoinPoint) {
        println!("💾 Save operation in service layer: {}", join_point.method_name());
    }
}

// ============================================================================
// Example 7: Retry Aspect / 重试切面
// ============================================================================

/// Retry aspect for transient failures
/// 瞬态故障的重试切面
#[Aspect]
struct RetryAspect;

impl RetryAspect {
    /// Retry failed operations
    /// 重试失败的操作
    #[Around("execution(* com.example.service.external.*(..))")]
    fn retry_on_failure(&self, join_point: JoinPoint) -> Result<(), Error> {
        let max_retries = 3;
        let mut attempts = 0;

        loop {
            attempts += 1;

            match join_point.proceed() {
                Ok(result) => {
                    if attempts > 1 {
                        println!("🎉 Success after {} attempts", attempts);
                    }
                    return Ok(result);
                },
                Err(e) if attempts < max_retries => {
                    println!("⚠️ Attempt {}/{} failed, retrying...", attempts, max_retries);
                    std::thread::sleep(std::time::Duration::from_millis(100));
                },
                Err(e) => {
                    println!("❌ All {} attempts failed", max_retries);
                    return Err(e);
                },
            }
        }
    }
}

// ============================================================================
// Example 8: Rate Limiting Aspect / 限流切面
// ============================================================================

/// Rate limiting aspect
/// 限流切面
#[Aspect]
struct RateLimitingAspect;

impl RateLimitingAspect {
    /// Limit method call rate
    /// 限制方法调用速率
    #[Before("execution(* com.example.api.*.*(..))")]
    fn check_rate_limit(&self, join_point: &JoinPoint) {
        let user = get_current_user();
        let key = format!("rate_limit:{}:{}", user.id(), join_point.method_name());

        println!("🚦 Checking rate limit for key: {}", key);

        if check_rate_limit_exceeded(&key) {
            panic!("❌ Rate limit exceeded for user {}", user.id());
        }

        println!("✅ Rate limit OK for user {}", user.id());
    }
}

// ============================================================================
// Example 9: Validation Aspect / 验证切面
// ============================================================================

/// Validation aspect
/// 验证切面
#[Aspect]
struct ValidationAspect;

impl ValidationAspect {
    /// Validate parameters before method execution
    /// 在方法执行前验证参数
    #[Before("execution(* com.example.service.*.*(..)) && args(..)")]
    fn validate_parameters(&self, join_point: &JoinPoint) {
        println!("✅ Validating parameters for: {}", join_point.method_name());

        let args = join_point.args();

        // Validate each argument
        // 验证每个参数
        for arg in args {
            if let Some(string_arg) = arg.as_str() {
                if string_arg.is_empty() {
                    panic!("❌ Validation failed: empty string");
                }
            }
        }

        println!("✅ All parameters validated");
    }
}

// ============================================================================
// Example 10: Audit Logging Aspect / 审计日志切面
// ============================================================================

/// Audit logging aspect
/// 审计日志切面
#[Aspect]
struct AuditLoggingAspect;

impl AuditLoggingAspect {
    /// Log all modifications
    /// 记录所有修改操作
    #[After(
        "execution(* com.example.service.*.update*(..)) || execution(* com.example.service.*.delete*(..))"
    )]
    fn log_modifications(&self, join_point: &JoinPoint) {
        let user = get_current_user();
        println!(
            "📝 AUDIT: User {} performed {} at {}",
            user.username(),
            join_point.method_name(),
            chrono::Utc::now()
        );
    }
}

// ============================================================================
// Helper Types / 辅助类型
// ============================================================================

/// Mock JoinPoint (in real implementation, this would be generated)
/// 模拟 JoinPoint（在实际实现中，这会被生成）
struct JoinPoint {
    method_name: String,
    args: Vec<String>,
}

impl JoinPoint {
    fn method_name(&self) -> &str {
        &self.method_name
    }

    fn args(&self) -> &[String] {
        &self.args
    }

    fn proceed(self) -> Result<(), Error> {
        // Execute the actual method
        // 执行实际的方法
        Ok(())
    }
}

/// Mock User
/// 模拟用户
struct User {
    id: u64,
    username: String,
}

impl User {
    fn id(&self) -> u64 {
        self.id
    }

    fn username(&self) -> &str {
        &self.username
    }

    fn has_permission(&self, method: &str) -> bool {
        // Mock permission check
        // 模拟权限检查
        true
    }
}

/// Mock PointcutExpression
/// 模拟 PointcutExpression
struct PointcutExpression;

/// Mock Error type
/// 模拟错误类型
type Error = String;

/// Mock Data type
/// 模拟数据类型
type Data = String;

// Mock helper functions / 模拟辅助函数

fn get_current_user() -> User {
    User {
        id: 1,
        username: "alice".to_string(),
    }
}

fn check_rate_limit_exceeded(key: &str) -> bool {
    // Mock rate limit check
    // 模拟限流检查
    false
}

fn main() {
    println!("=== Nexus AOP Examples ===\n");

    println!("Example 1: Basic Logging Aspect");
    println!("✨ @Before and @After for logging method entry/exit\n");

    println!("Example 2: Transaction Management");
    println!("🔒 @Around for managing transactions\n");

    println!("Example 3: Caching");
    println!("💾 @Around for caching repository results\n");

    println!("Example 4: Security");
    println!("🔐 @Before for authorization checks\n");

    println!("Example 5: Performance Monitoring");
    println!("⏱️ @Around for measuring execution time\n");

    println!("Example 6: Reusable Pointcuts");
    println!("🎯 @Pointcut for defining reusable expressions\n");

    println!("Example 7: Retry Logic");
    println!("🔄 @Around for retrying failed operations\n");

    println!("Example 8: Rate Limiting");
    println!("🚦 @Before for rate limiting API calls\n");

    println!("Example 9: Parameter Validation");
    println!("✅ @Before for validating input parameters\n");

    println!("Example 10: Audit Logging");
    println!("📝 @After for logging modifications\n");

    println!("=== Available Pointcut Designators ===");
    println!("execution() - Match method execution");
    println!("within() - Match within certain types");
    println!("this() - Match bean reference");
    println!("target() - Match target object");
    println!("args() - Match method arguments");
    println!("@annotation() - Match annotated methods");

    println!("\n=== Combining Pointcuts ===");
    println!("&& - AND (both must match)");
    println!("|| - OR (either must match)");
    println!("! - NOT (negation)");
}
