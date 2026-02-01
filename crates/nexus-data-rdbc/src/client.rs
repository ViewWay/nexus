//! Database client
//! 数据库客户端
//!
//! # Overview / 概述
//!
//! High-level database client for executing queries.
//! 用于执行查询的高级数据库客户端。

use crate::{Error, Result};
use std::sync::Arc;

/// Database client
/// 数据库客户端
///
/// High-level client for database operations.
/// 用于数据库操作的高级客户端。
pub trait DatabaseClient: Send + Sync {
    /// Execute a query and return row count (placeholder - returns count for now)
    /// 执行查询并返回行数（占位符 - 现在返回计数）
    fn query(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
    ) -> impl std::future::Future<Output = Result<u64>> + Send;

    /// Execute a query and return the first row count (placeholder - returns count for now)
    /// 执行查询并返回第一行计数（占位符 - 现在返回计数）
    fn query_one(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
    ) -> impl std::future::Future<Output = Result<u64>> + Send;

    /// Execute a command (INSERT, UPDATE, DELETE)
    /// 执行命令 (INSERT, UPDATE, DELETE)
    fn execute(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
    ) -> impl std::future::Future<Output = Result<u64>> + Send;

    /// Begin a transaction
    /// 开始事务
    fn begin_transaction(
        &self,
    ) -> impl std::future::Future<Output = Result<crate::Transaction>> + Send;

    /// Ping the database
    /// Ping 数据库
    fn ping(&self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Close the client
    /// 关闭客户端
    fn close(&self) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// SQLx-based pool client
/// 基于 SQLx 的连接池客户端
pub struct SqlxPoolClient {
    /// Inner SQLx pool
    /// 内部 SQLx 连接池
    inner: sqlx::Pool<sqlx::Postgres>,
}

impl SqlxPoolClient {
    /// Create a new pool client
    /// 创建新的连接池客户端
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(database_url)
            .await
            .map_err(|e| Error::connection(e.to_string()))?;

        Ok(Self { inner: pool })
    }

    /// Get the underlying SQLx pool
    /// 获取底层 SQLx 连接池
    pub fn pool(&self) -> &sqlx::Pool<sqlx::Postgres> {
        &self.inner
    }
}

/// Trait for SQL parameter conversion
pub trait ToSql: Send + Sync {
    /// Convert to SQL value
    fn to_sql(&self) -> String;
}

impl ToSql for i32 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ToSql for i64 {
    fn to_sql(&self) -> String {
        self.to_string()
    }
}

impl ToSql for &str {
    fn to_sql(&self) -> String {
        format!("'{}'", self.replace("'", "''"))
    }
}

impl ToSql for String {
    fn to_sql(&self) -> String {
        format!("'{}'", self.replace("'", "''"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_sql_i32() {
        assert_eq!(42i32.to_sql(), "42");
    }

    #[test]
    fn test_to_sql_string() {
        assert_eq!("hello".to_sql(), "'hello'");
        assert_eq!("it's".to_sql(), "'it''s'");
    }
}
