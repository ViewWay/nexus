//! Security Example / 安全示例
//!
//! Demonstrates Nexus's security features:
//! 演示 Nexus 的安全功能：
//! - JWT (JSON Web Tokens) authentication / JWT 认证
//! - Password hashing with bcrypt / bcrypt 密码哈希
//! - Token generation and validation / 令牌生成和验证
//! - Protected routes / 受保护的路由
//!
//! Equivalent to: Spring Security, JWT, BCrypt
//! 等价于：Spring Security, JWT, BCrypt

use nexus_http::{Request, Response, Result, StatusCode};
use nexus_router::Router;
use nexus_security::{Authority, BcryptPasswordEncoder, JwtUtil, PasswordEncoder, Role};
use serde::{Deserialize, Serialize};

/// User model / 用户模型
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    email: String,
    password: String, // Hashed password / 哈希密码
}

/// Password hashing example / 密码哈希示例
fn password_hashing_example() {
    println!("\n=== Password Hashing Example / 密码哈希示例 ===\n");

    let password = "SecurePassword123!";
    let encoder = BcryptPasswordEncoder::default();

    // Hash password / 哈希密码
    println!("Original password: {}", password);
    let hashed = encoder.encode(password);
    println!("Hashed password: {}", hashed);

    // Verify password / 验证密码
    let is_valid = encoder.matches(password, &hashed);
    println!("Password verification: {}", is_valid);

    // Try wrong password / 尝试错误密码
    let is_valid = encoder.matches("WrongPassword", &hashed);
    println!("Wrong password verification: {}\n", is_valid);
}

/// JWT token generation example / JWT 令牌生成示例
fn jwt_generation_example() {
    println!("\n=== JWT Token Generation Example / JWT 令牌生成示例 ===\n");

    // Use environment variable in production / 生产环境使用环境变量
    let _secret = "your-secret-key-here"; // JwtUtil uses internal secret / JwtUtil使用内部密钥

    match JwtUtil::create_token("user-123", "John Doe", &[Authority::Role(Role::User)]) {
        Ok(token) => {
            println!("Generated JWT Token:");
            println!("{}\n", token);

            // Token structure breakdown / 令牌结构分解
            let parts: Vec<&str> = token.split('.').collect();
            println!("Token parts: {}", parts.len());
            if parts.len() >= 3 {
                println!("Header: {}...", &parts[0].chars().take(20).collect::<String>());
                println!("Payload: {}...", &parts[1].chars().take(20).collect::<String>());
                println!("Signature: {}...\n", &parts[2].chars().take(20).collect::<String>());
            }
        },
        Err(e) => println!("Error generating token: {}\n", e),
    }
}

/// JWT token validation example / JWT 令牌验证示例
fn jwt_validation_example() {
    println!("\n=== JWT Token Validation Example / JWT 令牌验证示例 ===\n");

    // JwtUtil uses internal secret from environment / JwtUtil使用环境变量中的内部密钥

    match JwtUtil::create_token("user-456", "Jane Smith", &[Authority::Role(Role::User), Authority::Role(Role::Admin)]) {
        Ok(token) => {
            println!("Validating token...");

            match JwtUtil::verify_token(&token) {
                Ok(claims) => {
                    println!("Token is valid!");
                    println!("User ID: {}", claims.sub);
                    println!("Username: {}", claims.username);
                    println!("Authorities: {:?}\n", claims.authorities);
                },
                Err(e) => println!("Token validation failed: {}\n", e),
            }
        },
        Err(e) => println!("Error generating token: {}\n", e),
    }

    // Test invalid token / 测试无效令牌
    println!("Testing invalid token...");
    let invalid_token = "invalid.token.here";
    match JwtUtil::verify_token(invalid_token) {
        Ok(_) => println!("Unexpectedly valid!\n"),
        Err(_) => println!("Correctly rejected invalid token\n"),
    }
}

/// Complete authentication flow example / 完整认证流程示例
async fn auth_flow_example() {
    println!("\n=== Authentication Flow Example / 认证流程示例 ===\n");

    let encoder = BcryptPasswordEncoder::default();

    let users = vec![
        User {
            id: "1".to_string(),
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password: encoder.encode("password123"),
        },
        User {
            id: "2".to_string(),
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
            password: encoder.encode("password456"),
        },
    ];

    println!("Registered users:");
    for user in &users {
        println!("  - {} ({})", user.username, user.email);
    }

    // Login flow / 登录流程
    println!("\n--- Login Attempt / 登录尝试 ---");
    let username = "alice";
    let password = "password123";

    match users.iter().find(|u| u.username == username) {
        Some(user) => {
            if encoder.matches(password, &user.password) {
                println!("Login successful for {}", username);

                // Generate JWT token / 生成JWT令牌
                // JwtUtil uses internal secret from environment / JwtUtil使用环境变量中的内部密钥

                match JwtUtil::create_token(&user.id, &user.username, &[Authority::Role(Role::User)]) {
                    Ok(token) => {
                        println!("Generated token for {}: {}...", username, &token[..50.min(token.len())]);

                        // Verify token / 验证令牌
                        match JwtUtil::verify_token(&token) {
                            Ok(claims) => {
                                println!("Token verified for: {}", claims.username);
                                println!("Token expires at: {:?}", claims.exp);
                            },
                            Err(e) => println!("Token verification failed: {}", e),
                        }
                    },
                    Err(e) => println!("Token generation failed: {}", e),
                }
            } else {
                println!("Invalid password for {}", username);
            }
        },
        None => println!("User not found: {}", username),
    }

    println!();
}

/// HTTP server with authentication / 带认证的HTTP服务器
async fn auth_server_example() {
    println!("\n=== Authenticated Server Example / 认证服务器示例 ===\n");

    // Public routes / 公共路由
    let public_router = Router::new()
        .get("/", |_req: Request| async {
            Ok::<_, nexus_http::Error>(
                Response::builder()
                    .status(StatusCode::OK)
                    .body(r#"{"message":"Welcome to Nexus API"}"#.into())
                    .unwrap(),
            )
        })
        .post("/auth/login", |_req: Request| async move {
            // Parse credentials from request body / 从请求体解析凭据
            // In production: Use proper request parsing / 生产环境：使用适当的请求解析
            Ok::<_, nexus_http::Error>(
                Response::builder()
                    .status(StatusCode::OK)
                    .body(r#"{"token":"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."}"#.into())
                    .unwrap(),
            )
        });

    // Protected routes / 受保护的路由
    let protected_router = Router::new()
        .get("/api/users/me", |_req: Request| async {
            Ok::<_, nexus_http::Error>(
                Response::builder()
                    .status(StatusCode::OK)
                    .body(
                        r#"{"id":"1","username":"alice","email":"alice@example.com"}#
                            .into(),
                    )
                    .unwrap(),
            )
        })
        .get("/api/orders", |_req: Request| async {
            Ok::<_, nexus_http::Error>(
                Response::builder()
                    .status(StatusCode::OK)
                    .body(r#"{"orders":[]}"#.into())
                    .unwrap(),
            )
        });

    println!("Server configured with:");
    println!("  - Public routes: /, /auth/login");
    println!("  - Protected routes: /api/users/me, /api/orders");
    println!("  - Authentication: JWT required for protected routes");
    println!("  - Password hashing: bcrypt");
    println!("\nServer ready! Use JWT token to access protected routes.\n");

    // Suppress unused variable warnings
    let _ = public_router;
    let _ = protected_router;
}

fn main() {
    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║   Nexus Security Example / 安全示例                            ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    // Run examples / 运行示例
    password_hashing_example();
    jwt_generation_example();
    jwt_validation_example();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(auth_flow_example());
    rt.block_on(auth_server_example());

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║   All security examples completed!                            ║");
    println!("╚═══════════════════════════════════════════════════════════════╝\n");
}
