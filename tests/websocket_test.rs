use tokio::sync::mpsc;
use aqt_stock::collect::brokers::BrokersCollectors;
use aqt_stock::collect::depth::DepthCollectors;
use aqt_stock::collect::quote::QuoteCollectors;
use aqt_stock::collect::trade::TradeCollectors;

#[tokio::test]
async fn test_quote_subscription() {
    let (sender, receiver) = mpsc::channel(100);
    let symbols = vec![String::from("700.HK"), String::from("AAPL.US")];
    let mut collector = QuoteCollectors::new(symbols).await;
    collector.subscribe(sender).await;
}

#[tokio::test]
async fn test_depth_subscription() {
    let symbols = vec![String::from("700.HK"), String::from("AAPL.US")];
    let mut collector = DepthCollectors::new(symbols).await;
    collector.subscribe().await;
}

#[tokio::test]
async fn test_brokers_subscription() {
    let symbols = vec![String::from("700.HK"), String::from("AAPL.US")];
    let mut collector = BrokersCollectors::new(symbols).await;
    collector.subscribe().await;
}