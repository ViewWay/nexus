//! # Complete User Authentication Example with JWT
//! # 使用JWT的完整用户认证示例
//!
//! This example demonstrates a complete authentication system similar to Spring Security + JWT
//! 本示例演示类似 Spring Security + JWT 的完整认证系统
//!
//! ## Features / 功能
//!
//! - User registration / 用户注册
//! - User login with JWT token generation / 用户登录及JWT token生成
//! - Protected endpoint with JWT authentication / JWT认证的保护端点
//! - Get current user info / 获取当前用户信息
//!
//! ## Run Example / 运行示例
//!
//! ```bash
//! cargo run --example jwt_auth_example
//! ```
//!
//! ## Test with curl / 使用curl测试
//!
//! ```bash
//! # Register user / 注册用户
//! curl -X POST http://localhost:8080/api/auth/register \
//!   -H "Content-Type: application/json" \
//!   -d '{"username":"alice","password":"password123","email":"alice@example.com"}'
//!
//! # Login / 登录
//! curl -X POST http://localhost:8080/api/auth/login \
//!   -H "Content-Type: application/json" \
//!   -d '{"username":"alice","password":"password123"}'
//!
//! # Get user info (replace TOKEN with actual token) / 获取用户信息（替换TOKEN为实际token）
//! curl -X GET http://localhost:8080/api/users/me \
//!   -H "Authorization: Bearer TOKEN"
//! ```

use nexus_http::{Request, Response, StatusCode};
use nexus_middleware::{JwtAuthenticationMiddleware, JwtRequestExt};
use nexus_security::{
    Authority, Authentication, AuthenticationManager, PasswordEncoder,
    Role, SimpleAuthenticationManager, User, UserService, JwtUtil,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ========================================================================
// Data Models / 数据模型
// ========================================================================

/// User registration request
/// 用户注册请求
#[derive(Debug, Deserialize)]
struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

/// User login request
/// 用户登录请求
#[derive(Debug, Deserialize)]
struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Login response with JWT token
/// 登录响应（包含JWT token）
#[derive(Debug, Serialize)]
struct LoginResponse {
    pub token: String,
    pub user_id: String,
    pub username: String,
    pub authorities: Vec<String>,
}

/// User info response
/// 用户信息响应
#[derive(Debug, Serialize)]
struct UserInfoResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub authorities: Vec<String>,
}

/// API error response
/// API错误响应
#[derive(Debug, Serialize)]
struct ErrorResponse {
    pub error: String,
    pub message: String,
}

/// Custom password encoder using BCrypt
/// 使用BCrypt的自定义密码编码器
struct BcryptPasswordEncoder;

impl PasswordEncoder for BcryptPasswordEncoder {
    fn encode(&self, raw: &str) -> String {
        bcrypt::hash(raw, bcrypt::DEFAULT_COST).unwrap_or_else(|_| {
            // Fallback to simple hash if bcrypt fails
            format!("{:x}", md5::compute(raw))
        })
    }

    fn matches(&self, raw: &str, encoded: &str) -> bool {
        if let Ok(matches) = bcrypt::verify(raw, encoded) {
            return matches;
        }

        // Fallback comparison
        encoded == &format!("{:x}", md5::compute(raw))
    }
}

/// In-memory user service
/// 内存用户服务
struct InMemoryUserService {
    users: Arc<RwLock<HashMap<String, User>>>,
    id_counter: Arc<RwLock<i64>>,
}

impl InMemoryUserService {
    fn new() -> Self {
        let encoder = BcryptPasswordEncoder;

        // Create default admin user
        let admin_password = encoder.encode("admin123");
        let mut users = HashMap::new();

        users.insert(
            "admin".to_string(),
            User::with_roles("admin", &admin_password, &[Role::Admin])
                .add_authority(Authority::Permission("user:write".to_string()))
                .add_authority(Authority::Permission("user:read".to_string()))
        );

        Self {
            users: Arc::new(RwLock::new(users)),
            id_counter: Arc::new(RwLock::new(1)),
        }
    }

    async fn generate_id(&self) -> i64 {
        let mut counter = self.id_counter.write().await;
        *counter += 1;
        *counter
    }
}

#[async_trait::async_trait]
impl UserService for InMemoryUserService {
    async fn load_user_by_username(&self, username: &str) -> nexus_security::SecurityResult<Arc<dyn nexus_security::UserDetails>> {
        let users = self.users.read().await;
        users
            .get(username)
            .map(|u| Arc::new(u.clone()) as Arc<dyn nexus_security::UserDetails>)
            .ok_or_else(|| nexus_security::SecurityError::UserNotFound(username.to_string()))
    }

    async fn create_user(&self, user: User) -> nexus_security::SecurityResult<()> {
        let mut users = self.users.write().await;
        if users.contains_key(&user.username) {
            return Err(nexus_security::SecurityError::UserAlreadyExists(
                user.username,
            ));
        }
        users.insert(user.username.clone(), user);
        Ok(())
    }

    async fn update_user(&self, user: User) -> nexus_security::SecurityResult<()> {
        let mut users = self.users.write().await;
        if !users.contains_key(&user.username) {
            return Err(nexus_security::SecurityError::UserNotFound(
                user.username,
            ));
        }
        users.insert(user.username.clone(), user);
        Ok(())
    }

    async fn delete_user(&self, username: &str) -> nexus_security::SecurityResult<()> {
        let mut users = self.users.write().await;
        users
            .remove(username)
            .ok_or_else(|| nexus_security::SecurityError::UserNotFound(username.to_string()))?;
        Ok(())
    }

    async fn user_exists(&self, username: &str) -> bool {
        let users = self.users.read().await;
        users.contains_key(username)
    }
}

// ========================================================================
// Controller Layer / 控制器层
// ========================================================================

/// Auth controller
/// 认证控制器
struct AuthController {
    user_service: Arc<InMemoryUserService>,
    auth_manager: Arc<SimpleAuthenticationManager>,
    password_encoder: Arc<BcryptPasswordEncoder>,
}

impl AuthController {
    fn new(user_service: Arc<InMemoryUserService>) -> Self {
        let password_encoder = Arc::new(BcryptPasswordEncoder);
        let auth_manager = Arc::new(SimpleAuthenticationManager::new(
            user_service.clone(),
            password_encoder.clone(),
        ));

        Self {
            user_service,
            auth_manager,
            password_encoder,
        }
    }

    /// Register new user / 注册新用户
    ///
    /// # Spring Equivalent / Spring等价物
    ///
    /// ```java
    /// @PostMapping("/register")
    /// public ResponseEntity<?> registerUser(@RequestBody SignUpRequest request) {
    ///     if (userRepository.existsByUsername(request.getUsername())) {
    ///         return ResponseEntity.badRequest()
    ///             .body(new MessageResponse("Error: Username is already taken!"));
    ///     }
    ///
    ///     User user = new User(request.getUsername(),
    ///                          encoder.encode(request.getPassword()),
    ///                          request.getEmail());
    ///     userRepository.save(user);
    ///     return ResponseEntity.ok(new MessageResponse("User registered successfully!"));
    /// }
    /// ```
    async fn register(&self, req: RegisterRequest) -> Response {
        // Check if user already exists
        if self.user_service.user_exists(&req.username).await {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "USER_EXISTS".to_string(),
                    message: "Username is already taken".to_string(),
                }).unwrap_or_default())
                .unwrap_or_default();
        }

        // Encode password
        let encoded_password = self.password_encoder.encode(&req.password);

        // Create user with USER role
        let user = User::with_roles(&req.username, &encoded_password, &[Role::User])
            .add_authority(Authority::Permission("user:read".to_string()));

        // Save user
        if let Err(e) = self.user_service.create_user(user).await {
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "REGISTRATION_FAILED".to_string(),
                    message: format!("Failed to create user: {:?}", e),
                }).unwrap_or_default())
                .unwrap_or_default();
        }

        // Return success response
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(serde_json::json!({
                "message": "User registered successfully!",
                "username": req.username
            }).to_string())
            .unwrap_or_default()
    }

    /// User login / 用户登录
    ///
    /// # Spring Equivalent / Spring等价物
    ///
    /// ```java
    /// @PostMapping("/signin")
    /// public ResponseEntity<?> authenticateUser(@RequestBody LoginRequest loginRequest) {
    ///     Authentication authentication = authenticationManager.authenticate(
    ///         new UsernamePasswordAuthenticationToken(loginRequest.getUsername(),
    ///                                                   loginRequest.getPassword())
    ///     );
    ///
    ///     SecurityContextHolder.getContext().setAuthentication(authentication);
    ///     String jwt = jwtUtils.generateJwtToken(authentication);
    ///
    ///     UserDetailsImpl userDetails = (UserDetailsImpl) authentication.getPrincipal();
    ///     List<String> roles = userDetails.getAuthorities().stream()
    ///         .map(item -> item.getAuthority())
    ///         .collect(Collectors.toList());
    ///
    ///     return ResponseEntity.ok(new JwtResponse(jwt,
    ///                                              userDetails.getId(),
    ///                                              userDetails.getUsername(),
    ///                                              roles));
    /// }
    /// ```
    async fn login(&self, req: LoginRequest) -> Response {
        // Create authentication token
        let auth_token = Authentication::new(&req.username, &req.password);

        // Authenticate
        let authentication = match self.auth_manager.authenticate(auth_token).await {
            Ok(auth) => auth,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "INVALID_CREDENTIALS".to_string(),
                        message: "Invalid username or password".to_string(),
                    }).unwrap_or_default())
                    .unwrap_or_default();
            }
        };

        // Generate JWT token
        let token = match JwtUtil::create_token(
            &authentication.principal, // Use username as user_id for this example
            &authentication.principal,
            &authentication.authorities,
        ) {
            Ok(token) => token,
            Err(e) => {
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "TOKEN_GENERATION_FAILED".to_string(),
                        message: format!("Failed to generate token: {:?}", e),
                    }).unwrap_or_default())
                    .unwrap_or_default();
            }
        };

        // Return token
        let authorities: Vec<String> = authentication
            .authorities
            .iter()
            .map(|a| a.to_string())
            .collect();

        let response = LoginResponse {
            token,
            user_id: authentication.principal.clone(),
            username: authentication.principal,
            authorities,
        };

        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&response).unwrap_or_default())
            .unwrap_or_default()
    }
}

/// User controller / 用户控制器
struct UserController {
    user_service: Arc<InMemoryUserService>,
}

impl UserController {
    fn new(user_service: Arc<InMemoryUserService>) -> Self {
        Self { user_service }
    }

    /// Get current user info / 获取当前用户信息
    ///
    /// # Spring Equivalent / Spring等价物
    ///
    /// ```java
    /// @GetMapping("/user/me")
    /// @PreAuthorize("isAuthenticated()")
    /// public ResponseEntity<?> getCurrentUser(@RequestHeader("Authorization") String authHeader) {
    ///     String jwt = authHeader.substring(7);
    ///     String username = jwtUtils.getUserNameFromJwtToken(jwt);
    ///     UserDetailsImpl userDetails = (UserDetailsImpl) authentication.getPrincipal();
    ///
    ///     User user = userRepository.findById(userDetails.getId())
    ///         .orElseThrow(() -> new ResourceNotFoundException("User", "id", userDetails.getId()));
    ///
    ///     List<String> roles = userDetails.getAuthorities().stream()
    ///         .map(item -> item.getAuthority())
    ///         .collect(Collectors.toList());
    ///
    ///     return ResponseEntity.ok(new UserSummary(user.getId(), user.getUsername(), user.getEmail(), roles));
    /// }
    /// ```
    async fn get_current_user(&self, req: &Request) -> Response {
        // Get authentication from request (injected by middleware)
        let auth = match req.get_jwt_auth() {
            Some(auth) => auth,
            None => {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "UNAUTHORIZED".to_string(),
                        message: "Authentication required".to_string(),
                    }).unwrap_or_default())
                    .unwrap_or_default();
            }
        };

        // Load user from database
        let user = match self.user_service.load_user_by_username(&auth.username).await {
            Ok(user) => user,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "USER_NOT_FOUND".to_string(),
                        message: "User not found".to_string(),
                    }).unwrap_or_default())
                    .unwrap_or_default();
            }
        };

        // Return user info
        let authorities: Vec<String> = user.authorities()
            .iter()
            .map(|a| a.to_string())
            .collect();

        let response = UserInfoResponse {
            id: auth.user_id.clone(),
            username: auth.username.clone(),
            email: format!("{}@example.com", auth.username), // Mock email
            authorities,
        };

        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&response).unwrap_or_default())
            .unwrap_or_default()
    }

    /// Get all users (admin only)
    async fn get_all_users(&self, req: &Request) -> Response {
        // Check if user is admin
        let auth = match req.get_jwt_auth() {
            Some(auth) => auth,
            None => {
                return Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&ErrorResponse {
                        error: "UNAUTHORIZED".to_string(),
                        message: "Authentication required".to_string(),
                    }).unwrap_or_default())
                    .unwrap_or_default();
            }
        };

        if !auth.has_role(&Role::Admin) {
            return Response::builder()
                .status(StatusCode::FORBIDDEN)
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&ErrorResponse {
                    error: "FORBIDDEN".to_string(),
                    message: "Admin access required".to_string(),
                }).unwrap_or_default())
                .unwrap_or_default();
        }

        // Return mock list of users
        Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(serde_json::json!({
                "users": [
                    {
                        "id": "1",
                        "username": "admin",
                        "email": "admin@example.com",
                        "roles": ["ADMIN"]
                    },
                    {
                        "id": "2",
                        "username": auth.username,
                        "email": format!("{}@example.com", auth.username),
                        "roles": auth.authorities.iter().map(|a| a.to_string()).collect::<Vec<_>>()
                    }
                ]
            }).to_string())
            .unwrap_or_default()
    }
}

// ========================================================================
// Security Configuration / 安全配置
// ========================================================================

/// Security configuration (similar to Spring Security config)
/// 安全配置（类似Spring Security配置）
///
/// # Spring Equivalent / Spring等价物
///
/// ```java
/// @EnableWebSecurity
/// public class WebSecurityConfig extends WebSecurityConfigurerAdapter {
///     @Override
///     protected void configure(HttpSecurity http) throws Exception {
///         http.cors().and().csrf().disable()
///             .exceptionHandling().authenticationEntryPoint(unauthorizedHandler).and()
///             .sessionManagement().sessionCreationPolicy(SessionCreationPolicy.STATELESS).and()
///             .authorizeRequests()
///                 .antMatchers("/api/auth/**").permitAll()
///                 .antMatchers("/api/user/**").authenticated()
///                 .antMatchers("/api/admin/**").hasRole("ADMIN")
///                 .anyRequest().authenticated();
///
///         http.addFilterBefore(jwtAuthenticationFilter, UsernamePasswordAuthenticationFilter.class);
///     }
/// }
/// ```
struct SecurityConfig {
    pub jwt_middleware: Arc<JwtAuthenticationMiddleware>,
}

impl SecurityConfig {
    fn new() -> Self {
        let jwt_middleware = Arc::new(
            JwtAuthenticationMiddleware::new()
                .skip_path("/api/auth/login")
                .skip_path("/api/auth/register")
        );

        Self { jwt_middleware }
    }
}

// ========================================================================
// Main Application / 主应用
// ========================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║     JWT Authentication System / JWT 认证系统                    ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // Initialize services / 初始化服务
    let user_service = Arc::new(InMemoryUserService::new());
    let auth_controller = AuthController::new(user_service.clone());
    let user_controller = UserController::new(user_service);
    let security_config = SecurityConfig::new();

    println!("✅ Services initialized");
    println!("✅ JWT middleware configured");
    println!("\n📡 Server would start on http://localhost:8080\n");

    // ====================================================================
    // Simulate API calls / 模拟API调用
    // ====================================================================

    println!("═══════════════════════════════════════════════════════════════");
    println!("Scenario 1: Register new user / 场景 1：注册新用户");
    println!("═══════════════════════════════════════════════════════════════\n");

    let register_req = RegisterRequest {
        username: "alice".to_string(),
        password: "password123".to_string(),
        email: "alice@example.com".to_string(),
    };

    println!("POST /api/auth/register");
    println!("Body: {}", serde_json::to_string(&register_req).unwrap());
    println!();

    let response = auth_controller.register(register_req).await;
    println!("Response Status: {}", response.status());
    println!("Response Body: {}", String::from_utf8_lossy(response.body().as_ref()));
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("Scenario 2: Login with wrong password / 场景 2：使用错误密码登录");
    println!("═══════════════════════════════════════════════════════════════\n");

    let login_req_wrong = LoginRequest {
        username: "alice".to_string(),
        password: "wrongpassword".to_string(),
    };

    println!("POST /api/auth/login");
    println!("Body: {}", serde_json::to_string(&login_req_wrong).unwrap());
    println!();

    let response = auth_controller.login(login_req_wrong).await;
    println!("Response Status: {}", response.status());
    println!("Response Body: {}", String::from_utf8_lossy(response.body().as_ref()));
    println!();

    println!("═══════════════════════════════════════════════════════════════");
    println!("Scenario 3: Login with correct password / 场景 3：使用正确密码登录");
    println!("═══════════════════════════════════════════════════════════════\n");

    let login_req = LoginRequest {
        username: "alice".to_string(),
        password: "password123".to_string(),
    };

    println!("POST /api/auth/login");
    println!("Body: {}", serde_json::to_string(&login_req).unwrap());
    println!();

    let response = auth_controller.login(login_req).await;
    println!("Response Status: {}", response.status());

    let token = if response.status().is_success() {
        let body = String::from_utf8_lossy(response.body().as_ref());
        println!("Response Body: {}", body);

        // Parse token from response
        if let Ok(login_resp) = serde_json::from_str::<LoginResponse>(&body) {
            Some(login_resp.token)
        } else {
            None
        }
    } else {
        println!("Response Body: {}", String::from_utf8_lossy(response.body().as_ref()));
        None
    };
    println!();

    // ====================================================================
    // Scenario 4: Access protected endpoint without token
    // ====================================================================

    println!("═══════════════════════════════════════════════════════════════");
    println!("Scenario 4: Access protected endpoint WITHOUT token");
    println!("场景 4：不带token访问受保护端点");
    println!("═══════════════════════════════════════════════════════════════\n");

    let mut request = Request::builder()
        .uri("/api/users/me")
        .body("".to_string())
        .unwrap();

    let response = user_controller.get_current_user(&request).await;
    println!("GET /api/users/me (no token)");
    println!("Response Status: {}", response.status());
    println!("Response Body: {}", String::from_utf8_lossy(response.body().as_ref()));
    println!();

    // ====================================================================
    // Scenario 5: Access protected endpoint WITH token
    // ====================================================================

    if let Some(ref jwt_token) = token {
        println!("═══════════════════════════════════════════════════════════════");
        println!("Scenario 5: Access protected endpoint WITH token");
        println!("场景 5：带token访问受保护端点");
        println!("═══════════════════════════════════════════════════════════════\n");

        // Simulate JWT verification and add auth to request
        use nexus_security::JwtUtil;

        match JwtUtil::verify_token(jwt_token) {
            Ok(claims) => {
                println!("✅ Token verified successfully");
                println!("User: {}", claims.username);
                println!("Authorities: {:?}", claims.authorities);
                println!();

                // Create request with simulated authentication
                let mut request = Request::builder()
                    .uri("/api/users/me")
                    .header("Authorization", format!("Bearer {}", jwt_token))
                    .body("".to_string())
                    .unwrap();

                // Inject authentication into request extensions
                let auth = nexus_security::JwtAuthentication::from_claims(&claims);
                request.extensions_mut().insert(auth);

                let response = user_controller.get_current_user(&request).await;
                println!("GET /api/users/me (with token)");
                println!("Response Status: {}", response.status());
                println!("Response Body: {}", String::from_utf8_lossy(response.body().as_ref()));
            }
            Err(e) => {
                println!("❌ Token verification failed: {:?}", e);
            }
        }
        println!();

        // ====================================================================
        // Scenario 6: Admin tries to access all users endpoint
        // ====================================================================

        println!("═══════════════════════════════════════════════════════════════");
        println!("Scenario 6: Regular user tries to access admin endpoint");
        println!("场景 6：普通用户尝试访问管理员端点");
        println!("═══════════════════════════════════════════════════════════════\n");

        let mut request = Request::builder()
            .uri("/api/users/all")
            .header("Authorization", format!("Bearer {}", jwt_token))
            .body("".to_string())
            .unwrap();

        // Re-inject authentication
        let claims = JwtUtil::verify_token(jwt_token).unwrap();
        let auth = nexus_security::JwtAuthentication::from_claims(&claims);
        request.extensions_mut().insert(auth);

        let response = user_controller.get_all_users(&request).await;
        println!("GET /api/users/all (with user token)");
        println!("Response Status: {}", response.status());
        println!("Response Body: {}", String::from_utf8_lossy(response.body().as_ref()));
        println!();

        // ====================================================================
        // Scenario 7: Admin login and access admin endpoint
        // ====================================================================

        println!("═══════════════════════════════════════════════════════════════");
        println!("Scenario 7: Admin login and access admin endpoint");
        println!("场景 7：管理员登录并访问管理员端点");
        println!("═══════════════════════════════════════════════════════════════\n");

        let admin_login = LoginRequest {
            username: "admin".to_string(),
            password: "admin123".to_string(),
        };

        println!("POST /api/auth/login");
        println!("Body: {}", serde_json::to_string(&admin_login).unwrap());
        println!();

        let response = auth_controller.login(admin_login).await;
        println!("Response Status: {}", response.status());

        let admin_token = if response.status().is_success() {
            let body = String::from_utf8_lossy(response.body().as_ref());
            println!("Response Body: {}", body);

            if let Ok(login_resp) = serde_json::from_str::<LoginResponse>(&body) {
                println!("✅ Admin logged in successfully");
                Some(login_resp.token)
            } else {
                None
            }
        } else {
            println!("Response Body: {}", String::from_utf8_lossy(response.body().as_ref()));
            None
        };

        if let Some(ref admin_jwt) = admin_token {
            println!();

            let mut request = Request::builder()
                .uri("/api/users/all")
                .header("Authorization", format!("Bearer {}", admin_jwt))
                .body("".to_string())
                .unwrap();

            // Inject admin authentication
            let claims = JwtUtil::verify_token(admin_jwt).unwrap();
            let auth = nexus_security::JwtAuthentication::from_claims(&claims);
            request.extensions_mut().insert(auth);

            let response = user_controller.get_all_users(&request).await;
            println!("GET /api/users/all (with admin token)");
            println!("Response Status: {}", response.status());
            println!("Response Body: {}", String::from_utf8_lossy(response.body().as_ref()));
        }
    }

    println!();
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║              Examples completed! / 示例完成！                      ║");
    println!("╚════════════════════════════════════════════════════════════════╝");

    Ok(())
}
