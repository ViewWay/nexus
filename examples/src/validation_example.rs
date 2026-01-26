// Validation Example / 验证示例
//
// Demonstrates Nexus's validation features:
// 演示 Nexus 的验证功能：
// - Request validation / 请求验证
// - Field validation rules / 字段验证规则
// - Custom validators / 自定义验证器
// - Validation error handling / 验证错误处理
//
// Equivalent to: Spring Validation, Hibernate Validator, Bean Validation
// 等价于：Spring Validation, Hibernate Validator, Bean Validation

use nexus_http::{Request, Response, Result, StatusCode};
use nexus_router::Router;
// Note: Validation framework is under development
// The nexus_validation module structure is being finalized
// 注：验证框架正在开发中，nexus_validation 模块结构正在最终确定
use nexus_validation::{ValidationError, Validate};
use serde::{Deserialize, Serialize};

/// User registration request / 用户注册请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RegisterUserRequest {
    username: String,
    email: String,
    password: String,
    age: u8,
}

/// Product creation request / 产品创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateProductRequest {
    name: String,
    description: String,
    price: f64,
    stock: u32,
}

/// Order creation request / 订单创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateOrderRequest {
    customer_email: String,
    items: Vec<OrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OrderItem {
    product_id: String,
    quantity: u32,
}

/// Manual validation example / 手动验证示例
fn manual_validation_example() {
    println!("\n=== Manual Validation Example / 手动验证示例 ===\n");
    println!("Note: Automatic validation via derive macros is under development");
    println!("注：通过派生宏进行自动验证正在开发中");
    println!("\nPlanned features:");
    println!("  - #[validate] attribute for field-level validation");
    println!("  - Custom validator functions");
    println!("  - Nested object validation");
    println!();
}

/// Request validation in HTTP handlers / HTTP处理程序中的请求验证
async fn validate_request_handler(_req: RegisterUserRequest) -> Result<Response> {
    // TODO: Implement automatic request validation
    // TODO：实现自动请求验证
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(
            serde_json::json!({
                "message": "User registration - validation under development"
            })
            .to_string()
            .into(),
        )
        .unwrap())
}

/// Product validation example / 产品验证示例
async fn product_validation_example() {
    println!("\n=== Product Validation Example / 产品验证示例 ===\n");
    println!("Product validation is under development");
    println!("产品验证正在开发中");
    println!();
}

/// Order validation example / 订单验证示例
async fn order_validation_example() {
    println!("\n=== Order Validation Example / 订单验证示例 ===\n");
    println!("Order validation is under development");
    println!("订单验证正在开发中");
    println!();
}

/// HTTP server with validation / 带验证的HTTP服务器
async fn validation_server_example() {
    println!("\n=== Validation Server Example / 验证服务器示例 ===\n");

    // Note: The router now requires Request type in handlers
    // 注：路由器现在要求处理程序使用 Request 类型
    // TODO: Implement extractors for automatic request body deserialization
    // TODO：实现提取器以自动请求体反序列化

    println!("Server configuration skipped - awaiting extractor implementation");
    println!("跳过服务器配置 - 等待提取器实现");
    println!("\nPlanned endpoints:");
    println!("  POST /api/users/register - User registration");
    println!("  POST /api/products - Product creation");
    println!("  POST /api/orders - Order creation");
    println!("\nAll endpoints will validate requests using validator derive!");
    println!();
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║   Nexus Validation Example / 验证示例                          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    manual_validation_example();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(product_validation_example());
    rt.block_on(order_validation_example());
    rt.block_on(validation_server_example());

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║   All validation examples completed!                          ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}
