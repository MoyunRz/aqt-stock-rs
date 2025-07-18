use std::error::Error;
use std::sync::Arc;
use log::error;
use longport::{QuoteContext, TradeContext};
use tokio::sync::mpsc;
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

    // 运行执行器，接收市场数据并传递给内部策略
    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // 首先初始化内部策略
        self.executor.run().await?;

        // 然后处理接收到的市场数据
        while let Some(event) = self.quote_receiver.recv().await {
            if let Err(e) = self.executor.execute(&event).await {
                error!("Error executing strategy: {:?}", e);
            }
        }
        // 最后停止内部策略
        self.executor.stop()?;

        Ok(())
    }
}