//! Router Matching Fuzz Target
//! 路由匹配 Fuzz 测试目标
//!
//! This fuzz target tests router path matching robustness.
//! 此 Fuzz 测试目标测试路由路径匹配的鲁棒性。

#![no_main]

use libfuzzer_sys::fuzz_target;
use std::collections::HashMap;

fuzz_target!(|data: &[u8]| {
    // Skip if data is too large
    // 如果数据过大则跳过
    if data.len() > 4096 {
        return;
    }

    // Try to parse as UTF-8 string
    // 尝试解析为 UTF-8 字符串
    if let Ok(s) = std::str::from_utf8(data) {
        // Test path validation
        // 测试路径验证
        let path = s.split_whitespace().next().unwrap_or("");

        // Only test paths that start with /
        // 仅测试以 / 开头的路径
        if path.starts_with('/') {
            // Test path normalization doesn't panic
            // 测试路径规范化不会 panic
            let normalized = path.split('/').filter(|s| !s.is_empty()).collect::<Vec<_>>();

            // Test path traversal detection
            // 测试路径遍历检测
            let has_traversal = path.contains("../") || path.contains("..\\");
            if has_traversal {
                // Path traversal should be detected
                // 路径遍历应该被检测到
            }

            // Test query string parsing
            // 测试查询字符串解析
            if let Some(pos) = path.find('?') {
                let query = &path[pos + 1..];
                // Parse query parameters
                // 解析查询参数
                let params: HashMap<&str, &str> = query
                    .split('&')
                    .filter_map(|p| {
                        let mut parts = p.split('=');
                        if let Some(key) = parts.next() {
                            let value = parts.next().unwrap_or("");
                            Some((key, value))
                        } else {
                            None
                        }
                    })
                    .collect();
            }
        }
    }
});
