//! Nexus Data R2DBC
//! Nexus 数据 R2DBC 层
//!
//! # Overview / 概述
//!
//! This crate provides reactive database access for the Nexus framework.
//! It is equivalent to Spring Data R2DBC in the Spring ecosystem.
//!
//! 本 crate 为 Nexus 框架提供响应式数据库访问。
//! 它等价于 Spring 生态系统中的 Spring Data R2DBC。
//!
//! # Equivalent to Spring Boot / 等价于 Spring Boot
//!
//! - Spring Data R2DBC
//! - ReactiveCrudRepository
//! - DatabaseClient
//! - R2dbcEntityTemplate
//!
//! # Features / 功能
//!
//! - Reactive database operations / 响应式数据库操作
//! - Connection pooling / 连接池
//! - Transaction support / 事务支持
//! - Row mapping / 行映射
//! - Query execution / 查询执行
//!
//! # Example / 示例
//!
//! ```rust,no_run,ignore
//! use nexus_data_rdbc::{R2dbcRepository, DatabaseClient};
//! use nexus_data_commons::{Repository, PageRequest};
//!
//! struct User {
//!     id: i32,
//!     name: String,
//! }
//!
//! struct UserRepository {
//!     client: DatabaseClient,
//! }
//!
//! #[async_trait]
//! impl R2dbcRepository<User, i32> for UserRepository {
//!     fn table_name() -> &'static str {
//!         "users"
//!     }
//!
//!     fn client(&self) -> &DatabaseClient {
//!         &self.client
//!     }
//! }
//! ```

#![warn(missing_docs)]
#![warn(unreachable_pub)]

mod connection;
mod error;
mod executor;
mod mapper;
mod query_runtime;
mod row;
mod transaction;

pub use connection::{Connection, ConnectionPool, PoolConfig};
pub use error::{R2dbcError, R2dbcResult};
pub use executor::{Executor, QueryExecutor};
pub use mapper::{BaseMapper, R2dbcCrudRepository, R2dbcRepository};
pub use query_runtime::{AnnotatedQueryExecutor, ParamStyle, QueryMetadata, QueryType};
pub use row::{Row, RowMapper};
pub use transaction::{Transaction, TransactionManager};

// Re-exports from nexus-data-commons
pub use nexus_data_commons::{
    AggregateRoot, CrudRepository, Direction, Error, Identifier, Order, Page, PageRequest,
    PagingAndSortingRepository, QueryWrapper, Repository, Result, Sort, UpdateWrapper,
};

/// Core re-exports
/// 核心重新导出
pub mod prelude {
    pub use crate::{
        BaseMapper, Connection, ConnectionPool, Executor, PoolConfig, QueryExecutor,
        R2dbcCrudRepository, R2dbcError, R2dbcRepository, R2dbcResult, Row, RowMapper, Transaction,
        TransactionManager,
    };

    pub use nexus_data_commons::prelude::*;
}

/// Database type
/// 数据库类型
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

impl DatabaseType {
    /// Get the JDBC URL prefix for this database type
    /// 获取此数据库类型的 JDBC URL 前缀
    pub fn jdbc_prefix(&self) -> &str {
        match self {
            DatabaseType::PostgreSQL => "jdbc:postgresql:",
            DatabaseType::MySQL => "jdbc:mysql:",
            DatabaseType::SQLite => "jdbc:sqlite:",
            DatabaseType::H2 => "jdbc:h2:",
        }
    }

    /// Get the R2DBC URL prefix for this database type
    /// 获取此数据库类型的 R2DBC URL 前缀
    pub fn r2dbc_prefix(&self) -> &str {
        match self {
            DatabaseType::PostgreSQL => "r2dbc:postgresql:",
            DatabaseType::MySQL => "r2dbc:mysql:",
            DatabaseType::SQLite => "r2dbc:sqlite:",
            DatabaseType::H2 => "r2dbc:h2:",
        }
    }

    /// Get the default port for this database type
    /// 获取此数据库类型的默认端口
    pub fn default_port(&self) -> u16 {
        match self {
            DatabaseType::PostgreSQL => 5432,
            DatabaseType::MySQL => 3306,
            DatabaseType::SQLite => 0,
            DatabaseType::H2 => 9092,
        }
    }
}

/// Database client for executing queries
/// 用于执行查询的数据库客户端
///
/// This is the main entry point for database operations.
/// It wraps a connection pool and provides high-level query methods.
///
/// 这是数据库操作的主要入口点。
/// 它包装连接池并提供高级查询方法。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_data_rdbc::{DatabaseClient, ConnectionPool};
///
/// let pool = ConnectionPool::connect("postgresql://localhost/mydb").await?;
/// let client = DatabaseClient::new(pool);
///
/// // Execute a query
/// let rows: Vec<User> = client.query("SELECT * FROM users WHERE id = $1", &[&id]).await?;
///
/// // Execute an update
/// let affected = client.execute("UPDATE users SET name = $1 WHERE id = $2", &[&name, &id]).await?;
/// ```
#[derive(Clone)]
pub struct DatabaseClient {
    pool: ConnectionPool,
    database_type: DatabaseType,
}

impl DatabaseClient {
    /// Create a new database client
    /// 创建新的数据库客户端
    pub fn new(pool: ConnectionPool) -> Self {
        let database_type = pool.database_type();
        Self {
            pool,
            database_type,
        }
    }

    /// Get the underlying connection pool
    /// 获取底层连接池
    pub fn pool(&self) -> &ConnectionPool {
        &self.pool
    }

    /// Get the database type
    /// 获取数据库类型
    pub fn database_type(&self) -> DatabaseType {
        self.database_type
    }

    /// Begin a new transaction
    /// 开始新事务
    pub async fn begin_transaction(&self) -> R2dbcResult<Transaction> {
        self.pool.begin().await
    }

    /// Execute a query and return the first row
    /// 执行查询并返回第一行
    pub async fn query_one(&self, sql: &str) -> R2dbcResult<Option<Row>> {
        self.pool
            .fetch_one(sql)
            .await
            .map_err(|e| R2dbcError::Unknown(e.to_string()))
    }

    /// Execute a query and return all rows
    /// 执行查询并返回所有行
    pub async fn query(&self, sql: &str) -> R2dbcResult<Vec<Row>> {
        self.pool
            .fetch_all(sql)
            .await
            .map_err(|e| R2dbcError::Unknown(e.to_string()))
    }

    /// Execute a statement and return the number of affected rows
    /// 执行语句并返回受影响的行数
    pub async fn execute(&self, sql: &str) -> R2dbcResult<u64> {
        self.pool
            .execute(sql)
            .await
            .map_err(|e| R2dbcError::Unknown(e.to_string()))
    }

    /// Fetch a single row (alias for query_one)
    /// 获取单行（query_one 的别名）
    pub async fn fetch_one(&self, sql: &str) -> R2dbcResult<Option<Row>> {
        self.query_one(sql).await
    }

    /// Fetch all rows (alias for query)
    /// 获取所有行（query 的别名）
    pub async fn fetch_all(&self, sql: &str) -> R2dbcResult<Vec<Row>> {
        self.query(sql).await
    }

    /// Close the database client and connection pool
    /// 关闭数据库客户端和连接池
    pub async fn close(self) -> R2dbcResult<()> {
        self.pool.close().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_type_prefixes() {
        assert_eq!(DatabaseType::PostgreSQL.jdbc_prefix(), "jdbc:postgresql:");
        assert_eq!(DatabaseType::MySQL.r2dbc_prefix(), "r2dbc:mysql:");
        assert_eq!(DatabaseType::SQLite.r2dbc_prefix(), "r2dbc:sqlite:");
    }

    #[test]
    fn test_database_type_ports() {
        assert_eq!(DatabaseType::PostgreSQL.default_port(), 5432);
        assert_eq!(DatabaseType::MySQL.default_port(), 3306);
        assert_eq!(DatabaseType::SQLite.default_port(), 0);
    }
}
