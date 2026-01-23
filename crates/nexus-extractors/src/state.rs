//! State extractor module
//! 状态提取器模块
//!
//! # Overview / 概述
//!
//! This module provides application state extraction.
//! 本模块提供应用状态提取。

use std::sync::Arc;

// TODO: Implement in Phase 2
// 将在第2阶段实现

/// State extractor
/// 状态提取器
pub struct State<T>(pub Arc<T>);
