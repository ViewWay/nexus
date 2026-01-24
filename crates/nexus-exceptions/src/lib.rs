//! Nexus Exceptions - Spring @ControllerAdvice equivalent features
//! Nexus Exceptions - Spring @ControllerAdvice 等价功能
//!
//! # Equivalent to Spring / 等价于 Spring
//!
//! - `@ControllerAdvice` - `ControllerAdvice` trait
//! - `@ExceptionHandler` - `ExceptionHandler` trait
//! - `ResponseEntityExceptionHandler` - `DefaultExceptionHandler`
//! - `@ResponseStatus` - `status_code()` method
//!
//! # Example / 示例
//!
//! ```rust,no_run,ignore
//! use nexus_exceptions::{ControllerAdvice, ExceptionHandler};
//! use nexus_http::{Request, Response, StatusCode};
//!
//! struct MyExceptionHandler;
//!
//! impl ControllerAdvice for MyExceptionHandler {
//!     // Auto-generated from @ExceptionHandler methods
//! }
//!
//! impl ExceptionHandler<ValidationError> for MyExceptionHandler {
//!     fn handle(&self, err: ValidationError, _req: &Request) -> Response {
//!         Response::bad_request().json(serde_json::json!({
//!             "error": err.message
//!         }))
//!     }
//! }
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod advice;
pub mod handler;
pub mod response;
pub mod error_body;

pub use advice::ControllerAdvice;
pub use handler::{ExceptionHandler, HandlerResult};
pub use response::ErrorResponse;
pub use error_body::ErrorBody;

/// Version of the exceptions module
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Re-exports of commonly used types
/// 常用类型的重新导出
pub mod prelude {
    pub use super::{ControllerAdvice, ExceptionHandler, ErrorResponse, ErrorBody};
}
