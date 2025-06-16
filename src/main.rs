use longport::quote::SubFlags;
use aqt_stock::collect::quote::QuoteCollectors;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let symbols = vec![String::from("700.HK"), String::from("AAPL.US")];
    let mut collector = QuoteCollectors::new(symbols).await;
    collector.subscribe().await;
    Ok(())
}