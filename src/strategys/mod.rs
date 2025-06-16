use longport::{Config, QuoteContext, TradeContext};
use std::sync::Arc;
use tokio::sync::mpsc;
use crate::models::market::MarketData; // 新增：引入 MarketData 类型

pub struct StrategyExecutor {
    quote_ctx: Arc<QuoteContext>,
    trade_ctx: Arc<TradeContext>,
    quote_receiver: mpsc::Receiver<MarketData>, // 修改：使用 MarketData 类型
}

impl StrategyExecutor {
    pub fn new(quote_ctx: Arc<QuoteContext>, trade_ctx: Arc<TradeContext>, quote_receiver: mpsc::Receiver<MarketData>) -> Self {
        StrategyExecutor {
            quote_ctx,
            trade_ctx,
            quote_receiver,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(event) = self.quote_receiver.recv().await {
            // Process quote event and execute strategy
            self.execute_strategy(&event).await?;
        }
        Ok(())
    }

    async fn execute_strategy(&mut self, event: &MarketData) -> Result<(), Box<dyn std::error::Error>> {
        // Implement strategy logic here
        println!("Executing strategy for event: {:?}", event); // 示例策略逻辑
        Ok(())
    }
}