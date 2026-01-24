//! Nexus Validation - Spring @Validated equivalent features
//! Nexus Validation - Spring @Validated 等价功能
//!
//! # Equivalent to Spring / 等价于 Spring
//!
//! - `@Validated` - `Validated` extractor
//! - `@Valid` - `Valid` extractor
//! - `@Validated` on method parameters - automatic validation
//! - Validation errors - structured error response
//!
//! # Example / 示例
//!
//! ```rust,no_run,ignore
//! use nexus_validation::Valid;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, Validate)]
//! struct CreateUser {
//!     #[validate(length(min = 3, max = 50))]
//!     username: String,
//!
//!     #[validate(email)]
//!     email: String,
//!
//!     #[validate(length(min = 8))]
//!     password: String,
//! }
//!
//! async fn create_user(Valid(user): Valid<CreateUser>) -> String {
//!     format!("User created: {}", user.username)
//! }
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod error;
pub mod extractor;
pub mod validate;

pub use error::{ValidationError, ValidationResult};
pub use extractor::Valid;
pub use validate::Validate;

/// Version of the validation module
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Re-exports of commonly used types
/// 常用类型的重新导出
pub mod prelude {
    pub use super::{Valid, Validate, ValidationError, ValidationResult};
}
