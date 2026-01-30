//! Row mapping and data extraction
//! 行映射和数据提取
//!
//! # Overview / 概述
//!
//! This module provides row mapping from database results to Rust types.
//! 本模块提供从数据库结果到 Rust 类型的行映射。

use crate::{R2dbcError, R2dbcResult};

/// Database row
/// 数据库行
///
/// Represents a single row from a query result.
/// 表示查询结果中的单行。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_data_rdbc::Row;
///
/// let row: Row = ...;
///
/// let id: i32 = row.get("id")?;
/// let name: String = row.get("name")?;
/// let created_at: chrono::DateTime<chrono::Utc> = row.get("created_at")?;
/// ```
#[derive(Clone, Debug)]
pub struct Row {
    /// Column values
    /// 列值
    pub columns: std::collections::HashMap<String, ColumnValue>,

    /// Column names in order
    /// 有序的列名
    pub column_names: Vec<String>,
}

/// Column value
/// 列值
///
/// Represents a value from a database column.
/// 表示数据库列中的值。
#[derive(Clone, Debug)]
pub enum ColumnValue {
    /// Null value
    /// 空值
    Null,

    /// Boolean value
    /// 布尔值
    Bool(bool),

    /// Integer value
    /// 整数值
    I32(i32),

    /// Long integer value
    /// 长整数值
    I64(i64),

    /// Float value
    /// 浮点数值
    F64(f64),

    /// String value
    /// 字符串值
    String(String),

    /// Byte array value
    /// 字节数组值
    Bytes(Vec<u8>),

    /// UUID value
    /// UUID 值
    Uuid(uuid::Uuid),

    /// DateTime value
    /// 日期时间值
    DateTime(chrono::DateTime<chrono::Utc>),

    /// JSON value
    /// JSON 值
    Json(serde_json::Value),
}

impl Row {
    /// Create a new empty row
    /// 创建新的空行
    pub fn new() -> Self {
        Self {
            columns: std::collections::HashMap::new(),
            column_names: Vec::new(),
        }
    }

    /// Add a column to this row
    /// 向此行添加列
    pub fn add_column(&mut self, name: impl Into<String>, value: ColumnValue) {
        let name = name.into();
        self.column_names.push(name.clone());
        self.columns.insert(name, value);
    }

    /// Get the number of columns
    /// 获取列数
    pub fn column_count(&self) -> usize {
        self.column_names.len()
    }

    /// Get column names
    /// 获取列名
    pub fn column_names(&self) -> &[String] {
        &self.column_names
    }

    /// Check if a column exists
    /// 检查列是否存在
    pub fn contains_column(&self, name: &str) -> bool {
        self.columns.contains_key(name)
    }

    /// Get a column value by name
    /// 按名称获取列值
    pub fn get_value(&self, name: &str) -> Option<&ColumnValue> {
        self.columns.get(name)
    }

    /// Get a column value by index
    /// 按索引获取列值
    pub fn get_value_by_index(&self, index: usize) -> Option<&ColumnValue> {
        self.column_names
            .get(index)
            .and_then(|name| self.columns.get(name))
    }

    /// Get a typed value from a column
    /// 从列中获取类型化值
    ///
    /// # Type Conversions / 类型转换
    ///
    /// - `bool` - from Bool
    /// - `i32` - from I32
    /// - `i64` - from I64
    /// - `f64` - from F64
    /// - `String` - from String
    /// - `Vec<u8>` - from Bytes
    /// - `uuid::Uuid` - from Uuid
    /// - `chrono::DateTime<chrono::Utc>` - from DateTime
    /// - `serde_json::Value` - from Json
    pub fn get<T: FromColumn>(&self, name: &str) -> Result<T, R2dbcError> {
        let value = self
            .get_value(name)
            .ok_or_else(|| R2dbcError::row_mapping(format!("Column '{}' not found", name)))?;

        T::from_column(value).ok_or_else(|| {
            R2dbcError::row_mapping(format!(
                "Failed to convert column '{}' to type {}",
                name,
                std::any::type_name::<T>()
            ))
        })
    }

    /// Get a typed value by index
    /// 按索引获取类型化值
    pub fn get_by_index<T: FromColumn>(&self, index: usize) -> Result<T, R2dbcError> {
        let value = self
            .get_value_by_index(index)
            .ok_or_else(|| R2dbcError::row_mapping(format!("Index {} out of bounds", index)))?;

        T::from_column(value).ok_or_else(|| {
            R2dbcError::row_mapping(format!(
                "Failed to convert index {} to type {}",
                index,
                std::any::type_name::<T>()
            ))
        })
    }

    /// Try to get an optional value
    /// 尝试获取可选值
    ///
    /// Returns None if the column is null or doesn't exist.
    /// 如果列为 null 或不存在，则返回 None。
    pub fn try_get<T: FromColumn>(&self, name: &str) -> Result<Option<T>, R2dbcError> {
        match self.get_value(name) {
            Some(ColumnValue::Null) => Ok(None),
            Some(value) => T::from_column(value).map(Some).ok_or_else(|| {
                R2dbcError::row_mapping(format!(
                    "Failed to convert column '{}' to type {}",
                    name,
                    std::any::type_name::<T>()
                ))
            }),
            None => Ok(None),
        }
    }

    /// Convert row to JSON value
    /// 将行转换为 JSON 值
    ///
    /// This method is used by the query executor to map database rows to entities.
    /// 此方法被查询执行器用于将数据库行映射到实体。
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// let mut row = Row::new();
    /// row.add_column("id", ColumnValue::I32(1));
    /// row.add_column("name", ColumnValue::String("Alice".to_string()));
    ///
    /// let json = row.to_json()?;
    /// ```
    pub fn to_json(&self) -> R2dbcResult<serde_json::Value> {
        let mut map = serde_json::Map::new();

        for (column_name, column_value) in &self.columns {
            let json_value = match column_value {
                ColumnValue::Null => serde_json::Value::Null,
                ColumnValue::Bool(b) => serde_json::Value::Bool(*b),
                ColumnValue::I32(i) => serde_json::Value::Number((*i).into()),
                ColumnValue::I64(i) => serde_json::Value::Number((*i).into()),
                ColumnValue::F64(f) => {
                    serde_json::Number::from_f64(*f)
                        .map(serde_json::Value::Number)
                        .unwrap_or_else(|| serde_json::json!(0))
                },
                ColumnValue::String(s) => serde_json::Value::String(s.clone()),
                ColumnValue::Bytes(b) => serde_json::Value::Array(
                    b.iter()
                        .map(|byte| serde_json::Value::Number((*byte).into()))
                        .collect(),
                ),
                ColumnValue::Uuid(u) => serde_json::Value::String(u.to_string()),
                ColumnValue::DateTime(dt) => serde_json::Value::String(dt.to_rfc3339()),
                ColumnValue::Json(j) => j.clone(),
            };

            map.insert(column_name.clone(), json_value);
        }

        Ok(serde_json::Value::Object(map))
    }

    /// Convert row to JSON value with specific columns
    /// 使用特定列将行转换为 JSON 值
    ///
    /// # Example / 示例
    ///
    /// ```rust,no_run,ignore
    /// let row = ...; // row has columns: id, name, email
    ///
    /// let json = row.to_json_selected(&["id", "name"])?;
    /// // Result: {"id": 1, "name": "Alice"}
    /// ```
    pub fn to_json_selected(&self, columns: &[&str]) -> R2dbcResult<serde_json::Value> {
        let mut map = serde_json::Map::new();

        for column_name in columns {
            if let Some(column_value) = self.get_value(column_name) {
                let json_value = match column_value {
                    ColumnValue::Null => serde_json::Value::Null,
                    ColumnValue::Bool(b) => serde_json::Value::Bool(*b),
                    ColumnValue::I32(i) => serde_json::Value::Number((*i).into()),
                    ColumnValue::I64(i) => serde_json::Value::Number((*i).into()),
                    ColumnValue::F64(f) => {
                        serde_json::Number::from_f64(*f)
                            .map(serde_json::Value::Number)
                            .unwrap_or_else(|| serde_json::json!(0))
                    },
                    ColumnValue::String(s) => serde_json::Value::String(s.clone()),
                    ColumnValue::Bytes(b) => serde_json::Value::Array(
                        b.iter()
                            .map(|byte| serde_json::Value::Number((*byte).into()))
                            .collect(),
                    ),
                    ColumnValue::Uuid(u) => serde_json::Value::String(u.to_string()),
                    ColumnValue::DateTime(dt) => serde_json::Value::String(dt.to_rfc3339()),
                    ColumnValue::Json(j) => j.clone(),
                };

                map.insert(column_name.to_string(), json_value);
            }
        }

        Ok(serde_json::Value::Object(map))
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for converting column values to Rust types
/// 将列值转换为 Rust 类型的 trait
pub trait FromColumn: Sized {
    /// Try to convert a column value to this type
    /// 尝试将列值转换为此类型
    fn from_column(value: &ColumnValue) -> Option<Self>;
}

// Implement FromColumn for primitive types
impl FromColumn for bool {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl FromColumn for i32 {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::I32(i) => Some(*i),
            ColumnValue::I64(i) => i32::try_from(*i).ok(),
            _ => None,
        }
    }
}

impl FromColumn for i64 {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::I64(i) => Some(*i),
            ColumnValue::I32(i) => Some(*i as i64),
            _ => None,
        }
    }
}

impl FromColumn for f64 {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::F64(f) => Some(*f),
            ColumnValue::I32(i) => Some(*i as f64),
            ColumnValue::I64(i) => Some(*i as f64),
            _ => None,
        }
    }
}

impl FromColumn for String {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::String(s) => Some(s.clone()),
            ColumnValue::Bool(b) => Some(b.to_string()),
            ColumnValue::I32(i) => Some(i.to_string()),
            ColumnValue::I64(i) => Some(i.to_string()),
            ColumnValue::F64(f) => Some(f.to_string()),
            ColumnValue::Uuid(u) => Some(u.to_string()),
            ColumnValue::DateTime(dt) => Some(dt.to_rfc3339()),
            ColumnValue::Json(j) => Some(j.to_string()),
            ColumnValue::Null => None,
            ColumnValue::Bytes(_) => None,
        }
    }
}

impl FromColumn for Vec<u8> {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::Bytes(b) => Some(b.clone()),
            ColumnValue::String(s) => Some(s.as_bytes().to_vec()),
            _ => None,
        }
    }
}

impl FromColumn for uuid::Uuid {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::Uuid(u) => Some(*u),
            ColumnValue::String(s) => uuid::Uuid::parse_str(s).ok(),
            _ => None,
        }
    }
}

impl FromColumn for chrono::DateTime<chrono::Utc> {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::DateTime(dt) => Some(*dt),
            ColumnValue::String(s) => chrono::DateTime::parse_from_rfc3339(s)
                .ok()
                .map(|dt| dt.with_timezone(&chrono::Utc)),
            ColumnValue::I64(i) => Some(chrono::DateTime::from_timestamp(*i, 0)?),
            _ => None,
        }
    }
}

impl FromColumn for serde_json::Value {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::Json(j) => Some(j.clone()),
            ColumnValue::String(s) => serde_json::from_str(s).ok(),
            ColumnValue::Null => Some(serde_json::Value::Null),
            ColumnValue::Bool(b) => Some(serde_json::Value::Bool(*b)),
            ColumnValue::I32(i) => Some(serde_json::Value::Number((*i).into())),
            ColumnValue::I64(i) => Some(serde_json::Value::Number((*i).into())),
            ColumnValue::F64(f) => serde_json::Number::from_f64(*f).map(serde_json::Value::Number),
            _ => None,
        }
    }
}

impl FromColumn for Option<bool> {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::Null => Some(None),
            _ => FromColumn::from_column(value).map(Some),
        }
    }
}

impl FromColumn for Option<i32> {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::Null => Some(None),
            _ => FromColumn::from_column(value).map(Some),
        }
    }
}

impl FromColumn for Option<i64> {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::Null => Some(None),
            _ => FromColumn::from_column(value).map(Some),
        }
    }
}

impl FromColumn for Option<f64> {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::Null => Some(None),
            _ => FromColumn::from_column(value).map(Some),
        }
    }
}

impl FromColumn for Option<String> {
    fn from_column(value: &ColumnValue) -> Option<Self> {
        match value {
            ColumnValue::Null => Some(None),
            _ => FromColumn::from_column(value).map(Some),
        }
    }
}

/// Row mapper trait
/// 行映射器 trait
///
/// Defines how to map a database row to a Rust type.
/// 定义如何将数据库行映射到 Rust 类型。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_data_rdbc::{Row, RowMapper};
///
/// struct User {
///     id: i32,
///     name: String,
/// }
///
/// struct UserRowMapper;
///
/// impl RowMapper<User> for UserRowMapper {
///     fn map(&self, row: &Row) -> Result<User, Box<dyn std::error::Error>> {
///         Ok(User {
///             id: row.get("id")?,
///             name: row.get("name")?,
///         })
///     }
/// }
/// ```
pub trait RowMapper<T>: Send + Sync {
    /// Map a row to the target type
    /// 将行映射到目标类型
    fn map(&self, row: &Row) -> Result<T, Box<dyn std::error::Error>>;
}

/// Function-based row mapper
/// 基于函数的行映射器
///
/// A row mapper that uses a closure to map rows.
/// 使用闭包映射行的行映射器。
pub(crate) struct FnRowMapper<T, F>
where
    F: Fn(&Row) -> Result<T, Box<dyn std::error::Error>> + Send + Sync,
{
    f: F,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, F> FnRowMapper<T, F>
where
    F: Fn(&Row) -> Result<T, Box<dyn std::error::Error>> + Send + Sync,
{
    /// Create a new function-based row mapper
    /// 创建新的基于函数的行映射器
    pub(crate) fn new(f: F) -> Self {
        Self {
            f,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, F> RowMapper<T> for FnRowMapper<T, F>
where
    T: Send + Sync,
    F: Fn(&Row) -> Result<T, Box<dyn std::error::Error>> + Send + Sync,
{
    fn map(&self, row: &Row) -> Result<T, Box<dyn std::error::Error>> {
        (self.f)(row)
    }
}

/// Derive macro for RowMapper
/// RowMapper 的派生宏
///
/// Automatically implements RowMapper for a type.
/// 自动为类型实现 RowMapper。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_data_rdbc::Row;
/// use nexus_data_macros::RowMapper;
///
/// #[derive(RowMapper)]
/// struct User {
///     #[column = "id"]
///     id: i32,
///
///     #[column = "user_name"]
///     name: String,
/// }
/// ```
pub(crate) trait DeriveRowMapper: Sized {
    /// Map a row to this type
    /// 将行映射到此类型
    fn from_row(row: &Row) -> Result<Self, Box<dyn std::error::Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_row_new() {
        let row = Row::new();
        assert_eq!(row.column_count(), 0);
    }

    #[test]
    fn test_row_add_column() {
        let mut row = Row::new();
        row.add_column("id", ColumnValue::I32(42));
        row.add_column("name", ColumnValue::String("Alice".to_string()));

        assert_eq!(row.column_count(), 2);
        assert!(row.contains_column("id"));
        assert!(row.contains_column("name"));
    }

    #[test]
    fn test_row_get() {
        let mut row = Row::new();
        row.add_column("id", ColumnValue::I32(42));
        row.add_column("name", ColumnValue::String("Alice".to_string()));

        let id: i32 = row.get("id").unwrap();
        assert_eq!(id, 42);

        let name: String = row.get("name").unwrap();
        assert_eq!(name, "Alice");
    }

    #[test]
    fn test_from_column_bool() {
        assert_eq!(FromColumn::from_column(&ColumnValue::Bool(true)), Some(true));
        let result: Option<bool> = FromColumn::from_column(&ColumnValue::Null);
        assert_eq!(result, None);
    }

    #[test]
    fn test_from_column_i32() {
        assert_eq!(FromColumn::from_column(&ColumnValue::I32(42)), Some(42));
        assert_eq!(FromColumn::from_column(&ColumnValue::I64(42)), Some(42));
        let result: Option<i32> = FromColumn::from_column(&ColumnValue::Null);
        assert_eq!(result, None);
    }

    #[test]
    fn test_from_column_string() {
        assert_eq!(
            FromColumn::from_column(&ColumnValue::String("hello".to_string())),
            Some("hello".to_string())
        );
        assert_eq!(FromColumn::from_column(&ColumnValue::Bool(true)), Some("true".to_string()));
    }

    #[test]
    fn test_try_get_null() {
        let mut row = Row::new();
        row.add_column("optional", ColumnValue::Null);

        let result: Option<i32> = row.try_get("optional").unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_try_get_value() {
        let mut row = Row::new();
        row.add_column("value", ColumnValue::I32(42));

        let result: Option<i32> = row.try_get("value").unwrap();
        assert_eq!(result, Some(42));
    }
}
