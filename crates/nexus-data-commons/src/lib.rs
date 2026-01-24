//! Nexus Data Commons
//! Nexus 数据通用层
//!
//! # Overview / 概述
//!
//! This crate provides the core abstractions for data access in Nexus framework.
//! It is equivalent to Spring Data Commons in the Spring ecosystem.
//!
//! 本 crate 提供 Nexus 框架数据访问的核心抽象。
//! 它等价于 Spring 生态系统中的 Spring Data Commons。
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - Spring Data Commons
//! - Repository abstraction
//! - CrudRepository
//! - PagingAndSortingRepository
//!
//! # Features / 功能
//!
//! - Repository trait hierarchy / Repository trait 层次结构
//! - CRUD operations / CRUD 操作
//! - Pagination support / 分页支持
//! - Sorting support / 排序支持
//! - Query wrappers / 查询包装器
//!
//! # Example / 示例
//!
//! ```rust,no_run,ignore
//! use nexus_data_commons::{Repository, CrudRepository, PageRequest, Sort};
//! use async_trait::async_trait;
//!
//! struct User {
//!     id: i32,
//!     name: String,
//! }
//!
//! struct UserRepository;
//!
//! #[async_trait]
//! impl CrudRepository<User, i32> for UserRepository {
//!     type Error = nexus_data_commons::Error;
//!
//!     async fn save(&self, entity: User) -> Result<User, Self::Error> {
//!         // Save implementation
//!         Ok(entity)
//!     }
//!
//!     async fn find_by_id(&self, id: i32) -> Result<Option<User>, Self::Error> {
//!         // Find implementation
//!         Ok(None)
//!     }
//! }
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]

mod error;
mod repository;
mod page;
mod sort;
mod query;
mod entity;

pub use error::{Error, Result};
pub use repository::{Repository, CrudRepository, PagingAndSortingRepository};
pub use page::{Page, PageRequest, Slice, List};
pub use sort::{Sort, Order, Direction};
pub use query::{QueryWrapper, UpdateWrapper, LambdaQueryWrapper, Predicate, Specification};
pub use entity::{AggregateRoot, Auditable, LifecycleEvent};

/// Core re-exports
/// 核心重新导出
pub mod prelude {
    pub use crate::{
        Repository,
        CrudRepository,
        PagingAndSortingRepository,
        Page,
        PageRequest,
        Sort,
        Order,
        Direction,
        QueryWrapper,
        UpdateWrapper,
        Predicate,
        Specification,
        Error,
        Result,
        AggregateRoot,
        Auditable,
    };
}
