# nexus-extractors

[![Crates.io](https://img.shields.io/crates/v/nexus-extractors)](https://crates.io/crates/nexus-extractors)
[![Documentation](https://docs.rs/nexus-extractors/badge.svg)](https://docs.rs/nexus-extractors)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](../../LICENSE)

> Type-safe request data extractors for Nexus framework
> 
> Nexus框架的类型安全请求数据提取器

---

## 📋 Overview / 概述

`nexus-extractors` provides type-safe extractors for extracting data from HTTP requests, similar to Spring Boot's method parameter resolution.

`nexus-extractors` 提供类型安全的提取器，用于从HTTP请求中提取数据，类似于Spring Boot的方法参数解析。

**Key Features** / **核心特性**:
- ✅ **Type-safe** / **类型安全** - Compile-time guarantees
- ✅ **Zero-copy** / **零拷贝** - Efficient data extraction
- ✅ **Async** / **异步** - Non-blocking extraction
- ✅ **Spring-like** / **Spring风格** - Familiar API for Spring developers

---

## ✨ Extractors / 提取器

| Extractor | Spring Equivalent | Description | Status |
|-----------|------------------|-------------|--------|
| **Path<T>** | `@PathVariable` | Extract path parameters | ✅ |
| **Query<T>** | `@RequestParam` | Extract query parameters | ✅ |
| **Json<T>** | `@RequestBody` | Extract JSON body | ✅ |
| **Form<T>** | `@ModelAttribute` | Extract form data | ✅ |
| **State<T>** | `@Autowired` | Extract application state | ✅ |
| **Header<T>** | `@RequestHeader` | Extract headers | ✅ |
| **Cookie<T>** | `@CookieValue` | Extract cookies | ✅ |

---

## 🚀 Quick Start / 快速开始

### Installation / 安装

```toml
[dependencies]
nexus-extractors = "0.1.0-alpha"
serde = { version = "1.0", features = ["derive"] }
```

### Basic Usage / 基本用法

```rust
use nexus_extractors::{Path, Query, Json, State};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// Extract path parameter / 提取路径参数
async fn get_user(Path(id): Path<u64>) -> Json<User> {
    Json(User {
        id,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    })
}

// Extract query parameters / 提取查询参数
async fn list_users(
    Query(page): Query<u32>,
    Query(per_page): Query<u32>,
) -> Json<Vec<User>> {
    // Use page and per_page / 使用 page 和 per_page
    Json(vec![])
}

// Extract JSON body / 提取JSON body
async fn create_user(Json(user): Json<CreateUser>) -> Json<User> {
    Json(User {
        id: 1,
        name: user.name,
        email: user.email,
    })
}

// Extract application state / 提取应用状态
async fn get_config(State(config): State<AppConfig>) -> Json<AppConfig> {
    Json(config)
}
```

---

## 📖 Extractor Details / 提取器详情

### Path<T> - Path Parameters / 路径参数

Extract path parameters from route patterns:

从路由模式提取路径参数：

```rust
use nexus_extractors::Path;
use nexus_router::Router;

// Route: GET /users/:id
async fn get_user(Path(id): Path<u64>) -> Json<User> {
    // id is extracted from /users/123 → id = 123
    // id 从 /users/123 提取 → id = 123
    Json(find_user(id).await)
}

// Multiple path parameters / 多个路径参数
// Route: GET /users/:user_id/posts/:post_id
async fn get_post(
    Path(user_id): Path<u64>,
    Path(post_id): Path<u64>,
) -> Json<Post> {
    Json(find_post(user_id, post_id).await)
}

// With struct / 使用结构体
#[derive(Deserialize)]
struct UserPostParams {
    user_id: u64,
    post_id: u64,
}

async fn get_post_struct(Path(params): Path<UserPostParams>) -> Json<Post> {
    Json(find_post(params.user_id, params.post_id).await)
}

let router = Router::new()
    .get("/users/:id", get_user)
    .get("/users/:user_id/posts/:post_id", get_post);
```

**Supported Types** / **支持的类型**:
- `u8`, `u16`, `u32`, `u64`, `usize`
- `i8`, `i16`, `i32`, `i64`, `isize`
- `String`, `&str` (borrowed)
- `bool` (parses "true"/"false")
- Custom types with `Deserialize`

---

### Query<T> - Query Parameters / 查询参数

Extract query parameters from URL:

从URL提取查询参数：

```rust
use nexus_extractors::Query;

// Single parameter / 单个参数
// GET /users?page=1
async fn list_users(Query(page): Query<u32>) -> Json<Vec<User>> {
    Json(get_users_page(page).await)
}

// Multiple parameters / 多个参数
// GET /users?page=1&per_page=20
async fn list_users_paged(
    Query(page): Query<u32>,
    Query(per_page): Query<u32>,
) -> Json<Vec<User>> {
    Json(get_users_page_size(page, per_page).await)
}

// With struct / 使用结构体
#[derive(Deserialize)]
struct Pagination {
    page: u32,
    per_page: u32,
    sort: Option<String>,
}

// GET /users?page=1&per_page=20&sort=name
async fn list_users_struct(Query(pagination): Query<Pagination>) -> Json<Vec<User>> {
    Json(get_users_paginated(&pagination).await)
}

// Optional parameters / 可选参数
async fn search_users(Query(query): Query<Option<String>>) -> Json<Vec<User>> {
    match query {
        Some(q) => Json(search_users_by_name(&q).await),
        None => Json(get_all_users().await),
    }
}
```

**Query Parameter Parsing** / **查询参数解析**:
- `?key=value` → `Query(key): Query<String>`
- `?page=1` → `Query(page): Query<u32>`
- `?active=true` → `Query(active): Query<bool>`
- `?tags=rust&tags=async` → `Query(tags): Query<Vec<String>>`

---

### Json<T> - JSON Body / JSON Body

Extract JSON from request body:

从请求body提取JSON：

```rust
use nexus_extractors::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
    age: Option<u8>,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// POST /users with JSON body
async fn create_user(Json(user): Json<CreateUser>) -> Result<Json<User>, Error> {
    // Validate / 验证
    if user.name.is_empty() {
        return Err(Error::bad_request("Name is required"));
    }
    
    // Create user / 创建用户
    let created = save_user(user).await?;
    
    Ok(Json(created))
}

// Nested JSON / 嵌套JSON
#[derive(Deserialize)]
struct CreatePost {
    title: String,
    content: String,
    author: AuthorInfo,
}

#[derive(Deserialize)]
struct AuthorInfo {
    name: String,
    email: String,
}

async fn create_post(Json(post): Json<CreatePost>) -> Json<Post> {
    Json(save_post(post).await)
}
```

**Content-Type**: Automatically handles `application/json`  
**自动处理**: 自动处理 `application/json`

---

### Form<T> - Form Data / 表单数据

Extract form data from request body:

从请求body提取表单数据：

```rust
use nexus_extractors::Form;
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginForm {
    username: String,
    password: String,
    remember_me: Option<bool>,
}

// POST /login with form data
async fn login(Form(form): Form<LoginForm>) -> Result<Json<AuthToken>, Error> {
    // Validate credentials / 验证凭据
    let user = authenticate(&form.username, &form.password).await?;
    
    // Generate token / 生成token
    let token = generate_token(&user).await?;
    
    Ok(Json(token))
}

// URL-encoded form / URL编码表单
// POST /submit with application/x-www-form-urlencoded
async fn submit_form(Form(data): Form<FormData>) -> Response {
    process_form(data).await
}
```

**Content-Type**: Handles `application/x-www-form-urlencoded` and `multipart/form-data`  
**处理类型**: 处理 `application/x-www-form-urlencoded` 和 `multipart/form-data`

---

### State<T> - Application State / 应用状态

Extract application-wide state:

提取应用级状态：

```rust
use nexus_extractors::State;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db: Arc<Database>,
    cache: Arc<Cache>,
    config: AppConfig,
}

// Access state in handler / 在处理器中访问状态
async fn get_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<User>, Error> {
    // Use database from state / 使用状态中的数据库
    let user = state.db.find_user(id).await?;
    
    // Cache result / 缓存结果
    state.cache.set(&format!("user:{}", id), &user).await;
    
    Ok(Json(user))
}

// Register state with router / 在路由器中注册状态
let state = AppState {
    db: Arc::new(Database::new()),
    cache: Arc::new(Cache::new()),
    config: load_config(),
};

let router = Router::new()
    .get("/users/:id", get_user)
    .with_state(state);
```

**Use Cases** / **使用场景**:
- Database connections / 数据库连接
- Cache instances / 缓存实例
- Configuration / 配置
- Service clients / 服务客户端
- Shared resources / 共享资源

---

### Header<T> - HTTP Headers / HTTP Headers

Extract HTTP headers:

提取HTTP headers：

```rust
use nexus_extractors::{Header, HeaderOption, NamedHeader};

// Extract specific header / 提取特定header
async fn get_user(
    Path(id): Path<u64>,
    Header(auth): Header<NamedHeader<"Authorization">>,
) -> Result<Json<User>, Error> {
    // Validate token / 验证token
    let token = auth.value();
    verify_token(token)?;
    
    Ok(Json(find_user(id).await))
}

// Optional header / 可选header
async fn api_handler(
    HeaderOption(auth): HeaderOption<NamedHeader<"Authorization">>,
) -> Response {
    if let Some(token) = auth {
        // Use token / 使用token
    } else {
        // No auth header / 无auth header
    }
    Response::ok("")
}

// Extract header as type / 将header提取为类型
async fn get_user_agent(
    Header(ua): Header<NamedHeader<"User-Agent">>,
) -> Response {
    Response::ok(format!("User-Agent: {}", ua.value()))
}

// Custom header type / 自定义header类型
#[derive(Deserialize)]
struct ApiKey(String);

async fn protected_handler(
    Header(key): Header<NamedHeader<"X-API-Key">>,
) -> Result<Response, Error> {
    let api_key: ApiKey = key.value().parse()?;
    verify_api_key(&api_key.0)?;
    Ok(Response::ok(""))
}
```

**Header Types** / **Header类型**:
- `NamedHeader<"Header-Name">` - Specific header / 特定header
- `HeaderOption<T>` - Optional header / 可选header
- Custom types with `FromStr` or `Deserialize`

---

### Cookie<T> - HTTP Cookies / HTTP Cookies

Extract HTTP cookies:

提取HTTP cookies：

```rust
use nexus_extractors::{Cookie, CookieOption, NamedCookie};

// Extract specific cookie / 提取特定cookie
async fn get_profile(
    Cookie(session): Cookie<NamedCookie<"session_id">>,
) -> Result<Json<Profile>, Error> {
    let session_id = session.value();
    let user = find_user_by_session(session_id).await?;
    Ok(Json(user.profile))
}

// Optional cookie / 可选cookie
async fn dashboard(
    CookieOption(theme): CookieOption<NamedCookie<"theme">>,
) -> Response {
    let theme = theme.map(|c| c.value()).unwrap_or("light");
    render_dashboard(theme)
}

// Cookie with deserialization / 带反序列化的cookie
#[derive(Deserialize)]
struct SessionData {
    user_id: u64,
    expires: u64,
}

async fn get_user_data(
    Cookie(session): Cookie<NamedCookie<"session">>,
) -> Result<Json<User>, Error> {
    // Parse cookie value as JSON / 将cookie值解析为JSON
    let data: SessionData = serde_json::from_str(session.value())?;
    Ok(Json(find_user(data.user_id).await))
}
```

---

## 🎯 Combining Extractors / 组合提取器

You can use multiple extractors in a single handler:

可以在单个处理器中使用多个提取器：

```rust
use nexus_extractors::{Path, Query, Json, State, Header};

async fn update_user(
    Path(id): Path<u64>,                    // From URL path / 从URL路径
    Query(version): Query<Option<u32>>,     // From query string / 从查询字符串
    Header(auth): Header<NamedHeader<"Authorization">>,  // From header / 从header
    State(db): State<Arc<Database>>,        // From app state / 从应用状态
    Json(update): Json<UpdateUser>,         // From body / 从body
) -> Result<Json<User>, Error> {
    // Verify auth / 验证认证
    verify_token(auth.value())?;
    
    // Check version for optimistic locking / 检查版本以进行乐观锁
    if let Some(v) = version {
        check_version(id, v).await?;
    }
    
    // Update user / 更新用户
    let user = db.update_user(id, update).await?;
    
    Ok(Json(user))
}
```

**Extractor Order** / **提取器顺序**: Extractors can be in any order  
**提取器顺序**: 提取器可以任意顺序

---

## 🔧 Custom Extractors / 自定义提取器

Implement `FromRequest` trait for custom extractors:

实现 `FromRequest` trait 以创建自定义提取器：

```rust
use nexus_extractors::{FromRequest, Request, ExtractorError};
use std::pin::Pin;
use std::future::Future;

struct UserId(u64);

impl FromRequest for UserId {
    fn from_request(req: &Request) -> ExtractorFuture<Self> {
        Box::pin(async move {
            // Extract from header / 从header提取
            let header = req.header("X-User-Id")
                .ok_or_else(|| ExtractorError::Missing("X-User-Id".to_string()))?;
            
            // Parse as u64 / 解析为u64
            let id = header.parse::<u64>()
                .map_err(|e| ExtractorError::Invalid(format!("Invalid user ID: {}", e)))?;
            
            Ok(UserId(id))
        })
    }
}

// Use custom extractor / 使用自定义提取器
async fn handler(user_id: UserId) -> Response {
    Response::ok(format!("User ID: {}", user_id.0))
}
```

---

## ⚡ Performance / 性能

### Zero-Copy Extraction / 零拷贝提取

Extractors are designed for efficiency:

提取器设计用于高效：

```rust
// ✅ Good: Borrowed string / 好：借用字符串
async fn handler(Path(id): Path<&str>) -> Response {
    // No allocation / 无分配
}

// ⚠️ Acceptable: Owned string when needed / 可接受：需要时拥有字符串
async fn handler(Path(id): Path<String>) -> Response {
    // One allocation / 一次分配
}

// ❌ Avoid: Unnecessary cloning / 避免：不必要的克隆
async fn handler(Path(id): Path<String>) -> Response {
    let id2 = id.clone(); // Unnecessary / 不必要
}
```

### Lazy Evaluation / 惰性求值

Extractors only parse when accessed:

提取器仅在访问时解析：

```rust
// Only parses if handler is called / 仅在调用处理器时解析
async fn handler(Query(params): Query<ComplexParams>) -> Response {
    // Parsing happens here / 解析在这里发生
}
```

---

## 🧪 Testing / 测试

### Unit Testing / 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use nexus_http::Request;

    #[tokio::test]
    async fn test_path_extractor() {
        let req = Request::builder()
            .uri("/users/123")
            .build();
        
        let id: Path<u64> = Path::from_request(&req).await.unwrap();
        assert_eq!(id.0, 123);
    }

    #[tokio::test]
    async fn test_query_extractor() {
        let req = Request::builder()
            .uri("/users?page=1&per_page=20")
            .build();
        
        let page: Query<u32> = Query::from_request(&req).await.unwrap();
        assert_eq!(page.0, 1);
    }
}
```

---

## 🚦 Roadmap / 路线图

### Phase 2: Core Extractors ✅ (Completed / 已完成)
- [x] Path<T>
- [x] Query<T>
- [x] Json<T>
- [x] Form<T>
- [x] State<T>
- [x] Header<T>
- [x] Cookie<T>

### Phase 3: Advanced Extractors 🔄 (In Progress / 进行中)
- [ ] File upload extractor
- [ ] Multipart form data
- [ ] Streaming body extractor
- [ ] Custom validation

---

## 📚 Documentation / 文档

- **API Documentation**: [docs.rs/nexus-extractors](https://docs.rs/nexus-extractors)
- **Book**: [Extractors Guide](../../docs/book/src/core-concepts/extractors.md)
- **Examples**: [examples/](../../examples/)

---

## 🤝 Contributing / 贡献

We welcome contributions! Please see:

- [CONTRIBUTING.md](../../CONTRIBUTING.md)
- [Design Spec](../../docs/design-spec.md)
- [GitHub Issues](https://github.com/nexus-framework/nexus/issues)

---

## 📄 License / 许可证

Licensed under Apache License 2.0. See [LICENSE](../../LICENSE) for details.

---

## 🙏 Acknowledgments / 致谢

Nexus Extractors is inspired by:

- **[Spring Boot](https://spring.io/projects/spring-boot)** - Method parameter resolution
- **[Axum](https://github.com/tokio-rs/axum)** - Extractor patterns
- **[Actix Web](https://github.com/actix/actix-web)** - Request extraction

---

**Built with ❤️ for type-safe request handling**

**为类型安全的请求处理构建 ❤️**
