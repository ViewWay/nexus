//! Route module
//! 路由模块

#![warn(missing_docs)]
#![warn(unreachable_pub)]

use std::fmt;

/// Route handler function signature
/// 路由处理函数签名
pub type HandlerFn = fn();

/// A route in the router
/// 路由器中的路由
#[derive(Clone)]
pub struct Route {
    /// The HTTP method(s) this route handles
    /// 此路由处理的HTTP方法
    pub methods: Vec<Method>,
    /// The path pattern (e.g., "/users/:id")
    /// 路径模式（如 "/users/:id"）
    pub path: String,
    /// The handler for this route
    /// 此路由的处理程序
    pub handler: Handler,
}

impl Route {
    /// Create a new route
    /// 创建新路由
    pub fn new(path: impl Into<String>, handler: Handler) -> Self {
        Self {
            methods: Vec::new(),
            path: path.into(),
            handler,
        }
    }

    /// Add an HTTP method to this route
    /// 向此路由添加HTTP方法
    pub fn method(mut self, method: Method) -> Self {
        self.methods.push(method);
        self
    }

    /// Set the methods for this route
    /// 设置此路由的方法
    pub fn methods(mut self, methods: Vec<Method>) -> Self {
        self.methods = methods;
        self
    }

    /// Check if this route matches the given method and path
    /// 检查此路由是否匹配给定的方法和路径
    pub fn matches(&self, method: &Method, path: &str) -> bool {
        if !self.methods.is_empty() && !self.methods.contains(method) {
            return false;
        }

        self.path_matches(path)
    }

    /// Check if the path pattern matches the given path
    /// 检查路径模式是否匹配给定路径
    fn path_matches(&self, path: &str) -> bool {
        let route_parts: Vec<&str> = self.path.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();

        if route_parts.len() != path_parts.len() {
            return false;
        }

        for (route_part, path_part) in route_parts.iter().zip(path_parts.iter()) {
            if route_part.starts_with(':') || route_part.starts_with('*') {
                // Path parameter - matches anything
                // 路径参数 - 匹配任何内容
                continue;
            }
            if route_part != path_part {
                return false;
            }
        }

        true
    }

    /// Extract path parameters from the given path
    /// 从给定路径中提取路径参数
    pub fn extract_params(&self, path: &str) -> Vec<(String, String)> {
        let mut params = Vec::new();
        let route_parts: Vec<&str> = self.path.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();

        for (route_part, path_part) in route_parts.iter().zip(path_parts.iter()) {
            if let Some(name) = route_part.strip_prefix(':') {
                params.push((name.to_string(), path_part.to_string()));
            }
        }

        params
    }
}

impl fmt::Debug for Route {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Route")
            .field("methods", &self.methods)
            .field("path", &self.path)
            .finish()
    }
}

/// Handler enum that can hold different types of handlers
/// 可以容纳不同类型处理程序的Handler枚举
#[derive(Clone)]
pub enum Handler {
    /// A function pointer handler
    /// 函数指针处理程序
    Fn(HandlerFn),
    /// A string reference to a function (for macro-generated code)
    /// 函数的字符串引用（用于宏生成的代码）
    Static(&'static str),
    /// Unimplemented handler
    /// 未实现的处理程序
    Unimplemented,
}

impl Handler {
    /// Create an unimplemented handler
    /// 创建未实现的处理程序
    pub fn unimplemented() -> Self {
        Self::Unimplemented
    }
}

impl fmt::Debug for Handler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fn(_) => write!(f, "Handler::Fn"),
            Self::Static(s) => write!(f, "Handler::Static({})", s),
            Self::Unimplemented => write!(f, "Handler::Unimplemented"),
        }
    }
}

impl Default for Handler {
    fn default() -> Self {
        Self::Unimplemented
    }
}

// Re-export Method
use crate::Method;
