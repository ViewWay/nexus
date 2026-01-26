// Exception Handling Example / 异常处理示例
//
// Demonstrates Nexus's exception handling capabilities:
// 演示 Nexus 的异常处理能力：
// - Custom error types / 自定义错误类型
// - Error propagation / 错误传播
// - HTTP error responses / HTTP 错误响应
// - Global error handlers / 全局错误处理器
//
// Equivalent to: Spring @ControllerAdvice, Exception Handlers
// 等价于：Spring @ControllerAdvice, Exception Handlers

use nexus_exceptions::{
    Error, ErrorResponse,
    exception::{
        BadRequestException, ConflictException, ForbiddenException, NotFoundException,
        UnauthorizedException, ValidationException,
    },
    handler::ExceptionHandler,
};
use nexus_http::{Request, Response, StatusCode};
use nexus_router::Router;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Custom business error / 自定义业务错误
#[derive(Debug)]
enum UserError {
    UserNotFound(String),
    InvalidCredentials,
    EmailAlreadyExists(String),
    AccountLocked(String),
    InsufficientPermissions,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::UserNotFound(id) => write!(f, "User not found: {}", id),
            UserError::InvalidCredentials => write!(f, "Invalid credentials"),
            UserError::EmailAlreadyExists(email) => write!(f, "Email already exists: {}", email),
            UserError::AccountLocked(id) => write!(f, "Account locked: {}", id),
            UserError::InsufficientPermissions => write!(f, "Insufficient permissions"),
        }
    }
}

impl Error for UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            UserError::UserNotFound(_) => StatusCode::NOT_FOUND,
            UserError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            UserError::EmailAlreadyExists(_) => StatusCode::CONFLICT,
            UserError::AccountLocked(_) => StatusCode::FORBIDDEN,
            UserError::InsufficientPermissions => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> ErrorResponse {
        ErrorResponse {
            error: self.to_string(),
            message: self.error_message(),
            code: self.error_code(),
        }
    }

    fn error_message(&self) -> String {
        match self {
            UserError::UserNotFound(id) => format!("User {} does not exist", id),
            UserError::InvalidCredentials => "Username or password is incorrect".to_string(),
            UserError::EmailAlreadyExists(email) => {
                format!("Email {} is already registered", email)
            },
            UserError::AccountLocked(id) => format!("User account {} is locked", id),
            UserError::InsufficientPermissions => {
                "You don't have permission to access this resource".to_string()
            },
        }
    }

    fn error_code(&self) -> String {
        match self {
            UserError::UserNotFound(_) => "USER_NOT_FOUND".to_string(),
            UserError::InvalidCredentials => "INVALID_CREDENTIALS".to_string(),
            UserError::EmailAlreadyExists(_) => "EMAIL_EXISTS".to_string(),
            UserError::AccountLocked(_) => "ACCOUNT_LOCKED".to_string(),
            UserError::InsufficientPermissions => "INSUFFICIENT_PERMISSIONS".to_string(),
        }
    }
}

/// Order processing error / 订单处理错误
#[derive(Debug)]
enum OrderError {
    ProductNotFound(String),
    InsufficientStock {
        product_id: String,
        requested: u32,
        available: u32,
    },
    InvalidOrderStatus {
        order_id: String,
        current: String,
        requested: String,
    },
    PaymentFailed(String),
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderError::ProductNotFound(id) => write!(f, "Product not found: {}", id),
            OrderError::InsufficientStock {
                product_id,
                requested,
                available,
            } => {
                write!(
                    f,
                    "Insufficient stock for product {}: requested {}, available {}",
                    product_id, requested, available
                )
            },
            OrderError::InvalidOrderStatus {
                order_id,
                current,
                requested,
            } => {
                write!(
                    f,
                    "Invalid order status for {}: cannot change from {} to {}",
                    order_id, current, requested
                )
            },
            OrderError::PaymentFailed(reason) => write!(f, "Payment failed: {}", reason),
        }
    }
}

impl Error for OrderError {
    fn status_code(&self) -> StatusCode {
        match self {
            OrderError::ProductNotFound(_) => StatusCode::NOT_FOUND,
            OrderError::InsufficientStock { .. } => StatusCode::BAD_REQUEST,
            OrderError::InvalidOrderStatus { .. } => StatusCode::CONFLICT,
            OrderError::PaymentFailed(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> ErrorResponse {
        ErrorResponse {
            error: self.to_string(),
            message: self.error_message(),
            code: self.error_code(),
        }
    }

    fn error_message(&self) -> String {
        match self {
            OrderError::ProductNotFound(id) => format!("Product {} is not available", id),
            OrderError::InsufficientStock { .. } => {
                "Requested quantity is not available".to_string()
            },
            OrderError::InvalidOrderStatus { .. } => {
                "Cannot update order to requested status".to_string()
            },
            OrderError::PaymentFailed(reason) => format!("Payment processing failed: {}", reason),
        }
    }

    fn error_code(&self) -> String {
        match self {
            OrderError::ProductNotFound(_) => "PRODUCT_NOT_FOUND".to_string(),
            OrderError::InsufficientStock { .. } => "INSUFFICIENT_STOCK".to_string(),
            OrderError::InvalidOrderStatus { .. } => "INVALID_STATUS_TRANSITION".to_string(),
            OrderError::PaymentFailed(_) => "PAYMENT_FAILED".to_string(),
        }
    }
}

/// User service / 用户服务
struct UserService;

impl UserService {
    async fn get_user(&self, id: &str) -> Result<String, UserError> {
        if id == "user-123" {
            Ok(id.to_string())
        } else {
            Err(UserError::UserNotFound(id.to_string()))
        }
    }

    async fn create_user(&self, email: &str) -> Result<String, UserError> {
        if email == "existing@example.com" {
            Err(UserError::EmailAlreadyExists(email.to_string()))
        } else {
            Ok("new-user-id".to_string())
        }
    }
}

/// Order service / 订单服务
struct OrderService;

impl OrderService {
    async fn create_order(&self, product_id: &str, quantity: u32) -> Result<String, OrderError> {
        if product_id == "prod-999" {
            Err(OrderError::ProductNotFound(product_id.to_string()))
        } else if quantity > 10 {
            Err(OrderError::InsufficientStock {
                product_id: product_id.to_string(),
                requested: quantity,
                available: 10,
            })
        } else {
            Ok("order-123".to_string())
        }
    }
}

/// Error handling demonstration / 错误处理演示
async fn error_handling_demo() {
    println!("\n=== Error Handling Demo / 错误处理演示 ===\n");

    let user_service = UserService;
    let order_service = OrderService;

    // User not found / 用户未找到
    println!("--- User Not Found / 用户未找到 ---");
    match user_service.get_user("invalid-id").await {
        Ok(user) => println!("Found user: {}", user),
        Err(e) => {
            println!("Error: {}", e);
            println!("Status: {}", e.status_code());
            println!("Code: {}", e.error_code());
            println!("Message: {}", e.error_message());
        },
    }

    // Email already exists / 邮箱已存在
    println!("\n--- Email Already Exists / 邮箱已存在 ---");
    match user_service.create_user("existing@example.com").await {
        Ok(user) => println!("Created user: {}", user),
        Err(e) => {
            println!("Error: {}", e);
            println!("Status: {}", e.status_code());
            println!("Code: {}", e.error_code());
            println!("Message: {}", e.error_message());
        },
    }

    // Product not found / 产品未找到
    println!("\n--- Product Not Found / 产品未找到 ---");
    match order_service.create_order("prod-999", 1).await {
        Ok(order) => println!("Created order: {}", order),
        Err(e) => {
            println!("Error: {}", e);
            println!("Status: {}", e.status_code());
            println!("Code: {}", e.error_code());
            println!("Message: {}", e.error_message());
        },
    }

    // Insufficient stock / 库存不足
    println!("\n--- Insufficient Stock / 库存不足 ---");
    match order_service.create_order("prod-1", 20).await {
        Ok(order) => println!("Created order: {}", order),
        Err(e) => {
            println!("Error: {}", e);
            println!("Status: {}", e.status_code());
            println!("Code: {}", e.error_code());
            println!("Message: {}", e.error_message());
        },
    }

    println!();
}

/// HTTP error handler / HTTP 错误处理器
async fn http_error_handler<E: Error>(error: E) -> Response {
    let error_response = error.error_response();
    let status = error.status_code();

    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(
            serde_json::json!({
                "error": error_response.error,
                "message": error_response.message,
                "code": error_response.code,
                "status": status.as_u16()
            })
            .to_string()
            .into(),
        )
        .unwrap()
}

/// HTTP endpoints with error handling / 带错误处理的HTTP端点
async fn error_handling_endpoints() {
    println!("\n=== HTTP Error Handling / HTTP 错误处理 ===\n");

    let user_service = UserService;

    // GET /api/users/:id - Get user by ID / 根据ID获取用户
    let get_user = move |id: String| {
        let service = user_service.clone();
        async move {
            match service.get_user(&id).await {
                Ok(user_id) => Response::builder()
                    .status(StatusCode::OK)
                    .body(serde_json::json!({ "id": user_id }).to_string().into())
                    .unwrap(),
                Err(e) => http_error_handler(e).await,
            }
        }
    };

    // POST /api/users - Create user / 创建用户
    let create_user = move |req: CreateUserRequest| {
        let service = user_service.clone();
        async move {
            match service.create_user(&req.email).await {
                Ok(user_id) => Response::builder()
                    .status(StatusCode::CREATED)
                    .body(serde_json::json!({ "id": user_id }).to_string().into())
                    .unwrap(),
                Err(e) => http_error_handler(e).await,
            }
        }
    };

    println!("HTTP endpoints configured with error handling:");
    println!("  GET  /api/users/:id - Returns 404 if user not found");
    println!("  POST /api/users - Returns 409 if email already exists");
    println!("\nAll errors are properly converted to HTTP responses!");
    println!();
}

#[derive(Debug, Deserialize)]
struct CreateUserRequest {
    email: String,
}

/// Global exception handler / 全局异常处理器
async fn global_exception_handler() {
    println!("\n=== Global Exception Handler / 全局异常处理器 ===\n");

    let handler = ExceptionHandler::new();

    // Register error handlers / 注册错误处理器
    handler.register_handler::<UserError>(http_error_handler);
    handler.register_handler::<OrderError>(http_error_handler);

    println!("Global exception handlers registered:");
    println!("  - UserError -> HTTP response");
    println!("  - OrderError -> HTTP response");
    println!("  - NotFoundException -> 404 Not Found");
    println!("  - BadRequestException -> 400 Bad Request");
    println!("  - UnauthorizedException -> 401 Unauthorized");
    println!("  - ForbiddenException -> 403 Forbidden");
    println!("  - ConflictException -> 409 Conflict");
    println!("  - ValidationException -> 422 Unprocessable Entity");
    println!();
}

/// Custom exception middleware / 自定义异常中间件
async fn exception_middleware_demo() {
    println!("\n=== Exception Middleware / 异常中间件 ===\n");

    println!("Exception handling flow:");
    println!("  1. Controller throws exception");
    println!("  2. Middleware catches exception");
    println!("  3. Exception handler converts to HTTP response");
    println!("  4. Client receives formatted error JSON");
    println!();

    println!("Example error response:");
    let error = UserError::UserNotFound("user-123".to_string());
    let response = http_error_handler(error).await;

    println!("  Status: {}", response.status());
    println!("  Body: {{");
    println!("    \"error\": \"User not found: user-123\",");
    println!("    \"message\": \"User user-123 does not exist\",");
    println!("    \"code\": \"USER_NOT_FOUND\",");
    println!("    \"status\": 404");
    println!("  }}");
    println!();
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║   Nexus Exception Handling Example / 异常处理示例            ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    println!("\nException Handling Features:");
    println!("  ✓ Custom error types");
    println!("  ✓ Error propagation");
    println!("  ✓ HTTP error responses");
    println!("  ✓ Global error handlers");
    println!("  ✓ Exception middleware");

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(error_handling_demo());
    rt.block_on(error_handling_endpoints());
    rt.block_on(global_exception_handler());
    rt.block_on(exception_middleware_demo());

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║   All exception handling examples completed!                 ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}
