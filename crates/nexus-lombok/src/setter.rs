//! Setter derive macro implementation
//! Setter 派生宏实现

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, DataStruct, Fields};

/// Implement #[Setter] derive macro
/// 实现 #[Setter] 派生宏
pub fn impl_setter(input: DeriveInput) -> TokenStream {
    let struct_name = &input.ident;

    // Extract fields from struct
    // 从结构体中提取字段
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => {
            return syn::Error::new_spanned(
                struct_name,
                "#[Setter] can only be used on structs with named fields",
            )
            .to_compile_error()
            .into()
        }
    };

    // Check for chain attribute
    // 检查 chain 属性
    let has_chain_attr = input.attrs.iter().any(|attr| {
        attr.path()
            .segments
            .last()
            .map(|seg| seg.ident == "setter")
            .unwrap_or(false)
    });

    // Generate setter methods for each field
    // 为每个字段生成 setter 方法
    let setters = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref()?;
        let field_type = &field.ty;

        // Check if field should be skipped
        // 检查是否应跳过字段
        let should_skip = field.attrs.iter().any(|attr| {
            attr.path()
                .segments
                .iter()
                .any(|seg| seg.ident == "set" || seg.ident == "skip")
        });

        if should_skip {
            return None;
        }

        // Return type based on chain attribute
        // 根据 chain 属性确定返回类型
        if has_chain_attr {
            // Return &mut Self for chaining
            // 返回 &mut Self 以支持链式调用
            Some(quote! {
                #[inline]
                pub fn #field_name(&mut self, #field_name: #field_type) -> &mut Self {
                    self.#field_name = #field_name;
                    self
                }
            })
        } else {
            // Return ()
            // 返回 ()
            Some(quote! {
                #[inline]
                pub fn set_#field_name(&mut self, #field_name: #field_type) {
                    self.#field_name = #field_name;
                }
            })
        }
    });

    let expanded = quote! {
        impl #struct_name {
            #(#setters)*
        }
    };

    TokenStream::from(expanded)
}
