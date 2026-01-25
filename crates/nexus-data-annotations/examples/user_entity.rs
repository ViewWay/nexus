//! Spring Data + MyBatis-Plus Style Example / Spring Data + MyBatis-Plus 风格示例
//!
//! This example demonstrates combining Spring Data JPA annotations with MyBatis-Plus patterns
//! 此示例演示了如何将 Spring Data JPA 注解与 MyBatis-Plus 模式结合使用

use nexus_data_annotations::{Entity, Table, Id, GeneratedValue, Column, Query, Insert, Update, Delete};
use nexus_lombok::Data;
use serde::{Serialize, Deserialize};

// ============================================================================
// Example 1: Basic Entity with Spring Data Annotations / 带有 Spring Data 注解的基本实体
// ============================================================================

/// User entity with Spring Data JPA style annotations
/// 使用 Spring Data JPA 风格注解的用户实体
#[Entity]
#[Table(name = "users")]
#[Data]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Primary key with auto-increment strategy
    /// 带有自增策略的主键
    #[Id]
    #[GeneratedValue(strategy = "AUTO")]
    #[Column(name = "id")]
    pub id: i64,

    /// Username with unique constraint
    /// 带有唯一约束的用户名
    #[Column(name = "username", nullable = false, unique = true)]
    pub username: String,

    /// Email address
    /// 电子邮件地址
    #[Column(name = "email", nullable = false)]
    pub email: String,

    /// User age
    /// 用户年龄
    #[Column(name = "age")]
    pub age: i32,
}

#[allow(dead_code)]
fn example_basic_entity() {
    // Create a new user using Lombok @Data generated constructor
    // 使用 Lombok @Data 生成的构造函数创建新用户
    let user = User::new(0, "alice".into(), "alice@example.com".into(), 25);

    // Access table name
    // 访问表名
    println!("Table name: {}", User::table_name());

    // Getters (from @Data)
    println!("Username: {}", user.username());
    println!("Email: {}", user.email());

    // Setters (from @Data)
    user.set_age(26);

    // With methods (from @Data) - creates modified copy
    // With 方法（来自 @Data）- 创建修改后的副本
    let user2 = user.with_age(27);
    println!("User2 age: {}", user2.age());
}

// ============================================================================
// Example 2: Repository with Custom Queries / 带有自定义查询的 Repository
// ============================================================================

/// User repository with custom SQL queries (similar to MyBatis-Plus Mapper)
/// 带有自定义 SQL 查询的用户 repository（类似 MyBatis-Plus Mapper）
#[allow(dead_code)]
trait UserRepository {
    /// Find user by ID
    /// 通过 ID 查找用户
    #[Query("SELECT * FROM users WHERE id = :id")]
    async fn find_by_id(&self, id: i64) -> Option<User>;

    /// Find user by username
    /// 通过用户名查找用户
    #[Query("SELECT * FROM users WHERE username = :username")]
    async fn find_by_username(&self, username: &str) -> Option<User>;

    /// Find users by age range
    /// 通过年龄范围查找用户
    #[Query("SELECT * FROM users WHERE age >= :min_age AND age <= :max_age")]
    async fn find_by_age_range(&self, min_age: i32, max_age: i32) -> Vec<User>;

    /// Search users by username pattern (LIKE query)
    /// 通过用户名模式搜索用户（LIKE 查询）
    #[Query("SELECT * FROM users WHERE username LIKE :pattern%")]
    async fn search_by_username(&self, pattern: &str) -> Vec<User>;

    /// Insert new user
    /// 插入新用户
    #[Insert("INSERT INTO users (username, email, age) VALUES (:username, :email, :age)")]
    async fn insert_user(&self, username: &str, email: &str, age: i32) -> Result<u64, String>;

    /// Update user email
    /// 更新用户电子邮件
    #[Update("UPDATE users SET email = :email WHERE id = :id")]
    async fn update_email(&self, id: i64, email: &str) -> Result<u64, String>;

    /// Update user age
    /// 更新用户年龄
    #[Update("UPDATE users SET age = :age WHERE id = :id")]
    async fn update_age(&self, id: i64, age: i32) -> Result<u64, String>;

    /// Delete user by ID
    /// 通过 ID 删除用户
    #[Delete("DELETE FROM users WHERE id = :id")]
    async fn delete_by_id(&self, id: i64) -> Result<u64, String>;

    /// Delete users by age range
    /// 通过年龄范围删除用户
    #[Delete("DELETE FROM users WHERE age < :min_age")]
    async fn delete_by_age_less_than(&self, min_age: i32) -> Result<u64, String>;
}

// ============================================================================
// Example 3: Complex Entity with Relations / 带有关系的复杂实体
// ============================================================================

/// Order entity
/// 订单实体
#[Entity]
#[Table(name = "orders")]
#[Data]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    #[Id]
    #[GeneratedValue(strategy = "AUTO")]
    #[Column(name = "id")]
    pub id: i64,

    #[Column(name = "user_id", nullable = false)]
    pub user_id: i64,

    #[Column(name = "total_amount", nullable = false)]
    pub total_amount: f64,

    #[Column(name = "status")]
    pub status: String,
}

/// Order repository
/// 订单 repository
#[allow(dead_code)]
trait OrderRepository {
    /// Find all orders for a user
    /// 查找用户的所有订单
    #[Query("SELECT * FROM orders WHERE user_id = :user_id")]
    async fn find_by_user_id(&self, user_id: i64) -> Vec<Order>;

    /// Find orders by status
    /// 通过状态查找订单
    #[Query("SELECT * FROM orders WHERE status = :status")]
    async fn find_by_status(&self, status: &str) -> Vec<Order>;

    /// Find orders by total amount range
    /// 通过总金额范围查找订单
    #[Query("SELECT * FROM orders WHERE total_amount >= :min AND total_amount <= :max")]
    async fn find_by_amount_range(&self, min: f64, max: f64) -> Vec<Order>;

    /// Count orders by user
    /// 统计用户订单数
    #[Query("SELECT COUNT(*) FROM orders WHERE user_id = :user_id")]
    async fn count_by_user_id(&self, user_id: i64) -> i64;

    /// Calculate total sales for a user
    /// 计算用户总销售额
    #[Query("SELECT SUM(total_amount) FROM orders WHERE user_id = :user_id AND status = 'completed'")]
    async fn total_sales_by_user(&self, user_id: i64) -> Option<f64>;
}

// ============================================================================
// Example 4: Batch Operations / 批量操作
// ============================================================================

/// Product entity
/// 产品实体
#[Entity]
#[Table(name = "products")]
#[Data]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    #[Id]
    #[GeneratedValue(strategy = "AUTO")]
    #[Column(name = "id")]
    pub id: i64,

    #[Column(name = "name", nullable = false)]
    pub name: String,

    #[Column(name = "price", nullable = false)]
    pub price: f64,

    #[Column(name = "stock")]
    pub stock: i32,
}

/// Product repository
/// 产品 repository
#[allow(dead_code)]
trait ProductRepository {
    /// Batch insert products
    /// 批量插入产品
    #[Insert("INSERT INTO products (name, price, stock) VALUES (:name, :price, :stock)")]
    async fn insert_product(&self, name: &str, price: f64, stock: i32) -> Result<u64, String>;

    /// Update product stock
    /// 更新产品库存
    #[Update("UPDATE products SET stock = stock + :delta WHERE id = :id")]
    async fn update_stock(&self, id: i64, delta: i32) -> Result<u64, String>;

    /// Update multiple product prices
    /// 批量更新产品价格
    #[Update("UPDATE products SET price = price * :multiplier WHERE category = :category")]
    async fn update_prices_by_category(&self, category: &str, multiplier: f64) -> Result<u64, String>;
}

// ============================================================================
// Example 5: Transaction Operations / 事务操作
// ============================================================================

/// Account entity
/// 账户实体
#[Entity]
#[Table(name = "accounts")]
#[Data]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    #[Id]
    #[GeneratedValue(strategy = "AUTO")]
    #[Column(name = "id")]
    pub id: i64,

    #[Column(name = "account_number", unique = true, nullable = false)]
    pub account_number: String,

    #[Column(name = "balance", nullable = false)]
    pub balance: f64,
}

/// Account repository
/// 账户 repository
#[allow(dead_code)]
trait AccountRepository {
    /// Debit from account
    /// 从账户借记
    #[Update("UPDATE accounts SET balance = balance - :amount WHERE id = :id AND balance >= :amount")]
    async fn debit(&self, id: i64, amount: f64) -> Result<u64, String>;

    /// Credit to account
    /// 向账户贷记
    #[Update("UPDATE accounts SET balance = balance + :amount WHERE id = :id")]
    async fn credit(&self, id: i64, amount: f64) -> Result<u64, String>;

    /// Transfer between accounts (would use transaction in real implementation)
    /// 账户间转账（实际实现中会使用事务）
    #[Update("UPDATE accounts SET balance = balance - :amount WHERE id = :from_id AND balance >= :amount")]
    async fn transfer_debit(&self, from_id: i64, amount: f64) -> Result<u64, String>;

    #[Update("UPDATE accounts SET balance = balance + :amount WHERE id = :to_id")]
    async fn transfer_credit(&self, to_id: i64, amount: f64) -> Result<u64, String>;
}

fn main() {
    println!("=== Example 1: Basic Entity ===");
    example_basic_entity();

    println!("\n=== Spring Data + MyBatis-Plus Style ===");
    println!("Combine @Entity, @Table with @Data for powerful ORM-like experience");
    println!("结合 @Entity, @Table 与 @Data，获得强大的 ORM 体验");

    println!("\n=== Query Examples ===");
    println!("@Query(\"SELECT * FROM users WHERE id = :id\")");
    println!("@Query(\"SELECT * FROM users WHERE username LIKE :pattern%\")");
    println!("Supports: :param, #{param}, $1, $2 parameter styles");
    println!("支持: :param, #{param}, $1, $2 参数样式");

    println!("\n=== CRUD Operations ===");
    println!("@Insert - Custom INSERT queries");
    println!("@Update - Custom UPDATE queries");
    println!("@Delete - Custom DELETE queries");

    println!("\n=== Entity Features ===");
    println!("@Entity - Marks struct as JPA entity");
    println!("@Table(name = \"table_name\") - Specifies table mapping");
    println!("@Id - Marks primary key");
    println!("@GeneratedValue(strategy = \"AUTO\") - ID generation strategy");
    println!("@Column(name = \"col\", nullable = false) - Column mapping");
}
