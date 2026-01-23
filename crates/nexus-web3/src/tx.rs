//! Transaction module
//! 交易模块
//!
//! # Overview / 概述
//!
//! This module provides transaction management.
//! 本模块提供交易管理。

// TODO: Implement in Phase 6 / 将在第6阶段实现

/// Transaction hash
/// 交易哈希
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TxHash(pub [u8; 32]);

/// Transaction
/// 交易
#[derive(Clone, Debug)]
pub struct Transaction {
    _phantom: std::marker::PhantomData<()>,
}

/// Transaction builder
/// 交易构建器
#[derive(Clone, Debug)]
pub struct TransactionBuilder {
    _phantom: std::marker::PhantomData<()>,
}
