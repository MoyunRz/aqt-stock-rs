use std::sync::Arc;
use aqt_stock::strategys::strategy::Strategy;
use longport::{Config, QuoteContext, TradeContext};
use tokio::sync::mpsc;
use aqt_stock::models::market::MarketData;
use aqt_stock::strategys::executor::Executor;

// 新增: 封装 Executor 创建逻辑的函数
fn create_executor<T: Strategy>(
    quote_ctx: Arc<QuoteContext>,
    trade_ctx: Arc<TradeContext>,
    receiver: mpsc::Receiver<MarketData>,
) -> Executor<T> {
    Executor::<T>::new(quote_ctx, trade_ctx, receiver)
}

#[tokio::test]
async fn test_vecor_executor() {
    // 初始化长桥配置
    let config = Arc::new(Config::from_env().unwrap());

    // 创建 QuoteContext 和 TradeContext 实例
    let (quote_ctx, _) = QuoteContext::try_new(config.clone()).await.unwrap();
    let (trade_ctx, _) = TradeContext::try_new(config).await.unwrap();

    // 创建一个简单的策略实现
    struct MockStrategy {
        quote_ctx: Arc<QuoteContext>,
        trade_ctx: Arc<TradeContext>,
        quote_receiver: mpsc::Receiver<MarketData>,
    }
    
    impl Strategy for MockStrategy {
        fn new(
            quote_ctx: Arc<QuoteContext>,
            trade_ctx: Arc<TradeContext>,
            quote_receiver: mpsc::Receiver<MarketData>,
        ) -> Self {
            MockStrategy {
                quote_ctx,
                trade_ctx,
                quote_receiver,
            }
        }

        async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            // 模拟运行逻辑
            Ok(())
        }

        async fn execute(&mut self, event: &MarketData) -> Result<(), Box<dyn std::error::Error>> {
            // 模拟执行逻辑
            Ok(())
        }

        fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            // 模拟停止逻辑
            Ok(())
        }
    }
    
    // 使用封装函数创建 Executor 实例
    let (sender, receiver) = mpsc::channel(10);
    let mut executor = create_executor::<MockStrategy>(Arc::new(quote_ctx), Arc::new(trade_ctx), receiver);

    // 测试运行方法
    let result = executor.run().await;
    assert!(result.is_ok(), "运行策略失败");

    // 测试执行方法
    let market_data = MarketData {
        // 填充必要的字段
        symbol: "".to_string(),
        price: 0.0,
        change: 0.0,
        volume: 0,
        high: 0.0,
        low: 0.0,
        open: 0.0,
        close: 0.0,
    };
    let execute_result = executor.execute(&market_data).await;
    assert!(execute_result.is_ok(), "执行策略失败");

    // 测试停止方法
    let stop_result = executor.stop();
    assert!(stop_result.is_ok(), "停止策略失败");
}