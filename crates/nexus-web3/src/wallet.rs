//! Wallet module
//! 钱包模块
//!
//! # Overview / 概述
//!
//! This module provides wallet management.
/// 本模块提供钱包管理。

// TODO: Implement in Phase 6
// 将在第6阶段实现

/// Wallet trait
/// 钱包trait
pub trait Wallet: Send + Sync {
    /// Get wallet address
    /// 获取钱包地址
    fn address(&self) -> Address;

    /// Sign a transaction
    /// 签名交易
    fn sign_transaction(&self, tx: &mut Transaction) -> Result<Signature, WalletError>;
}

/// Local wallet implementation
/// 本地钱包实现
pub struct LocalWallet {
    _private_key: Vec<u8>,
}

/// Address type
/// 地址类型
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Address(pub [u8; 20]);

/// Transaction type
/// 交易类型
pub struct Transaction;

/// Signature type
/// 签名类型
pub struct Signature;

/// Wallet error
/// 钱包错误
#[derive(Debug)]
pub enum WalletError {
    /// Invalid key
    InvalidKey,
}
