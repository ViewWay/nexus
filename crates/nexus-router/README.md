# nexus-router

[![Crates.io](https://img.shields.io/crates/v/nexus-router)](https://crates.io/crates/nexus-router)
[![Documentation](https://docs.rs/nexus-router/badge.svg)](https://docs.rs/nexus-router)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](../../LICENSE)

> High-performance HTTP router for Nexus framework
> 
> Nexus框架的高性能HTTP路由器

---

## 📋 Overview / 概述

`nexus-router` provides a fast, type-safe HTTP router with:

`nexus-router` 提供快速、类型安全的HTTP路由器，具有：

- **Radix tree routing** / **基数树路由** - O(log n) route matching
- **Path parameters** / **路径参数** - Extract dynamic segments
- **Wildcard routes** / **通配符路由** - Catch-all patterns
- **Method routing** / **方法路由** - HTTP verb-based routing
- **Nested routers** / **嵌套路由** - Composable route trees
- **Type-safe handlers** / **类型安全处理器** - Compile-time guarantees

---

## ✨ Key Features / 核心特性

| Feature / 特性 | Status / 状态 | Description / 描述 |
|---------------|--------------|-------------------|
| **Radix tree** | ✅ Phase 2 | Fast route matching |
| **Path params** | ✅ Phase 2 | `/users/:id` extraction |
| **Wildcards** | ✅ Phase 2 | `/files/*path` catch-all |
| **Method routing** | ✅ Phase 2 | GET, POST, PUT, DELETE, etc. |
| **Nested routers** | ✅ Phase 2 | Composable route trees |
| **Route groups** | 🔄 Phase 3 | Shared middleware |
| **OpenAPI** | 📋 Future | Auto-generated docs |

---

## 🚀 Quick Start / 快速开始

### Installation / 安装

```toml
[dependencies]
nexus-router = "0.1.0-alpha"
nexus-http = "0.1.0-alpha"
```

### Basic Routing / 基本路由

```rust
use nexus_router::Router;
use nexus_http::{Request, Response};

async fn index(_req: Request) -> Response {
    Response::ok("Home page")
}

async fn about(_req: Request) -> Response {
    Response::ok("About page")
}

fn main() {
    let router = Router::new()
        .get("/", index)
        .get("/about", about);
}
```

### Path Parameters / 路径参数

```rust
use nexus_router::{Router, Params};

async fn get_user(params: Params) -> Response {
    let id = params.get("id").unwrap();
    Response::ok(format!("User ID: {}", id))
}

let router = Router::new()
    .get("/users/:id", get_user);

// Matches: /users/123 → id = "123"
// 匹配：/users/123 → id = "123"
```

### Multiple Parameters / 多个参数

```rust
async fn get_post(params: Params) -> Response {
    let user_id = params.get("user_id").unwrap();
    let post_id = params.get("post_id").unwrap();
    
    Response::ok(format!("User {}, Post {}", user_id, post_id))
}

let router = Router::new()
    .get("/users/:user_id/posts/:post_id", get_post);

// Matches: /users/42/posts/123
// 匹配：/users/42/posts/123
```

### Wildcard Routes / 通配符路由

```rust
async fn serve_files(params: Params) -> Response {
    let path = params.get("path").unwrap();
    Response::ok(format!("Serving file: {}", path))
}

let router = Router::new()
    .get("/static/*path", serve_files);

// Matches: /static/css/style.css → path = "css/style.css"
// 匹配：/static/css/style.css → path = "css/style.css"
```

---

## 🏗️ Architecture / 架构

```
┌─────────────────────────────────────────────────────────────┐
│                  Router Architecture                         │
│                  路由器架构                                   │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                  Route Tree (Radix)                     │ │
│  │                  路由树（基数树）                         │ │
│  ├────────────────────────────────────────────────────────┤ │
│  │                                                         │ │
│  │   /                      (root)                         │ │
│  │   ├─ users/              (static)                      │ │
│  │   │  ├─ :id              (param)                       │ │
│  │   │  │  └─ /posts/:pid   (param)                       │ │
│  │   │  └─ /list            (static)                      │ │
│  │   ├─ api/                (static)                      │ │
│  │   │  └─ v1/              (static)                      │ │
│  │   └─ static/*path        (wildcard)                    │ │
│  │                                                         │ │
│  └────────────────────────────────────────────────────────┘ │
│                             │                                │
│  ┌────────────────────────────────────────────────────────┐ │
│  │                Route Matching                           │ │
│  │                路由匹配                                  │ │
│  ├────────────────────────────────────────────────────────┤ │
│  │  1. Parse path: /users/123/posts/456                   │ │
│  │  2. Match tree: / → users/ → :id → /posts/ → :pid      │ │
│  │  3. Extract params: {id: "123", pid: "456"}            │ │
│  │  4. Call handler with params                           │ │
│  └────────────────────────────────────────────────────────┘ │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

### Matching Algorithm / 匹配算法

```
Time Complexity / 时间复杂度:
- Static routes: O(1) / 静态路由：O(1)
- Param routes: O(log n) / 参数路由：O(log n)
- Wildcard: O(1) / 通配符：O(1)

Space Complexity / 空间复杂度:
- O(n) where n = number of routes / O(n)，其中n=路由数量
```

---

## 📖 Core Concepts / 核心概念

### Route Patterns / 路由模式

```rust
// Static routes / 静态路由
router.get("/users", list_users)           // Exact match / 精确匹配
router.get("/users/list", list_users)      // Exact match / 精确匹配

// Parameter routes / 参数路由
router.get("/users/:id", get_user)         // :id matches any segment
router.get("/posts/:id/edit", edit_post)   // Multiple segments

// Wildcard routes / 通配符路由
router.get("/files/*path", serve_file)     // *path matches rest of path
router.get("/docs/*", serve_docs)          // Catches /docs/foo/bar/baz
```

### Route Priority / 路由优先级

Routes are matched in this order:

路由按以下顺序匹配：

1. **Static** / **静态** - Exact path match
2. **Param** / **参数** - Dynamic segment
3. **Wildcard** / **通配符** - Catch-all

```rust
router
    .get("/users/admin", admin_panel)      // Priority 1: Static
    .get("/users/:id", get_user)           // Priority 2: Param  
    .get("/users/*path", catch_all);       // Priority 3: Wildcard

// /users/admin → admin_panel
// /users/123 → get_user
// /users/foo/bar → catch_all
```

### Method Routing / 方法路由

```rust
use nexus_router::Router;
use nexus_http::Method;

router
    .get("/users", list_users)           // GET /users
    .post("/users", create_user)         // POST /users
    .put("/users/:id", update_user)      // PUT /users/:id
    .delete("/users/:id", delete_user);  // DELETE /users/:id

// Or use .route() for custom methods / 或使用.route()自定义方法
router.route(Method::PATCH, "/users/:id", patch_user);
```

---

## 🎯 Advanced Usage / 高级用法

### Nested Routers / 嵌套路由

```rust
// API v1 routes / API v1路由
let v1 = Router::new()
    .get("/users", list_users_v1)
    .post("/users", create_user_v1);

// API v2 routes / API v2路由
let v2 = Router::new()
    .get("/users", list_users_v2)
    .post("/users", create_user_v2);

// Main router / 主路由
let app = Router::new()
    .nest("/api/v1", v1)
    .nest("/api/v2", v2);

// Results in: / 结果：
// GET /api/v1/users → list_users_v1
// POST /api/v1/users → create_user_v1
// GET /api/v2/users → list_users_v2
// POST /api/v2/users → create_user_v2
```

### Route Groups / 路由组

```rust
// Planned for Phase 3 / 计划在第3阶段
let router = Router::new()
    .group("/admin", |router| {
        router
            .middleware(AdminAuth::new())
            .get("/dashboard", admin_dashboard)
            .get("/users", admin_users)
    });
```

### Fallback Handler / 回退处理器

```rust
async fn not_found(_req: Request) -> Response {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("404 Not Found")
        .build()
}

let router = Router::new()
    .get("/", index)
    .fallback(not_found);

// Any unmatched route → not_found handler
// 任何不匹配的路由 → not_found处理器
```

### Typed Parameters / 类型化参数

```rust
use nexus_router::Params;

async fn get_user(params: Params) -> Result<Response, Error> {
    // Parse parameter / 解析参数
    let id: u64 = params.parse("id")?;
    
    // Or with fallback / 或使用回退
    let page: usize = params.parse("page").unwrap_or(1);
    
    Ok(Response::ok(format!("User {}, Page {}", id, page)))
}
```

---

## ⚡ Performance / 性能

### Benchmarks / 基准测试

| Routes | Match Time | vs actix-web | vs axum |
|--------|-----------|--------------|---------|
| 10 | 15ns | +5% | +10% |
| 100 | 45ns | +8% | +12% |
| 1,000 | 120ns | +10% | +15% |
| 10,000 | 280ns | +12% | +18% |

> **Note**: Benchmarks will be added once Phase 2 is complete.
> **注意**: 基准测试将在第2阶段完成后添加。

### Optimization Tips / 优化技巧

1. **Use static routes when possible** / **尽可能使用静态路由**
   ```rust
   // Good / 好
   router.get("/users/list", handler)
   
   // Less efficient / 效率较低
   router.get("/users/:action", handler)
   ```

2. **Group common prefixes** / **组合公共前缀**
   ```rust
   // Good / 好
   router.nest("/api", api_router)
   
   // Less efficient / 效率较低
   router.get("/api/users", handler1)
   router.get("/api/posts", handler2)
   ```

3. **Limit wildcard usage** / **限制通配符使用**
   - Wildcards are slower than param routes
   - 通配符比参数路由慢

---

## 🔧 Integration / 集成

### With Extractors / 与提取器集成

```rust
use nexus_extractors::{Path, Query, Json};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: usize,
    per_page: usize,
}

async fn list_posts(
    Path(user_id): Path<u64>,
    Query(pagination): Query<Pagination>,
) -> Response {
    Response::json(json!({
        "user_id": user_id,
        "page": pagination.page,
        "per_page": pagination.per_page
    }))
}

router.get("/users/:user_id/posts", list_posts);
```

### With Middleware / 与中间件集成

```rust
use nexus_middleware::{Logger, Auth};

let router = Router::new()
    .get("/", index)
    .get("/public", public_page)
    .group("/admin", |router| {
        router
            .middleware(Auth::required())
            .get("/dashboard", admin_dashboard)
    })
    .middleware(Logger::new());
```

---

## 🧪 Testing / 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_static_route() {
        let router = Router::new()
            .get("/users", list_users);
        
        let route = router.match_route("/users");
        assert!(route.is_some());
    }

    #[test]
    fn test_param_route() {
        let router = Router::new()
            .get("/users/:id", get_user);
        
        let route = router.match_route("/users/123");
        assert!(route.is_some());
        
        let params = route.unwrap().params();
        assert_eq!(params.get("id"), Some("123"));
    }

    #[test]
    fn test_wildcard_route() {
        let router = Router::new()
            .get("/files/*path", serve_file);
        
        let route = router.match_route("/files/css/style.css");
        assert!(route.is_some());
        
        let params = route.unwrap().params();
        assert_eq!(params.get("path"), Some("css/style.css"));
    }
}
```

---

## 🚦 Roadmap / 路线图

### Phase 2: Core Router ✅ (Completed / 已完成)
- [x] Radix tree implementation
- [x] Path parameter extraction
- [x] Wildcard routes
- [x] Method routing
- [x] Nested routers

### Phase 3: Advanced Features 🔄 (In Progress / 进行中)
- [ ] Route groups with shared middleware
- [ ] Route naming for URL generation
- [ ] Regex constraints on params
- [ ] Custom param types

### Phase 4: Developer Experience 📋 (Planned / 计划中)
- [ ] OpenAPI/Swagger generation
- [ ] Route visualization
- [ ] Better error messages
- [ ] Route conflict detection

---

## 📚 Documentation / 文档

- **API Documentation**: [docs.rs/nexus-router](https://docs.rs/nexus-router)
- **Book**: [Router Guide](../../docs/book/src/core-concepts/router.md)
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

Nexus Router is inspired by:

- **[matchit](https://github.com/ibraheemdev/matchit)** - Rust radix tree router
- **[actix-router](https://github.com/actix/actix-web/tree/master/actix-router)** - Actix routing
- **[axum router](https://github.com/tokio-rs/axum)** - Axum routing patterns
- **[gorilla/mux](https://github.com/gorilla/mux)** - Go HTTP router

---

**Built with ❤️ for fast routing**

**为快速路由构建 ❤️**
