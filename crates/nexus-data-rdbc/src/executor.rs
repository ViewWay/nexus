//! Query execution
//! 查询执行
//!
//! # Overview / 概述
//!
//! This module provides query execution capabilities.
//! 本模块提供查询执行功能。

use crate::{DatabaseClient, R2dbcResult, Row};
use nexus_data_commons::{Page, PageRequest, QueryWrapper, UpdateWrapper};
use std::future::Future;
use std::pin::Pin;

/// Query executor trait
/// 查询执行器 trait
///
/// Defines methods for executing queries.
/// 定义执行查询的方法。
pub trait Executor: Send + Sync {
    /// Execute a query and return the first row
    /// 执行查询并返回第一行
    fn fetch_one(
        &self,
        sql: &str,
        params: Vec<serde_json::Value>,
    ) -> Pin<Box<dyn Future<Output = R2dbcResult<Option<Row>>> + Send + '_>>;

    /// Execute a query and return all rows
    /// 执行查询并返回所有行
    fn fetch_all(
        &self,
        sql: &str,
        params: Vec<serde_json::Value>,
    ) -> Pin<Box<dyn Future<Output = R2dbcResult<Vec<Row>>> + Send + '_>>;

    /// Execute a statement and return affected rows
    /// 执行语句并返回受影响的行数
    fn execute(
        &self,
        sql: &str,
        params: Vec<serde_json::Value>,
    ) -> Pin<Box<dyn Future<Output = R2dbcResult<u64>> + Send + '_>>;
}

/// Query executor - specialized for database queries
/// 查询执行器 - 专门用于数据库查询
///
/// Provides high-level methods for executing queries with wrappers.
/// 提供使用包装器执行查询的高级方法。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_data_rdbc::QueryExecutor;
/// use nexus_data_commons::QueryWrapper;
///
/// let executor = QueryExecutor::new(client);
///
/// // Select with query wrapper
/// let users: Vec<User> = executor.select(
///     QueryWrapper::new().eq("status", "active"),
///     "users"
/// ).await?;
///
/// // Paginated select
/// let page = executor.select_page(
///     QueryWrapper::new().eq("role", "user"),
///     PageRequest::of(0, 20),
///     "users"
/// ).await?;
/// ```
pub struct QueryExecutor {
    client: DatabaseClient,
}

impl QueryExecutor {
    /// Create a new query executor
    /// 创建新的查询执行器
    pub fn new(client: DatabaseClient) -> Self {
        Self { client }
    }

    /// Get the underlying database client
    /// 获取底层数据库客户端
    pub fn client(&self) -> &DatabaseClient {
        &self.client
    }

    /// Execute a SELECT query with a wrapper
    /// 使用包装器执行 SELECT 查询
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// let users: Vec<User> = executor.select(
    ///     QueryWrapper::new()
    ///         .eq("status", "active")
    ///         .ge("age", 18)
    ///         .order_by_asc("name"),
    ///     "users"
    /// ).await?;
    /// ```
    pub async fn select<T>(
        &self,
        wrapper: &QueryWrapper,
        table: &str,
    ) -> R2dbcResult<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        let (sql, _params) = self.build_select_query(wrapper, table);
        let rows = self.client.fetch_all(&sql).await?;

        self.map_rows(rows)
    }

    /// Execute a paginated SELECT query
    /// 执行分页 SELECT 查询
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// let page = executor.select_page::<User>(
    ///     QueryWrapper::new().eq("role", "user"),
    ///     PageRequest::of(0, 20),
    ///     "users"
    /// ).await?;
    /// ```
    pub async fn select_page<T>(
        &self,
        wrapper: &QueryWrapper,
        page_request: PageRequest,
        table: &str,
    ) -> R2dbcResult<Page<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        // Build count query
        let (count_sql, _count_params) = self.build_count_query(wrapper, table);
        let total_elements = self.execute_count(&count_sql).await?;

        // Build data query with pagination
        let (sql, _params) = self.build_select_page_query(wrapper, &page_request, table);
        let rows = self.client.fetch_all(&sql).await?;
        let content = self.map_rows(rows)?;

        // Build page metadata
        let total_pages = if page_request.size > 0 {
            ((total_elements as f64) / (page_request.size as f64)).ceil() as u32
        } else {
            0
        };

        let has_next = (page_request.page + 1) < total_pages;
        let has_previous = page_request.page > 0;

        Ok(Page {
            content,
            number: page_request.page,
            size: page_request.size,
            total_elements,
            total_pages,
            has_next,
            has_previous,
        })
    }

    /// Execute an UPDATE query with a wrapper
    /// 使用包装器执行 UPDATE 查询
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// let affected = executor.update(
    ///     UpdateWrapper::new()
    ///         .set("status", "inactive")
    ///         .eq("last_login", "< 30 days ago"),
    ///     "users"
    /// ).await?;
    /// ```
    pub async fn update(&self, wrapper: &UpdateWrapper, table: &str) -> R2dbcResult<u64> {
        let (sql, _params) = self.build_update_query(wrapper, table);
        self.client.execute(&sql).await
    }

    /// Execute a DELETE query with a wrapper
    /// 使用包装器执行 DELETE 查询
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// let affected = executor.delete(
    ///     QueryWrapper::new().eq("status", "deleted"),
    ///     "users"
    /// ).await?;
    /// ```
    pub async fn delete(&self, wrapper: &QueryWrapper, table: &str) -> R2dbcResult<u64> {
        let (sql, _params) = self.build_delete_query(wrapper, table);
        self.client.execute(&sql).await
    }

    /// Count records matching a query
    /// 统计匹配查询的记录数
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// let count = executor.count(
    ///     QueryWrapper::new().eq("status", "active"),
    ///     "users"
    /// ).await?;
    /// ```
    pub async fn count(&self, wrapper: &QueryWrapper, table: &str) -> R2dbcResult<u64> {
        let (sql, _params) = self.build_count_query(wrapper, table);
        self.execute_count(&sql).await
    }

    /// Check if any records match the query
    /// 检查是否有任何记录匹配查询
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// let exists = executor.exists(
    ///     QueryWrapper::new().eq("email", "user@example.com"),
    ///     "users"
    /// ).await?;
    /// ```
    pub async fn exists(&self, wrapper: &QueryWrapper, table: &str) -> R2dbcResult<bool> {
        Ok(self.count(wrapper, table).await? > 0)
    }

    // Query building methods

    fn build_select_query(&self, wrapper: &QueryWrapper, table: &str) -> (String, Vec<serde_json::Value>) {
        let mut sql = String::new();
        let mut params = Vec::new();

        // SELECT clause
        if let Some(columns) = &wrapper.select {
            sql.push_str("SELECT ");
            sql.push_str(&columns.join(", "));
        } else {
            sql.push_str("SELECT *");
        }

        sql.push_str(" FROM ");
        sql.push_str(table);

        // WHERE clause
        let (where_clause, mut where_params) = self.build_where_clause(&wrapper.conditions);
        if !where_clause.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&where_clause);
        }
        params.append(&mut where_params);

        // ORDER BY clause
        if !wrapper.orders.is_empty() {
            sql.push_str(" ORDER BY ");
            let order_clauses: Vec<String> = wrapper
                .orders
                .iter()
                .map(|o| match o {
                    nexus_data_commons::QueryOrder::Asc(field) => format!("{} ASC", field),
                    nexus_data_commons::QueryOrder::Desc(field) => format!("{} DESC", field),
                })
                .collect();
            sql.push_str(&order_clauses.join(", "));
        }

        // LIMIT clause
        if let Some(limit) = wrapper.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        // OFFSET clause
        if let Some(offset) = wrapper.offset {
            sql.push_str(&format!(" OFFSET {}", offset));
        }

        (sql, params)
    }

    fn build_select_page_query(
        &self,
        wrapper: &QueryWrapper,
        page_request: &PageRequest,
        table: &str,
    ) -> (String, Vec<serde_json::Value>) {
        let mut sql = String::new();
        let mut params = Vec::new();

        // SELECT clause
        if let Some(columns) = &wrapper.select {
            sql.push_str("SELECT ");
            sql.push_str(&columns.join(", "));
        } else {
            sql.push_str("SELECT *");
        }

        sql.push_str(" FROM ");
        sql.push_str(table);

        // WHERE clause
        let (where_clause, mut where_params) = self.build_where_clause(&wrapper.conditions);
        if !where_clause.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&where_clause);
        }
        params.append(&mut where_params);

        // ORDER BY clause (from page request sort)
        if let Some(sort) = &page_request.sort {
            if !sort.is_empty() {
                sql.push_str(" ORDER BY ");
                let order_clauses: Vec<String> = sort
                    .iter()
                    .map(|o| format!("{} {}", o.property, o.direction))
                    .collect();
                sql.push_str(&order_clauses.join(", "));
            }
        } else if !wrapper.orders.is_empty() {
            // Use wrapper orders if page_request has no sort
            sql.push_str(" ORDER BY ");
            let order_clauses: Vec<String> = wrapper
                .orders
                .iter()
                .map(|o| match o {
                    nexus_data_commons::QueryOrder::Asc(field) => format!("{} ASC", field),
                    nexus_data_commons::QueryOrder::Desc(field) => format!("{} DESC", field),
                })
                .collect();
            sql.push_str(&order_clauses.join(", "));
        }

        // LIMIT and OFFSET
        sql.push_str(&format!(" LIMIT {}", page_request.size));
        sql.push_str(&format!(" OFFSET {}", page_request.get_offset()));

        (sql, params)
    }

    fn build_count_query(&self, wrapper: &QueryWrapper, table: &str) -> (String, Vec<serde_json::Value>) {
        let mut sql = format!("SELECT COUNT(*) FROM {}", table);
        let mut params = Vec::new();

        let (where_clause, where_params) = self.build_where_clause(&wrapper.conditions);
        if !where_clause.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&where_clause);
            params = where_params;
        }

        (sql, params)
    }

    fn build_update_query(&self, wrapper: &UpdateWrapper, table: &str) -> (String, Vec<serde_json::Value>) {
        let mut sql = format!("UPDATE {} SET ", table);
        let mut params = Vec::new();
        let mut set_clauses = Vec::new();

        for (column, value) in &wrapper.sets {
            let param_name = format!("set_{}", column);
            set_clauses.push(format!("{} = @{}", column, param_name));
            params.push(serde_json::json!({
                "name": param_name,
                "value": value
            }));
        }

        sql.push_str(&set_clauses.join(", "));

        let (where_clause, where_params) = self.build_where_clause(&wrapper.conditions);
        if !where_clause.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&where_clause);
            params.extend(where_params);
        }

        (sql, params)
    }

    fn build_delete_query(&self, wrapper: &QueryWrapper, table: &str) -> (String, Vec<serde_json::Value>) {
        let mut sql = format!("DELETE FROM {}", table);
        let mut params = Vec::new();

        let (where_clause, where_params) = self.build_where_clause(&wrapper.conditions);
        if !where_clause.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&where_clause);
            params = where_params;
        }

        (sql, params)
    }

    fn build_where_clause(
        &self,
        conditions: &[nexus_data_commons::Condition],
    ) -> (String, Vec<serde_json::Value>) {
        if conditions.is_empty() {
            return (String::new(), Vec::new());
        }

        let mut sql = String::new();
        let mut params = Vec::new();
        let mut param_index = 1;

        for (i, condition) in conditions.iter().enumerate() {
            if i > 0 {
                sql.push_str(" AND ");
            }

            match condition {
                nexus_data_commons::Condition::Eq { field, value } => {
                    sql.push_str(&format!("{} = @{}", field, param_index));
                    params.push(serde_json::json!({
                        "name": param_index.to_string(),
                        "value": value
                    }));
                    param_index += 1;
                }
                nexus_data_commons::Condition::Ne { field, value } => {
                    sql.push_str(&format!("{} != @{}", field, param_index));
                    params.push(serde_json::json!({
                        "name": param_index.to_string(),
                        "value": value
                    }));
                    param_index += 1;
                }
                nexus_data_commons::Condition::Gt { field, value } => {
                    sql.push_str(&format!("{} > @{}", field, param_index));
                    params.push(serde_json::json!({
                        "name": param_index.to_string(),
                        "value": value
                    }));
                    param_index += 1;
                }
                nexus_data_commons::Condition::Ge { field, value } => {
                    sql.push_str(&format!("{} >= @{}", field, param_index));
                    params.push(serde_json::json!({
                        "name": param_index.to_string(),
                        "value": value
                    }));
                    param_index += 1;
                }
                nexus_data_commons::Condition::Lt { field, value } => {
                    sql.push_str(&format!("{} < @{}", field, param_index));
                    params.push(serde_json::json!({
                        "name": param_index.to_string(),
                        "value": value
                    }));
                    param_index += 1;
                }
                nexus_data_commons::Condition::Le { field, value } => {
                    sql.push_str(&format!("{} <= @{}", field, param_index));
                    params.push(serde_json::json!({
                        "name": param_index.to_string(),
                        "value": value
                    }));
                    param_index += 1;
                }
                nexus_data_commons::Condition::Like { field, pattern } => {
                    sql.push_str(&format!("{} LIKE @{}", field, param_index));
                    params.push(serde_json::json!({
                        "name": param_index.to_string(),
                        "value": pattern
                    }));
                    param_index += 1;
                }
                nexus_data_commons::Condition::In { field, values } => {
                    let placeholders: Vec<String> = (0..values.len())
                        .map(|j| format!("@{}", param_index + j))
                        .collect();
                    sql.push_str(&format!("{} IN ({})", field, placeholders.join(", ")));
                    for value in values {
                        params.push(serde_json::json!({
                            "name": param_index.to_string(),
                            "value": value
                        }));
                        param_index += 1;
                    }
                }
                nexus_data_commons::Condition::NotIn { field, values } => {
                    let placeholders: Vec<String> = (0..values.len())
                        .map(|j| format!("@{}", param_index + j))
                        .collect();
                    sql.push_str(&format!("{} NOT IN ({})", field, placeholders.join(", ")));
                    for value in values {
                        params.push(serde_json::json!({
                            "name": param_index.to_string(),
                            "value": value
                        }));
                        param_index += 1;
                    }
                }
                nexus_data_commons::Condition::Between { field, low, high } => {
                    sql.push_str(&format!("{} BETWEEN @{} AND @{}", field, param_index, param_index + 1));
                    params.push(serde_json::json!({
                        "name": param_index.to_string(),
                        "value": low
                    }));
                    params.push(serde_json::json!({
                        "name": (param_index + 1).to_string(),
                        "value": high
                    }));
                    param_index += 2;
                }
                nexus_data_commons::Condition::IsNull { field } => {
                    sql.push_str(&format!("{} IS NULL", field));
                }
                nexus_data_commons::Condition::IsNotNull { field } => {
                    sql.push_str(&format!("{} IS NOT NULL", field));
                }
                nexus_data_commons::Condition::NotLike { field, pattern } => {
                    sql.push_str(&format!("{} NOT LIKE @{}", field, param_index));
                    params.push(serde_json::json!({
                        "name": param_index.to_string(),
                        "value": pattern
                    }));
                    param_index += 1;
                }
                nexus_data_commons::Condition::NotBetween { field, low, high } => {
                    sql.push_str(&format!("{} NOT BETWEEN @{} AND @{}", field, param_index, param_index + 1));
                    params.push(serde_json::json!({
                        "name": param_index.to_string(),
                        "value": low
                    }));
                    params.push(serde_json::json!({
                        "name": (param_index + 1).to_string(),
                        "value": high
                    }));
                    param_index += 2;
                }
                nexus_data_commons::Condition::And(_) | nexus_data_commons::Condition::Or(_) => {
                    // Nested conditions - for now, skip (would need recursive handling)
                    continue;
                }
            }
        }

        (sql, params)
    }

    // Convert params to the format expected by the database driver
    fn convert_params(&self) -> R2dbcResult<()> {
        // This is a placeholder - actual implementation would convert
        // serde_json::Value to the appropriate database types
        Ok(())
    }

    // Execute a count query
    async fn execute_count(&self, _sql: &str) -> R2dbcResult<u64> {
        // Placeholder
        Ok(0)
    }

    // Map rows to target type
    fn map_rows<T>(&self, _rows: Vec<Row>) -> R2dbcResult<Vec<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        // This is a placeholder - actual implementation would deserialize rows
        Ok(Vec::new())
    }
}

impl Executor for QueryExecutor {
    fn fetch_one(
        &self,
        sql: &str,
        _params: Vec<serde_json::Value>,
    ) -> Pin<Box<dyn Future<Output = R2dbcResult<Option<Row>>> + Send + '_>> {
        let sql = sql.to_string();
        Box::pin(async move {
            self.client.fetch_one(&sql).await
        })
    }

    fn fetch_all(
        &self,
        sql: &str,
        _params: Vec<serde_json::Value>,
    ) -> Pin<Box<dyn Future<Output = R2dbcResult<Vec<Row>>> + Send + '_>> {
        let sql = sql.to_string();
        Box::pin(async move {
            self.client.fetch_all(&sql).await
        })
    }

    fn execute(
        &self,
        sql: &str,
        _params: Vec<serde_json::Value>,
    ) -> Pin<Box<dyn Future<Output = R2dbcResult<u64>> + Send + '_>> {
        let sql = sql.to_string();
        Box::pin(async move {
            self.client.execute(&sql).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_select_query_simple() {
        // Create a mock executor for testing
        let client = DatabaseClient::new(
            ConnectionPool::connect("postgresql://localhost/test")
                .await
                .unwrap()
        );
        let executor = QueryExecutor::new(client);

        let wrapper = QueryWrapper::new().eq("status", "active");
        let (sql, _params) = executor.build_select_query(&wrapper, "users");

        assert!(sql.contains("SELECT * FROM users"));
        assert!(sql.contains("status"));
    }

    #[test]
    fn test_build_select_query_with_order() {
        let client = DatabaseClient::new(
            ConnectionPool::connect("postgresql://localhost/test")
                .await
                .unwrap()
        );
        let executor = QueryExecutor::new(client);

        let wrapper = QueryWrapper::new()
            .eq("status", "active")
            .order_by_asc("name");
        let (sql, _params) = executor.build_select_query(&wrapper, "users");

        assert!(sql.contains("ORDER BY"));
        assert!(sql.contains("ASC"));
    }
}
