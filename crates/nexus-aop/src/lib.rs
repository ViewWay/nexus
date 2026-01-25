//! # Nexus AOP
//!
//! Spring AOP style annotations for Nexus framework
//! Nexus 框架的 Spring AOP 风格注解
//!
//! ## Features / 功能
//!
//! - **`#[Aspect]`** - Marks a struct as an aspect
//! - **`@Before`** - Before advice (runs before method execution)
//! - **`@After`** - After advice (runs after method execution)
//! - **`@Around`** - Around advice (wraps method execution)
//! - **`@Pointcut`** - Pointcut definition (reusable join points)
//!
//! ## Example / 示例
//!
//! ```rust,no_run,ignore
//! use nexus_aop::{Aspect, Before, After, Around};
//!
//! #[Aspect]
//! struct LoggingAspect;
//!
//! impl LoggingAspect {
//!     #[Before("execution(* com.example..*.*(..))")]
//!     fn log_before(&self, join_point: &JoinPoint) {
//!         println!("Entering: {}", join_point.method_name());
//!     }
//!
//!     #[After("execution(* com.example..*.*(..))")]
//!     fn log_after(&self, join_point: &JoinPoint) {
//!         println!("Exiting: {}", join_point.method_name());
//!     }
//!
//!     #[Around("execution(* com.example..*.*(..))")]
//!     fn log_around(&self, join_point: JoinPoint) -> Result<(), Error> {
//!         println!("Before: {}", join_point.method_name());
//!         let result = join_point.proceed()?;
//!         println!("After: {}", join_point.method_name());
//!         Ok(result)
//!     }
//! }
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]

use proc_macro::TokenStream;

// ========================================================================
// Aspect Annotations / Aspect 注解
// ========================================================================

pub mod aspect;
pub mod advice;
pub mod pointcut;
pub mod runtime;

/// Marks a struct as an AOP aspect
/// 将结构体标记为 AOP 切面
///
/// # Example / 示例
///
/// ```rust
/// use nexus_aop::Aspect;
///
/// #[Aspect]
/// struct LoggingAspect;
/// ```
#[proc_macro_attribute]
pub fn aspect(_attr: TokenStream, item: TokenStream) -> TokenStream {
    aspect::impl_aspect(_attr, item)
}

// ========================================================================
// Advice Annotations / 通知注解
// ========================================================================

/// Marks a method as before advice
/// 将方法标记为前置通知
///
/// # Example / 示例
///
/// ```rust
/// use nexus_aop::Before;
///
/// #[Before("execution(* com.example..*.*(..))")]
/// fn log_before(&self, join_point: &JoinPoint) {
///     println!("Entering method");
/// }
/// ```
#[proc_macro_attribute]
pub fn before(attr: TokenStream, item: TokenStream) -> TokenStream {
    advice::impl_before(attr, item)
}

/// Marks a method as after advice
/// 将方法标记为后置通知
///
/// # Example / 示例
///
/// ```rust
/// use nexus_aop::After;
///
/// #[After("execution(* com.example..*.*(..))")]
/// fn log_after(&self, join_point: &JoinPoint) {
///     println!("Exiting method");
/// }
/// ```
#[proc_macro_attribute]
pub fn after(attr: TokenStream, item: TokenStream) -> TokenStream {
    advice::impl_after(attr, item)
}

/// Marks a method as around advice
/// 将方法标记为环绕通知
///
/// # Example / 示例
///
/// ```rust
/// use nexus_aop::Around;
///
/// #[Around("execution(* com.example..*.*(..))")]
/// fn log_around(&self, join_point: JoinPoint) -> Result<(), Error> {
///     println!("Before method");
///     let result = join_point.proceed()?;
///     println!("After method");
///     Ok(result)
/// }
/// ```
#[proc_macro_attribute]
pub fn around(attr: TokenStream, item: TokenStream) -> TokenStream {
    advice::impl_around(attr, item)
}

// ========================================================================
// Pointcut Annotations / 切点注解
// ========================================================================

/// Defines a reusable pointcut expression
/// 定义可重用的切点表达式
///
/// # Example / 示例
///
/// ```rust
/// use nexus_aop::Pointcut;
///
/// #[Pointcut("execution(* com.example..*.*(..))")]
/// fn all_methods() -> PointcutExpression {
///     // Returns a reusable pointcut
/// }
/// ```
#[proc_macro_attribute]
pub fn pointcut(attr: TokenStream, item: TokenStream) -> TokenStream {
    pointcut::impl_pointcut(attr, item)
}

// ========================================================================
// Re-exports / 重新导出
// ========================================================================

pub use aspect::aspect as Aspect;
pub use advice::{before as Before, after as After, around as Around};
pub use pointcut::pointcut as Pointcut;

// ============================================================================
// Runtime Re-exports / 运行时重新导出
// ============================================================================

pub use runtime::{
    JoinPoint,
    PointcutExpression,
    AdviceType,
    AspectRegistry,
    global_registry,
};
