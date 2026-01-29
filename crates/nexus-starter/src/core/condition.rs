//! 条件注解系统 / Conditional Annotation System
//!
//! 实现类似 Spring Boot 的条件装配功能。
//! Implements conditional assembly similar to Spring Boot.
//!
//! 参考 Spring Boot 的 @Conditional* 注解：
//! - @ConditionalOnClass → ConditionalOnFeature
//! - @ConditionalOnProperty → ConditionalOnProperty
//! - @ConditionalOnMissingBean → ConditionalOnMissingBean

use std::any::TypeId;
use std::fmt::Debug;

use super::container::ApplicationContext;

// ============================================================================
// Conditional Trait / 条件 Trait
// ============================================================================

/// 条件 trait
/// Condition trait
///
/// 用于判断是否应该应用某个配置。
/// Used to determine whether a configuration should be applied.
pub trait Conditional: Send + Sync {
    /// 检查条件是否满足
    /// Check if the condition is met
    fn matches(&self, ctx: &ApplicationContext) -> bool;
}

// ============================================================================
// 内置条件实现 / Built-in Conditions
// ============================================================================

/// 属性条件
/// Property condition
///
/// 检查配置属性是否存在或有特定值。
/// Checks if a configuration property exists or has a specific value.
///
/// 等价于 Spring Boot 的 `@ConditionalOnProperty`。
/// Equivalent to Spring Boot's `@ConditionalOnProperty`.
#[derive(Debug, Clone)]
pub struct ConditionalOnProperty {
    /// 属性键
    pub key: String,

    /// 期望的值（None 表示只需存在）
    pub value: Option<String>,

    /// 是否要求值不为空
    pub match_if_empty: bool,
}

impl ConditionalOnProperty {
    /// 创建新的属性条件
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
            match_if_empty: false,
        }
    }

    /// 设置期望的值
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// 设置 match_if_empty
    pub fn match_if_empty(mut self, match_if_empty: bool) -> Self {
        self.match_if_empty = match_if_empty;
        self
    }
}

impl Conditional for ConditionalOnProperty {
    fn matches(&self, ctx: &ApplicationContext) -> bool {
        if let Some(actual_value) = ctx.get_property(&self.key) {
            if let Some(ref expected_value) = self.value {
                actual_value == *expected_value
            } else {
                self.match_if_empty || !actual_value.is_empty()
            }
        } else {
            false
        }
    }
}

// ============================================================================
// Bean 缺失条件 / Bean Missing Condition
// ============================================================================

/// Bean 缺失条件
/// Bean missing condition
///
/// 检查容器中是否不存在指定类型的 Bean。
/// Checks if a bean of the specified type doesn't exist in the container.
///
/// 等价于 Spring Boot 的 `@ConditionalOnMissingBean`。
/// Equivalent to Spring Boot's `@ConditionalOnMissingBean`.
#[derive(Debug, Clone)]
pub struct ConditionalOnMissingBean {
    /// Bean 类型 ID
    pub type_id: TypeId,

    /// Bean 类型名称
    pub type_name: &'static str,
}

impl ConditionalOnMissingBean {
    /// 创建新的 Bean 缺失条件
    pub fn new<T: 'static>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            type_name: std::any::type_name::<T>(),
        }
    }
}

impl Conditional for ConditionalOnMissingBean {
    fn matches(&self, ctx: &ApplicationContext) -> bool {
        !ctx.contains_bean_by_id(self.type_id)
    }
}

// ============================================================================
// Bean 存在条件 / Bean Present Condition
// ============================================================================

/// Bean 存在条件
/// Bean present condition
///
/// 检查容器中是否存在指定类型的 Bean。
/// Checks if a bean of the specified type exists in the container.
///
/// 等价于 Spring Boot 的 `@ConditionalOnBean`。
/// Equivalent to Spring Boot's `@ConditionalOnBean`.
#[derive(Debug, Clone)]
pub struct ConditionalOnBean {
    /// Bean 类型 ID
    pub type_id: TypeId,

    /// Bean 类型名称
    pub type_name: &'static str,
}

impl ConditionalOnBean {
    /// 创建新的 Bean 存在条件
    pub fn new<T: 'static>() -> Self {
        Self {
            type_id: TypeId::of::<T>(),
            type_name: std::any::type_name::<T>(),
        }
    }
}

impl Conditional for ConditionalOnBean {
    fn matches(&self, ctx: &ApplicationContext) -> bool {
        ctx.contains_bean_by_id(self.type_id)
    }
}

// ============================================================================
// Feature 条件 / Feature Condition
// ============================================================================

/// Feature 条件
/// Feature condition
///
/// 检查某个 feature 是否启用。
/// Checks if a feature is enabled.
///
/// 等价于 Spring Boot 的 `@ConditionalOnClass`。
/// Equivalent to Spring Boot's `@ConditionalOnClass`.
#[derive(Debug, Clone)]
pub struct ConditionalOnFeature {
    /// Feature 名称
    pub feature: String,
}

impl ConditionalOnFeature {
    /// 创建新的 Feature 条件
    pub fn new(feature: impl Into<String>) -> Self {
        Self {
            feature: feature.into(),
        }
    }
}

impl Conditional for ConditionalOnFeature {
    fn matches(&self, _ctx: &ApplicationContext) -> bool {
        // 检查 feature 是否启用（通过环境变量）
        std::env::var("CARGO_FEATURE_")
            .or_else(|_| std::env::var(format!("NEXUS_FEATURE_{}", self.feature.to_uppercase())))
            .is_ok()
    }
}

// ============================================================================
// 表达式条件 / Expression Condition
// ============================================================================

/// 表达式条件
/// Expression condition
///
/// 支持类似 SpEL 的简单表达式。
/// Supports simple expressions similar to SpEL.
///
/// # 示例 / Example
///
/// ```rust,ignore
/// // "hasRole('ADMIN') and !disabled"
/// // "cache.enabled == true"
/// ```
#[derive(Debug, Clone)]
pub struct ConditionalOnExpression {
    /// 表达式字符串
    pub expression: String,
}

impl ConditionalOnExpression {
    /// 创建新的表达式条件
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
        }
    }
}

impl Conditional for ConditionalOnExpression {
    fn matches(&self, ctx: &ApplicationContext) -> bool {
        // 简单的表达式解析（TODO: 实现完整的表达式解析器）
        let expr = self.expression.trim();

        // 处理 "key == value" 形式
        if let Some(eq_pos) = expr.find("==") {
            let key = expr[..eq_pos].trim();
            let expected_value = expr[eq_pos + 2..].trim().trim_matches('"');
            if let Some(actual_value) = ctx.get_property(key) {
                return actual_value == expected_value;
            }
            return false;
        }

        // 处理 "key != value" 形式
        if let Some(ne_pos) = expr.find("!=") {
            let key = expr[..ne_pos].trim();
            let expected_value = expr[ne_pos + 2..].trim().trim_matches('"');
            if let Some(actual_value) = ctx.get_property(key) {
                return actual_value != expected_value;
            }
            return true;
        }

        // 处理单个 key（检查是否存在）
        if let Some(value) = ctx.get_property(expr) {
            return !value.is_empty();
        }

        false
    }
}

// ============================================================================
// 条件组合 / Condition Composition
// ============================================================================

/// 所有条件都必须满足（AND）
/// All conditions must be met (AND)
pub struct AllConditions {
    conditions: Vec<Box<dyn Conditional>>,
}

impl AllConditions {
    /// 创建新的 AND 条件组合
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    /// 添加条件
    pub fn add(mut self, condition: Box<dyn Conditional>) -> Self {
        self.conditions.push(condition);
        self
    }
}

impl Default for AllConditions {
    fn default() -> Self {
        Self::new()
    }
}

impl Conditional for AllConditions {
    fn matches(&self, ctx: &ApplicationContext) -> bool {
        self.conditions.iter().all(|c| c.matches(ctx))
    }
}

/// 任一条件满足即可（OR）
/// Any condition can be met (OR)
pub struct AnyConditions {
    conditions: Vec<Box<dyn Conditional>>,
}

impl AnyConditions {
    /// 创建新的 OR 条件组合
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
        }
    }

    /// 添加条件
    pub fn add(mut self, condition: Box<dyn Conditional>) -> Self {
        self.conditions.push(condition);
        self
    }
}

impl Default for AnyConditions {
    fn default() -> Self {
        Self::new()
    }
}

impl Conditional for AnyConditions {
    fn matches(&self, ctx: &ApplicationContext) -> bool {
        self.conditions.iter().any(|c| c.matches(ctx))
    }
}

/// 条件不满足（NOT）
/// Condition is not met (NOT)
pub struct NotCondition {
    condition: Box<dyn Conditional>,
}

impl NotCondition {
    /// 创建新的 NOT 条件
    pub fn new(condition: Box<dyn Conditional>) -> Self {
        Self { condition }
    }
}

impl Conditional for NotCondition {
    fn matches(&self, ctx: &ApplicationContext) -> bool {
        !self.condition.matches(ctx)
    }
}

// ============================================================================
// 辅助宏 / Helper Macros
// ============================================================================

/// 创建属性条件的宏
/// Macro to create property condition
#[macro_export]
macro_rules! cond_on_property {
    ($key:expr) => {
        $crate::core::condition::ConditionalOnProperty::new($key)
    };
    ($key:expr, $value:expr) => {
        $crate::core::condition::ConditionalOnProperty::new($key).value($value)
    };
}

/// 创建 Bean 缺失条件的宏
/// Macro to create bean missing condition
#[macro_export]
macro_rules! cond_on_missing_bean {
    ($type:ty) => {
        $crate::core::condition::ConditionalOnMissingBean::new::<$type>()
    };
}

/// 创建组合条件的宏
/// Macro to create composite conditions
#[macro_export]
macro_rules! all_conditions {
    ($($condition:expr),* $(,)?) => {{
        let mut all = $crate::core::condition::AllConditions::new();
        $(
            all = all.add(Box::new($condition));
        )*
        all
    }};
}

#[macro_export]
macro_rules! any_conditions {
    ($($condition:expr),* $(,)?) => {{
        let mut any = $crate::core::condition::AnyConditions::new();
        $(
            any = any.add(Box::new($condition));
        )*
        any
    }};
}

// ============================================================================
// 测试 / Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditional_on_property() {
        let ctx = ApplicationContext::new();
        let cond = ConditionalOnProperty::new("test.key");

        // 属性不存在，应该返回 false
        assert!(!cond.matches(&ctx));
    }

    #[test]
    fn test_conditional_on_expression() {
        let mut ctx = ApplicationContext::new();
        ctx.register_bean("test_value".to_string());

        // 测试简单表达式
        let cond = ConditionalOnExpression::new("test_key");
        // 由于没有这个属性，应该返回 false
        assert!(!cond.matches(&ctx));
    }

    #[test]
    fn test_all_conditions() {
        let ctx = ApplicationContext::new();
        let cond1 = ConditionalOnProperty::new("key1");
        let cond2 = ConditionalOnProperty::new("key2");
        let all = all_conditions!(cond1, cond2);

        // 两个条件都不满足
        assert!(!all.matches(&ctx));
    }

    #[test]
    fn test_any_conditions() {
        let ctx = ApplicationContext::new();
        let cond1 = ConditionalOnProperty::new("key1");
        let cond2 = ConditionalOnProperty::new("key2");
        let any = any_conditions!(cond1, cond2);

        // 两个条件都不满足
        assert!(!any.matches(&ctx));
    }
}
