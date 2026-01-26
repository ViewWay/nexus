//! Tests for Nexus Data Annotations
//! Nexus Data 注解测试

use nexus_data_annotations::{Column, Entity, GeneratedValue, Id, Table};

// ========================================================================
// Entity Annotation Tests / @Entity 注解测试
// ========================================================================

#[test]
fn test_entity_macro() {
    // This test verifies that the #[Entity] macro compiles correctly
    // 此测试验证 #[Entity] 宏能正确编译

    #[Entity]
    struct TestUser {
        id: i64,
        username: String,
    }

    // Verify table_name method exists
    // 验证 table_name 方法存在
    assert_eq!(TestUser::table_name(), "TestUser");
}

#[test]
fn test_table_macro() {
    // This test verifies that the #[Table] macro compiles correctly
    // 此测试验证 #[Table] 宏能正确编译

    #[Entity]
    #[Table(name = "custom_users")]
    struct TestUser {
        id: i64,
        username: String,
    }

    // Verify custom table name
    // 验证自定义表名
    assert_eq!(TestUser::table_name(), "custom_users");
}

#[test]
fn test_table_default_name() {
    // Test default table name (lowercase struct name)
    // 测试默认表名（小写的结构体名）

    #[Entity]
    #[Table]
    struct TestUser {
        id: i64,
        username: String,
    }

    // Should use lowercase struct name
    // 应该使用小写的结构体名
    assert_eq!(TestUser::table_name(), "testuser");
}

// ========================================================================
// Column Annotation Tests / @Column 注解测试
// ========================================================================

#[test]
fn test_column_macro() {
    // This test verifies that the #[Column] macro compiles correctly
    // 此测试验证 #[Column] 宏能正确编译

    #[Entity]
    struct TestUser {
        #[Column(name = "user_id")]
        id: i64,

        #[Column(name = "user_name", nullable = false)]
        username: String,

        #[Column(name = "email_address")]
        email: String,
    }

    // Verify column metadata
    // 验证列元数据
    assert_eq!(TestUser::id(), "id");
    assert_eq!(TestUser::username(), "username");
    assert_eq!(TestUser::email(), "email");
}

// ========================================================================
// Id Annotation Tests / @Id 注解测试
// ========================================================================

#[test]
fn test_id_macro() {
    // This test verifies that the #[Id] macro compiles correctly
    // 此测试验证 #[Id] 宏能正确编译

    #[Entity]
    struct TestUser {
        #[Id]
        id: i64,

        username: String,
    }

    // Verify id_type method exists
    // 验证 id_type 方法存在
    assert_eq!(TestUser::id_type(), "auto");
}

#[test]
fn test_id_with_custom_type() {
    // Test custom ID type
    // 测试自定义 ID 类型

    #[Entity]
    struct TestUser {
        #[Id]
        #[GeneratedValue(strategy = "INPUT")]
        id: i64,

        username: String,
    }

    // Verify custom ID type
    // 验证自定义 ID 类型
    assert_eq!(TestUser::id_type(), "INPUT");
}

// ========================================================================
// Combined Annotation Tests / 组合注解测试
// ========================================================================

#[test]
fn test_combined_annotations() {
    // Test combining multiple annotations
    // 测试组合多个注解

    #[Entity]
    #[Table(name = "users")]
    struct User {
        #[Id]
        #[GeneratedValue(strategy = "AUTO")]
        #[Column(name = "id")]
        id: i64,

        #[Column(name = "username", nullable = false, unique = true)]
        username: String,

        #[Column(name = "email")]
        email: String,
    }

    // Verify all annotations work together
    // 验证所有注解协同工作
    assert_eq!(User::table_name(), "users");
}
