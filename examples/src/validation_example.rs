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

use nexus_http::{Request, Response, StatusCode};
use nexus_router::Router;
use nexus_validation::{
    validator::{ValidationError, Validator},
    rules::{email::EmailRule, length::LengthRule, range::RangeRule},
};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// User registration request / 用户注册请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct RegisterUserRequest {
    #[validate(length(min = 3, max = 50), message = "Username must be 3-50 characters")]
    username: String,

    #[validate(email(message = "Invalid email format"))]
    email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(custom(function = "validate_password_strength"))]
    password: String,

    #[validate(range(min = 18, max = 120, message = "Age must be between 18 and 120"))]
    age: u8,
}

/// Custom password strength validator / 自定义密码强度验证器
fn validate_password_strength(password: &str) -> Result<(), validator::ValidationError> {
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));

    if has_upper && has_lower && has_digit && has_special {
        Ok(())
    } else {
        Err(validator::ValidationError::new("password_strength"))
    }
}

/// Product creation request / 产品创建请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct CreateProductRequest {
    #[validate(length(min = 1, max = 100))]
    name: String,

    #[validate(length(min = 10, max = 5000))]
    description: String,

    #[validate(range(min = 0.01))]
    price: f64,

    #[validate(range(min = 0))]
    stock: u32,
}

/// Order creation request / 订单创建请求
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct CreateOrderRequest {
    #[validate(email)]
    customer_email: String,

    #[validate(length(min = 1))]
    items: Vec<OrderItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct OrderItem {
    product_id: String,

    #[validate(range(min = 1))]
    quantity: u32,
}

/// Manual validation example / 手动验证示例
fn manual_validation_example() {
    println!("\n=== Manual Validation Example / 手动验证示例 ===\n");

    // Valid request / 有效请求
    println!("--- Testing Valid Request / 测试有效请求 ---");
    let valid_request = RegisterUserRequest {
        username: "alice".to_string(),
        email: "alice@example.com".to_string(),
        password: "SecurePass123!".to_string(),
        age: 25,
    };

    match valid_request.validate() {
        Ok(_) => println!("✓ Valid request"),
        Err(e) => println!("✗ Validation errors: {:?}", e),
    }

    // Invalid request / 无效请求
    println!("\n--- Testing Invalid Request / 测试无效请求 ---");
    let invalid_request = RegisterUserRequest {
        username: "ab".to_string(),        // Too short / 太短
        email: "invalid-email".to_string(), // Invalid email / 无效邮箱
        password: "weak".to_string(),       // Too weak / 太弱
        age: 15,                            // Too young / 太年轻
    };

    match invalid_request.validate() {
        Ok(_) => println!("✓ Valid request"),
        Err(errors) => {
            println!("✗ Validation errors:");
            for error in errors.field_errors() {
                println!("  - {}: {:?}", error.0, error.1);
            }
        }
    }

    println!();
}

/// Request validation in HTTP handlers / HTTP处理程序中的请求验证
async fn validate_request_handler(req: RegisterUserRequest) -> Response {
    match req.validate() {
        Ok(_) => {
            Response::builder()
                .status(StatusCode::OK)
                .body(
                    serde_json::json!({
                        "message": "User registered successfully",
                        "username": req.username
                    })
                    .to_string()
                    .into(),
                )
                .unwrap()
        }
        Err(errors) => {
            Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(
                    serde_json::json!({
                        "error": "Validation failed",
                        "details": errors.to_string()
                    })
                    .to_string()
                    .into(),
                )
                .unwrap()
        }
    }
}

/// Product validation example / 产品验证示例
async fn product_validation_example() {
    println!("\n=== Product Validation Example / 产品验证示例 ===\n");

    // Valid product / 有效产品
    let valid_product = CreateProductRequest {
        name: "Laptop".to_string(),
        description: "High-performance laptop with 16GB RAM".to_string(),
        price: 999.99,
        stock: 100,
    };

    match valid_product.validate() {
        Ok(_) => println!("✓ Valid product: {}", valid_product.name),
        Err(e) => println!("✗ Validation errors: {:?}", e),
    }

    // Invalid product / 无效产品
    let invalid_product = CreateProductRequest {
        name: "".to_string(),        // Empty name / 空名称
        description: "Short".to_string(), // Too short / 太短
        price: -10.0,                // Negative price / 负价格
        stock: 0,
    };

    match invalid_product.validate() {
        Ok(_) => println!("✓ Valid product"),
        Err(errors) => {
            println!("✗ Invalid product:");
            for error in errors.field_errors() {
                for field_error in error.1 {
                    if let Some(message) = field_error.message {
                        println!("  - {}: {}", error.0, message);
                    }
                }
            }
        }
    }

    println!();
}

/// Order validation example / 订单验证示例
async fn order_validation_example() {
    println!("\n=== Order Validation Example / 订单验证示例 ===\n");

    // Valid order / 有效订单
    let valid_order = CreateOrderRequest {
        customer_email: "customer@example.com".to_string(),
        items: vec![
            OrderItem {
                product_id: "prod-1".to_string(),
                quantity: 2,
            },
            OrderItem {
                product_id: "prod-2".to_string(),
                quantity: 1,
            },
        ],
    };

    match valid_order.validate() {
        Ok(_) => println!("✓ Valid order with {} items", valid_order.items.len()),
        Err(e) => println!("✗ Validation errors: {:?}", e),
    }

    // Invalid order / 无效订单
    let invalid_order = CreateOrderRequest {
        customer_email: "not-an-email".to_string(),
        items: vec![], // Empty items / 空商品
    };

    match invalid_order.validate() {
        Ok(_) => println!("✓ Valid order"),
        Err(errors) => {
            println!("✗ Invalid order:");
            for error in errors.field_errors() {
                for field_error in error.1 {
                    if let Some(message) = field_error.message {
                        println!("  - {}: {}", error.0, message);
                    }
                }
            }
        }
    }

    println!();
}

/// HTTP server with validation / 带验证的HTTP服务器
async fn validation_server_example() {
    println!("\n=== Validation Server Example / 验证服务器示例 ===\n");

    let app = Router::new()
        // User registration / 用户注册
        .post("/api/users/register", |req: RegisterUserRequest| async move {
            validate_request_handler(req).await
        })
        // Product creation / 产品创建
        .post("/api/products", |req: CreateProductRequest| async move {
            match req.validate() {
                Ok(_) => Response::builder()
                    .status(StatusCode::CREATED)
                    .body(
                        serde_json::json!({
                            "message": "Product created",
                            "product": req
                        })
                        .to_string()
                        .into(),
                    )
                    .unwrap(),
                Err(e) => Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(serde_json::json!({"error": e.to_string()}).to_string().into())
                    .unwrap(),
            }
        })
        // Order creation / 订单创建
        .post("/api/orders", |req: CreateOrderRequest| async move {
            match req.validate() {
                Ok(_) => Response::builder()
                    .status(StatusCode::CREATED)
                    .body(
                        serde_json::json!({
                            "message": "Order created",
                            "order_id": ulid::Ulid::new().to_string()
                        })
                        .to_string()
                        .into(),
                    )
                    .unwrap(),
                Err(e) => Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(serde_json::json!({"error": e.to_string()}).to_string().into())
                    .unwrap(),
            }
        });

    println!("Server configured with validation on:");
    println!("  POST /api/users/register - User registration");
    println!("  POST /api/products - Product creation");
    println!("  POST /api/orders - Order creation");
    println!("\nAll endpoints validate requests using validator derive!");
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
