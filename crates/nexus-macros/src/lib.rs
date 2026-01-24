//! Nexus Macros - Spring Boot Style Procedural Macros
//! Nexus宏 - Spring Boot风格的过程宏
//!
//! # Overview / 概述
//!
//! `nexus-macros` provides Spring Boot-style procedural macros for the Nexus framework.
//!
//! `nexus-macros` 为 Nexus 框架提供 Spring Boot 风格的过程宏。
//!
//! # Example / 示例
//!
//! ```rust,no_run,ignore
//! use nexus_macros::{main, controller, get};
//!
//! #[main]
//! struct Application;
//!
//! #[controller]
//! struct DemoController;
//!
//! #[get("/helloworld")]
//! fn hello() -> &'static str {
//!     "Hello World!"
//! }
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    parse_macro_input, Attribute, DeriveInput,
    Expr, FnArg, ItemFn, ItemStatic, ItemStruct, ItemTrait, Meta, PatType,
    PathSegment, ReturnType, Signature, Type, TypePath, Visibility,
};

// ============================================================================
// Spring Boot Style Main Macro (equivalent to @SpringBootApplication)
// Spring Boot 风格主宏（等价于 @SpringBootApplication）
// ============================================================================

/// Marks the main application entry point
/// 标记主应用程序入口点
///
/// Equivalent to Spring Boot's `@SpringBootApplication`.
/// 等价于 Spring Boot 的 `@SpringBootApplication`。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::main;
///
/// #[main]
/// struct Application;
/// ```
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    let expanded = quote! {
        #input

        impl #name {
            /// Run the application
            /// 运行应用程序
            pub fn run() -> std::io::Result<()> {
                use nexus_runtime::Runtime;

                let runtime = Runtime::new()?;
                runtime.block_on(async {
                    // Start the server
                    // 启动服务器
                    nexus_http::Server::new().run().await
                })
            }
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Controller Macro (equivalent to @RestController)
// 控制器宏（等价于 @RestController）
// ============================================================================

/// Marks a struct as a REST controller
/// 将结构体标记为 REST 控制器
///
/// Equivalent to Spring's `@RestController` or `@Controller`.
/// 等价于 Spring 的 `@RestController` 或 `@Controller`。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::{controller, get};
///
/// #[controller]
/// struct DemoController;
///
/// #[get("/hello")]
/// fn hello() -> &'static str {
///     "Hello!"
/// }
/// ```
#[proc_macro_attribute]
pub fn controller(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let expanded = quote! {
        #input

        impl #input {
            /// Register this controller's routes
            /// 注册此控制器的路由
            pub fn register() -> nexus_router::Router {
                nexus_router::Router::new()
                    .prefix(concat!("/", stringify!(#input)))
            }
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Service Macro (equivalent to @Service)
// 服务宏（等价于 @Service）
// ============================================================================

/// Marks a struct as a service
/// 将结构体标记为服务
///
/// Equivalent to Spring's `@Service`.
/// 等价于 Spring 的 `@Service`。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::service;
/// use std::sync::Arc;
///
/// #[service]
/// struct UserService {
///     repository: Arc<UserRepository>,
/// }
///
/// impl UserService {
///     fn new(repository: Arc<UserRepository>) -> Self {
///         Self { repository }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn service(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    // Generate a new() method if not exists
    let name = &input.ident;
    let fields = &input.fields;

    let field_names: Vec<_> = fields.iter().filter_map(|f| f.ident.as_ref()).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let expanded = quote! {
        #input

        impl #name {
            /// Create a new instance (constructor injection)
            /// 创建新实例（构造函数注入）
            pub fn new(#(#field_names: #field_types),*) -> Self {
                Self {
                    #(#field_names),*
                }
            }
        }

        // Implement Into<Arc<Self>> for easy DI container integration
        // 实现 Into<Arc<Self>> 以便与 DI 容器集成
        impl Into<std::sync::Arc<Self>> for #name {
            fn into(self) -> std::sync::Arc<Self> {
                std::sync::Arc::new(self)
            }
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Repository Macro (equivalent to Spring Data JPA Repository)
// 仓储宏（等价于 Spring Data JPA Repository）
// ============================================================================

/// Marks a trait as a repository
/// 将 trait 标记为仓储
///
/// Equivalent to Spring Data JPA's `@Repository` or `JpaRepository`.
/// 等价于 Spring Data JPA 的 `@Repository` 或 `JpaRepository`。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::repository;
///
/// #[repository]
/// trait UserRepository: Send + Sync {
///     async fn find_by_id(&self, id: u64) -> Option<User>;
///     async fn save(&self, user: User) -> Result<User, Error>;
/// }
/// ```
#[proc_macro_attribute]
pub fn repository(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);

    let expanded = quote! {
        #input
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Configuration Macro (equivalent to @ConfigurationProperties)
// 配置宏（等价于 @ConfigurationProperties）
// ============================================================================

/// Marks a struct as configuration properties
/// 将结构体标记为配置属性
///
/// Equivalent to Spring Boot's `@ConfigurationProperties`.
/// 等价于 Spring Boot 的 `@ConfigurationProperties`。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::config;
///
/// #[config(prefix = "app")]
/// struct AppConfig {
///     name: String,
///     port: u16,
/// }
/// ```
#[proc_macro_attribute]
pub fn config(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    // Parse the prefix attribute
    let prefix = if attr.is_empty() {
        Ident::new("config", Span::call_site())
    } else {
        parse_macro_input!(attr as ConfigArgs).prefix
    };

    let name = &input.ident;

    let expanded = quote! {
        #input

        impl #name {
            /// Load configuration from environment/config files
            /// 从环境/配置文件加载配置
            pub fn load() -> Result<Self, config::ConfigError> {
                let mut cfg = config::Config::builder();

                // Load from config files (application.toml, application.yml)
                // 从配置文件加载
                cfg = cfg.add_source(config::File::with_name("application"));

                // Load from environment with prefix
                // 从环境变量加载（带前缀）
                cfg = cfg.add_source(
                    config::Environment::with_prefix(stringify!(#prefix))
                        .separator("__")
                        .try_parsing(true)
                );

                // Build and deserialize
                // 构建并反序列化
                let config = cfg.build()?;
                config.try_deserialize()
            }
        }
    };

    TokenStream::from(expanded)
}

struct ConfigArgs {
    prefix: Ident,
}

impl Parse for ConfigArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _eq_token = syn::token::Eq::parse(input)?;

        let lookahead = input.lookahead1();
        if lookahead.peek(syn::LitStr) {
            let lit_str: syn::LitStr = input.parse()?;
            Ok(Self {
                prefix: Ident::new(&lit_str.value(), lit_str.span()),
            })
        } else {
            let ident: Ident = input.parse()?;
            Ok(Self { prefix: ident })
        }
    }
}

// ============================================================================
// Route Macros (equivalent to @GetMapping, @PostMapping, etc.)
// 路由宏（等价于 @GetMapping, @PostMapping 等）
// ============================================================================

/// Helper function to parse route attributes
/// 解析路由属性的辅助函数
fn parse_route_path(attr: TokenStream) -> syn::Result<String> {
    let attr_str = attr.to_string();

    // Handle both #[get("/path")] and #[get(path)]
    // 处理 #[get("/path")] 和 #[get(path)] 两种形式
    let path = if attr_str.contains("\"") {
        // Extract string between quotes
        // 提取引号之间的字符串
        let start = attr_str.find('"').unwrap_or(0) + 1;
        let end = attr_str.rfind('"').unwrap_or(attr_str.len());
        attr_str[start..end].to_string()
    } else {
        // Use the raw value
        // 使用原始值
        attr_str.trim().to_string()
    };

    Ok(path)
}

macro_rules! impl_route_macro {
    ($name:ident, $method:ident) => {
        /// Route attribute macro
        /// 路由属性宏
        #[proc_macro_attribute]
        pub fn $name(attr: TokenStream, item: TokenStream) -> TokenStream {
            let input = parse_macro_input!(item as ItemFn);
            let func_name = &input.sig.ident;

            // Parse the route path
            // 解析路由路径
            let path = match parse_route_path(attr) {
                Ok(p) => p,
                Err(e) => return TokenStream::from(e.to_compile_error()),
            };

            // Build the route registration
            // 构建路由注册
            let expanded = quote! {
                #input

                // Register this route with the router
                // 在路由器中注册此路由
                #[automatically_derived]
                impl #func_name {
                    pub fn register_route(router: nexus_router::Router) -> nexus_router::Router {
                        router.route(#path, nexus_http::Method::$method, #func_name)
                    }
                }
            };

            TokenStream::from(expanded)
        }
    };
}

// Implement route macros for each HTTP method
// 为每个 HTTP 方法实现路由宏
impl_route_macro!(get, GET);
impl_route_macro!(post, POST);
impl_route_macro!(put, PUT);
impl_route_macro!(delete, DELETE);
impl_route_macro!(patch, PATCH);
impl_route_macro!(head, HEAD);
impl_route_macro!(options, OPTIONS);
impl_route_macro!(trace, TRACE);

// ============================================================================
// Component Macro (equivalent to @Component)
// 组件宏（等价于 @Component）
// ============================================================================

/// Marks a struct as a Spring component
/// 将结构体标记为 Spring 组件
///
/// Equivalent to Spring's `@Component`.
/// 等价于 Spring 的 `@Component`。
///
/// Components are automatically detected and registered in the application context.
/// 组件会被自动检测并注册到应用程序上下文中。
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let expanded = quote! {
        #input

        impl #input {
            /// Initialize this component
            /// 初始化此组件
            pub fn init() -> Self {
                Self::default()
            }
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Autowired Macro (equivalent to @Autowired)
// 自动装配宏（等价于 @Autowired）
// ============================================================================

/// Marks a field or constructor for autowiring
/// 标记字段或构造函数用于自动装配
///
/// Equivalent to Spring's `@Autowired`.
/// 等价于 Spring 的 `@Autowired`。
#[proc_macro_attribute]
pub fn autowired(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // For now, just pass through
    // 目前，直接传递
    item
}

// ============================================================================
// Scheduled Macro (equivalent to @Scheduled)
// 定时任务宏（等价于 @Scheduled）
// ============================================================================

/// Marks a function to be scheduled
/// 标记函数为定时执行
///
/// Equivalent to Spring's `@Scheduled`.
/// 等价于 Spring 的 `@Scheduled`。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::scheduled;
///
/// #[scheduled(cron = "0 * * * * *")] // Every hour
/// async fn cleanup_task() {
///     // Cleanup logic
/// }
///
/// #[scheduled(fixed_rate = 5000)] // Every 5 seconds
/// async fn periodic_task() {
///     // Periodic logic
/// }
/// ```
#[proc_macro_attribute]
pub fn scheduled(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;

    // Parse scheduling parameters
    // 解析调度参数
    let schedule_expr = parse_schedule_args(attr);

    let expanded = quote! {
        #input

        // Register this scheduled task
        // 注册此定时任务
        #[automatically_derived]
        impl #func_name {
            pub fn schedule(self) {
                use nexus_runtime::time::Duration;
                use nexus_runtime::spawn;

                spawn(async move {
                    #schedule_expr
                });
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_schedule_args(attr: TokenStream) -> proc_macro2::TokenStream {
    let attr_str = attr.to_string();

    if attr_str.contains("cron") {
        quote! {
            // Use cron expression
            // 使用 cron 表达式
            todo!("Cron scheduling not yet implemented");
        }
    } else if attr_str.contains("fixed_rate") {
        quote! {
            // Fixed rate scheduling
            // 固定频率调度
            loop {
                self().await;
                tokio::time::sleep(Duration::from_millis(5000)).await;
            }
        }
    } else if attr_str.contains("fixed_delay") {
        quote! {
            // Fixed delay scheduling
            // 固定延迟调度
            loop {
                self().await;
                tokio::time::sleep(Duration::from_millis(5000)).await;
            }
        }
    } else {
        quote! {
            // Default: run once on startup
            // 默认：启动时运行一次
            self().await;
        }
    }
}

// ============================================================================
// Async Macro (equivalent to @Async)
// 异步宏（等价于 @Async）
// ============================================================================

/// Marks a method to run asynchronously
/// 标记方法为异步执行
///
/// Equivalent to Spring's `@Async`.
/// 等价于 Spring 的 `@Async`。
#[proc_macro_attribute]
pub fn async_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    // Ensure the function is async
    // 确保函数是异步的
    let expanded = quote! {
        #input
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Derive Macros
// 派生宏
// ============================================================================

/// Derive macro for FromRequest trait
/// FromRequest trait 的派生宏
#[proc_macro_derive(FromRequest)]
pub fn from_request_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl nexus_http::FromRequest for #name {
            type Error = nexus_http::Error;

            async fn from_request(
                req: &nexus_http::Request,
            ) -> Result<Self, Self::Error> {
                // Implementation provided by the framework
                // 由框架提供实现
                todo!("FromRequest derive for {}", stringify!(#name));
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derive macro for IntoResponse trait
/// IntoResponse trait 的派生宏
#[proc_macro_derive(IntoResponse)]
pub fn into_response_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {
        impl nexus_http::IntoResponse for #name {
            fn into_response(self) -> nexus_http::Response {
                // Convert to JSON response
                // 转换为 JSON 响应
                use nexus_http::Json;
                Json(self).into_response()
            }
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Slf4j Macro (equivalent to Lombok @Slf4j)
// Slf4j 宏（等价于 Lombok @Slf4j）
// ============================================================================

/// Automatically adds a logger field to the struct
/// 自动为结构体添加日志字段
///
/// Equivalent to Lombok's `@Slf4j` annotation in Java.
/// 等价于 Java 中 Lombok 的 `@Slf4j` 注解。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::slf4j;
///
/// #[slf4j]
/// struct MyController {
///     // The macro automatically adds: log: nexus_observability::log::LoggerHandle
/// }
///
/// impl MyController {
///     fn do_something(&self) {
///         self.log.info(format_args!("Doing something..."));
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn slf4j(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;

    // Check if struct already has a log field
    let has_log_field = input.fields.iter().any(|f| {
        f.ident.as_ref().map(|i| i.to_string() == "log").unwrap_or(false)
    });

    if has_log_field {
        // Already has log field, just return the original
        // 已有 log 字段，直接返回原样
        return TokenStream::from(quote! { #input });
    }

    let expanded = quote! {
        #input

        impl #name {
            /// Get the logger for this struct
            /// 获取此结构体的日志记录器
            fn log(&self) -> nexus_observability::log::LoggerHandle {
                nexus_observability::log::LoggerFactory::get_for::<#name>()
            }
        }
    };

    TokenStream::from(expanded)
}

/// Creates a static logger in the current scope
/// 在当前作用域中创建静态日志记录器
///
/// This is a simpler alternative to `#[slf4j]` that creates a `log` binding.
/// 这是 `#[slf4j]` 的更简单替代方案，创建一个 `log` 绑定。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::logger;
///
/// #[logger]
/// fn my_function() {
///     log.info("Hello from logger");
/// }
/// ```
#[proc_macro_attribute]
pub fn logger(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;

    let expanded = quote! {
        #input

        // Create logger binding
        // 创建日志记录器绑定
        let log = nexus_observability::log::LoggerFactory::get(stringify!(#func_name));
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Transactional Macro (equivalent to @Transactional)
// 事务宏（等价于 @Transactional）
// ============================================================================

/// Marks a function or method to be executed within a transaction
/// 标记函数或方法在事务中执行
///
/// Equivalent to Spring's `@Transactional`.
/// 等价于 Spring 的 `@Transactional`。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::transactional;
///
/// #[transactional]
/// async fn transfer_money(from: Account, to: Account, amount: f64) -> Result<(), Error> {
///     // Database operations here will be executed in a transaction
///     // 这里的数据库操作将在事务中执行
/// }
/// ```
#[proc_macro_attribute]
pub fn transactional(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let expanded = quote! {
        #[nexus_macros::async_trait::async_trait]
        #input

        // TODO: Implement transaction wrapper
        // The actual transaction management will be implemented in Phase 4
        // TODO: 实现事务包装器
        // 实际事务管理将在第4阶段实现
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Cacheable Macros (equivalent to @Cacheable, @CacheEvict, @CachePut)
// 缓存宏（等价于 @Cacheable, @CacheEvict, @CachePut）
// ============================================================================

/// Cache the result of a method
/// 缓存方法结果
///
/// Equivalent to Spring's `@Cacheable`.
/// 等价于 Spring 的 `@Cacheable`。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::cacheable;
///
/// #[cacheable("users")]
/// async fn get_user(id: u64) -> Option<User> {
///     // Result will be cached
///     // 结果将被缓存
/// }
/// ```
#[proc_macro_attribute]
pub fn cacheable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let cache_name = if attr.is_empty() {
        quote! { "default" }
    } else {
        let cache_name = parse_macro_input!(attr as syn::LitStr);
        quote! { #cache_name }
    };

    let input = parse_macro_input!(item as ItemFn);

    let expanded = quote! {
        #input

        // TODO: Implement caching wrapper
        // TODO: 实现缓存包装器
    };

    TokenStream::from(expanded)
}

/// Evict cache entries
/// 清除缓存条目
///
/// Equivalent to Spring's `@CacheEvict`.
/// 等价于 Spring 的 `@CacheEvict`。
#[proc_macro_attribute]
pub fn cache_evict(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _cache_name = attr;
    let input = parse_macro_input!(item as ItemFn);

    let expanded = quote! {
        #input
    };

    TokenStream::from(expanded)
}

/// Update cache entry
/// 更新缓存条目
///
/// Equivalent to Spring's `@CachePut`.
/// 等价于 Spring 的 `@CachePut`。
#[proc_macro_attribute]
pub fn cache_put(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _cache_name = attr;
    let input = parse_macro_input!(item as ItemFn);

    let expanded = quote! {
        #input
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Conditional Macros (equivalent to @ConditionalOn* annotations)
// 条件宏（等价于 @ConditionalOn* 注解）
// ============================================================================

/// Only enable bean if class is on classpath
/// 仅当类在类路径上时才启用 bean
///
/// Equivalent to Spring Boot's `@ConditionalOnClass`.
/// 等价于 Spring Boot 的 `@ConditionalOnClass`。
#[proc_macro_attribute]
pub fn conditional_on_class(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Only enable bean if property is set
/// 仅当属性设置时才启用 bean
///
/// Equivalent to Spring Boot's `@ConditionalOnProperty`.
/// 等价于 Spring Boot 的 `@ConditionalOnProperty`。
#[proc_macro_attribute]
pub fn conditional_on_property(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// Only enable bean if another bean is missing
/// 仅当另一个 bean 不存在时才启用 bean
///
/// Equivalent to Spring Boot's `@ConditionalOnMissingBean`.
/// 等价于 Spring Boot 的 `@ConditionalOnMissingBean`。
#[proc_macro_attribute]
pub fn conditional_on_missing_bean(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

// ============================================================================
// Value Macro (equivalent to @Value)
// 值注入宏（等价于 @Value）
// ============================================================================

/// Inject a value from environment or config
/// 从环境或配置注入值
///
/// Equivalent to Spring's `@Value("${property.name}")`.
/// 等价于 Spring 的 `@Value("${property.name}")`。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::value;
///
/// #[value("${app.name}")]
/// static APP_NAME: &str = "Nexus Application";
///
/// #[value("${server.port:8080}")]
/// static SERVER_PORT: u16 = 8080;
/// ```
#[proc_macro_attribute]
pub fn value(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStatic);

    // Parse the property expression
    let attr_str = attr.to_string();

    // Extract property name from "${property.name}" or "${property.name:default}"
    let property_name = if attr_str.contains("${") && attr_str.contains("}") {
        let start = attr_str.find("${").map(|i| i + 2).unwrap_or(0);
        let end = attr_str.find('}').unwrap_or(attr_str.len());

        let prop = &attr_str[start..end];
        if let Some(colon_pos) = prop.find(':') {
            // Has default value
            // 有默认值
            prop[..colon_pos].to_string()
        } else {
            prop.to_string()
        }
    } else {
        attr_str.trim().to_string()
    };

    // Extract default value from the static expression
    let default_value = if let Expr::Lit(expr_lit) = &*input.expr {
        Some(expr_lit.clone())
    } else {
        None
    };

    let name = &input.ident;
    let ty = &input.ty;

    let expanded = if let Some(default) = default_value {
        quote! {
            #input

            impl #name {
                fn load_value() -> #ty {
                    std::env::var(#property_name)
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or_else(|| #default)
                }
            }
        }
    } else {
        quote! {
            #input
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// Profile Macro (equivalent to @Profile)
// 配置宏（等价于 @Profile）
// ============================================================================

/// Only enable component when specific profile is active
/// 仅当特定配置文件激活时才启用组件
///
/// Equivalent to Spring's `@Profile("dev")`.
/// 等价于 Spring 的 `@Profile("dev")`。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::profile;
///
/// #[profile("dev")]
/// #[service]
/// struct DevService {
///     // Only available in dev profile
///     // 仅在 dev 配置文件中可用
/// }
///
/// #[profile("prod")]
/// #[service]
/// struct ProdService {
///     // Only available in production
///     // 仅在生产环境中可用
/// }
/// ```
#[proc_macro_attribute]
pub fn profile(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _profile = parse_macro_input!(attr as syn::LitStr);
    let input = parse_macro_input!(item as ItemStruct);

    let expanded = quote! {
        #input

        // TODO: Implement profile checking
        // TODO: 实现配置文件检查
        #[cfg(feature = "profile")]
        impl #input {
            fn is_active_profile() -> bool {
                std::env::var("SPRING_PROFILES_ACTIVE")
                    .map(|p| p.contains("dev"))
                    .unwrap_or(false)
            }
        }
    };

    TokenStream::from(expanded)
}

// ============================================================================
// ExceptionHandler Macro (equivalent to @ExceptionHandler)
// 异常处理宏（等价于 @ExceptionHandler）
// ============================================================================

/// Mark method as exception handler
/// 标记方法为异常处理器
///
/// Equivalent to Spring's `@ExceptionHandler`.
/// 等价于 Spring 的 `@ExceptionHandler`。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_macros::exception_handler;
///
/// #[exception_handler]
/// async fn handle_not_found(e: NotFoundError) -> Response {
///     Response::builder()
///         .status(404)
///         .body(e.to_string())
///         .unwrap()
/// }
/// ```
#[proc_macro_attribute]
pub fn exception_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let expanded = quote! {
        #input

        // Register exception handler
        // 注册异常处理器
        nexus_core::register_exception_handler(#input);
    };

    TokenStream::from(expanded)
}

