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
pub mod subscribe;

pub use chain::{ChainId, Eip155Chain, ChainConfig, Block, BlockNumber};
pub use wallet::{Wallet, LocalWallet, Address};
pub use tx::{Transaction, TransactionBuilder, TxHash};
pub use rpc::RpcError;
pub use contract::{ContractError, FunctionSelector, CallParams};

#[cfg(feature = "rpc")]
pub use rpc::RpcClient;

#[cfg(feature = "rpc")]
pub use contract::{Contract, ContractCall, ERC20, ERC721};

#[cfg(feature = "ws")]
pub use subscribe::{
    WsClient, SubscriptionManager, SubscriptionType, SubscriptionId,
    SubscriptionNotification, LogFilter, NewBlockHeader, LogNotification,
    PendingTransaction, WsError
};
