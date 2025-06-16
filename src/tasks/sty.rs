use std::sync::Arc;
use longport::{Config, QuoteContext, TradeContext};
use tokio::sync::mpsc;
use crate::collect::quote::QuoteCollectors;
use crate::config::config::Configs;
use crate::strategys::executor::Executor;
use crate::strategys::vecor_v1::VecorStrategy;

pub async fn start_sty(config: Configs) -> Result<(), Box<dyn std::error::Error>> {
    // 初始化长桥配置
    let cfg = Arc::new(Config::from_env().unwrap());

    // 创建 QuoteContext 和 TradeContext 实例
    let (quote_ctx, _) = QuoteContext::try_new(cfg.clone()).await.unwrap();
    let (trade_ctx, _) = TradeContext::try_new(cfg).await.unwrap();
    let mut symbols = Vec::new();
    for symbol in config.symbols {
        symbols.push(symbol.symbol.clone());
    }
    let (sender, receiver) = mpsc::channel(100);

    // 创建执行器
    let mut executor = Executor::<VecorStrategy>::new(
        Arc::new(quote_ctx),
        Arc::new(trade_ctx),
        receiver,
    );
    // 在单独的任务中运行执行器
    let executor_handle = tokio::spawn(async move {
        if let Err(e) = executor.run().await {
            eprintln!("Executor error: {}", e);
        }
    });
    // 异步执行收集器
    let mut collector = QuoteCollectors::new(symbols).await;
    collector.subscribe(sender).await;
    // 等待执行器完成
    executor_handle.await?;
    Ok(())
}
