//! Data derive macro implementation
//! Data 派生宏实现

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, DataStruct, Fields};

/// Implement #[Data] derive macro
/// 实现 #[Data] 派生宏
///
/// Combines functionality from:
/// 结合以下功能：
/// - AllArgsConstructor (constructor with all fields)
/// - Getter (getter methods)
/// - Setter (setter methods)
/// - With (with_xxx methods for chaining)
pub fn impl_data(input: DeriveInput) -> TokenStream {
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
                "#[Data] can only be used on structs with named fields",
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

    // Generate constructor: new()
    // 生成构造函数: new()
    let constructor = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[inline]
            #[doc = "Creates a new instance of the struct with all fields.\n"]
            #[doc = "创建包含所有字段的结构体新实例。"]
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(#field_names),*
                }
            }
        }
    };

    // Generate getters
    // 生成 getters
    let getters = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        quote! {
            #[inline]
            pub fn #name(&self) -> #ty {
                self.#name
            }
        }
    });

    // Generate setters
    // 生成 setters
    let setters = field_names.iter().zip(field_types.iter()).map(|(name, ty)| {
        quote! {
            #[inline]
            pub fn set_#name(&mut self, #name: #ty) {
                self.#name = #name;
            }
        }
    });

    // Generate with_ methods (requires Clone)
    // 生成 with_ 方法（需要 Clone）
    let with_methods = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause
        where
            #struct_name: Clone,
        {
            #(
                #[inline]
                #[doc = concat!("Creates a modified copy with `", stringify!(#field_names), "` changed.")]
                pub fn with_#field_names(&self, #field_names: #field_types) -> Self {
                    let mut clone = self.clone();
                    clone.#field_names = #field_names;
                    clone
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
            #(#setters)*
        }

        #with_methods
    };

    TokenStream::from(expanded)
}
