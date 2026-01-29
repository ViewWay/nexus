//! 自动配置加载器 / Auto-Configuration Loader
//!
//! 从 META-INF/nexus/autoconfiguration.imports 加载自动配置类。
//! Loads auto-configuration classes from META-INF/nexus/autoconfiguration.imports.
//!
//! # 功能 / Features
//!
//! - 从类路径资源文件加载配置类列表
//! - 按优先级排序配置类
//! - 支持条件评估
//! - 处理配置间依赖关系
//!
//! # 使用方式 / Usage
//!
//! ```rust,ignore
//! use nexus_starter::core::AutoConfigurationLoader;
//!
//! let loader = AutoConfigurationLoader::new();
//! let configs = loader.load_configurations()?;
//! ```

use std::path::Path;
use anyhow::{Context, Result};

// ============================================================================
// AutoConfigurationLoader / 自动配置加载器
// ============================================================================

/// 自动配置加载器
/// Auto-configuration loader
///
/// 负责从 `META-INF/nexus/autoconfiguration.imports` 文件加载自动配置类列表。
/// Responsible for loading auto-configuration class list from
/// `META-INF/nexus/autoconfiguration.imports` file.
///
/// # 示例 / Example
///
/// ```rust,ignore
/// use nexus_starter::core::AutoConfigurationLoader;
///
/// let loader = AutoConfigurationLoader::new();
/// let configs = loader.load_configurations()?;
/// ```
#[derive(Debug, Clone)]
pub struct AutoConfigurationLoader {
    /// 搜索路径
    /// Search paths
    search_paths: Vec<String>,
}

impl AutoConfigurationLoader {
    /// 创建新的加载器
    /// Create a new loader
    ///
    /// # 示例 / Example
    ///
    /// ```rust
    /// use nexus_starter::core::AutoConfigurationLoader;
    ///
    /// let loader = AutoConfigurationLoader::new();
    /// ```
    pub fn new() -> Self {
        Self {
            search_paths: vec![
                ".".to_string(),
                "resources".to_string(),
                "src/resources".to_string(),
            ],
        }
    }

    /// 添加搜索路径
    /// Add search path
    ///
    /// # 参数 / Parameters
    ///
    /// - `path`: 要添加的搜索路径 / Search path to add
    ///
    /// # 示例 / Example
    ///
    /// ```rust
    /// use nexus_starter::core::AutoConfigurationLoader;
    ///
    /// let loader = AutoConfigurationLoader::new()
    ///     .add_search_path("config");
    /// ```
    pub fn add_search_path(mut self, path: impl Into<String>) -> Self {
        self.search_paths.push(path.into());
        self
    }

    /// 设置搜索路径
    /// Set search paths
    ///
    /// # 参数 / Parameters
    ///
    /// - `paths`: 搜索路径列表 / List of search paths
    pub fn with_search_paths(mut self, paths: Vec<String>) -> Self {
        self.search_paths = paths;
        self
    }

    /// 从默认位置加载自动配置列表
    /// Load auto-configuration list from default location
    ///
    /// 默认从 `META-INF/nexus/autoconfiguration.imports` 读取。
    /// Reads from `META-INF/nexus/autoconfiguration.imports` by default.
    ///
    /// # 返回 / Returns
    ///
    /// 返回配置类的全限定名列表。
    /// Returns a list of fully qualified configuration class names.
    ///
    /// # 示例 / Example
    ///
    /// ```rust,ignore
    /// let loader = AutoConfigurationLoader::new();
    /// let configs = loader.load_configurations()?;
    /// ```
    pub fn load_configurations(&self) -> Result<Vec<String>> {
        let meta_inf_path = "META-INF/nexus/autoconfiguration.imports";
        self.load_from_file(meta_inf_path)
    }

    /// 从指定文件加载配置列表
    /// Load configuration list from specified file
    ///
    /// # 参数 / Parameters
    ///
    /// - `file`: 相对于搜索路径的文件名 / File name relative to search paths
    ///
    /// # 返回 / Returns
    ///
    /// 返回配置类的全限定名列表（去除注释和空行）。
    /// Returns a list of fully qualified configuration class names
    /// (comments and empty lines removed).
    ///
    /// # 示例 / Example
    ///
    /// ```rust,ignore
    /// let loader = AutoConfigurationLoader::new();
    /// let configs = loader.load_from_file("custom-config.imports")?;
    /// ```
    pub fn load_from_file(&self, file: &str) -> Result<Vec<String>> {
        // 在所有搜索路径中查找文件
        // Search for the file in all search paths
        for search_path in &self.search_paths {
            let full_path = Path::new(search_path).join(file);
            if full_path.exists() {
                return self.parse_config_file(&full_path);
            }
        }

        // 如果没找到，返回空列表（不是错误）
        // Return empty list if not found (not an error)
        tracing::debug!(
            "Auto-configuration file not found: {} (searched in: {:?})",
            file,
            self.search_paths
        );
        Ok(Vec::new())
    }

    /// 解析配置文件
    /// Parse configuration file
    ///
    /// # 参数 / Parameters
    ///
    /// - `path`: 配置文件的完整路径 / Full path to the configuration file
    ///
    /// # 返回 / Returns
    ///
    /// 返回解析后的配置类名称列表。
    /// Returns parsed list of configuration class names.
    fn parse_config_file(&self, path: &Path) -> Result<Vec<String>> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read auto-configuration file: {:?}", path))?;

        let mut configs = Vec::new();

        for line in content.lines() {
            let line = line.trim();

            // 跳过空行和注释
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // 跳过注释符号后还有空格的情况
            // Skip lines that are just comments with whitespace
            if line.starts_with("//") {
                continue;
            }

            // 添加有效的配置类名
            // Add valid configuration class name
            configs.push(line.to_string());
        }

        tracing::debug!(
            "Loaded {} auto-configuration classes from {:?}",
            configs.len(),
            path
        );

        Ok(configs)
    }

    /// 验证配置类名称格式
    /// Validate configuration class name format
    ///
    /// # 参数 / Parameters
    ///
    /// - `class_name`: 要验证的类名 / Class name to validate
    ///
    /// # 返回 / Returns
    ///
    /// 返回 `true` 如果格式正确，否则返回 `false`。
    /// Returns `true` if the format is valid, `false` otherwise.
    pub fn is_valid_class_name(&self, class_name: &str) -> bool {
        // 基本格式验证：应该是类似 `module::path::ClassName` 的形式
        // Basic format validation: should be like `module::path::ClassName`
        if class_name.is_empty() {
            return false;
        }

        // 检查是否包含非法字符
        // Check for illegal characters
        for ch in class_name.chars() {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | ':' | '_' | '.' => continue,
                _ => return false,
            }
        }

        // 检查双冒号格式
        // Check double colon format
        if class_name.contains("::") {
            return true;
        }

        false
    }
}

impl Default for AutoConfigurationLoader {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// AutoConfigurationRegistry / 自动配置注册表
// ============================================================================

/// 自动配置注册表
/// Auto-configuration registry
///
/// 管理所有已加载的自动配置类。
/// Manages all loaded auto-configuration classes.
///
/// # 示例 / Example
///
/// ```rust,ignore
/// use nexus_starter::core::AutoConfigurationRegistry;
///
/// let registry = AutoConfigurationRegistry::new();
/// registry.load_from_defaults()?;
/// ```
#[derive(Debug)]
pub struct AutoConfigurationRegistry {
    /// 已注册的配置类（按优先级排序）
    /// Registered configuration classes (sorted by priority)
    configurations: Vec<AutoConfigurationEntry>,
}

/// 自动配置条目
/// Auto-configuration entry
#[derive(Debug, Clone)]
struct AutoConfigurationEntry {
    /// 配置类名称
    /// Configuration class name
    name: String,

    /// 优先级（数字越小优先级越高）
    /// Priority (lower number = higher priority)
    order: i32,
}

impl AutoConfigurationRegistry {
    /// 创建新的注册表
    /// Create a new registry
    pub fn new() -> Self {
        Self {
            configurations: Vec::new(),
        }
    }

    /// 从默认位置加载配置
    /// Load configurations from default location
    ///
    /// # 示例 / Example
    ///
    /// ```rust,ignore
    /// use nexus_starter::core::AutoConfigurationRegistry;
    ///
    /// let registry = AutoConfigurationRegistry::new();
    /// registry.load_from_defaults()?;
    /// ```
    pub fn load_from_defaults(&mut self) -> Result<usize> {
        let loader = AutoConfigurationLoader::new();
        let configs = loader.load_configurations()?;
        self.register_all(configs)
    }

    /// 注册配置类列表
    /// Register a list of configuration classes
    ///
    /// # 参数 / Parameters
    ///
    /// - `configs`: 配置类名称列表 / List of configuration class names
    ///
    /// # 返回 / Returns
    ///
    /// 返回注册的配置类数量。
    /// Returns the number of registered configuration classes.
    pub fn register_all(&mut self, configs: Vec<String>) -> Result<usize> {
        let count = configs.len();
        for config in configs {
            self.register(config)?;
        }
        Ok(count)
    }

    /// 注册单个配置类
    /// Register a single configuration class
    ///
    /// # 参数 / Parameters
    ///
    /// - `class_name`: 配置类的全限定名 / Fully qualified configuration class name
    pub fn register(&mut self, class_name: String) -> Result<()> {
        // 提取优先级（如果类名包含优先级注释）
        // Extract priority (if class name contains priority comment)
        // 格式：# priority: 100
        let order = Self::extract_priority(&class_name).unwrap_or(0);

        self.configurations.push(AutoConfigurationEntry {
            name: class_name,
            order,
        });

        Ok(())
    }

    /// 从类名提取优先级
    /// Extract priority from class name
    ///
    /// # 参数 / Parameters
    ///
    /// - `class_name`: 配置类名称 / Configuration class name
    ///
    /// # 返回 / Returns
    ///
    /// 返回提取的优先级，如果没有找到则返回 `None`。
    /// Returns extracted priority, or `None` if not found.
    fn extract_priority(class_name: &str) -> Option<i32> {
        // 简单实现：从注释中提取优先级
        // Simple implementation: extract priority from comments
        // TODO: 实现更复杂的优先级提取逻辑
        // TODO: Implement more complex priority extraction logic
        None
    }

    /// 获取排序后的配置列表
    /// Get sorted configuration list
    ///
    /// # 返回 / Returns
    ///
    /// 返回按优先级排序的配置类名称列表。
    /// Returns a list of configuration class names sorted by priority.
    pub fn get_sorted(&self) -> Vec<String> {
        let mut entries = self.configurations.clone();
        entries.sort_by_key(|e| e.order);
        entries.into_iter().map(|e| e.name).collect()
    }

    /// 获取配置数量
    /// Get configuration count
    pub fn len(&self) -> usize {
        self.configurations.len()
    }

    /// 检查是否为空
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.configurations.is_empty()
    }
}

impl Default for AutoConfigurationRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 测试 / Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_new() {
        let loader = AutoConfigurationLoader::new();
        assert_eq!(loader.search_paths.len(), 3);
    }

    #[test]
    fn test_loader_add_search_path() {
        let loader = AutoConfigurationLoader::new().add_search_path("custom");
        assert_eq!(loader.search_paths.len(), 4);
    }

    #[test]
    fn test_loader_with_search_paths() {
        let loader = AutoConfigurationLoader::new()
            .with_search_paths(vec!["path1".to_string(), "path2".to_string()]);
        assert_eq!(loader.search_paths.len(), 2);
    }

    #[test]
    fn test_is_valid_class_name() {
        let loader = AutoConfigurationLoader::new();
        assert!(loader.is_valid_class_name("module::ClassName"));
        assert!(loader.is_valid_class_name("a::b::c::ClassName"));
        assert!(!loader.is_valid_class_name(""));
        assert!(!loader.is_valid_class_name("invalid@name"));
    }

    #[test]
    fn test_registry_new() {
        let registry = AutoConfigurationRegistry::new();
        assert!(registry.is_empty());
        assert_eq!(registry.len(), 0);
    }

    #[test]
    fn test_registry_register() {
        let mut registry = AutoConfigurationRegistry::new();
        registry.register("module::TestConfig".to_string()).unwrap();
        assert_eq!(registry.len(), 1);
    }

    #[test]
    fn test_registry_register_all() {
        let mut registry = AutoConfigurationRegistry::new();
        let configs = vec![
            "module::Config1".to_string(),
            "module::Config2".to_string(),
        ];
        let count = registry.register_all(configs).unwrap();
        assert_eq!(count, 2);
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn test_registry_get_sorted() {
        let mut registry = AutoConfigurationRegistry::new();
        registry.register("module::Config1".to_string()).unwrap();
        registry.register("module::Config2".to_string()).unwrap();
        let sorted = registry.get_sorted();
        assert_eq!(sorted.len(), 2);
    }
}
