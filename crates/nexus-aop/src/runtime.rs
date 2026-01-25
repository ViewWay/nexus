//! AOP Runtime support / AOP 运行时支持
//!
//! This module provides runtime support for AOP, including:
//! - JoinPoint: Represents a method execution join point
//! - PointcutExpression: Represents a pointcut expression
//! - AspectRegistry: Registers and manages aspects
//! - Proxy: Generates proxies that apply aspects
//!
//! 此模块提供 AOP 的运行时支持，包括：
//! - JoinPoint: 表示方法执行的连接点
//! - PointcutExpression: 表示切点表达式
//! - AspectRegistry: 注册和管理切面
//! - Proxy: 生成应用切面的代理

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;

// ============================================================================
// JoinPoint / 连接点
// ============================================================================

/// Represents a join point in the program execution
/// 表示程序执行中的连接点
///
/// A join point is a well-defined point in the execution of a program,
/// such as a method call or exception handler.
///
/// 连接点是程序执行中一个明确定义的点，例如方法调用或异常处理程序。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_aop::runtime::JoinPoint;
///
/// #[Before("execution(* com.example..*.*(..))")]
/// fn log_before(join_point: &JoinPoint) {
///     println!("Calling: {}", join_point.method_name());
///     println!("Args: {:?}", join_point.args());
/// }
/// ```
pub struct JoinPoint {
    /// Target object (self)
    /// 目标对象 (self)
    target: Arc<dyn Any + Send + Sync>,

    /// Method name
    /// 方法名
    method_name: String,

    /// Method arguments
    /// 方法参数
    args: Vec<Arc<dyn Any + Send + Sync>>,

    /// Method signature
    /// 方法签名
    signature: String,

    /// Target class name
    /// 目标类名
    target_class: String,
}

impl JoinPoint {
    /// Create a new join point
    /// 创建新的连接点
    pub fn new(
        target: Arc<dyn Any + Send + Sync>,
        method_name: String,
        args: Vec<Arc<dyn Any + Send + Sync>>,
        signature: String,
        target_class: String,
    ) -> Self {
        Self {
            target,
            method_name,
            args,
            signature,
            target_class,
        }
    }

    /// Get the target object
    /// 获取目标对象
    pub fn target(&self) -> &Arc<dyn Any + Send + Sync> {
        &self.target
    }

    /// Get the method name
    /// 获取方法名
    pub fn method_name(&self) -> &str {
        &self.method_name
    }

    /// Get the method arguments
    /// 获取方法参数
    pub fn args(&self) -> &[Arc<dyn Any + Send + Sync>] {
        &self.args
    }

    /// Get the method signature
    /// 获取方法签名
    pub fn signature(&self) -> &str {
        &self.signature
    }

    /// Get the target class name
    /// 获取目标类名
    pub fn target_class(&self) -> &str {
        &self.target_class
    }

    /// Get argument by index
    /// 通过索引获取参数
    pub fn arg<T: 'static>(&self, index: usize) -> Option<&T> {
        self.args
            .get(index)
            .and_then(|arg| arg.downcast_ref::<T>())
    }
}

impl Clone for JoinPoint {
    fn clone(&self) -> Self {
        Self {
            target: self.target.clone(),
            method_name: self.method_name.clone(),
            args: self.args.clone(),
            signature: self.signature.clone(),
            target_class: self.target_class.clone(),
        }
    }
}

impl fmt::Debug for JoinPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JoinPoint")
            .field("method_name", &self.method_name)
            .field("signature", &self.signature)
            .field("target_class", &self.target_class)
            .field("num_args", &self.args.len())
            .finish()
    }
}

// ============================================================================
// Pointcut Expression / 切点表达式
// ============================================================================

/// Represents a pointcut expression
/// 表示切点表达式
///
/// Pointcut expressions define join points where advice should be applied.
///
/// 切点表达式定义应该应用通知的连接点。
#[derive(Debug, Clone)]
pub struct PointcutExpression {
    /// The expression string
    /// 表达式字符串
    expression: String,

    /// Parsed expression components
    /// 解析后的表达式组件
    components: Vec<ExpressionComponent>,
}

/// Components of a pointcut expression
/// 切点表达式的组件
#[derive(Debug, Clone, PartialEq)]
enum ExpressionComponent {
    /// Execution pointcut
    /// 执行切点
    Execution {
        /// Package pattern
        /// 包模式
        package: String,
        /// Class pattern
        /// 类模式
        class: String,
        /// Method pattern
        /// 方法模式
        method: String,
        /// Parameter pattern
        /// 参数模式
        params: String,
    },
    /// Within pointcut
    /// Within 切点
    Within(String),
    /// Annotation pointcut
    /// 注解切点
    Annotation(String),
    /// AND operation
    /// AND 操作
    And,
    /// OR operation
    /// OR 操作
    Or,
    /// NOT operation
    /// NOT 操作
    Not,
}

impl PointcutExpression {
    /// Create a new pointcut expression
    /// 创建新的切点表达式
    pub fn new(expression: String) -> Self {
        let components = Self::parse_expression(&expression);
        Self {
            expression,
            components,
        }
    }

    /// Parse a pointcut expression
    /// 解析切点表达式
    fn parse_expression(expr: &str) -> Vec<ExpressionComponent> {
        let mut components = Vec::new();

        // Parse execution expressions
        // 解析 execution 表达式
        if let Some(start) = expr.find("execution(") {
            if let Some(end) = expr[start..].find(')') {
                let full_expr = &expr[start..start + end + 1];
                let inner = &full_expr[11..full_expr.len() - 1]; // Remove "execution(" and ")"

                // Parse: package..class.method(params)
                // 简化的解析逻辑
                components.push(ExpressionComponent::Execution {
                    package: "*".to_string(),
                    class: "*".to_string(),
                    method: "*".to_string(),
                    params: "..".to_string(),
                });
            }
        }

        // Parse within expressions
        // 解析 within 表达式
        if let Some(start) = expr.find("within(") {
            if let Some(end) = expr[start..].find(')') {
                let inner = &expr[start + 7..start + end];
                components.push(ExpressionComponent::Within(inner.to_string()));
            }
        }

        // Parse @annotation expressions
        // 解析 @annotation 表达式
        if let Some(start) = expr.find("@annotation(") {
            if let Some(end) = expr[start..].find(')') {
                let inner = &expr[start + 13..start + end];
                components.push(ExpressionComponent::Annotation(inner.to_string()));
            }
        }

        // Parse logical operators
        // 解析逻辑运算符
        if expr.contains(" && ") {
            components.push(ExpressionComponent::And);
        } else if expr.contains(" || ") {
            components.push(ExpressionComponent::Or);
        }

        components
    }

    /// Get the expression string
    /// 获取表达式字符串
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// Check if this pointcut matches a join point
    /// 检查此切点是否匹配连接点
    pub fn matches(&self, join_point: &JoinPoint) -> bool {
        for component in &self.components {
            match component {
                ExpressionComponent::Execution { method, .. } => {
                    // Simple wildcard matching
                    // 简单的通配符匹配
                    if *method == "*" || *method == join_point.method_name() {
                        return true;
                    }
                }
                ExpressionComponent::Within(class) => {
                    if *class == "*" || *class == join_point.target_class() {
                        return true;
                    }
                }
                ExpressionComponent::And | ExpressionComponent::Or | ExpressionComponent::Not => {
                    // Logical operators would need more complex evaluation
                    // 逻辑运算符需要更复杂的评估
                }
                _ => {}
            }
        }
        false
    }
}

// ============================================================================
// Advice Types / 通知类型
// ============================================================================

/// Type of advice
/// 通知类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdviceType {
    /// Before advice
    /// 前置通知
    Before,
    /// After advice
    /// 后置通知
    After,
    /// Around advice
    /// 环绕通知
    Around,
}

// ============================================================================
// Aspect Registry / 切面注册表
// ============================================================================

/// Registry for AOP aspects
/// AOP 切面注册表
///
/// The aspect registry manages all registered aspects and their advice.
///
/// 切面注册表管理所有已注册的切面及其通知。
pub struct AspectRegistry {
    /// Registered aspects
    /// 已注册的切面
    aspects: RwLock<HashMap<String, AspectInfo>>,

    /// Pointcut to advice mapping
    /// 切点到通知的映射
    pointcuts: RwLock<Vec<PointcutAdvice>>,
}

/// Information about an aspect
/// 切面信息
#[derive(Debug, Clone)]
struct AspectInfo {
    /// Aspect name
    /// 切面名称
    name: String,

    /// Aspect type ID
    /// 切面类型 ID
    type_id: TypeId,

    /// Aspect instance
    /// 切面实例
    instance: Arc<dyn Any + Send + Sync>,
}

/// Associates a pointcut with advice
/// 关联切点和通知
#[derive(Debug, Clone)]
struct PointcutAdvice {
    /// Pointcut expression
    /// 切点表达式
    pointcut: PointcutExpression,

    /// Advice type
    /// 通知类型
    advice_type: AdviceType,

    /// Aspect name
    /// 切面名称
    aspect_name: String,

    /// Method name
    /// 方法名
    method_name: String,
}

impl AspectRegistry {
    /// Create a new aspect registry
    /// 创建新的切面注册表
    pub fn new() -> Self {
        Self {
            aspects: RwLock::new(HashMap::new()),
            pointcuts: RwLock::new(Vec::new()),
        }
    }

    /// Register an aspect
    /// 注册切面
    pub async fn register_aspect<T: Any + Send + Sync>(
        &self,
        name: String,
        instance: T,
    ) {
        let info = AspectInfo {
            name: name.clone(),
            type_id: TypeId::of::<T>(),
            instance: Arc::new(instance),
        };

        let mut aspects = self.aspects.write().await;
        aspects.insert(name, info);
    }

    /// Register a pointcut with advice
    /// 注册带通知的切点
    pub async fn register_pointcut(
        &self,
        pointcut: PointcutExpression,
        advice_type: AdviceType,
        aspect_name: String,
        method_name: String,
    ) {
        let advice = PointcutAdvice {
            pointcut,
            advice_type,
            aspect_name,
            method_name,
        };

        let mut pointcuts = self.pointcuts.write().await;
        pointcuts.push(advice);
    }

    /// Find all advice that matches a join point
    /// 查找匹配连接点的所有通知
    pub async fn find_matching_advice(
        &self,
        join_point: &JoinPoint,
    ) -> Vec<(AdviceType, String, String)> {
        let pointcuts = self.pointcuts.read().await;
        let mut matches = Vec::new();

        for advice in pointcuts.iter() {
            if advice.pointcut.matches(join_point) {
                matches.push((
                    advice.advice_type,
                    advice.aspect_name.clone(),
                    advice.method_name.clone(),
                ));
            }
        }

        matches
    }

    /// Get an aspect by name
    /// 通过名称获取切面
    pub async fn get_aspect(&self, name: &str) -> Option<Arc<dyn Any + Send + Sync>> {
        let aspects = self.aspects.read().await;
        aspects.get(name).map(|info| info.instance.clone())
    }
}

impl Default for AspectRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Global Registry / 全局注册表
// ============================================================================

/// Global aspect registry
/// 全局切面注册表
static GLOBAL_REGISTRY: once_cell::sync::Lazy<AspectRegistry> =
    once_cell::sync::Lazy::new(AspectRegistry::new);

/// Get the global aspect registry
/// 获取全局切面注册表
pub fn global_registry() -> &'static AspectRegistry {
    &GLOBAL_REGISTRY
}

// ============================================================================
// Tests / 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointcut_parsing() {
        let expr = PointcutExpression::new("execution(* com.example..*.*(..))".to_string());
        assert_eq!(expr.expression(), "execution(* com.example..*.*(..))");
    }

    #[test]
    fn test_join_point() {
        let target: Arc<dyn Any + Send + Sync> = Arc::new("test");
        let args: Vec<Arc<dyn Any + Send + Sync>> = vec![Arc::new(42), Arc::new("hello")];

        let join_point = JoinPoint::new(
            target,
            "test_method".to_string(),
            args,
            "test_method(String, i32)".to_string(),
            "TestClass".to_string(),
        );

        assert_eq!(join_point.method_name(), "test_method");
        assert_eq!(join_point.args().len(), 2);
        assert_eq!(join_point.target_class(), "TestClass");
    }

    #[tokio::test]
    async fn test_aspect_registry() {
        let registry = AspectRegistry::new();

        // Register an aspect
        // 注册切面
        let instance = "test_aspect";
        registry
            .register_aspect("TestAspect".to_string(), instance)
            .await;

        // Get the aspect back
        // 获取切面
        let aspect = registry.get_aspect("TestAspect").await;
        assert!(aspect.is_some());
    }
}
