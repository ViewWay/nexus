# Migration Guide / 迁移指南

This guide helps you migrate from other web frameworks to Nexus.

本指南帮助您从其他Web框架迁移到Nexus。

---

## Table of Contents / 目录

1. [From Axum](#from-axum)
2. [From Actix-Web](#from-actix-web)
3. [From Spring Boot](#from-spring-boot)
4. [From Express.js](#from-expressjs)

---

## From Axum

### Handler Changes / 处理器变更

**Axum / Axum:**
```rust
use axum::{Json, extract::Path};

async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User { id, name: "Alice".to_string() })
}
```

**Nexus / Nexus:**
```rust
use nexus_extractors::{Path, Json};

async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User { id, name: "Alice".to_string() })
}
```

### Router Changes / 路由变更

**Axum:**
```rust
let app = Router::new()
    .route("/users", get(list_users).post(create_user))
    .route("/users/:id", get(get_user));
```

**Nexus:**
```rust
let app = Router::new()
    .get("/users", list_users)
    .post("/users", create_user)
    .get("/users/:id", get_user);
```

### State Management / 状态管理

**Axum:**
```rust
let app = Router::new()
    .route("/", handler)
    .with_state(AppState::new());
```

**Nexus:**
```rust
let app = Router::new()
    .get("/", handler)
    .with_state(AppState::new());
```

---

## From Actix-Web

### Handler Changes / 处理器变更

**Actix-Web:**
```rust
use actix_web::{web, HttpResponse};

async fn get_user(path: web::Path<u32>) -> HttpResponse {
    let id = path.into_inner();
    HttpResponse::Ok().json(User { id, name: "Alice".to_string() })
}
```

**Nexus:**
```rust
use nexus_extractors::Path;

async fn get_user(Path(id): Path<u32>) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_vec(&User { id, name: "Alice".to_string() }).unwrap().into())
        .unwrap()
}
```

### Data Extraction / 数据提取

**Actix-Web:**
```rust
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
}

async fn create_user(payload: web::Json<CreateUserRequest>) -> HttpResponse {
    HttpResponse::Ok().json(payload.into_inner())
}
```

**Nexus:**
```rust
use nexus_extractors::Json;

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
}

async fn create_user(Json(payload): Json<CreateUserRequest>) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_vec(&payload).unwrap().into())
        .unwrap()
}
```

---

## From Spring Boot

### Controller → Handler / 控制器到处理器

**Spring Boot:**
```java
@RestController
@RequestMapping("/users")
public class UserController {

    @GetMapping("/{id}")
    public ResponseEntity<User> getUser(@PathVariable Long id) {
        User user = userService.findById(id);
        return ResponseEntity.ok(user);
    }

    @PostMapping
    public ResponseEntity<User> createUser(@RequestBody CreateUserRequest request) {
        User user = userService.create(request);
        return ResponseEntity.status(HttpStatus.CREATED).body(user);
    }
}
```

**Nexus:**
```rust
use nexus_router::Router;
use nexus_extractors::{Path, Json};

async fn get_user(Path(id): Path<u64>) -> Result<Response, AppError> {
    let user = user_service.find_by_id(id).await?;
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_vec(&user)?.into())?)
}

async fn create_user(Json(request): Json<CreateUserRequest>) -> Result<Response, AppError> {
    let user = user_service.create(request).await?;
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(serde_json::to_vec(&user)?.into())?)
}

let app = Router::new()
    .get("/users/:id", get_user)
    .post("/users", create_user);
```

### Dependency Injection / 依赖注入

**Spring Boot:**
```java
@Service
public class UserService {
    private final UserRepository userRepository;

    @Autowired
    public UserService(UserRepository userRepository) {
        this.userRepository = userRepository;
    }
}
```

**Nexus (with nexus-core):**
```rust
use nexus_core::container::{Bean, Container};

#[derive(Bean)]
pub struct UserService {
    #[inject]
    repository: Arc<UserRepository>,
}

impl UserService {
    pub async fn find_by_id(&self, id: u64) -> Result<User, AppError> {
        self.repository.find_by_id(id).await
    }
}
```

### Configuration / 配置

**Spring Boot (application.properties):**
```properties
server.port=8080
database.url=jdbc:postgresql://localhost/mydb
```

**Nexus:**
```rust
use nexus_config::Config;

#[derive(Config, Deserialize)]
struct AppConfig {
    #[serde(default = "default_port")]
    server_port: u16,
    database_url: String,
}

fn default_port() -> u16 { 8080 }
```

---

## From Express.js

### Route Handler / 路由处理器

**Express.js:**
```javascript
app.get('/users/:id', (req, res) => {
    const id = req.params.id;
    res.json({ id, name: 'Alice' });
});

app.post('/users', express.json(), (req, res) => {
    const { name } = req.body;
    res.status(201).json({ id: 1, name });
});
```

**Nexus:**
```rust
use nexus_router::Router;
use nexus_extractors::{Path, Json};

async fn get_user(Path(id): Path<String>) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(serde_json::to_vec(&json!({"id": id, "name": "Alice"})).unwrap().into())
        .unwrap()
}

async fn create_user(Json(payload): Json<Value>) -> Response {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(serde_json::to_vec(&json!({"id": 1, "name": payload["name"]})).unwrap().into())
        .unwrap()
}

let app = Router::new()
    .get("/users/:id", get_user)
    .post("/users", create_user);
```

### Middleware / 中间件

**Express.js:**
```javascript
app.use((req, res, next) => {
    console.log(`${req.method} ${req.path}`);
    next();
});
```

**Nexus:**
```rust
use nexus_middleware::{Middleware, Next};

struct Logger;

impl<S> Middleware<S> for Logger {
    type Output = LoggerWrapped<S>;

    fn wrap(&self, inner: S) -> Self::Output {
        LoggerWrapped(inner)
    }
}

async fn log_middleware<M>(
    req: Request,
    next: Next<M>,
) -> Response
where
    M: Service<Request, Response = Response>,
{
    println!("{} {}", req.method(), req.uri());
    next.run(req).await
}
```

---

## Key Concepts Mapping / 关键概念映射

| Concept / 概念 | Axum | Actix | Spring | Express | Nexus |
|----------------|------|-------|--------|---------|-------|
| Handler / 处理器 | `async fn` | `async fn` | `@RequestMapping` | `(req, res)` | `async fn` |
| Router / 路由 | `Router` | `Route` | `@RestController` | `app.get()` | `Router` |
| Extractor / 提取器 | `Extractor` | `FromRequest` | `@PathVariable` | `req.params` | `FromRequest` |
| Middleware / 中间件 | `Middleware` | `Middleware` | `@Filter` | `app.use()` | `Middleware` |
| Error / 错误 | `Result` | `Result` | `@ExceptionHandler` | `next(err)` | `Result` |

---

## Common Patterns / 常见模式

### JSON Response / JSON响应

```rust
// All frameworks / 所有框架
async fn handler() -> Json<MyData> {
    Json(MyData { field: "value" })
}
```

### Path Parameters / 路径参数

```rust
// All frameworks / 所有框架
async fn handler(Path(id): Path<String>) {
    // Use id / 使用id
}
```

### Query Parameters / 查询参数

```rust
// Nexus
async fn handler(Query(params): Query<HashMap<String, String>>) {
    let search = params.get("q").unwrap_or(&String::new());
}
```

### Custom Error / 自定义错误

```rust
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found")]
    NotFound,
    #[error("Internal: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            Self::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, &msg),
        };
        Response::builder()
            .status(status)
            .body(message.into())
            .unwrap()
    }
}
```

---

## Next Steps / 下一步

- Read the [Tutorial](./book/src/getting-started/tutorial.md)
- Explore [API Reference](./book/src/reference/api.md)
- Check [Examples](https://github.com/nexus-rs/nexus/tree/main/examples)
