# Router
# 路由

The `nexus-router` crate provides high-performance HTTP request routing using a trie-based data structure.

`nexus-router` crate 使用基于 trie 的数据结构提供高性能 HTTP 请求路由。

## Overview / 概述

The router maps HTTP method + path combinations to handler functions:

路由器将 HTTP 方法 + 路径组合映射到处理函数：

```rust
use nexus_router::Router;
use nexus_http::{Response, StatusCode, Body};

let router = Router::new()
    .get("/", index)
    .get("/users", list_users)
    .get("/users/:id", get_user)
    .post("/users", create_user)
    .put("/users/:id", update_user)
    .delete("/users/:id", delete_user);
```

## Route Patterns / 路由模式

### Static Routes / 静态路由

```rust
router.get("/api/health", health_check)
      .get("/api/version", version_info)
```

### Path Parameters / 路径参数

Use `:name` syntax for dynamic segments:
使用 `:name` 语法表示动态片段：

```rust
// Single parameter / 单参数
router.get("/users/:id", get_user)

// Multiple parameters / 多参数
router.get("/users/:user_id/posts/:post_id", get_user_post)

// Access in handler / 在处理器中访问
async fn get_user(req: Request) -> Response {
    let id = req.path_var("id").unwrap();
    // ...
}
```

### Wildcard Routes / 通配符路由

Use `*name` for catch-all segments:
使用 `*name` 表示捕获所有片段：

```rust
// Matches /files/a, /files/a/b, /files/a/b/c, etc.
router.get("/files/*path", serve_file)

async fn serve_file(req: Request) -> Response {
    let path = req.path_var("path").unwrap();
    // path = "a/b/c" for /files/a/b/c
    // ...
}
```

## HTTP Methods / HTTP 方法

```rust
use nexus_router::Router;

let router = Router::new()
    .get("/resource", handler)      // GET
    .post("/resource", handler)     // POST
    .put("/resource", handler)      // PUT
    .patch("/resource", handler)    // PATCH
    .delete("/resource", handler)   // DELETE
    .head("/resource", handler)     // HEAD
    .options("/resource", handler)  // OPTIONS
    
    // Generic method / 通用方法
    .route("/resource", Method::GET, handler);
```

## Route Groups / 路由分组

### Merging Routers / 合并路由器

```rust
// API routes / API 路由
let api = Router::new()
    .get("/users", list_users)
    .post("/users", create_user);

// Admin routes / 管理路由
let admin = Router::new()
    .get("/stats", get_stats)
    .post("/config", update_config);

// Merge into main router / 合并到主路由
let router = Router::new()
    .get("/", index)
    .merge(api)
    .merge(admin);
```

### Nested Routes / 嵌套路由

```rust
let users = Router::new()
    .get("/", list_users)
    .get("/:id", get_user)
    .post("/", create_user);

let posts = Router::new()
    .get("/", list_posts)
    .get("/:id", get_post);

// Nest under prefixes / 嵌套到前缀下
let router = Router::new()
    .nest("/api/users", users)   // /api/users, /api/users/:id
    .nest("/api/posts", posts);  // /api/posts, /api/posts/:id
```

## Middleware Integration / 中间件集成

```rust
use nexus_middleware::{LoggerMiddleware, CorsMiddleware};

let router = Router::new()
    .get("/", index)
    .get("/api/data", get_data)
    // Apply middleware to all routes / 为所有路由应用中间件
    .layer(LoggerMiddleware::new())
    .layer(CorsMiddleware::any());
```

## Path Extraction / 路径提取

### Using Path<T> Extractor / 使用 Path<T> 提取器

```rust
use nexus_router::Path;

// Extract single value / 提取单个值
async fn get_user(Path(id): Path<u64>) -> Response {
    // id is u64
}

// Extract multiple values / 提取多个值
async fn get_user_post(
    Path((user_id, post_id)): Path<(u64, u64)>
) -> Response {
    // user_id and post_id are u64
}

// Using struct / 使用结构体
#[derive(Deserialize)]
struct UserParams {
    user_id: u64,
    post_id: u64,
}

async fn get_user_post(Path(params): Path<UserParams>) -> Response {
    // params.user_id, params.post_id
}
```

## Fallback Handler / 回退处理器

Handle unmatched routes:
处理未匹配的路由：

```rust
let router = Router::new()
    .get("/", index)
    .fallback(not_found_handler);

async fn not_found_handler(_req: Request) -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Page not found"))
        .unwrap()
}
```

## Spring Boot Comparison / Spring Boot 对比

| Spring Boot | Nexus Router | Description |
|-------------|--------------|-------------|
| `@GetMapping("/path")` | `.get("/path", handler)` | GET route |
| `@PostMapping("/path")` | `.post("/path", handler)` | POST route |
| `@PathVariable` | `Path<T>` | Path parameter |
| `@RequestMapping` | `.route()` | Generic route |
| Router composition | `.nest()`, `.merge()` | Route grouping |

## Performance / 性能

The trie-based router provides O(n) route matching where n is the path length, regardless of the number of registered routes.

基于 trie 的路由器提供 O(n) 的路由匹配，其中 n 是路径长度，与注册的路由数量无关。

| Routes | Match Time |
|--------|------------|
| 10 | ~50ns |
| 100 | ~50ns |
| 1000 | ~50ns |
| 10000 | ~50ns |

## Complete Example / 完整示例

```rust
use nexus_router::{Router, Path};
use nexus_http::{Body, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}

async fn list_users(_req: Request) -> Response {
    let users = vec![
        User { id: 1, name: "Alice".into() },
        User { id: 2, name: "Bob".into() },
    ];
    json_response(&users)
}

async fn get_user(req: Request) -> Response {
    let id: u64 = req.path_var("id")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    
    let user = User { id, name: format!("User {}", id) };
    json_response(&user)
}

async fn create_user(_req: Request) -> Response {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from(r#"{"status": "created"}"#))
        .unwrap()
}

fn json_response<T: Serialize>(data: &T) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(data).unwrap()))
        .unwrap()
}

fn main() {
    let router = Router::new()
        .get("/users", list_users)
        .get("/users/:id", get_user)
        .post("/users", create_user);
    
    // Use with server...
}
```

---

*← [Previous / 上一页](./http.md) | [Next / 下一页](./middleware.md) →*
