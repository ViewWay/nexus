//! Value derive macro implementation
//! Value 派生宏实现

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, DataStruct, Fields};

/// Implement #[Value] derive macro
/// 实现 #[Value] 派生宏
///
/// Similar to @Data but for immutable value objects.
/// Similar to @Data but creates immutable value objects.
///
/// Generates:
/// 生成：
/// - Constructor with all fields / 包含所有字段的构造函数
/// - Getter methods / Getter 方法
/// - with_xxx methods (for creating modified copies) / with_xxx 方法（用于创建修改副本）
pub fn impl_value(input: DeriveInput) -> TokenStream {
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
                "#[Value] can only be used on structs with named fields",
            )
            .to_compile_error()
            .into()
        }
    };

    // Get field names and types
    // 获取字段名和类型
    let field_names: Vec<_> = fields
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .collect();

    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    // Generate constructor
    // 生成构造函数
    let constructor = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[inline]
            #[doc = "Creates a new value instance.\n"]
            #[doc = "创建新的值实例。"]
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(#field_names),*
                }
            }
        }
    };

    // Generate getters (only read access)
    // 生成 getters（只读访问）
    let getters = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        quote! {
            #[inline]
            pub fn #name(&self) -> &#ty {
                &self.#name
            }
        }
    });

    // Generate with_ methods (for creating modified copies)
    // 生成 with_ 方法（用于创建修改副本）
    let with_methods = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause
        where
            #struct_name: Clone,
        {
            #(
                #[inline]
                #[must_use]
                #[doc = concat!("Creates a modified copy with `", stringify!(#field_names), "` changed.")]
                pub fn with_#field_names(&self, #field_names: #field_types) -> Self {
                    Self {
                        #(#field_names: self.#field_names.clone(),)*
                        #field_names: #field_names,
                        ..self.clone()
                    }
                }
            )*
        }
    };

    // Combine all expansions
    // 合并所有展开
    let expanded = quote! {
        #constructor

        impl #impl_generics #struct_name #ty_generics #where_clause {
            #(#getters)*
        }

        #with_methods
    };

    TokenStream::from(expanded)
}
