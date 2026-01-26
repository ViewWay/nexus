//! # Nexus Data Annotations
//!
//! Spring Data JPA style annotations for Nexus framework
//! Nexus 框架的 Spring Data JPA 风格注解
//!
//! ## Features / 功能
//!
//! - **`#[Entity]`** - Marks a struct as a JPA entity
//! - **`#[Table]`** - Specifies database table mapping
//! - **`#[Id]`** - Marks a field as primary key
//! - **`#[GeneratedValue]`** - Specifies ID generation strategy
//! - **`#[Column]`** - Specifies column mapping
//! - **`#[Query]`** - Custom SQL query for repository methods
//! - **`#[Insert]`**, **`#[Update]`**, **`#[Delete]`** - CRUD operation annotations
//!
//! ## Example / 示例
//!
//! ```rust,no_run
//! use nexus_data_annotations::{Entity, Table, Id, GeneratedValue, Column};
//! use nexus_lombok::Data;
//! use serde::{Serialize, Deserialize};
//!
//! #[Entity]
//! #[Table(name = "users")]
//! #[Data]
//! #[derive(Debug, Clone, Serialize, Deserialize)]
//! pub struct User {
//!     #[Id]
//!     #[GeneratedValue(strategy = "AUTO")]
//!     #[Column(name = "id")]
//!     pub id: i64,
//!
//!     #[Column(name = "username", nullable = false)]
//!     pub username: String,
//!
//!     #[Column(name = "email")]
//!     pub email: String,
//!
//!     #[Column(name = "age")]
//!     pub age: i32,
//! }
//!
//! // Usage with MyBatis-Plus style repository / 与 MyBatis-Plus 风格的 repository 配合使用
//! #[nexus_mapper]
//! pub trait UserMapper: BaseMapper<User> {
//!     #[Query("SELECT * FROM users WHERE id = #{id}")]
//!     async fn find_by_id(&self, id: i64) -> Result<Option<User>, Error>;
//!
//!     #[Query("SELECT * FROM users WHERE username LIKE #{username}%")]
//!     async fn find_by_username_starts_with(&self, username: &str) -> Result<Vec<User>, Error>;
//! }
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]

use proc_macro::TokenStream;

// ========================================================================
// Entity Annotations / 实体注解
// ========================================================================

pub mod column;
pub mod entity;
pub mod id;
pub mod pre_authorize_macro;
pub mod query;
pub mod repository;
pub mod transactional;
pub mod transactional_macro;

/// Marks a struct as a JPA entity
/// 将结构体标记为 JPA 实体
///
/// # Example / 示例
///
/// ```rust
/// use nexus_data_annotations::Entity;
///
/// #[Entity]
/// pub struct User {
///     pub id: i64,
///     pub username: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn entity(attr: TokenStream, item: TokenStream) -> TokenStream {
    entity::impl_entity(attr, item)
}

/// Specifies the database table for an entity
/// 指定实体的数据库表
///
/// # Attributes / 属性
///
/// - `name` - Table name (default: struct name in lowercase)
///   表名（默认：结构体名小写）
///
/// # Example / 示例
///
/// ```rust
/// use nexus_data_annotations::Table;
///
/// #[Entity]
/// #[Table(name = "users")]
/// pub struct User {
///     pub id: i64,
/// }
/// ```
#[proc_macro_attribute]
pub fn table(attr: TokenStream, item: TokenStream) -> TokenStream {
    entity::impl_table(attr, item)
}

// ========================================================================
// Field Annotations / 字段注解
// ========================================================================

/// Marks a field as primary key
/// 将字段标记为主键
///
/// # Attributes / 属性
///
/// - `type` - Primary key type: "auto", "input", "assign_id"
///   主键类型："auto", "input", "assign_id"
///
/// # Example / 示例
///
/// ```rust
/// use nexus_data_annotations::{Id, GeneratedValue};
///
/// #[Entity]
/// pub struct User {
///     #[Id]
///     #[GeneratedValue(strategy = "AUTO")]
///     pub id: i64,
/// }
/// ```
#[proc_macro_attribute]
pub fn id(attr: TokenStream, item: TokenStream) -> TokenStream {
    id::impl_id(attr, item)
}

/// Specifies the strategy for generating ID values
/// 指定 ID 值的生成策略
///
/// # Example / 示例
///
/// ```rust
/// use nexus_data_annotations::{GeneratedValue, Id};
///
/// #[Entity]
/// pub struct User {
///     #[Id]
///     #[GeneratedValue(strategy = "AUTO")]
///     pub id: i64,
/// }
/// ```
#[proc_macro_attribute]
pub fn generated_value(attr: TokenStream, item: TokenStream) -> TokenStream {
    id::impl_generated_value(attr, item)
}

/// Specifies database column mapping
/// 指定数据库列映射
///
/// # Attributes / 属性
///
/// - `name` - Column name (default: field name)
///   列名（默认：字段名）
/// - `nullable` - Whether column can be null (default: true)
///   列是否可为 null（默认：true）
/// - `unique` - Whether column has unique constraint (default: false)
///   列是否有唯一约束（默认：false）
/// - `length` - Column length for string types
///   字符串类型的列长度
///
/// # Example / 示例
///
/// ```rust
/// use nexus_data_annotations::Column;
///
/// #[Entity]
/// pub struct User {
///     #[Column(name = "username", nullable = false, unique = true)]
///     pub username: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn column(attr: TokenStream, item: TokenStream) -> TokenStream {
    column::impl_column(attr, item)
}

// ========================================================================
// Query Annotations / 查询注解
// ========================================================================

/// Specifies a custom SQL query for a repository method
/// 为 repository 方法指定自定义 SQL 查询
///
/// # Parameters / 参数
///
/// Supports parameter binding with:
/// 支持以下参数绑定：
///
/// - `:param` - Named parameter (recommended)
///   命名参数（推荐）
/// - `#{param}` - MyBatis-Plus style
///   MyBatis-Plus 风格
/// - `$1, $2` - Positional parameter (PostgreSQL style)
///   位置参数（PostgreSQL 风格）
///
/// # Example / 示例
///
/// ```rust
/// use nexus_data_annotations::Query;
/// use nexus_data::Repository;
///
/// trait UserRepository: Repository<User, i64> {
///     #[Query("SELECT * FROM users WHERE id = :id")]
///     async fn find_by_id(&self, id: i64) -> Result<Option<User>, Error>;
///
///     #[Query("SELECT * FROM users WHERE username LIKE :pattern%")]
///     async fn find_by_username_starts_with(&self, pattern: &str) -> Result<Vec<User>, Error>;
/// }
/// ```
#[proc_macro_attribute]
pub fn query(attr: TokenStream, item: TokenStream) -> TokenStream {
    query::impl_query(attr, item)
}

/// Marks a method as an insert operation
/// 将方法标记为插入操作
///
/// # Example / 示例
///
/// ```rust
/// use nexus_data_annotations::Insert;
///
/// trait UserRepository {
///     #[Insert("INSERT INTO users (username, email) VALUES (:username, :email)")]
///     async fn insert_user(&self, username: &str, email: &str) -> Result<u64, Error>;
/// }
/// ```
#[proc_macro_attribute]
pub fn insert(attr: TokenStream, item: TokenStream) -> TokenStream {
    query::impl_insert(attr, item)
}

/// Marks a method as an update operation
/// 将方法标记为更新操作
///
/// # Example / 示例
///
/// ```rust
/// use nexus_data_annotations::Update;
///
/// trait UserRepository {
///     #[Update("UPDATE users SET email = :email WHERE id = :id")]
///     async fn update_email(&self, id: i64, email: &str) -> Result<u64, Error>;
/// }
/// ```
#[proc_macro_attribute]
pub fn update(attr: TokenStream, item: TokenStream) -> TokenStream {
    query::impl_update(attr, item)
}

/// Marks a method as a delete operation
/// 将方法标记为删除操作
///
/// # Example / 示例
///
/// ```rust
/// use nexus_data_annotations::Delete;
///
/// trait UserRepository {
///     #[Delete("DELETE FROM users WHERE id = :id")]
///     async fn delete_by_id(&self, id: i64) -> Result<u64, Error>;
/// }
/// ```
#[proc_macro_attribute]
pub fn delete(attr: TokenStream, item: TokenStream) -> TokenStream {
    query::impl_delete(attr, item)
}

// ============================================================================
// Transactional Annotation / 事务注解
// ============================================================================

/// Marks a method or function to be executed within a transaction
/// 将方法或函数标记为在事务中执行
///
/// # Attributes / 属性
///
/// - `isolation` - Transaction isolation level (Default, ReadUncommitted, ReadCommitted, RepeatableRead, Serializable)
///   事务隔离级别
/// - `timeout` - Transaction timeout in seconds
///   事务超时时间（秒）
/// - `propagation` - Transaction propagation behavior (Required, Supports, Mandatory, RequiresNew, NotSupported, Never, Nested)
///   事务传播行为
/// - `read_only` - Whether transaction is read-only
///   事务是否只读
/// - `max_retries` - Max retry attempts for serialization failures
///   序列化失败的最大重试次数
///
/// # Example / 示例
///
/// ```rust,ignore
/// use nexus_data_annotations::Transactional;
///
/// #[Transactional]
/// async fn create_user(&self, user: User) -> Result<(), Error> {
///     // Automatically executed in a transaction
///     Ok(())
/// }
///
/// #[Transactional(isolation = ReadCommitted, timeout = 60)]
/// async fn transfer_funds(&self, from: i64, to: i64, amount: i64) -> Result<(), Error> {
///     // Executed with READ COMMITTED isolation and 60s timeout
///     Ok(())
/// }
/// ```
#[proc_macro_attribute]
pub fn transactional(attr: TokenStream, item: TokenStream) -> TokenStream {
    transactional_macro::impl_transactional(attr, item)
}

// ========================================================================
// Re-exports / 重新导出
// ========================================================================

pub use column::column as Column;
pub use entity::{entity as Entity, table as Table};
pub use id::{generated_value as GeneratedValue, id as Id};
pub use query::{delete as Delete, insert as Insert, query as Query, update as Update};

// ============================================================================
// Transactional Runtime Re-exports / 事务运行时重新导出
// ============================================================================

pub use transactional::{
    IsolationLevel as TransactionIsolationLevel, Propagation as TransactionPropagation,
    TransactionError, TransactionManager, TransactionalConfig, TransactionalExecutor,
};

pub use transactional_macro::transactional as Transactional;

// ============================================================================
// Security Annotations / 安全注解
// ============================================================================

/// Method-level security annotation
/// 方法级安全注解
///
/// # Attributes / 属性
///
/// - `expression` - Security expression to evaluate
///   要评估的安全表达式
///
/// # Supported Expressions / 支持的表达式
///
/// - `has_role('ROLE_NAME')` - Check if user has role
///   检查用户是否拥有角色
/// - `has_permission('PERMISSION_NAME')` - Check if user has permission
///   检查用户是否拥有权限
/// - `is_admin()` - Check if user is admin
///   检查用户是否为管理员
/// - `#param == value` - Check parameter values
///   检查参数值
/// - `expr1 and expr2` - Logical AND
///   逻辑与
/// - `expr1 or expr2` - Logical OR
///   逻辑或
///
/// # Example / 示例
///
/// ```rust,ignore
/// use nexus_data_annotations::PreAuthorize;
///
/// impl UserService {
///     #[PreAuthorize("has_role('ADMIN')")]
///     async fn delete_user(&self, id: i64) -> Result<(), Error> {
///         // Only ADMIN role can execute
///         // 只有 ADMIN 角色可以执行
///         self.repository.delete(id).await
///     }
///
///     #[PreAuthorize("has_role('ADMIN') or #id == auth.user_id()")]
///     async fn update_profile(&self, auth: &AuthContext, id: i64, data: UpdateData) -> Result<(), Error> {
///         // Admin or owner can modify
///         // 管理员或本人可以修改
///         self.repository.update(id, data).await
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn pre_authorize(attr: TokenStream, item: TokenStream) -> TokenStream {
    pre_authorize_macro::pre_authorize(attr, item)
}

// ============================================================================
// Repository Support / Repository 支持
// ============================================================================

pub use repository::{
    CrudRepository, Page, PageRequest, PagingRepository, QueryCriteria, Sort, SortDirection,
};

pub use pre_authorize_macro::{
    DefaultPermissionChecker, PermissionChecker, PreAuthorize as PreAuthorizeAnnotation,
    SecurityExpression,
};
