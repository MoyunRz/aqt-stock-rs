use std::sync::Arc;
use longport::{QuoteContext, TradeContext};
use crate::models::market::MarketData;
use crate::strategys::strategy::{ Strategy};

// 创建一个简单的策略实现
pub struct VecorStrategy {
    quote_ctx: Arc<QuoteContext>,
    trade_ctx: Arc<TradeContext>,
}

impl Strategy for VecorStrategy {
    fn new(
        quote_ctx: Arc<QuoteContext>,
        trade_ctx: Arc<TradeContext>,
    ) -> Self {
        VecorStrategy {
            quote_ctx,
            trade_ctx,
        }
    }

    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 模拟运行逻辑
        Ok(())
    }

    async fn execute(&mut self, event: &MarketData) -> Result<(), Box<dyn std::error::Error>> {
        
        
        
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}