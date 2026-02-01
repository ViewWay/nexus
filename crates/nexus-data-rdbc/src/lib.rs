//! Nexus Data R2DBC - Reactive database access
//! Nexus Data R2DBC - 响应式数据库访问
//!
//! # Equivalent to Spring / 等价于 Spring
//!
//! | Nexus | Spring |
//! |-------|--------|
//! | `R2dbcRepository` | `R2dbcRepository` |
//! | `DatabaseClient` | `DatabaseClient` / `R2dbcEntityTemplate` |
//! | `Connection` | `Connection` |
//! | `Transaction` | `TransactionalDatabaseClient` |
//! | `Row` | `Row` |
//! | `Rows` | `Result` |
//!
//! # Features / 功能
//!
//! - Reactive database access / 响应式数据库访问
//! - SQLx-based implementation / 基于 SQLx 的实现
//! - Connection pooling / 连接池
//! - Transaction support / 事务支持
//! - Repository pattern / Repository 模式
//! - Type-safe queries / 类型安全查询
//!
//! # Quick Start / 快速开始
//!
//! ```rust,ignore
//! use nexus_data_rdbc::{R2dbcRepository, SqlxRepository, PostgresConfig};
//! use nexus_data_commons::{CrudRepository, Error};
//!
//! #[derive(Debug, Clone)]
//! struct User {
//!     id: i64,
//!     name: String,
//!     email: String,
//! }
//!
//! struct UserRepository {
//!     inner: SqlxRepository<User, i64>,
//! }
//!
//! impl UserRepository {
//!     fn new() -> Self {
//!         Self {
//!             inner: SqlxRepository::new("users"),
//!         }
//!     }
//! }
//!
//! #[async_trait]
//! impl CrudRepository<User, i64> for UserRepository {
//!     type Error = Error;
//!
//!     async fn save(&self, entity: User) -> Result<User, Self::Error> {
//!         self.inner.save(entity).await
//!     }
//!
//!     async fn find_by_id(&self, id: i64) -> Result<Option<User>, Self::Error> {
//!         self.inner.find_by_id(id).await
//!     }
//!
//!     // ... other methods
//! }
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod error;
pub mod config;
pub mod row;
pub mod connection;
pub mod transaction;
pub mod client;
pub mod repository;
pub mod pool;

pub use error::{R2dbcError, Error, Result, R2dbcResult};
pub use config::{DatabaseConfig, PostgresConfig, MySqlConfig, SqliteConfig, SslMode};
pub use row::{Row, RowInternal, ColumnValue, ColumnType};
pub use connection::{Connection, PoolConfig};
pub use transaction::{Transaction, TransactionManager, TxManager, IsolationLevel};
pub use client::{DatabaseClient, SqlxPoolClient, ToSql};
pub use repository::{R2dbcRepository, SqlxRepository};
pub use pool::{Pool, PoolOptions};

/// Database type enum
/// 数据库类型枚值
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseType {
    /// PostgreSQL
    PostgreSQL,
    /// MySQL
    MySQL,
    /// SQLite
    SQLite,
    /// H2
    H2,
}

/// Version of the data-rdbc module
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Re-exports of commonly used types
/// 常用类型的重新导出
pub mod prelude {
    pub use super::{
        Error, Result,
        DatabaseClient, SqlxPoolClient,
        R2dbcRepository, SqlxRepository,
        Transaction, TransactionManager, TxManager,
        Row, DatabaseConfig, PostgresConfig, MySqlConfig, SqliteConfig,
    };
}

// Re-export nexus-data-commons Error for convenience
pub use nexus_data_commons::Error as DataError;
