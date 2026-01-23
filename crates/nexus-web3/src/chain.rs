//! Chain module
//! 链模块
//!
//! # Overview / 概述
//!
//! This module provides blockchain abstraction.
/// 本模块提供区块链抽象。

// TODO: Implement in Phase 6
// 将在第6阶段实现

/// Chain identifier
/// 链标识符
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChainId {
    Ethereum,
    Polygon,
    Bsc,
    Arbitrum,
    Optimism,
}

/// Chain trait
/// 链trait
pub trait Chain: Send + Sync {
    /// Get chain identifier
    /// 获取链标识符
    fn chain_id(&self) -> ChainId;

    /// Get current block number
    /// 获取当前区块号
    async fn block_number(&self) -> Result<u64, ChainError>;
}

/// Chain error
/// 链错误
#[derive(Debug)]
pub enum ChainError {
    /// RPC error
    RpcError(String),
}
