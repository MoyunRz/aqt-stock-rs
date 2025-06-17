use std::sync::Arc;
use longport::{QuoteContext, TradeContext};
use crate::models::market::MarketData;

pub trait Strategy {
    fn new(quote_ctx: Arc<QuoteContext>, trade_ctx: Arc<TradeContext>) -> Self;
    fn run(&mut self) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
    fn execute(&mut self, event: &MarketData) -> impl std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send;
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
