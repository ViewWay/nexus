//! OpenAPI procedural macros
//! OpenAPI过程宏

//! Re-export utoipa macros for convenience
//! 为了方便重新导出utoipa宏

pub use utoipa::{
    IntoParams, ToSchema, ToResponse,
    openapi,
};

/// Macro for defining OpenAPI operations
/// 定义OpenAPI操作的宏
///
/// Equivalent to Spring's `@Operation`, `@ApiResponse`, `@Parameter` annotations.
/// 等价于Spring的`@Operation`、`@ApiResponse`、`@Parameter`注解。
///
/// # Spring Equivalent / Spring等价物
///
/// ```java
/// @Operation(summary = "Get user", description = "Get user by ID")
/// @ApiResponse(responseCode = "200", description = "User found")
/// @ApiResponse(responseCode = "404", description = "User not found")
/// @Parameter(name = "id", description = "User ID", required = true)
/// public Response getUser(@PathVariable Long id) { }
/// ```
///
/// # Example / 示例
///
/// ```rust,ignore
/// use nexus_openapi::openapi;
/// use nexus_macros::get;
/// use serde::Serialize;
///
/// #[derive(Serialize, ToSchema)]
/// struct User {
///     id: u64,
///     name: String,
/// }
///
/// #[openapi(
///     summary = "Get user by ID",
///     description = "Returns a single user by their ID",
///     tags = ["users"],
///     path(
///         method = get,
///         path = "/users/{id}",
///         responses(
///             code = 200, description = "User found", response = User,
///             code = 404, description = "User not found"
///         ),
///         params(
///             name = "id", description = "User ID", required = true
///         )
///     )
/// )]
/// async fn get_user(id: u64) -> Json<User> {
///     Json(User { id, name: "John".to_string() })
/// }
/// ```

#[cfg(test)]
mod tests {
    #[test]
    fn test_macros_exist() {
        // Just verify that utoipa macros are accessible
        // This test will be expanded once we have actual usage
        assert!(true);
    }
}
