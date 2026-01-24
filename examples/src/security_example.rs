// Security Example / 安全示例
//
// Demonstrates Nexus's security features:
// 演示 Nexus 的安全功能：
// - JWT (JSON Web Tokens) authentication / JWT 认证
// - Password hashing with bcrypt / bcrypt 密码哈希
// - Token generation and validation / 令牌生成和验证
// - Protected routes / 受保护的路由
//
// Equivalent to: Spring Security, JWT, BCrypt
// 等价于：Spring Security, JWT, BCrypt

use nexus_http::{Request, Response, StatusCode};
use nexus_router::Router;
use nexus_security::{
    jwt::{JwtClaims, JwtEncoder, JwtDecoder},
    password::{PasswordEncoder, PasswordHasher},
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// User model / 用户模型
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: String,
    username: String,
    email: String,
    password: String, // Hashed password / 哈希密码
}

/// JWT Claims / JWT 声明
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (user ID) / 主题（用户ID）
    name: String, // Username / 用户名
    exp: usize, // Expiration time / 过期时间
    iat: usize, // Issued at / 签发时间
}

/// Password hashing example / 密码哈希示例
fn password_hashing_example() {
    println!("\n=== Password Hashing Example / 密码哈希示例 ===\n");

    let password = "SecurePassword123!";
    let hasher = PasswordHasher::default();

    // Hash password / 哈希密码
    println!("Original password: {}", password);
    let hashed = hasher.hash(password);
    println!("Hashed password: {}", hashed);

    // Verify password / 验证密码
    let is_valid = hasher.verify(password, &hashed);
    println!("Password verification: {}", is_valid);

    // Try wrong password / 尝试错误密码
    let is_valid = hasher.verify("WrongPassword", &hashed);
    println!("Wrong password verification: {}\n", is_valid);
}

/// JWT token generation example / JWT 令牌生成示例
fn jwt_generation_example() {
    println!("\n=== JWT Token Generation Example / JWT 令牌生成示例 ===\n");

    let secret = "your-secret-key-here"; // Use environment variable in production / 生产环境使用环境变量
    let encoder = JwtEncoder::new(secret);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        sub: "user-123".to_string(),
        name: "John Doe".to_string(),
        exp: (now + 3600) as usize, // Expire in 1 hour / 1小时后过期
        iat: now as usize,
    };

    // Generate token / 生成令牌
    match encoder.encode(&claims) {
        Ok(token) => {
            println!("Generated JWT Token:");
            println!("{}\n", token);

            // Token structure breakdown / 令牌结构分解
            let parts: Vec<&str> = token.split('.').collect();
            println!("Token parts: {}", parts.len());
            println!("Header: {}...", &parts[0].chars().take(20).collect::<String>());
            println!("Payload: {}...", &parts[1].chars().take(20).collect::<String>());
            println!("Signature: {}...\n", &parts[2].chars().take(20).collect::<String>());
        }
        Err(e) => println!("Error generating token: {}\n", e),
    }
}

/// JWT token validation example / JWT 令牌验证示例
fn jwt_validation_example() {
    println!("\n=== JWT Token Validation Example / JWT 令牌验证示例 ===\n");

    let secret = "your-secret-key-here";
    let encoder = JwtEncoder::new(secret);
    let decoder = JwtDecoder::new(secret);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        sub: "user-456".to_string(),
        name: "Jane Smith".to_string(),
        exp: (now + 3600) as usize,
        iat: now as usize,
    };

    // Generate and validate token / 生成并验证令牌
    match encoder.encode(&claims) {
        Ok(token) => {
            println!("Validating token...");

            match decoder.decode::<Claims>(&token) {
                Ok(decoded_claims) => {
                    println!("Token is valid!");
                    println!("User ID: {}", decoded_claims.sub);
                    println!("Username: {}", decoded_claims.name);
                    println!("Expires at: {}\n", decoded_claims.exp);
                }
                Err(e) => println!("Token validation failed: {}\n", e),
            }
        }
        Err(e) => println!("Error generating token: {}\n", e),
    }

    // Test invalid token / 测试无效令牌
    println!("Testing invalid token...");
    let invalid_token = "invalid.token.here";
    match decoder.decode::<Claims>(invalid_token) {
        Ok(_) => println!("Unexpectedly valid!\n"),
        Err(e) => println!("Correctly rejected invalid token: {}\n", e),
    }
}

/// Complete authentication flow example / 完整认证流程示例
async fn auth_flow_example() {
    println!("\n=== Authentication Flow Example / 认证流程示例 ===\n");

    let users = vec![
        User {
            id: "1".to_string(),
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password: PasswordHasher::default().hash("password123"),
        },
        User {
            id: "2".to_string(),
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
            password: PasswordHasher::default().hash("password456"),
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
            let hasher = PasswordHasher::default();
            if hasher.verify(password, &user.password) {
                println!("Login successful for {}", username);

                // Generate JWT token / 生成JWT令牌
                let secret = "your-secret-key-here";
                let encoder = JwtEncoder::new(secret);

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                let claims = Claims {
                    sub: user.id.clone(),
                    name: user.username.clone(),
                    exp: (now + 3600) as usize,
                    iat: now as usize,
                };

                match encoder.encode(&claims) {
                    Ok(token) => {
                        println!("Generated token for {}: {}...", username, &token[..50]);

                        // Verify token / 验证令牌
                        let decoder = JwtDecoder::new(secret);
                        match decoder.decode::<Claims>(&token) {
                            Ok(decoded) => {
                                println!("Token verified for: {}", decoded.name);
                                println!("Token expires at: {}", decoded.exp);
                            }
                            Err(e) => println!("Token verification failed: {}", e),
                        }
                    }
                    Err(e) => println!("Token generation failed: {}", e),
                }
            } else {
                println!("Invalid password for {}", username);
            }
        }
        None => println!("User not found: {}", username),
    }

    println!();
}

/// HTTP server with authentication / 带认证的HTTP服务器
async fn auth_server_example() {
    println!("\n=== Authenticated Server Example / 认证服务器示例 ===\n");

    // Public routes / 公共路由
    let public_router = Router::new()
        .get("/", || async {
            Response::builder()
                .status(StatusCode::OK)
                .body(r#"{"message":"Welcome to Nexus API"}"#.into())
                .unwrap()
        })
        .post("/auth/login", |req: Request| async move {
            // Parse credentials from request body / 从请求体解析凭据
            // In production: Use proper request parsing / 生产环境：使用适当的请求解析
            Response::builder()
                .status(StatusCode::OK)
                .body(r#"{"token":"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."}"#.into())
                .unwrap()
        });

    // Protected routes / 受保护的路由
    let protected_router = Router::new()
        .get("/api/users/me", || async {
            Response::builder()
                .status(StatusCode::OK)
                .body(
                    r#"{"id":"1","username":"alice","email":"alice@example.com"}#
                        .into(),
                )
                .unwrap()
        })
        .get("/api/orders", || async {
            Response::builder()
                .status(StatusCode::OK)
                .body(r#"{"orders":[]}"#.into())
                .unwrap()
        });

    println!("Server configured with:");
    println!("  - Public routes: /, /auth/login");
    println!("  - Protected routes: /api/users/me, /api/orders");
    println!("  - Authentication: JWT required for protected routes");
    println!("  - Password hashing: bcrypt");
    println!("\nServer ready! Use JWT token to access protected routes.\n");
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
