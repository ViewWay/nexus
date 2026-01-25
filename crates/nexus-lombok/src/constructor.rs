//! Constructor derive macros implementation
//! 构造函数派生宏实现

use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Data, DataStruct, Fields};

/// Implement #[AllArgsConstructor] derive macro
/// 实现 #[AllArgsConstructor] 派生宏
pub fn impl_all_args(input: DeriveInput) -> TokenStream {
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
                "#[AllArgsConstructor] can only be used on structs with named fields",
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

    // Check for static_factory attribute
    // 检查 static_factory 属性
    let factory_method_name = input
        .attrs
        .iter()
        .find(|attr| {
            attr.path()
                .segments
                .last()
                .map(|seg| seg.ident == "constructor")
                .unwrap_or(false)
        })
        .and_then(|attr| {
            attr.parse_meta()
                .ok()
                .and_then(|meta| {
                    if let syn::Meta::List(meta_list) = meta {
                        meta_list
                            .nested
                            .pairs
                            .find(|pair| {
                                pair
                                    .key()
                                    .ident
                                    .map(|id| id == "static_name")
                                    .unwrap_or(false)
                            })
                            .and_then(|pair| {
                                if let syn::NestedMeta::Meta(syn::Meta::NameValue(path)) =
                                    pair.value()
                                {
                                    if let syn::Lit::Str(lit_str) = &path.lit {
                                        Some(lit_str.value())
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                    } else {
                        None
                    }
                })
        })
        .unwrap_or_else(|| "new".to_string());

    let factory_ident = syn::Ident::new(&factory_method_name, struct_name.span());

    // Generate constructor
    // 生成构造函数
    let expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[inline]
            pub fn #factory_ident(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(#field_names),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Implement #[NoArgsConstructor] derive macro
/// 实现 #[NoArgsConstructor] 派生宏
pub fn impl_no_args(input: DeriveInput) -> TokenStream {
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
                "#[NoArgsConstructor] can only be used on structs with named fields",
            )
            .to_compile_error()
            .into()
        }
    };

    // Get field names
    // 获取字段名
    let field_names: Vec<_> = fields
        .iter()
        .filter_map(|f| f.ident.as_ref())
        .collect();

    // Generate constructor with Default::default() for each field
    // 为每个字段生成使用 Default::default() 的构造函数
    let constructor_expanded = quote! {
        impl #impl_generics #struct_name #ty_generics #where_clause {
            #[inline]
            pub fn new() -> Self
            where
                Self: Default,
            {
                Self::default()
            }
        }
    };

    // Generate Default implementation
    // 生成 Default 实现
    let default_expanded = quote! {
        impl #impl_generics Default for #struct_name #ty_generics #where_clause
        where
            #(#field_names: Default,)*
        {
            #[inline]
            fn default() -> Self {
                Self {
                    #(#field_names: Default::default()),*
                }
            }
        }
    };

    let expanded = quote! {
        #constructor_expanded
        #default_expanded
    };

    TokenStream::from(expanded)
}
