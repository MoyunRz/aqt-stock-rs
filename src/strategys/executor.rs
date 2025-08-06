use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use log::{error, info, warn};
use longport::{QuoteContext, TradeContext};
use tokio::sync::mpsc;
use tokio::time::timeout; // 添加 sleep 导入
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

    pub async fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Starting executor...");

        self.executor.run().await?;
        info!("Strategy initialized successfully");

        let timeout_duration = Duration::from_secs(15 * 60); // 15分钟超时
        

        loop {
            match timeout(timeout_duration, self.quote_receiver.recv()).await {
                Ok(Some(event)) => {
                    let symbol = event.symbol.clone();
                    let execute_result = self.executor.execute(&event).await;
                    if let Err(e) = execute_result {
                        error!("Error executing strategy for symbol {}: {:?}", symbol, e);
                    }
                }
                Ok(None) => {
                    info!("Channel closed, executor shutting down");
                    break;
                }
                Err(_) => {
                    warn!("No message received for 15 minutes, exiting");
                    break;
                }
            }
        }

        info!("Stopping executor...");
        self.executor.stop()?;
        info!("Executor stopped successfully");
        Ok(())
    }
}