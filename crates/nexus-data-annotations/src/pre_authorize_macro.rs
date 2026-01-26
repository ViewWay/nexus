//! @PreAuthorize 宏实现
//! @PreAuthorize Macro Implementation
//!
//! 提供方法级权限检查注解
//! Provides method-level permission checking annotation

use darling::{FromDeriveInput, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::{AttributeArgs, ItemFn, parse_macro_input};

/// @PreAuthorize 属性
/// @PreAuthorize Attributes
#[derive(Debug, FromMeta)]
#[darling(attributes(authorization))]
pub struct PreAuthorizeAttrs {
    /// 权限表达式 / Permission expression
    pub expression: String,
}

/// @PreAuthorize 注解
/// @PreAuthorize Annotation
///
/// 在方法执行前检查权限
/// Check permissions before method execution
///
/// # Example / 示例
///
/// ```rust
/// use nexus_data_annotations::PreAuthorize;
///
/// impl UserService {
///     #[PreAuthorize("has_role('ADMIN')")]
///     async fn delete_user(&self, id: i64) -> Result<(), Error> {
///         // 只有 ADMIN 角色才能执行 / Only ADMIN role can execute
///         self.repository.delete(id).await
///     }
///
///     #[PreAuthorize("has_role('ADMIN') or #id == auth.user_id()")]
///     async fn update_profile(&self, auth: &AuthContext, id: i64, data: UpdateData) -> Result<(), Error> {
///         // 管理员或本人可以修改 / Admin or owner can modify
///         self.repository.update(id, data).await
///     }
/// }
/// ```
pub fn pre_authorize_macro_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = parse_macro_input!(attr with AttributeArgs);

    // 解析权限表达式 / Parse permission expression
    let expression = if attrs.args.is_empty() {
        return syn::Error::new(
            proc_macro2::Span::call_site(),
            "Missing authorization expression".to_string(),
        )
        .to_compile_error();
    } else {
        attrs.args.first().unwrap().to_string().replace('\"', "")
    };

    // 生成包装代码 / Generate wrapper code
    let fn_name = &input.sig.ident;
    let fn_name_str = fn_name.to_string();
    let visibility = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let attrs = &input.attrs;

    // 生成权限检查代码 / Generate permission check code
    let expanded = quote! {
        fn authorization_check(auth: &nexus_security::AuthContext, args: &std::collections::HashMap<String, serde_json::Value>) -> bool {
            nexus_security::evaluate_expression(#expression, auth, args)
        }

        #(#attrs)*
        #visibility #sig {
            // 提取认证上下文 / Extract auth context
            let auth = nexus_security::SecurityContext::current()
                .expect("Auth context not found");

            // 提取方法参数 / Extract method arguments
            let mut args_map = std::collections::HashMap::new();
            // TODO: 从方法签名中提取参数
            // TODO: Extract parameters from method signature

            // 检查权限 / Check permission
            if !authorization_check(&auth, &args_map) {
                return Err(nexus_http::Error::Forbidden(
                    "Permission denied".to_string()
                ));
            }

            // 执行原方法 / Execute original method
            #block
        }
    };

    TokenStream::from(expanded)
}

/// 将 @PreAuthorize 添加到 transactional_macro 模块
/// Add @PreAuthorize to transactional_macro module
pub use darling::FromDeriveInput;

/// 实现 PreAuthorize 注解的函数
/// Implement PreAuthorize annotation function
#[proc_macro_attribute]
pub fn pre_authorize(attr: TokenStream, item: TokenStream) -> TokenStream {
    pre_authorize_macro_impl(attr, item)
}

/// 重新导出
/// Re-export
pub struct PreAuthorize {
    expression: String,
}

impl PreAuthorize {
    /// 创建 PreAuthorize 注解实例
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
        }
    }

    /// 获取表达式
    pub fn expression(&self) -> &str {
        &self.expression
    }
}

/// 权限表达式
/// Permission expression
pub struct SecurityExpression {
    expression: String,
}

impl SecurityExpression {
    /// 创建权限表达式
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
        }
    }

    /// 检查角色
    pub fn has_role(&self, role: impl Into<String>) -> Self {
        Self {
            expression: format!("has_role('{}')", role.into()),
        }
    }

    /// 检查权限
    pub fn has_permission(&self, permission: impl Into<String>) -> Self {
        Self {
            expression: format!("has_permission('{}')", permission.into()),
        }
    }

    /// 或操作
    pub fn or(&self, other: SecurityExpression) -> SecurityExpression {
        SecurityExpression {
            expression: format!("{} or {}", self.expression, other.expression),
        }
    }

    /// 与操作
    pub fn and(&self, other: SecurityExpression) -> SecurityExpression {
        SecurityExpression {
            expression: format!("{} and {}", self.expression, other.expression),
        }
    }

    /// 获取表达式字符串
    pub fn as_str(&self) -> &str {
        &self.expression
    }
}

/// 转换为字符串
impl From<SecurityExpression> for String {
    fn from(expr: SecurityExpression) -> Self {
        expr.expression
    }
}

/// 权限检查器 trait
/// Permission checker trait
pub trait PermissionChecker: Send + Sync {
    /// 检查权限
    /// Check permission
    fn check_permission(&self, expression: &str, auth: &nexus_security::AuthContext) -> bool;
}

/// 默认权限检查器
/// Default permission checker
pub struct DefaultPermissionChecker;

impl PermissionChecker for DefaultPermissionChecker {
    fn check_permission(&self, expression: &str, auth: &nexus_security::AuthContext) -> bool {
        nexus_security::evaluate_expression(expression, auth, &std::collections::HashMap::new())
    }
}
