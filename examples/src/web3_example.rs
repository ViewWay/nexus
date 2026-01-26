//! Web3 Example / Web3示例
//!
//! Demonstrates Web3 functionality including wallet management,
//! blockchain interaction, and smart contract calls.
//!
//! 演示Web3功能，包括钱包管理、区块链交互和智能合约调用。

use nexus_web3::{
    Address, BlockNumber, ChainConfig, ChainId, Contract, Eip155Chain, FunctionSelector,
    LocalWallet, RpcClient, TransactionBuilder, TxType, Wallet,
};

#[cfg(feature = "ws")]
use nexus_web3::{LogFilter, SubscriptionManager, SubscriptionType, WsClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger / 初始化日志
    env_logger::init();

    println!("=== Nexus Web3 Example / Nexus Web3示例 ===\n");

    // 1. Chain Configuration / 链配置
    println!("1. Chain Configuration / 链配置");
    println!("---");

    let mainnet = ChainConfig::ethereum_mainnet();
    println!("Ethereum Mainnet: Chain ID = {}", mainnet.chain_id.0);

    let sepolia = ChainConfig::sepolia_testnet();
    println!("Sepolia Testnet: Chain ID = {}", sepolia.chain_id.0);

    let polygon = ChainConfig::polygon();
    println!("Polygon: Chain ID = {}", polygon.chain_id.0);

    // Custom chain / 自定义链
    let custom = ChainConfig::new(
        Eip155Chain::custom(12345),
        "My Custom Chain",
        vec!["https://rpc.example.com".to_string()],
    );
    println!("Custom Chain: Chain ID = {}", custom.chain_id.0);
    println!();

    // 2. Wallet Management / 钱包管理
    println!("2. Wallet Management / 钱包管理");
    println!("---");

    let wallet = LocalWallet::new(Eip155Chain::ETHEREUM);
    let address = wallet.address();

    println!("Generated Address / 生成的地址: {}", address.to_checksummed());
    println!("Address (hex) / 地址（十六进制）: {}", address.to_hex());
    println!("Is Zero / 是否为零地址: {}", address.is_zero());
    println!();

    // 3. Signing / 签名
    println!("3. Message Signing / 消息签名");
    println!("---");

    let message = b"Hello, Nexus Web3!";
    println!("Message / 消息: {:?}", std::str::from_utf8(message).unwrap());

    match wallet.sign(message) {
        Ok(signature) => {
            println!("Signature / 签名:");
            println!("  r: {}", hex::encode(signature.r()));
            println!("  s: {}", hex::encode(signature.s()));
            println!("  v: {}", signature.v());
        },
        Err(e) => println!("Signing error / 签名错误: {}", e),
    }
    println!();

    // 4. Function Selectors / 函数选择器
    println!("4. Function Selectors / 函数选择器");
    println!("---");

    let balance_selector = FunctionSelector::from_signature("balanceOf(address)");
    println!("balanceOf selector: {}", balance_selector.to_hex());

    let transfer_selector = FunctionSelector::from_signature("transfer(address,uint256)");
    println!("transfer selector: {}", transfer_selector.to_hex());

    // Using ERC20 constants / 使用ERC20常量
    use nexus_web3::ERC20;
    println!("ERC20.BALANCE_OF: {}", ERC20::BALANCE_OF.to_hex());
    println!("ERC20.TRANSFER: {}", ERC20::TRANSFER.to_hex());
    println!("ERC20.APPROVE: {}", ERC20::APPROVE.to_hex());
    println!();

    // 5. Transaction Building / 交易构建
    println!("5. Transaction Building / 交易构建");
    println!("---");

    let tx = TransactionBuilder::new()
        .ty(TxType::Eip1559)
        .chain_id(1)
        .nonce(0)
        .max_priority_fee_per_gas(1_500_000_000) // 1.5 Gwei
        .max_fee_per_gas(30_000_000_000)        // 30 Gwei
        .gas_limit(21_000)
        .to(Some(address))
        .value(1000000000000000) // 0.001 ETH
        .data(vec![])
        .build()?;

    println!("Transaction / 交易:");
    println!("  Type / 类型: {:?}", tx.ty());
    println!("  Chain ID / 链ID: {}", tx.chain_id());
    println!("  To / 接收方: {}", tx.to().map(|a| a.to_hex()).unwrap_or_default());
    println!("  Value / 金额: {} wei", tx.value().to_string());
    println!();

    // 6. RPC Client (commented out - requires actual RPC connection)
    // 6. RPC客户端（注释掉 - 需要实际的RPC连接）
    println!("6. RPC Client / RPC客户端");
    println!("---");
    println!("RPC client requires an actual blockchain node connection.");
    println!("Uncomment the code below and add your RPC URL to test.");
    println!("RPC客户端需要连接到实际的区块链节点。");
    println!("取消下面的代码注释并添加您的RPC URL进行测试。");
    println!();

    /*
    let rpc_url = std::env::var("ETH_RPC_URL")
        .unwrap_or_else(|_| "https://eth.llamarpc.com".to_string());
    let rpc = RpcClient::new(&rpc_url)?;

    // Get block number / 获取区块号
    match rpc.get_block_number().await {
        Ok(block_number) => println!("Latest block / 最新区块: {}", block_number),
        Err(e) => println!("Error getting block number / 获取区块号错误: {}", e),
    }

    // Get balance / 获取余额
    match rpc.get_balance(&address, BlockNumber::Latest).await {
        Ok(balance) => println!("Balance / 余额: {} ETH", balance),
        Err(e) => println!("Error getting balance / 获取余额错误: {}", e),
    }

    // Get transaction count / 获取交易计数
    match rpc.get_transaction_count(&address, BlockNumber::Latest).await {
        Ok(nonce) => println!("Nonce / 交易计数: {}", nonce),
        Err(e) => println!("Error getting nonce / 获取nonce错误: {}", e),
    }

    // Contract call / 合约调用
    let usdc_address = Address::from_hex("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48")?;
    let contract = Contract::new(usdc_address, &rpc);

    match contract.call_read_only(&ERC20::BALANCE_OF, &[]).await {
        Ok(result) => println!("Contract call result / 合约调用结果: {:?}", result),
        Err(e) => println!("Contract call error / 合约调用错误: {}", e),
    }
    */

    // 7. ERC721 NFT Example / ERC721 NFT示例
    println!("7. ERC721 NFT Interface / ERC721 NFT接口");
    println!("---");

    use nexus_web3::ERC721;
    println!("ERC721.OWNER_OF: {}", ERC721::OWNER_OF.to_hex());
    println!("ERC721.TRANSFER_FROM: {}", ERC721::TRANSFER_FROM.to_hex());
    println!("ERC721.SAFE_TRANSFER_FROM: {}", ERC721::SAFE_TRANSFER_FROM.to_hex());
    println!();

    // 8. WebSocket Subscriptions (when ws feature is enabled)
    // 8. WebSocket订阅（启用ws功能时）
    #[cfg(feature = "ws")]
    {
        println!("8. WebSocket Subscriptions / WebSocket订阅");
        println!("---");
        println!("WebSocket subscriptions allow real-time event monitoring.");
        println!("WebSocket订阅允许实时事件监控。");
        println!();

        println!("Subscription types / 订阅类型:");
        println!("  - NewHeads: Subscribe to new blocks / 订阅新区块");
        println!("  - PendingTransactions: Subscribe to mempool / 订阅内存池");
        println!("  - Logs: Subscribe to contract events / 订阅合约事件");
        println!();

        println!("Example: Log filter for contract events / 示例：合约事件的日志过滤器");
        let filter = LogFilter::new().address(&address).topic(
            "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef".to_string(),
        ); // Transfer event
        println!("  Filter configured for address: {}", address.to_checksummed());
        println!();

        println!("Note: Actual WebSocket connection requires a real node.");
        println!("注意：实际的WebSocket连接需要真实的节点。");
        println!("Uncomment below to test with a real WebSocket endpoint:");
        println!("取消下面的注释以使用真实的WebSocket端点测试：");
        println!();

        /*
        let ws_url = std::env::var("ETH_WS_URL")
            .unwrap_or_else(|_| "wss://eth.llamarpc.com".to_string());

        // Subscribe to new blocks / 订阅新区块
        let client = WsClient::connect(&ws_url).await?;
        let mut blocks = client.subscribe_blocks().await?;

        println!("Listening for new blocks... / 正在监听新区块...");
        while let Some(block) = blocks.next().await {
            println!("New block / 新区块: {} (hash: {})",
                block.number_as_u64().unwrap_or(0),
                block.hash
            );
        }
        */

        println!();
    }

    #[cfg(not(feature = "ws"))]
    {
        println!("8. WebSocket Subscriptions / WebSocket订阅");
        println!("---");
        println!("Enable the 'ws' feature to use WebSocket subscriptions.");
        println!("启用'ws'功能以使用WebSocket订阅。");
        println!("Run with: cargo run --bin web3_example --features ws");
        println!();
    }

    println!("=== Example Complete / 示例完成 ===");

    Ok(())
}
