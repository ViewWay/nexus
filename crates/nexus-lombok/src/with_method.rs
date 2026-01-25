//! With derive macro implementation
//! With 派生宏实现

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, DataStruct, Fields};

/// Implement #[With] derive macro
/// 实现 #[With] 派生宏
///
/// Generates with_xxx methods for creating modified copies of struct instances.
/// 生成用于创建结构体实例修改副本的 with_xxx 方法。
///
/// Note: The struct must implement Clone for this to work.
/// 注意：结构体必须实现 Clone 才能使用此宏。
pub fn impl_with(input: DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

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
                "#[With] can only be used on structs with named fields",
            )
            .to_compile_error()
            .into()
        }
    };

    // Get field names and types
    // 获取字段名和类型
    let with_methods = fields.iter().filter_map(|field| {
        let field_name = field.ident.as_ref()?;
        let field_type = &field.ty;

        // Check if field should be skipped
        // 检查是否应跳过字段
        let should_skip = field.attrs.iter().any(|attr| {
            attr.path()
                .segments
                .iter()
                .any(|seg| seg.ident == "with" || seg.ident == "skip")
        });

        if should_skip {
            return None;
        }

        Some(quote! {
            #[inline]
            #[must_use]
            pub fn with_#field_name(&self, #field_name: #field_type) -> Self
            where
                Self: Clone,
            {
                let mut clone = self.clone();
                clone.#field_name = #field_name;
                clone
            }
        })
    });

    // Add Clone bound requirement
    // 添加 Clone bound 要求
    let where_clause = if where_clause.is_some() {
        quote! { #where_clause }
    } else {
        quote! { where #struct_name: Clone }
    };

    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#with_methods)*
        }
    };

    TokenStream::from(expanded)
}
