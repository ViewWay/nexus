//! Contract module
//! 合约模块
//!
//! # Overview / 概述
//!
//! This module provides smart contract interaction.
/// 本模块提供智能合约交互。

// TODO: Implement in Phase 6
// 将在第6阶段实现

/// Smart contract interface
/// 智能合约接口
pub struct Contract<C> {
    _phantom: std::marker::PhantomData<C>,
}

/// Contract error
/// 合约错误
#[derive(Debug)]
pub enum ContractError {
    /// ABI error
    AbiError(String),
}
