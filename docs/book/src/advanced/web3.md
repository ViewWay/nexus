# Web3 Integration / Web3集成

> **Status**: Phase 6 Planned 📋  
> **状态**: 第6阶段计划中 📋

Nexus provides native Web3 support for blockchain applications.

Nexus 为区块链应用程序提供原生 Web3 支持。

---

## Overview / 概述

Web3 features include:

Web3 功能包括：

- **Smart Contract Interaction** / **智能合约交互** - Call and send transactions
- **Wallet Management** / **钱包管理** - Local and hardware wallets
- **Transaction Handling** / **交易处理** - Sign and send transactions
- **Multi-chain Support** / **多链支持** - Ethereum, Polygon, etc.

---

## Quick Start / 快速开始

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

## Features / 功能

### Wallet Management / 钱包管理

```rust
use nexus_web3::{Wallet, LocalWallet};

// Create new wallet / 创建新钱包
let wallet = Wallet::random();

// From private key / 从私钥
let wallet = Wallet::from_private_key(hex::decode(private_key)?)?;

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

---

## Roadmap / 路线图

### Phase 6: Web3 Support 📋 (Planned / 计划中)
- [ ] Ethereum support
- [ ] Smart contract interaction
- [ ] Wallet management
- [ ] Multi-chain support

---

*← [Previous / 上一页](./observability.md) | [Next / 下一页](./testing.md) →*
