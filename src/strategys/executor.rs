use std::sync::Arc;
use longport::{QuoteContext, TradeContext};
use tokio::sync::mpsc;
use crate::models::market::MarketData;
use crate::strategys::strategy::{ Strategy};

// 修改: 将 Executor 改为泛型实现，继承 Strategy
pub struct Executor<T: Strategy> {
    executor: T,
}

impl<T: Strategy> Executor<T> {
    pub fn new(
        quote_ctx: Arc<QuoteContext>,
        trade_ctx: Arc<TradeContext>,
        quote_receiver: mpsc::Receiver<MarketData>,
    ) -> Self {
        Executor {
            executor: T::new(quote_ctx, trade_ctx, quote_receiver),
        }
    }
}

// 实现 Strategy trait
impl<T: Strategy> Strategy for Executor<T> {
    fn new(
        quote_ctx: Arc<QuoteContext>,
        trade_ctx: Arc<TradeContext>,
        quote_receiver: mpsc::Receiver<MarketData>,
    ) -> Self {
        Executor {
            executor: T::new(quote_ctx, trade_ctx, quote_receiver),
        }
    }

    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.executor.run().await
    }

    async fn execute(&mut self, event: &MarketData) -> Result<(), Box<dyn std::error::Error>> {
        self.executor.execute(event).await
    }

    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.executor.stop()
    }
}


