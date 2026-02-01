//! Row and result types
//! 行和结果类型
//!
//! # Overview / 概述
//!
//! Types for representing database rows and results.
//! 表示数据库行和结果的类型。

use std::any::Any;

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
/// let row: Row = // ... obtained from query
/// let id: i64 = row.get("id")?;
/// let name: String = row.get("name")?;
/// ```
pub trait Row: Send + Sync {
    /// Get a value by column name
    /// 通过列名获取值
    fn get<'a, T>(&'a self, name: &str) -> Result<T, crate::Error>
    where
        T: RowValue<'a>;

    /// Get a value by column index
    /// 通过列索引获取值
    fn get_by_index<'a, T>(&'a self, index: usize) -> Result<T, crate::Error>
    where
        T: RowValue<'a>;

    /// Get the number of columns
    /// 获取列数
    fn column_count(&self) -> usize;

    /// Get column names
    /// 获取列名
    fn column_names(&self) -> Vec<String>;

    /// Try to get a value by column name, returns None if column doesn't exist
    /// 尝试通过列名获取值，如果列不存在则返回 None
    fn try_get<'a, T>(&'a self, name: &str) -> Result<Option<T>, crate::Error>
    where
        T: RowValue<'a>;
}

/// Marker trait for types that can be extracted from a row
/// 可从行中提取的类型的标记 trait
pub trait RowValue<'a>: Sized {
    /// Extract this value from a row
    /// 从行中提取此值
    fn extract(row: &'a dyn RowInternal) -> Result<Self, crate::Error>;
}

/// Internal row trait for value extraction
/// 用于值提取的内部行 trait
pub trait RowInternal {
    /// Get raw column value
    /// 获取原始列值
    fn get_raw(&self, name: &str) -> Result<ColumnValue, crate::Error>;

    /// Get raw column value by index
    /// 通过索引获取原始列值
    fn get_raw_by_index(&self, index: usize) -> Result<ColumnValue, crate::Error>;
}

/// Column value
/// 列值
#[derive(Debug, Clone)]
pub enum ColumnValue {
    /// Null value
    /// 空值
    Null,

    /// Boolean value
    /// 布尔值
    Bool(bool),

    /// Integer value (32-bit)
    I32(i32),

    /// Integer value (64-bit)
    I64(i64),

    /// Float value
    /// 浮点数值
    F64(f64),

    /// String value
    /// 字符串值
    String(String),

    /// Bytes value
    /// 字节值
    Bytes(Vec<u8>),

    /// UUID value
    /// UUID 值
    Uuid(uuid::Uuid),
}

impl ColumnValue {
    /// Check if value is null
    /// 检查值是否为空
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Convert to i64
    /// 转换为 i64
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::I64(v) => Some(*v),
            Self::I32(v) => Some(*v as i64),
            _ => None,
        }
    }

    /// Convert to f64
    /// 转换为 f64
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::F64(v) => Some(*v),
            Self::I64(v) => Some(*v as f64),
            _ => None,
        }
    }

    /// Convert to String
    /// 转换为 String
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(v) => Some(v),
            _ => None,
        }
    }

    /// Convert to bytes
    /// 转换为字节
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            Self::Bytes(v) => Some(v),
            _ => None,
        }
    }
}

/// Database rows (result set) - placeholder
/// 数据库行（结果集）- 占位符
///
/// Represents a collection of rows from a query result.
/// 表示查询结果的行集合。
///
/// Note: This is currently a placeholder. A full implementation would need
/// to use a concrete row type rather than the Row trait (which is not dyn-compatible).
/// 注意：这目前是占位符。完整实现需要使用具体的行类型而不是 Row trait（不是 dyn 兼容的）。
pub trait Rows: Send + Sync {
    /// Get the number of rows (placeholder)
    /// 获取行数（占位符）
    fn count(&self) -> Result<u64, crate::Error>;

    /// Collect all rows into a count (placeholder)
    /// 收集所有行到计数（占位符）
    fn collect(self) -> Result<u64, crate::Error>
    where
        Self: Sized;
}

/// Column metadata
/// 列元数据
#[derive(Debug, Clone)]
pub struct Column {
    /// Column name
    /// 列名
    pub name: String,

    /// Column type
    /// 列类型
    pub type_: ColumnType,

    /// Whether the column is nullable
    /// 列是否可为空
    pub nullable: bool,
}

/// Column type
/// 列类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnType {
    /// Boolean type
    /// 布尔类型
    Bool,

    /// Integer type
    /// 整数类型
    I64,

    /// Float type
    /// 浮点类型
    F64,

    /// String type
    /// 字符串类型
    String,

    /// Bytes type
    /// 字节类型
    Bytes,

    /// UUID type
    /// UUID 类型
    Uuid,

    /// Timestamp type
    /// 时间戳类型
    Timestamp,

    /// Date type
    /// 日期类型
    Date,

    /// Unknown type
    /// 未知类型
    Unknown,
}

// Implement RowValue for common types
impl<'a> RowValue<'a> for i64 {
    fn extract(row: &'a dyn RowInternal) -> Result<Self, crate::Error> {
        match row.get_raw("id")? {
            ColumnValue::I64(v) => Ok(v),
            ColumnValue::I32(v) => Ok(v as i64),
            _ => Err(crate::Error::Deserialization("Expected i64".to_string())),
        }
    }
}

impl<'a> RowValue<'a> for String {
    fn extract(row: &'a dyn RowInternal) -> Result<Self, crate::Error> {
        match row.get_raw("name")? {
            ColumnValue::String(v) => Ok(v),
            _ => Err(crate::Error::Deserialization("Expected String".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_value_null() {
        let value = ColumnValue::Null;
        assert!(value.is_null());
        assert!(value.as_i64().is_none());
    }

    #[test]
    fn test_column_value_i64() {
        let value = ColumnValue::I64(42);
        assert!(!value.is_null());
        assert_eq!(value.as_i64(), Some(42));
        assert_eq!(value.as_f64(), Some(42.0));
    }

    #[test]
    fn test_column_value_string() {
        let value = ColumnValue::String("hello".to_string());
        assert_eq!(value.as_str(), Some("hello"));
    }
}
