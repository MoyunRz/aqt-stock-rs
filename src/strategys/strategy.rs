use std::sync::Arc;
use longport::{QuoteContext, TradeContext};
use crate::config::config::Configs;
use crate::models::market::MarketData;

pub trait Strategy {
    fn new(quote_ctx: Arc<QuoteContext>, trade_ctx: Arc<TradeContext>) -> Self;
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn execute(&mut self, event: &MarketData) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
