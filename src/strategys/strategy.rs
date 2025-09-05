use std::sync::Arc;
use longport::{QuoteContext, TradeContext};
use crate::models::market::MarketData;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Strategy {
    fn new(quote_ctx: Arc<QuoteContext>, trade_ctx: Arc<TradeContext>) -> Self;
    async fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>>;
    async fn execute(&mut self, event: &MarketData) -> Result<(), Box<dyn Error + Send + Sync>>;
    fn stop(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
}
