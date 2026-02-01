//! Database migrations
//! 数据库迁移
//!
//! # Overview / 概述
//!
//! This module provides database migration support.
//! 本模块提供数据库迁移支持。
//!
//! # Equivalent to Spring / 等价于 Spring
//!
//! | Nexus | Spring / Flyway |
//! |-------|-----------------|
//! | `Migration` | `FlywayMigration` / `Liquibase` |
//! | `Migrator` | `Flyway` / `Liquibase` |
//! | `Schema` | `SchemaCreator` / `JPA DDL` |
//!
//! # Example / 示例
//!
//! ```rust,no_run,ignore
//! use nexus_data_orm::migrations::{Migration, Migrator};
//!
//! let migration = Migration::new("001_create_users")
//!     .up("CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT);")
//!     .down("DROP TABLE users;");
//!
//! let migrator = Migrator::new(db);
//! migrator.register(migration);
//! migrator.up().await?;
//! ```

use crate::{Error, Result};
use std::collections::HashMap;

/// Migration
/// 迁移
///
/// Represents a single database migration with up and down SQL.
/// 表示具有向上和向下 SQL 的单个数据库迁移。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// let migration = Migration::new("001_create_users")
///     .up("CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT);")
///     .down("DROP TABLE users;");
/// ```
#[derive(Debug, Clone)]
pub struct Migration {
    /// Migration name/identifier
    /// 迁移名称/标识符
    pub name: String,

    /// Migration version (e.g., "001", "2023-01-01")
    /// 迁移版本（例如 "001", "2023-01-01"）
    pub version: String,

    /// Description of what the migration does
    /// 迁移所做事情的描述
    pub description: String,

    /// SQL to apply the migration
    /// 应用迁移的 SQL
    pub up_sql: String,

    /// SQL to rollback the migration
    /// 回滚迁移的 SQL
    pub down_sql: String,

    /// Whether this migration has been applied
    /// 此迁移是否已应用
    pub applied: bool,

    /// Migration attributes
    /// 迁移属性
    pub attributes: HashMap<String, String>,
}

impl Migration {
    /// Create a new migration
    /// 创建新迁移
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// let migration = Migration::new("001_create_users");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            name: name.clone(),
            version: Self::extract_version(&name),
            description: String::new(),
            up_sql: String::new(),
            down_sql: String::new(),
            applied: false,
            attributes: HashMap::new(),
        }
    }

    /// Create a new migration with explicit version
    /// 创建具有显式版本的新迁移
    pub fn with_version(version: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: String::new(),
            up_sql: String::new(),
            down_sql: String::new(),
            applied: false,
            attributes: HashMap::new(),
        }
    }

    /// Set the up SQL
    /// 设置向上 SQL
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// let migration = Migration::new("001_create_users")
    ///     .up("CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT);");
    /// ```
    pub fn up(mut self, sql: impl Into<String>) -> Self {
        self.up_sql = sql.into();
        self
    }

    /// Set the down SQL
    /// 设置向下 SQL
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// let migration = Migration::new("001_create_users")
    ///     .down("DROP TABLE users;");
    /// ```
    pub fn down(mut self, sql: impl Into<String>) -> Self {
        self.down_sql = sql.into();
        self
    }

    /// Set the description
    /// 设置描述
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    /// Add an attribute
    /// 添加属性
    pub fn attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Extract version from migration name
    /// 从迁移名称提取版本
    fn extract_version(name: &str) -> String {
        // Try to extract a leading number or date pattern
        if let Some(stripped) = name.strip_prefix('V') {
            if let Some(end) = stripped.find("__") {
                return format!("V{}", &stripped[..end]);
            }
        }
        // Look for leading number pattern
        if let Some(end) = name.find('_') {
            let prefix = &name[..end];
            if prefix.chars().all(|c| c.is_ascii_digit()) {
                return prefix.to_string();
            }
        }
        // Default: use the full name as version
        name.to_string()
    }

    /// Validate the migration
    /// 验证迁移
    pub fn validate(&self) -> Result<()> {
        if self.up_sql.is_empty() {
            return Err(Error::validation(format!(
                "Migration {} has no up_sql",
                self.name
            )));
        }
        if self.down_sql.is_empty() {
            return Err(Error::validation(format!(
                "Migration {} has no down_sql",
                self.name
            )));
        }
        Ok(())
    }
}

/// Migration direction
/// 迁移方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationDirection {
    /// Apply migrations (up)
    /// 应用迁移（向上）
    Up,

    /// Rollback migrations (down)
    /// 回滚迁移（向下）
    Down,
}

/// Migrator
/// 迁移器
///
/// Manages and executes database migrations.
/// 管理和执行数据库迁移。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// let migrator = Migrator::new(db);
/// migrator.register(migration);
/// migrator.up().await?;
/// ```
pub struct Migrator {
    /// Registered migrations
    /// 已注册的迁移
    migrations: Vec<Migration>,

    /// Connection (placeholder - would be actual connection type)
    /// 连接（占位符 - 将是实际的连接类型）
    #[allow(dead_code)]
    connection: Option<Connection>,

    /// Migration table name
    /// 迁移表名
    migration_table: String,
}

/// Connection placeholder
/// 连接占位符
#[derive(Clone)]
pub struct Connection;

impl Migrator {
    /// Create a new migrator
    /// 创建新迁移器
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// let migrator = Migrator::new(None);
    /// ```
    pub fn new(connection: Option<Connection>) -> Self {
        Self {
            migrations: Vec::new(),
            connection,
            migration_table: "_migrations".to_string(),
        }
    }

    /// Register a migration
    /// 注册迁移
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// migrator.register(migration);
    /// ```
    pub fn register(mut self, migration: Migration) -> Self {
        self.migrations.push(migration);
        self
    }

    /// Register multiple migrations
    /// 注册多个迁移
    pub fn register_all(mut self, migrations: Vec<Migration>) -> Self {
        self.migrations.extend(migrations);
        self
    }

    /// Set the migration table name
    /// 设置迁移表名
    pub fn migration_table(mut self, table: impl Into<String>) -> Self {
        self.migration_table = table.into();
        self
    }

    /// Get all migrations
    /// 获取所有迁移
    pub fn migrations(&self) -> &[Migration] {
        &self.migrations
    }

    /// Get pending migrations
    /// 获取待执行的迁移
    pub fn pending(&self) -> Vec<&Migration> {
        self.migrations
            .iter()
            .filter(|m| !m.applied)
            .collect()
    }

    /// Get applied migrations
    /// 获取已应用的迁移
    pub fn applied(&self) -> Vec<&Migration> {
        self.migrations
            .iter()
            .filter(|m| m.applied)
            .collect()
    }

    /// Run all pending migrations (placeholder)
    /// 运行所有待执行的迁移（占位符）
    pub async fn up(&mut self) -> Result<usize> {
        let pending = self.pending();
        let count = pending.len();

        for migration in pending {
            migration.validate()?;
            // Placeholder: execute up_sql
        }

        Ok(count)
    }

    /// Rollback the last migration (placeholder)
    /// 回滚最后的迁移（占位符）
    pub async fn down(&mut self) -> Result<bool> {
        if let Some(last) = self.migrations.iter().rev().find(|m| m.applied) {
            last.validate()?;
            // Placeholder: execute down_sql
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Rollback a specific number of migrations (placeholder)
    /// 回滚指定数量的迁移（占位符）
    pub async fn rollback(&mut self, steps: usize) -> Result<usize> {
        let applied: Vec<_> = self
            .migrations
            .iter()
            .filter(|m| m.applied)
            .rev()
            .take(steps)
            .collect();

        let count = applied.len();
        for migration in applied {
            migration.validate()?;
            // Placeholder: execute down_sql
        }

        Ok(count)
    }

    /// Refresh - rollback all migrations and reapply them (placeholder)
    /// 刷新 - 回滚所有迁移并重新应用它们（占位符）
    pub async fn refresh(&mut self) -> Result<usize> {
        self.down().await?;
        self.up().await
    }

    /// Reset - rollback all migrations (placeholder)
    /// 重置 - 回滚所有迁移（占位符）
    pub async fn reset(&mut self) -> Result<usize> {
        let count = self.applied().len();
        // Placeholder: rollback all
        Ok(count)
    }
}

impl Default for Migrator {
    fn default() -> Self {
        Self::new(None)
    }
}

/// Schema builder
/// Schema 构建器
///
/// Provides a fluent interface for building database schemas.
/// 提供用于构建数据库模式的流畅接口。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// let sql = Schema::create_table("users")
///     .column("id", ColumnType::I64).primary_key()
///     .column("name", ColumnType::String).max_length(255)
///     .column("email", ColumnType::String).max_length(255).unique()
///     .to_sql();
/// ```
pub struct Schema {
    /// Schema operations
    /// 模式操作
    operations: Vec<SchemaOperation>,
}

/// Schema operation
/// 模式操作
#[derive(Debug, Clone)]
pub enum SchemaOperation {
    /// Create table
    /// 创建表
    CreateTable {
        name: String,
        columns: Vec<ColumnDefinition>,
    },

    /// Drop table
    /// 删除表
    DropTable { name: String },

    /// Add column
    /// 添加列
    AddColumn {
        table: String,
        column: ColumnDefinition,
    },

    /// Drop column
    /// 删除列
    DropColumn { table: String, name: String },

    /// Rename table
    /// 重命名表
    RenameTable { from: String, to: String },

    /// Add index
    /// 添加索引
    AddIndex {
        table: String,
        name: String,
        columns: Vec<String>,
        unique: bool,
    },

    /// Drop index
    /// 删除索引
    DropIndex { table: String, name: String },
}

/// Column definition
/// 列定义
#[derive(Debug, Clone)]
pub struct ColumnDefinition {
    /// Column name
    /// 列名
    pub name: String,

    /// Column type
    /// 列类型
    pub type_: crate::ColumnType,

    /// Whether this is a primary key
    /// 是否为主键
    pub is_primary_key: bool,

    /// Whether this is nullable
    /// 是否可为空
    pub is_nullable: bool,

    /// Whether this is unique
    /// 是否唯一
    pub is_unique: bool,

    /// Default value
    /// 默认值
    pub default: Option<String>,

    /// Max length for string types
    /// 字符串类型的最大长度
    pub max_length: Option<usize>,

    /// References (foreign key)
    /// 引用（外键）
    pub references: Option<Reference>,
}

/// Foreign key reference
/// 外键引用
#[derive(Debug, Clone)]
pub struct Reference {
    /// Referenced table
    /// 被引用的表
    pub table: String,

    /// Referenced column
    /// 被引用的列
    pub column: String,

    /// On delete behavior
    /// 删除时行为
    pub on_delete: Option<super::relationships::OnDelete>,
}

impl Schema {
    /// Create a new schema builder
    /// 创建新的模式构建器
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Create a table
    /// 创建表
    pub fn create_table(name: impl Into<String>) -> TableBuilder {
        TableBuilder {
            name: name.into(),
            columns: Vec::new(),
        }
    }

    /// Drop a table
    /// 删除表
    pub fn drop_table(mut self, name: impl Into<String>) -> Self {
        self.operations.push(SchemaOperation::DropTable {
            name: name.into(),
        });
        self
    }

    /// Add the operation to the schema
    /// 将操作添加到模式
    pub fn add_operation(mut self, operation: SchemaOperation) -> Self {
        self.operations.push(operation);
        self
    }

    /// Generate SQL for the schema (placeholder)
    /// 为模式生成 SQL（占位符）
    pub fn to_sql(&self, dialect: crate::model::SqlDialect) -> String {
        let mut sql = String::new();
        for operation in &self.operations {
            sql.push_str(&self.operation_to_sql(operation, dialect));
            sql.push_str(";\n");
        }
        sql
    }

    fn operation_to_sql(&self, operation: &SchemaOperation, dialect: crate::model::SqlDialect) -> String {
        match operation {
            SchemaOperation::CreateTable { name, columns } => {
                let mut sql = format!("CREATE TABLE {} (", name);
                let column_defs: Vec<String> = columns
                    .iter()
                    .map(|c| self.column_to_sql(c, dialect))
                    .collect();
                sql.push_str(&column_defs.join(", "));
                sql.push(')');
                sql
            }
            SchemaOperation::DropTable { name } => {
                format!("DROP TABLE {}", name)
            }
            _ => format!("-- Not implemented: {:?}", operation),
        }
    }

    fn column_to_sql(&self, column: &ColumnDefinition, dialect: crate::model::SqlDialect) -> String {
        let mut sql = format!("{} {}", column.name, column.type_.as_sql(dialect));

        if column.is_primary_key {
            sql.push_str(" PRIMARY KEY");
        }

        if !column.is_nullable {
            sql.push_str(" NOT NULL");
        }

        if column.is_unique {
            sql.push_str(" UNIQUE");
        }

        if let Some(default) = &column.default {
            sql.push_str(&format!(" DEFAULT {}", default));
        }

        if let Some(len) = column.max_length {
            if matches!(column.type_, crate::ColumnType::String) {
                sql = format!("{}({})", sql.split(' ').next().unwrap_or(&column.name), len);
            }
        }

        sql
    }
}

impl Default for Schema {
    fn default() -> Self {
        Self::new()
    }
}

/// Table builder
/// 表构建器
pub struct TableBuilder {
    name: String,
    columns: Vec<ColumnDefinition>,
}

impl TableBuilder {
    /// Add a column
    /// 添加列
    pub fn column(mut self, name: impl Into<String>, type_: crate::ColumnType) -> ColumnBuilder {
        ColumnBuilder {
            table_builder: self,
            name: name.into(),
            type_,
            is_primary_key: false,
            is_nullable: false,
            is_unique: false,
            default: None,
            max_length: None,
            references: None,
        }
    }

    /// Finish building the table
    /// 完成表构建
    pub fn done(self) -> Schema {
        Schema {
            operations: vec![SchemaOperation::CreateTable {
                name: self.name,
                columns: self.columns,
            }],
        }
    }
}

/// Column builder
/// 列构建器
pub struct ColumnBuilder {
    table_builder: TableBuilder,
    name: String,
    type_: crate::ColumnType,
    is_primary_key: bool,
    is_nullable: bool,
    is_unique: bool,
    default: Option<String>,
    max_length: Option<usize>,
    references: Option<Reference>,
}

impl ColumnBuilder {
    /// Set as primary key
    /// 设置为主键
    pub fn primary_key(mut self) -> TableBuilder {
        self.is_primary_key = true;
        self.finish_column()
    }

    /// Set as nullable
    /// 设置为可为空
    pub fn nullable(mut self) -> Self {
        self.is_nullable = true;
        self
    }

    /// Set as unique
    /// 设置为唯一
    pub fn unique(mut self) -> Self {
        self.is_unique = true;
        self
    }

    /// Set default value
    /// 设置默认值
    pub fn default(mut self, value: impl Into<String>) -> Self {
        self.default = Some(value.into());
        self
    }

    /// Set max length
    /// 设置最大长度
    pub fn max_length(mut self, len: usize) -> Self {
        self.max_length = Some(len);
        self
    }

    /// Finish this column and add another
    /// 完成此列并添加另一列
    pub fn column(self, name: impl Into<String>, type_: crate::ColumnType) -> ColumnBuilder {
        let table_builder = self.finish_column();
        table_builder.column(name, type_)
    }

    /// Finish the table
    /// 完成表
    pub fn done(self) -> Schema {
        let table_builder = self.finish_column();
        table_builder.done()
    }

    fn finish_column(mut self) -> TableBuilder {
        self.table_builder.columns.push(ColumnDefinition {
            name: self.name,
            type_: self.type_,
            is_primary_key: self.is_primary_key,
            is_nullable: self.is_nullable,
            is_unique: self.is_unique,
            default: self.default,
            max_length: self.max_length,
            references: self.references,
        });
        self.table_builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_migration_creation() {
        let migration = Migration::new("001_create_users")
            .up("CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT);")
            .down("DROP TABLE users;");

        assert_eq!(migration.name, "001_create_users");
        assert_eq!(migration.version, "001");
        assert!(!migration.up_sql.is_empty());
        assert!(!migration.down_sql.is_empty());
    }

    #[test]
    fn test_migration_validation() {
        let migration = Migration::new("001_create_users")
            .up("CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT);")
            .down("DROP TABLE users;");

        assert!(migration.validate().is_ok());

        let invalid_migration = Migration::new("002_empty")
            .up("")
            .down("");
        assert!(invalid_migration.validate().is_err());
    }

    #[test]
    fn test_schema_create_table() {
        let schema = Schema::create_table("users")
            .column("id", crate::ColumnType::I64).primary_key()
            .column("name", crate::ColumnType::String)
            .done();

        assert_eq!(schema.operations.len(), 1);

        let sql = schema.to_sql(crate::model::SqlDialect::PostgreSQL);
        assert!(sql.contains("CREATE TABLE users"));
        assert!(sql.contains("id BIGINT PRIMARY KEY"));
        assert!(sql.contains("name VARCHAR"));
    }

    #[test]
    fn test_migrator() {
        let migrator = Migrator::new(None)
            .register(Migration::new("001_create_users"))
            .register(Migration::new("002_add_posts"));

        assert_eq!(migrator.migrations().len(), 2);
        assert_eq!(migrator.pending().len(), 2);
        assert_eq!(migrator.applied().len(), 0);
    }
}
