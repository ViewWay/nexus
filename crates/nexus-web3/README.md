# nexus-web3

[![Crates.io](https://img.shields.io/crates/v/nexus-web3)](https://crates.io/crates/nexus-web3)
[![Documentation](https://docs.rs/nexus-web3/badge.svg)](https://docs.rs/nexus-web3)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](../../LICENSE)

> Blockchain and Web3 support for Nexus framework
> 
> Nexus框架的区块链和Web3支持

---

## 📋 Overview / 概述

`nexus-web3` provides blockchain and Web3 functionality including smart contract interaction, wallet management, and transaction handling.

`nexus-web3` 提供区块链和Web3功能，包括智能合约交互、钱包管理和交易处理。

**Key Features** / **核心特性**:
- ✅ **Smart Contracts** - Contract interaction
- ✅ **Wallet Management** - Local and hardware wallets
- ✅ **Transaction Handling** - Sign and send transactions
- ✅ **Multi-chain** - Support for multiple blockchains
- ✅ **RPC Client** - Ethereum JSON-RPC

---

## 🚀 Quick Start / 快速开始

### Installation / 安装

```toml
[dependencies]
nexus-web3 = "0.1.0-alpha"
```

### Basic Usage / 基本用法

```rust
use nexus_web3::{Chain, Wallet, Contract, RpcClient};

// Connect to Ethereum / 连接到以太坊
let chain = Chain::ethereum();
let rpc = RpcClient::new("https://eth-mainnet.g.alchemy.com/v2/YOUR_KEY")?;

// Create wallet / 创建钱包
let wallet = Wallet::from_private_key(private_key)?;

// Interact with contract / 与合约交互
let contract = Contract::new(contract_address, abi, &rpc)?;
let result = contract.call("balanceOf", &[wallet.address()]).await?;
```

---

## 📖 Web3 Features / Web3功能

### Wallet Management / 钱包管理

```rust
use nexus_web3::{Wallet, LocalWallet};

// Create new wallet / 创建新钱包
let wallet = Wallet::random();

// From private key / 从私钥
let wallet = Wallet::from_private_key(hex::decode(private_key)?)?;

// From mnemonic / 从助记词
let wallet = Wallet::from_mnemonic(mnemonic)?;

// Sign message / 签名消息
let signature = wallet.sign_message(message).await?;
```

### Smart Contracts / 智能合约

```rust
use nexus_web3::Contract;

// Deploy contract / 部署合约
let contract = Contract::deploy(bytecode, abi, &rpc, &wallet).await?;

// Call view function / 调用视图函数
let balance: U256 = contract.call("balanceOf", &[address]).await?;

// Send transaction / 发送交易
let tx_hash = contract.send("transfer", &[to, amount], &wallet).await?;
```

### Transaction Handling / 交易处理

```rust
use nexus_web3::{Transaction, TransactionBuilder};

// Build transaction / 构建交易
let tx = TransactionBuilder::new()
    .to(recipient)
    .value(amount)
    .gas_limit(21000)
    .build();

// Sign and send / 签名并发送
let signed = wallet.sign_transaction(tx).await?;
let tx_hash = rpc.send_transaction(signed).await?;
```

---

## 🚦 Roadmap / 路线图

### Phase 6: Web3 Support 🔄 (In Progress / 进行中)
- [ ] Ethereum support
- [ ] Smart contract interaction
- [ ] Wallet management
- [ ] Multi-chain support

---

## 📚 Documentation / 文档

- **API Documentation**: [docs.rs/nexus-web3](https://docs.rs/nexus-web3)
- **Book**: [Web3 Guide](../../docs/book/src/advanced/web3.md)

---

**Built with ❤️ for Web3**

**为Web3构建 ❤️**
