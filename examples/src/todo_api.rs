//! Nexus Todo API Example
//! Nexus Todo API 示例
//!
//! A complete REST API example demonstrating:
//! 完整的 REST API 示例，演示：
//! - CRUD operations / CRUD 操作
//! - State management / 状态管理
//! - Error handling / 错误处理
//! - Middleware / 中间件
//! - Validation / 校验

use nexus::prelude::*;
use nexus_macros::{controller, get, post, put, delete};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Todo item / 待办事项
#[derive(Clone, Serialize, Deserialize)]
struct Todo {
    id: u64,
    title: String,
    completed: bool,
}

/// Application state / 应用状态
#[derive(Clone)]
struct AppState {
    todos: Arc<RwLock<Vec<Todo>>>,
    next_id: Arc<RwLock<u64>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            todos: Arc::new(RwLock::new(vec![
                Todo {
                    id: 1,
                    title: "Learn Nexus".to_string(),
                    completed: false,
                },
                Todo {
                    id: 2,
                    title: "Build something awesome".to_string(),
                    completed: false,
                },
            ])),
            next_id: Arc::new(RwLock::new(3)),
        }
    }
}

/// Todo controller / 待办控制器
#[controller]
struct TodoController;

/// List all todos / 列出所有待办
#[get("/api/todos")]
async fn list_todos(
    #[state] state: Arc<AppState>,
) -> Json<Vec<Todo>> {
    Json(state.todos.read().await.clone())
}

/// Get todo by ID / 根据 ID 获取待办
#[get("/api/todos/:id")]
async fn get_todo(
    id: u64,
    #[state] state: Arc<AppState>,
) -> Result<Json<Todo>, Error> {
    let todos = state.todos.read().await;
    todos
        .iter()
        .find(|t| t.id == id)
        .cloned()
        .map(Json)
        .ok_or_else(|| Error::not_found("Todo", &id.to_string()))
}

/// Create todo / 创建待办
#[post("/api/todos")]
async fn create_todo(
    #[request_body] input: CreateTodo,
    #[state] state: Arc<AppState>,
) -> Json<Todo> {
    let mut id = state.next_id.write().await;
    let todo = Todo {
        id: *id,
        title: input.title,
        completed: false,
    };
    *id += 1;
    
    state.todos.write().await.push(todo.clone());
    Json(todo)
}

/// Update todo / 更新待办
#[put("/api/todos/:id")]
async fn update_todo(
    id: u64,
    #[request_body] input: UpdateTodo,
    #[state] state: Arc<AppState>,
) -> Result<Json<Todo>, Error> {
    let mut todos = state.todos.write().await;
    let todo = todos
        .iter_mut()
        .find(|t| t.id == id)
        .ok_or_else(|| Error::not_found("Todo", &id.to_string()))?;
    
    if let Some(title) = input.title {
        todo.title = title;
    }
    if let Some(completed) = input.completed {
        todo.completed = completed;
    }
    
    Ok(Json(todo.clone()))
}

/// Delete todo / 删除待办
#[delete("/api/todos/:id")]
async fn delete_todo(
    id: u64,
    #[state] state: Arc<AppState>,
) -> Result<Status, Error> {
    let mut todos = state.todos.write().await;
    let len_before = todos.len();
    todos.retain(|t| t.id != id);
    
    if todos.len() < len_before {
        Ok(Status::NO_CONTENT)
    } else {
        Err(Error::not_found("Todo", &id.to_string()))
    }
}

/// Create todo request / 创建待办请求
#[derive(Deserialize)]
struct CreateTodo {
    title: String,
}

/// Update todo request / 更新待办请求
#[derive(Deserialize)]
struct UpdateTodo {
    title: Option<String>,
    completed: Option<bool>,
}

/// Main entry point / 主入口点
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let state = Arc::new(AppState::new());
    
    let app = Router::with_state(state)
        .get("/health", || async { "OK" })
        // Todo routes
        .get("/api/todos", list_todos)
        .get("/api/todos/:id", get_todo)
        .post("/api/todos", create_todo)
        .put("/api/todos/:id", update_todo)
        .delete("/api/todos/:id", delete_todo);
    
    println!("🚀 Todo API starting on http://127.0.0.1:8080");
    println!("📖 Endpoints:");
    println!("  GET    /api/todos       - List all todos");
    println!("  GET    /api/todos/:id   - Get todo by ID");
    println!("  POST   /api/todos       - Create todo");
    println!("  PUT    /api/todos/:id   - Update todo");
    println!("  DELETE /api/todos/:id   - Delete todo");
    
    Server::bind("127.0.0.1:8080")
        .serve(app)
        .await?;
    
    Ok(())
}
