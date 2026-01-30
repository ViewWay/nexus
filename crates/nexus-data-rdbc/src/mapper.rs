//! Repository and mapper implementations
//! Repository 和 Mapper 实现
//!
//! # Overview / 概述
//!
//! This module provides BaseMapper (MyBatis-Plus) and R2dbcRepository implementations.
//! 本模块提供 BaseMapper (MyBatis-Plus) 和 R2dbcRepository 实现。

use crate::{QueryExecutor, R2dbcError};
use async_trait::async_trait;
use nexus_data_commons::{Identifier, Page, PageRequest, QueryWrapper, ToValue};

/// Base Mapper trait (MyBatis-Plus equivalent)
/// Base Mapper trait (MyBatis-Plus 等价)
///
/// This trait provides the basic CRUD methods similar to MyBatis-Plus BaseMapper.
/// 此 trait 提供类似 MyBatis-Plus BaseMapper 的基本 CRUD 方法.
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_data_rdbc::BaseMapper;
/// use nexus_data_commons::QueryWrapper;
///
/// struct UserMapper {
///     executor: QueryExecutor,
/// }
///
/// impl BaseMapper<User> for UserMapper {
///     fn table_name() -> &'static str {
///         "users"
///     }
///
///     fn executor(&self) -> &QueryExecutor {
///         &self.executor
///     }
/// }
///
/// // Usage:
/// let users = mapper.select_list(
///     QueryWrapper::new().eq("status", "active")
/// ).await?;
///
/// let user = mapper.select_by_id(42).await?;
/// ```
#[async_trait]
pub trait BaseMapper<T: for<'de> serde::Deserialize<'de>>: Send + Sync {
    /// Get the table name for this mapper
    /// 获取此 mapper 的表名
    fn table_name() -> &'static str
    where
        Self: Sized;

    /// Get the query executor
    /// 获取查询执行器
    fn executor(&self) -> &QueryExecutor;

    /// Insert a record
    /// 插入记录
    ///
    /// Equivalent to MyBatis-Plus: `int insert(T entity);`
    async fn insert(&self, entity: &T) -> Result<i64, R2dbcError>;

    /// Delete by ID
    /// 根据 ID 删除
    ///
    /// Equivalent to MyBatis-Plus: `int deleteById(Serializable id);`
    async fn delete_by_id<I>(&self, id: I) -> Result<i64, R2dbcError>
    where
        I: Identifier + ToValue,
        Self: Sized,
    {
        let wrapper = QueryWrapper::new().eq("id", id.to_value());
        self.delete(wrapper).await
    }

    /// Delete by wrapper
    /// 根据条件删除
    ///
    /// Equivalent to MyBatis-Plus: `int delete(Wrapper<T> wrapper);`
    async fn delete(&self, wrapper: QueryWrapper) -> Result<i64, R2dbcError>
    where
        Self: Sized,
    {
        Ok(self.executor().delete(&wrapper, Self::table_name()).await? as i64)
    }

    /// Delete by batch of IDs
    /// 根据批量 ID 删除
    ///
    /// Equivalent to MyBatis-Plus: `int deleteBatchIds(Collection<? extends Serializable> idList);`
    async fn delete_batch_ids<I>(&self, ids: Vec<I>) -> Result<i64, R2dbcError>
    where
        I: Identifier + ToValue,
        Self: Sized,
    {
        let wrapper = QueryWrapper::new().in_("id", ids);
        self.delete(wrapper).await
    }

    /// Update by wrapper
    /// 根据条件更新
    ///
    /// Equivalent to MyBatis-Plus: `int update(Wrapper<T> updateWrapper);`
    async fn update(
        &self,
        wrapper: nexus_data_commons::UpdateWrapper,
    ) -> Result<i64, R2dbcError>
    where
        Self: Sized,
    {
        Ok(self.executor().update(&wrapper, Self::table_name()).await? as i64)
    }

    /// Update by ID
    /// 根据 ID 更新
    ///
    /// Equivalent to MyBatis-Plus: `int updateById(T entity);`
    async fn update_by_id(&self, entity: &T) -> Result<i64, R2dbcError>;

    /// Select by ID
    /// 根据 ID 查询
    ///
    /// Equivalent to MyBatis-Plus: `T selectById(Serializable id);`
    async fn select_by_id<I>(&self, id: I) -> Result<Option<T>, R2dbcError>
    where
        I: Identifier + ToValue,
    {
        let wrapper = QueryWrapper::new().eq("id", id.to_value());
        let results = self.select_list(wrapper).await?;
        Ok(results.into_iter().next())
    }

    /// Select batch by IDs
    /// 根据 ID 批量查询
    ///
    /// Equivalent to MyBatis-Plus: `List<T> selectBatchIds(Collection<? extends Serializable> idList);`
    async fn select_batch_ids<I>(&self, ids: Vec<I>) -> Result<Vec<T>, R2dbcError>
    where
        I: Identifier + ToValue,
    {
        let wrapper = QueryWrapper::new().in_("id", ids);
        self.select_list(wrapper).await
    }

    /// Select all records
    /// 查询所有记录
    ///
    /// Equivalent to MyBatis-Plus: `List<T> selectList(Wrapper<T> queryWrapper);`
    async fn select_list(&self, wrapper: QueryWrapper) -> Result<Vec<T>, R2dbcError>;

    /// Select records by wrapper (alias for select_list)
    /// 根据条件查询记录 (select_list 的别名)
    async fn select(&self, wrapper: QueryWrapper) -> Result<Vec<T>, R2dbcError> {
        self.select_list(wrapper).await
    }

    /// Select one record
    /// 查询单条记录
    ///
    /// Equivalent to MyBatis-Plus: `T selectOne(Wrapper<T> queryWrapper);`
    async fn select_one(
        &self,
        wrapper: QueryWrapper,
    ) -> Result<Option<T>, R2dbcError> {
        let limited = wrapper.clone().limit(1);
        let results = self.select_list(limited).await?;
        Ok(results.into_iter().next())
    }

    /// Select count
    /// 查询总数
    ///
    /// Equivalent to MyBatis-Plus: `Long selectCount(Wrapper<T> queryWrapper);`
    async fn select_count(&self, wrapper: QueryWrapper) -> Result<i64, R2dbcError>
    where
        Self: Sized,
    {
        Ok(self.executor().count(&wrapper, Self::table_name()).await? as i64)
    }

    /// Select page
    /// 分页查询
    ///
    /// Equivalent to MyBatis-Plus: `Page<T> selectPage(Page<T> page, Wrapper<T> queryWrapper);`
    async fn select_page(
        &self,
        page_request: PageRequest,
        wrapper: QueryWrapper,
    ) -> Result<Page<T>, R2dbcError>
    where
        Self: Sized,
    {
        self.executor()
            .select_page(&wrapper, page_request, Self::table_name())
            .await
            .map_err(|e| R2dbcError::Sql(e.to_string()))
    }
}

/// R2DBC Repository trait
/// R2DBC Repository trait
///
/// Base trait for R2DBC repositories.
/// R2DBC Repository 的基础 trait。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_data_rdbc::R2dbcRepository;
///
/// struct UserRepository {
///     executor: QueryExecutor,
/// }
///
/// impl R2dbcRepository<User, i32> for UserRepository {
///     fn table_name() -> &'static str {
///         "users"
///     }
///
///     fn executor(&self) -> &QueryExecutor {
///         &self.executor
///     }
///
///     fn id(&self, entity: &User) -> i32 {
///         entity.id
///     }
/// }
/// ```
pub trait R2dbcRepository<T, ID>: Send + Sync {
    /// Get the table name for this repository
    /// 获取此 repository 的表名
    fn table_name() -> &'static str
    where
        Self: Sized;

    /// Get the query executor
    /// 获取查询执行器
    fn executor(&self) -> &QueryExecutor;

    /// Get the ID from an entity
    /// 从实体获取 ID
    fn id(&self, entity: &T) -> ID;

    /// Get the ID type name
    /// 获取 ID 类型名称
    fn id_type_name() -> &'static str {
        std::any::type_name::<ID>()
    }
}

/// R2DBC CRUD Repository
/// R2DBC CRUD Repository
///
/// Provides CRUD operations for R2DBC repositories.
/// 为 R2DBC Repository 提供 CRUD 操作。
///
/// This trait uses nexus_data_commons::Error for all operations,
/// simplifying error handling.
///
/// 此 trait 对所有操作使用 nexus_data_commons::Error，简化了错误处理。
#[async_trait]
pub trait R2dbcCrudRepository<T: Send + 'static, ID: Send + Sync + 'static>:
    R2dbcRepository<T, ID>
{
    /// Save an entity (insert or update)
    /// 保存实体（插入或更新）
    async fn save(&self, entity: T) -> Result<T, nexus_data_commons::Error> {
        // Check if entity is new (ID is default/zero/null)
        let is_new = self.is_new(&entity);

        if is_new {
            self.insert(entity).await
        } else {
            self.update_entity(entity).await
        }
    }

    /// Insert a new entity
    /// 插入新实体
    async fn insert(&self, entity: T) -> Result<T, nexus_data_commons::Error>;

    /// Update an existing entity
    /// 更新现有实体
    async fn update_entity(&self, entity: T) -> Result<T, nexus_data_commons::Error>;

    /// Find by ID
    /// 根据 ID 查找
    async fn find_by_id(&self, id: ID)
    -> Result<Option<T>, nexus_data_commons::Error>;

    /// Find all entities
    /// 查找所有实体
    async fn find_all(&self) -> Result<Vec<T>, nexus_data_commons::Error>;

    /// Count all entities
    /// 统计所有实体
    async fn count(&self) -> Result<u64, nexus_data_commons::Error>;

    /// Delete by ID
    /// 根据 ID 删除
    async fn delete_by_id(&self, id: ID) -> Result<(), nexus_data_commons::Error>;

    /// Delete an entity
    /// 删除实体
    async fn delete(&self, entity: T) -> Result<(), nexus_data_commons::Error>;

    /// Delete all entities
    /// 删除所有实体
    async fn delete_all(&self) -> Result<(), nexus_data_commons::Error>;

    /// Check if an entity is new
    /// 检查实体是否为新
    fn is_new(&self, entity: &T) -> bool;

    /// Find all with pagination
    /// 分页查找所有实体
    async fn find_all_pageable(
        &self,
        pageable: PageRequest,
    ) -> Result<Page<T>, nexus_data_commons::Error>;

    /// Find by wrapper
    /// 根据条件查找
    async fn find_by_wrapper(
        &self,
        wrapper: QueryWrapper,
    ) -> Result<Vec<T>, nexus_data_commons::Error>;

    /// Find one by wrapper
    /// 根据条件查找单个
    async fn find_one_by_wrapper(
        &self,
        wrapper: QueryWrapper,
    ) -> Result<Option<T>, nexus_data_commons::Error> {
        let results = self.find_by_wrapper(wrapper).await?;
        Ok(results.into_iter().next())
    }

    /// Count by wrapper
    /// 根据条件统计
    async fn count_by_wrapper(
        &self,
        wrapper: QueryWrapper,
    ) -> Result<u64, nexus_data_commons::Error>;

    /// Exists by wrapper
    /// 根据条件检查是否存在
    async fn exists_by_wrapper(
        &self,
        wrapper: QueryWrapper,
    ) -> Result<bool, nexus_data_commons::Error> {
        Ok(self.count_by_wrapper(wrapper).await? > 0)
    }

    /// Delete by wrapper
    /// 根据条件删除
    async fn delete_by_wrapper(
        &self,
        wrapper: QueryWrapper,
    ) -> Result<u64, nexus_data_commons::Error>;
}

/// Simple R2DBC Repository placeholder
/// 简单的 R2DBC Repository 占位符
///
/// This is a placeholder for future implementation.
/// Users should implement R2dbcRepository or R2dbcCrudRepository traits
/// directly for their specific use cases.
///
/// 这是未来实现的占位符。
/// 用户应该直接为他们的特定用例实现 R2dbcRepository 或 R2dbcCrudRepository traits。
///
/// # Example / 示例
///
/// ```rust,no_run,ignore
/// use nexus_data_rdbc::{R2dbcCrudRepository, QueryExecutor};
/// use nexus_data_commons::{Error, Result, QueryWrapper};
/// use async_trait::async_trait;
///
/// struct UserRepository {
///     executor: QueryExecutor,
/// }
///
/// impl UserRepository {
///     pub fn new(client: nexus_data_rdbc::DatabaseClient) -> Self {
///         Self {
///             executor: QueryExecutor::new(client),
///         }
///     }
/// }
///
/// impl nexus_data_rdbc::R2dbcRepository<User, i32> for UserRepository {
///     fn table_name() -> &'static str {
///         "users"
///     }
///
///     fn executor(&self) -> &nexus_data_rdbc::QueryExecutor {
///         &self.executor
///     }
///
///     fn id(&self, entity: &User) -> i32 {
///         entity.id
///     }
/// }
///
/// #[async_trait]
/// impl nexus_data_rdbc::R2dbcCrudRepository<User, i32> for UserRepository {
///     // Implement required methods...
/// }
/// ```
pub(crate) struct SimpleR2dbcRepository<T, ID> {
    _phantom: std::marker::PhantomData<(T, ID)>,
}

impl<T, ID> SimpleR2dbcRepository<T, ID> {
    /// Create a new placeholder repository
    /// 创建新的占位符 repository
    pub(crate) fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T, ID> Default for SimpleR2dbcRepository<T, ID> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_repository_creation() {
        let _repo: SimpleR2dbcRepository<(), ()> = SimpleR2dbcRepository::new();
        // Just verify it compiles
    }
}
