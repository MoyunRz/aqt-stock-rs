use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use log::error;
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
        // 首先初始化内部策略
        self.executor.run().await?;

        // 设置 24 小时超时（以秒为单位）
        let timeout_duration = Duration::from_secs(24 * 60 * 60); // 24 hours

        // 处理接收到的市场数据
        loop {
            match timeout(timeout_duration, self.quote_receiver.recv()).await {
                Ok(Some(event)) => {
                    // 收到消息，执行策略
                    if let Err(e) = self.executor.execute(&event).await {
                        error!("Error executing strategy: {:?}", e);
                    }
                }
                Ok(None) => {
                    // 通道关闭，退出循环
                    break;
                }
                Err(_) => {
                    error!("No message received for 24 hours, exiting ");
                    break;
                }
            }
        }
        // 最后停止内部策略
        self.executor.stop()?;
        Ok(())
    }
}