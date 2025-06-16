use std::sync::Arc;
use longport::{QuoteContext, TradeContext};
use tokio::sync::mpsc;
use crate::models::market::MarketData;

pub trait Strategy {
    fn new(quote_ctx: Arc<QuoteContext>, trade_ctx: Arc<TradeContext>, quote_receiver: mpsc::Receiver<MarketData>) -> Self;
    async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    async fn execute(&mut self, event: &MarketData) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
