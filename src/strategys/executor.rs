use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use log::{error, info, warn};
use longport::{QuoteContext, TradeContext};
use tokio::sync::{mpsc, Mutex};
use tokio::time::timeout;
use std::collections::HashMap;
use crate::models::market::MarketData;
use crate::strategys::strategy::Strategy;

pub struct Executor<T: Strategy> {
    executor: T,
    quote_receiver: mpsc::Receiver<MarketData>,
    symbol_locks: Mutex<HashMap<String, Arc<Mutex<()>>>>,
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
            symbol_locks: Mutex::new(HashMap::new()),
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        info!("Starting executor...");

        // Initialize internal strategy
        self.executor.run().await?;
        info!("Strategy initialized successfully");

        // Set 24 hour timeout (in seconds)
        let timeout_duration = Duration::from_secs(15 * 60); // 24 hours

        // Process received market data
        loop {
            match timeout(timeout_duration, self.quote_receiver.recv()).await {
                Ok(Some(event)) => {
                    // Get or create lock for the specific symbol
                    let symbol = event.symbol.clone(); // Assuming MarketData has a symbol field
                    let lock = {
                        let mut locks = self.symbol_locks.lock().await;
                        locks
                            .entry(symbol.clone())
                            .or_insert_with(|| Arc::new(Mutex::new(())))
                            .clone()
                    };

                    // Acquire lock for this symbol
                    let _guard = lock.lock().await;

                    // Execute strategy with acquired lock
                    if let Err(e) = self.executor.execute(&event).await {
                        error!("Error executing strategy for symbol {}: {:?}", symbol, e);
                        // Continue processing despite error
                    }
                }
                Ok(None) => {
                    info!("Channel closed, executor shutting down");
                    break;
                }
                Err(_) => {
                    warn!("No message received for 24 hours, exiting");
                    break;
                }
            }
        }

        info!("Stopping executor...");
        // Stop internal strategy
        self.executor.stop()?;
        info!("Executor stopped successfully");
        Ok(())
    }
}