//! Transactional macro implementation
//! Transactional宏实现
//!
//! This module provides the #[transactional] procedural macro for automatic transaction management.
//! 本模块提供#[transactional]过程宏用于自动事务管理。

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, ItemFn, Lit, Meta, MetaNameValue, NestedMeta,
};

/// #[transactional] macro
/// #[transactional]宏
///
/// Equivalent to Spring's @Transactional annotation.
/// 等价于Spring的@Transactional注解。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// #[transactional]
/// async fn create_user(user: User) -> Result<User> {
///     // Transaction logic
/// }
///
/// #[transactional(propagation = "REQUIRES_NEW", isolation = "SERIALIZABLE")]
/// async fn transfer_money(from: u64, to: u64, amount: f64) -> Result<()> {
///     // Transaction logic
/// }
/// ```
#[proc_macro_attribute]
pub fn transactional(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = if attr.is_empty() {
        Vec::new()
    } else {
        // Parse as comma-separated list of meta items
        // 解析为逗号分隔的meta项列表
        syn::parse::Parser::parse2(
            syn::punctuated::Punctuated::<syn::NestedMeta, syn::Token![,]>::parse_terminated,
            proc_macro2::TokenStream::from(attr),
        )
        .unwrap_or_default()
        .into_iter()
        .collect()
    };
    let function = parse_macro_input!(item as ItemFn);

    // Parse attributes
    // 解析属性
    let options = parse_transactional_options(&args);

    let fn_name = &function.sig.ident;
    let fn_vis = &function.vis;
    let fn_async = &function.sig.asyncness;
    let fn_inputs = &function.sig.inputs;
    let fn_output = &function.sig.output;
    let fn_block = &function.block;
    let fn_attrs = &function.attrs;

    // Generate transaction wrapper code
    // 生成事务包装代码
    let expanded = quote! {
        #(#fn_attrs)*
        #fn_vis #fn_async fn #fn_name(#fn_inputs) #fn_output {
            use nexus_tx::{TransactionManager, TransactionDefinition, Propagation, IsolationLevel, TransactionalOptions};
            use std::sync::Arc;
            use once_cell::sync::Lazy;

            // Get transaction manager (simplified - in real implementation would be injected)
            // 获取事务管理器（简化版 - 实际实现中应该注入）
            static TX_MANAGER: Lazy<Arc<dyn TransactionManager>> = Lazy::new(|| {
                // In real implementation, get from container
                // 实际实现中，从容器获取
                todo!("Get transaction manager from container")
            });

            // Create transaction definition from options
            // 从选项创建事务定义
            let mut definition = TransactionDefinition::new(stringify!(#fn_name));

            #(if let Some(prop) = #options.propagation {
                definition = definition.propagation(prop);
            })*

            #(if let Some(iso) = #options.isolation {
                definition = definition.isolation(iso);
            })*

            #(if let Some(timeout) = #options.timeout_secs {
                definition = definition.timeout_secs(timeout);
            })*

            if #options.read_only {
                definition = definition.read_only(true);
            }

            // Begin transaction
            // 开始事务
            let status = match TX_MANAGER.begin(&definition).await {
                Ok(s) => s,
                Err(e) => return Err(e.into()),
            };

            // Execute the original function
            // 执行原函数
            let result = async move {
                #fn_block
            }.await;

            // Commit or rollback based on result
            // 根据结果提交或回滚
            match &result {
                Ok(_) => {
                    if let Err(e) = TX_MANAGER.commit(status).await {
                        return Err(e.into());
                    }
                }
                Err(ref err) => {
                    // Check if should rollback
                    // 检查是否应回滚
                    let options = TransactionalOptions {
                        propagation: #options.propagation,
                        isolation: #options.isolation,
                        timeout_secs: #options.timeout_secs,
                        read_only: #options.read_only,
                        rollback_for: vec![#(#options.rollback_for),*],
                        no_rollback_for: vec![#(#options.no_rollback_for),*],
                        name: #options.name,
                    };

                    // Simplified error check - in real implementation would use proper error types
                    // 简化的错误检查 - 实际实现中应使用正确的错误类型
                    let should_rollback = true; // Default: rollback on error

                    if should_rollback {
                        let _ = TX_MANAGER.rollback(status).await;
                    }
                }
            }

            result
        }
    };

    TokenStream::from(expanded)
}

/// Parsed transactional options
/// 解析的transactional选项
struct TransactionalOptionsParsed {
    propagation: Option<syn::Expr>,
    isolation: Option<syn::Expr>,
    timeout_secs: Option<syn::Expr>,
    read_only: bool,
    rollback_for: Vec<String>,
    no_rollback_for: Vec<String>,
    name: Option<String>,
}

/// Parse transactional options from attribute arguments
/// 从属性参数解析transactional选项
fn parse_transactional_options(args: &[syn::NestedMeta]) -> TransactionalOptionsParsed {
    let mut options = TransactionalOptionsParsed {
        propagation: None,
        isolation: None,
        timeout_secs: None,
        read_only: false,
        rollback_for: Vec::new(),
        no_rollback_for: Vec::new(),
        name: None,
    };

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) => {
                let ident = path.get_ident().map(|i| i.to_string()).unwrap_or_default();

                match ident.as_str() {
                    "propagation" => {
                        if let Lit::Str(s) = lit {
                            let prop_expr = match s.value().as_str() {
                                "REQUIRED" => quote! { Some(Propagation::Required) },
                                "REQUIRES_NEW" => quote! { Some(Propagation::RequiresNew) },
                                "SUPPORTS" => quote! { Some(Propagation::Supports) },
                                "NOT_SUPPORTED" => quote! { Some(Propagation::NotSupported) },
                                "MANDATORY" => quote! { Some(Propagation::Mandatory) },
                                "NEVER" => quote! { Some(Propagation::Never) },
                                "NESTED" => quote! { Some(Propagation::Nested) },
                                _ => quote! { None },
                            };
                            options.propagation = Some(syn::parse2(prop_expr).unwrap());
                        }
                    }
                    "isolation" => {
                        if let Lit::Str(s) = lit {
                            let iso_expr = match s.value().as_str() {
                                "READ_UNCOMMITTED" => quote! { Some(IsolationLevel::ReadUncommitted) },
                                "READ_COMMITTED" => quote! { Some(IsolationLevel::ReadCommitted) },
                                "REPEATABLE_READ" => quote! { Some(IsolationLevel::RepeatableRead) },
                                "SERIALIZABLE" => quote! { Some(IsolationLevel::Serializable) },
                                _ => quote! { None },
                            };
                            options.isolation = Some(syn::parse2(iso_expr).unwrap());
                        }
                    }
                    "timeout" | "timeout_secs" => {
                        if let Lit::Int(i) = lit {
                            options.timeout_secs = Some(syn::parse2(quote! { Some(#i) }).unwrap());
                        }
                    }
                    "read_only" => {
                        if let Lit::Bool(b) = lit {
                            options.read_only = b.value;
                        }
                    }
                    "name" => {
                        if let Lit::Str(s) = lit {
                            options.name = Some(s.value());
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    options
}
