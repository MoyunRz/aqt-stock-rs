use std::sync::Arc;
use longport::{QuoteContext, TradeContext};
use tokio::sync::mpsc;
use crate::models::market::MarketData;
use crate::strategys::strategy::{ Strategy};

// 创建一个简单的策略实现
struct VecorStrategy {
    quote_ctx: Arc<QuoteContext>,
    trade_ctx: Arc<TradeContext>,
    quote_receiver: mpsc::Receiver<MarketData>,
}

impl Strategy for VecorStrategy {
    fn new(
        quote_ctx: Arc<QuoteContext>,
        trade_ctx: Arc<TradeContext>,
        quote_receiver: mpsc::Receiver<MarketData>,
    ) -> Self {
        VecorStrategy {
            quote_ctx,
            trade_ctx,
            quote_receiver,
        }
    }

    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 模拟运行逻辑
        Ok(())
    }

    async fn execute(&mut self, event: &MarketData) -> Result<(), Box<dyn std::error::Error>> {
        // 模拟执行逻辑
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 模拟停止逻辑
        Ok(())
    }
}