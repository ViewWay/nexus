//! Nexus OpenAPI - OpenAPI/Swagger documentation support
//! Nexus OpenAPI - OpenAPI/Swagger 文档支持
//!
//! # Equivalent to Spring / 等价于 Spring
//!
//! - `@Operation` - `#[openapi]`
//! - `@ApiResponse` - `#[response]`
//! - `@Parameter` - `#[param]`
//! - `@Tag` - `#[tag]`
//! - `@Schema` - `#[schema]`
//! - SpringDoc OpenAPI - OpenApi
//!
//! # Example / 示例
//!
//! ```rust,ignore
//! use nexus_openapi::{OpenApi, OpenApiConfig};
//! use nexus_macros::get;
//! use serde::Serialize;
//!
//! #[derive(Serialize)]
//! struct User {
//!     id: u64,
//!     name: String,
//! }
//!
//! #[openapi(
//!     summary = "Get user by ID",
//!     description = "Returns a single user",
//!     tags = ["users"],
//!     responses(
//!         code = 200, description = "User found",
//!         code = 404, description = "User not found"
//!     )
//! )]
//! #[get("/users/{id}")]
//! async fn get_user(id: u64) -> Json<User> {
//!     Json(User { id, name: "John".to_string() })
//! }
//!
//! let openapi = OpenApi::new(OpenApiConfig::default());
//! let spec = openapi.generate().await?;
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod config;
pub mod schema;
pub mod operation;
pub mod response;
pub mod path;
pub mod openapi;
pub mod macros;

pub use config::{OpenApiConfig, ServerConfig, ContactConfig, LicenseConfig, InfoConfig, TagConfig, ExternalDocsConfig};
pub use schema::{Schema, SchemaType, SchemaFormat, SchemaProperty};
pub use operation::{Operation, Parameter, ParameterLocation, SecurityScheme, RequestBody};
pub use response::{Response, ResponseContent, ApiResponse};
pub use path::{PathItem, PathMethod, PathOperation, Components};
pub use openapi::OpenApi;

/// Version of the OpenAPI module
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default OpenAPI version
/// 默认OpenAPI版本
pub const OPENAPI_VERSION: &str = "3.0.3";

/// Re-exports of commonly used types
/// 常用类型的重新导出
pub mod prelude {
    pub use super::{
        OpenApi, OpenApiConfig, ServerConfig, ContactConfig, LicenseConfig,
        Schema, SchemaType, SchemaFormat, SchemaProperty,
        Operation, Parameter, ParameterLocation, SecurityScheme,
        Response, ResponseContent, ApiResponse,
        PathItem, PathMethod, PathOperation,
        OPENAPI_VERSION,
    };
}

/// OpenApi trait for generating documentation
/// OpenApi trait 用于生成文档
///
/// Equivalent to SpringDoc's OpenAPI annotation.
/// 等价于SpringDoc的OpenAPI注解。
///
/// # Spring Equivalent / Spring等价物
///
/// ```java
/// @OpenAPIDefinition(
///     info = @Info(
///         title = "My API",
///         version = "1.0.0"
///     )
/// )
/// ```
pub trait GenerateOpenApi {
    /// Generate OpenAPI specification
    /// 生成OpenAPI规范
    fn generate(&self) -> Result<utoipa::openapi::OpenApi, String>;
}

/// Default implementation for utoipa::OpenApi
impl<T: utoipa::OpenApi> GenerateOpenApi for T {
    fn generate(&self) -> Result<utoipa::openapi::OpenApi, String> {
        Ok(T::openapi())
    }
}
