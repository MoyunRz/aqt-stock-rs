use std::sync::Arc;
use log::error;
use longport::{Config, QuoteContext, TradeContext};
use tokio::sync::mpsc;
use tokio::sync::oneshot;
use crate::collect::quote::QuoteCollectors;
use crate::config::config::Configs;
use crate::strategys::executor::Executor;
use crate::strategys::vecor_v1::VecorStrategy;

pub async fn start_sty(config: Configs) -> Result<(), Box<dyn std::error::Error>> {
   
    loop {
       // 初始化长桥配置
       let cfg = Arc::new(Config::from_env().unwrap());
       // 创建 QuoteContext 和 TradeContext 实例
       let (quote_ctx, _) = QuoteContext::try_new(cfg.clone()).await.unwrap();
       let (trade_ctx, _) = TradeContext::try_new(cfg).await.unwrap();
       let mut symbols = Vec::new();
       for symbol in config.symbols.clone() {
           symbols.push(symbol.symbol.clone());
       }
       
       // 创建通道，增加缓冲区大小
       let (sender, receiver) = mpsc::channel(2048);
       
       // 创建关闭信号通道
       let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
       
       // 创建执行器
       let mut executor = Executor::<VecorStrategy>::new(
           Arc::new(quote_ctx),
           Arc::new(trade_ctx),
           receiver,
       );

       // 在单独的任务中运行执行器
       let executor_handle = tokio::spawn(async move {
           // 循环执行
           if let Err(e) = executor.run().await {
               error!("Executor error: {}", e);
           }
       });
       
       // 异步执行收集器，传递关闭信号
       let collector_handle = tokio::spawn(async move {
           let mut collector = QuoteCollectors::new(symbols).await;
           collector.subscribe_with_shutdown(sender, shutdown_rx).await;
       });
       
       // 等待执行器完成
       executor_handle.await?;
       
       // 发送关闭信号给收集器
       let _ = shutdown_tx.send(());
       
       // 等待收集器完成
       collector_handle.await?;
   }
}
