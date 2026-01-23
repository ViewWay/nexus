//! Nexus Web3 - Blockchain and Web3 support
//! Nexus Web3 - 区块链和Web3支持
//!
//! # Overview / 概述
//!
//! `nexus-web3` provides blockchain and Web3 functionality including smart
//! contract interaction, wallet management, and transaction handling.
//!
//! `nexus-web3` 提供区块链和Web3功能，包括智能合约交互、钱包管理和交易处理。

#![warn(missing_docs)]
#![warn(unreachable_pub)]

pub mod chain;
pub mod contract;
pub mod wallet;
pub mod tx;
pub mod rpc;

pub use chain::{Chain, ChainId};
pub use contract::Contract;
pub use wallet::{Wallet, LocalWallet};
pub use tx::{Transaction, TransactionBuilder, TxHash};
pub use rpc::RpcClient;
