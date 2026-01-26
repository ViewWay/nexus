//! Nexus Quick Start Example
//! Nexus 快速开始示例
//!
//! This example demonstrates the basic usage of Nexus framework.
//! 此示例演示 Nexus 框架的基本用法。

use nexus::prelude::*;
use nexus_macros::{main, controller, get, post};
use serde::{Deserialize, Serialize};

/// User model / 用户模型
#[derive(Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    username: String,
    email: String,
}

/// Create user request / 创建用户请求
#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

/// Main application / 主应用
#[main]
struct Application;

/// Root controller / 根控制器
#[controller]
struct RootController;

/// Health check endpoint / 健康检查端点
#[get("/health")]
async fn health_check() -> &'static str {
    "OK"
}

/// Hello world endpoint / Hello world 端点
#[get("/")]
async fn hello() -> &'static str {
    "Hello, Nexus!"
}

/// User controller / 用户控制器
#[controller]
struct UserController;

/// List all users / 列出所有用户
#[get("/api/users")]
async fn list_users() -> Json<Vec<User>> {
    Json(vec![
        User {
            id: 1,
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ])
}

/// Get user by ID / 根据 ID 获取用户
#[get("/api/users/:id")]
async fn get_user(id: u64) -> Result<Json<User>, Error> {
    if id == 1 {
        Ok(Json(User {
            id,
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
        }))
    } else {
        Err(Error::not_found("User", &id.to_string()))
    }
}

/// Create new user / 创建新用户
#[post("/api/users")]
async fn create_user(#[request_body] input: CreateUser) -> Json<User> {
    Json(User {
        id: 3,
        username: input.username,
        email: input.email,
    })
}
