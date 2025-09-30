use crate::collect::quote::QuoteCollectors;
use crate::config::config::Configs;
use crate::strategys::executor::Executor;
use crate::strategys::vecor_v1::VecorStrategy;
use log::{error, info};
use longport::{Config, QuoteContext, TradeContext};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::oneshot;

pub async fn start_sty(config: Configs) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        info!("初始化长桥配置");
        // 初始化长桥配置
        let cfg = Arc::new(Config::from_env().unwrap());
        // 创建 QuoteContext 和 TradeContext 实例
        let quote_res = QuoteContext::try_new(cfg.clone()).await;
        let quote_ctx: Arc<QuoteContext>;
        match quote_res {
            Ok(quotes) => {
                info!("初始化长桥行情成功");
                quote_ctx = Arc::new(quotes.0);
            }
            Err(e) => {
                error!("初始化长桥行情失败: {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                continue;
            }
        }
        let trade_res = TradeContext::try_new(cfg.clone()).await;

        let trade_ctx: Arc<TradeContext>;
        match trade_res {
            Ok(trades) => {
                info!("初始化长桥行情成功");
                trade_ctx = Arc::new(trades.0);
            }
            Err(e) => {
                error!("初始化长桥行情失败: {}", e);
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                continue;
            }
        }
        let mut symbols = Vec::new();
        for symbol in config.symbols.clone() {
            symbols.push(symbol.symbol.clone());
        }

        // 创建通道，增加缓冲区大小
        let (sender, receiver) = mpsc::channel(2048);

        // 创建关闭信号通道
        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        // 创建执行器
        let mut executor =
            Executor::<VecorStrategy>::new(quote_ctx, trade_ctx, receiver);

        // 异步执行收集器，传递关闭信号
        let collector_handle = tokio::spawn(async move {
            let mut collector = QuoteCollectors::new(symbols).await;
            collector.subscribe_with_shutdown(sender, shutdown_rx).await;
        });

        // 在单独的任务中运行执行器
        let executor_handle = tokio::spawn(async move {
            // 循环执行
            if let Err(e) = executor.run().await {
                error!("Executor error: {}", e);
            }
            // 发送关闭信号给收集器
            let _ = shutdown_tx.send(());
        });
        // 等待执行器完成
        executor_handle.await?;

        // 等待收集器完成（需要调整代码结构以便能访问 collector_handle）
        if let Err(e) = collector_handle.await {
            error!("Collector handle error: {}", e);
        }
        // 休眠
        tokio::time::sleep(std::time::Duration::from_secs(15)).await;
    }
}
