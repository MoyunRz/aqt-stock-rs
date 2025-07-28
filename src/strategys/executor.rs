use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use log::{error, info, warn};
use longport::{QuoteContext, TradeContext};
use tokio::sync::mpsc;
use tokio::time::timeout;
use crate::models::market::MarketData;
use crate::strategys::strategy::Strategy;

pub struct Executor<T: Strategy> {
    executor: T,
    quote_receiver: mpsc::Receiver<MarketData>,
}

impl<T: Strategy + Send> Executor<T> {
    pub fn new(
        quote_ctx: Arc<QuoteContext>,
        trade_ctx: Arc<TradeContext>,
        quote_receiver: mpsc::Receiver<MarketData>,
    ) -> Self {
        Executor {
            executor: T::new(quote_ctx, trade_ctx),
            quote_receiver,
        }
    }


    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Starting executor...");
        
        // 首先初始化内部策略
        self.executor.run().await?;
        info!("Strategy initialized successfully");

        // 设置 24 小时超时（以秒为单位）
        let timeout_duration = Duration::from_secs(15 * 60); // 24 hours
        let mut message_count = 0;

        // 处理接收到的市场数据
        loop {
            match timeout(timeout_duration, self.quote_receiver.recv()).await {
                Ok(Some(event)) => {
                    message_count += 1;
                    if message_count % 100 == 0 {
                        info!("Processed {} market data messages", message_count);
                    }
                    
                    // 收到消息，执行策略
                    if let Err(e) = self.executor.execute(&event).await {
                        error!("Error executing strategy: {:?}", e);
                        // 不要因为单次执行错误就退出，继续处理
                    }
                }
                Ok(None) => {
                    info!("Channel closed, executor shutting down");
                    break;
                }
                Err(_) => {
                    warn!("No message received for 24 hours, exiting");
                    break;
                }
            }
        }
        
        info!("Stopping executor...");
        // 最后停止内部策略
        self.executor.stop()?;
        info!("Executor stopped successfully");
        Ok(())
    }
}