use std::sync::Arc;
use aqt_stock::strategys::strategy::Strategy;
use longport::{Config, Decimal, QuoteContext, TradeContext};
use tokio::sync::mpsc;
use aqt_stock::config::config::Configs;
use aqt_stock::models::market::MarketData;
use aqt_stock::strategys::executor::Executor;

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
    }

    impl Strategy for MockStrategy {
        fn new(
            quote_ctx: Arc<QuoteContext>,
            trade_ctx: Arc<TradeContext>,
        ) -> Self {
            MockStrategy {
                quote_ctx,
                trade_ctx,
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
    let config = Configs::load().expect("TODO: panic message");
    // 使用封装函数创建 Executor 实例
    let (sender, receiver) = mpsc::channel(10);
    // 创建执行器并保存在变量中
    let mut executor = Executor::<MockStrategy>::new(
        Arc::new(quote_ctx),
        Arc::new(trade_ctx),
        receiver
    );

    // 测试运行方法
    let result = executor.run().await;
    assert!(result.is_ok(), "运行策略失败");

    // 测试执行方法
    let market_data = MarketData {
        // 填充必要的字段
        symbol: "".to_string(),
        price: Decimal::try_from(0.0).unwrap(),
        change:Decimal::try_from(0.0).unwrap(),
        volume:0,
        high:Decimal::try_from(0.0).unwrap(),
        low:Decimal::try_from(0.0).unwrap(),
        open:Decimal::try_from(0.0).unwrap(),
        close: Decimal::try_from(0.0).unwrap(),
        ts: time::OffsetDateTime::now_utc(),
    };

    // 在单独的任务中运行执行器
    let executor_handle = tokio::spawn(async move {
        if let Err(e) = executor.run().await {
            eprintln!("Executor error: {}", e);
        }
    });

    sender.send(market_data).await.unwrap();
    drop(sender);
    // 等待执行器完成
    executor_handle.await.unwrap_or_else(|e| {
        eprintln!("Executor task error: {}", e);
    });
}