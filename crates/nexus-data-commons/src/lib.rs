//! Nexus Data Commons - Common data access abstractions
//! Nexus Data Commons - 通用数据访问抽象
//!
//! # Equivalent to Spring / 等价于 Spring
//!
//! | Nexus | Spring Data |
//! |-------|-------------|
//! | `Repository` | `Repository` |
//! | `CrudRepository` | `CrudRepository` |
//! | `PagingAndSortingRepository` | `PagingAndSortingRepository` |
//! | `Page<T>` | `Page<T>` |
//! | `PageRequest` | `PageRequest` |
//! | `Sort` | `Sort` |
//!
//! # Features / 功能
//!
//! - Repository trait hierarchy / Repository trait 层次结构
//! - CRUD operations / CRUD 操作
//! - Pagination support / 分页支持
//! - Sorting support / 排序支持
//! - Type-safe queries / 类型安全查询
//! - Async/await support / 异步/等待支持
//!
//! # Quick Start / 快速开始
//!
//! ```rust,ignore
//! use nexus_data_commons::{CrudRepository, PageRequest};
//! use async_trait::async_trait;
//!
//! #[async_trait]
//! impl CrudRepository<User, u64> for UserRepository {
//!     async fn save(&self, entity: User) -> Result<User, Error> {
//!         // Save implementation
//!         Ok(entity)
//!     }
//!
//!     async fn find_by_id(&self, id: u64) -> Result<Option<User>, Error> {
//!         // Find by ID implementation
//!         Ok(None)
//!     }
//!
//!     // ... other methods
//! }
//! ```
//!
//! # Modules / 模块
//!
//! - [`repository`] - Repository trait definitions / Repository trait 定义
//! - [`page`] - Pagination types / 分页类型
//! - [`sort`] - Sorting types / 排序类型
//! - [`error`] - Error types / 错误类型
//! - [`entity`] - Entity traits / 实体 trait

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod error;
pub mod entity;
pub mod repository;
pub mod page;
pub mod sort;
#[cfg(feature = "query")]
pub mod query;

pub use error::{Error, Result};
pub use repository::{
    Repository, CrudRepository, PagingAndSortingRepository,
};
pub use page::{Page, PageRequest, Slice, List};
pub use sort::{Sort, Order, Direction, NullHandling};

/// Version of the data-commons module
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Re-exports of commonly used types
/// 常用类型的重新导出
pub mod prelude {
    pub use super::{
        Error, Result,
        Repository, CrudRepository, PagingAndSortingRepository,
        Page, PageRequest, Sort, Order, Direction,
    };
}
