use longport::{Config, quote::{QuoteContext, SubFlags}};
use std::sync::Arc;
use longport::quote::{PushEvent, PushEventDetail};
use tokio::sync::mpsc;
use crate::models::market::MarketData;

/// QuoteCollector 结构体用于管理行情订阅的上下文和接收器
pub struct QuoteCollectors {
    ctx: QuoteContext, // 行情上下文，用于处理订阅和取消订阅操作
    receiver: mpsc::UnboundedReceiver<PushEvent>, // 接收推送事件的消息接收器
    symbols: Vec<String>, // 当前关注的股票代码列表
    sub_flags: SubFlags, // 订阅标志，指定订阅的数据类型
}

impl QuoteCollectors {
    /// 创建一个新的 QuoteCollector 实例
    ///
    /// # 参数
    ///
    /// * `symbols` - 要订阅的股票代码列表
    ///
    /// # 返回值
    ///
    /// 返回一个新创建的 QuoteCollector 实例
    pub async fn new(symbols: Vec<String>) -> Self {
        let config = Arc::new(Config::from_env().unwrap()); // 从环境变量中加载配置
        // println!("{:?}",config);
        let (ctx, receiver) = QuoteContext::try_new(config).await.expect("QuoteContext init err"); // 初始化行情上下文和接收器
        QuoteCollectors {
            ctx,
            receiver,
            symbols,
            sub_flags:SubFlags::QUOTE, // 默认订阅报价数据
        }
    }

    /// 订阅当前保存的股票代码的行情数据
    pub async fn subscribe(&mut self, sender: mpsc::Sender<MarketData>) {
        self.ctx.subscribe(&self.symbols, self.sub_flags, true).await.unwrap();
        while let Some(msg) = self.receiver.recv().await {
            if let PushEventDetail::Quote(detail) = msg.detail {
                let market_data = MarketData {
                    symbol: msg.symbol,
                    price: detail.last_done,
                    change: detail.last_done - detail.open,
                    volume: detail.volume,
                    high: detail.high,
                    low: detail.low,
                    open: detail.open,
                    close: detail.last_done,
                    ts: detail.timestamp,
                };
                if let Err(e) = sender.send(market_data).await {
                    eprintln!("Failed to send market data: {}", e);
                }
            }
        }
    }

    /// 取消订阅指定的股票代码的行情数据
    ///
    /// # 参数
    ///
    /// * `symbols` - 要取消订阅的股票代码列表
    pub async fn unsubscribe(&mut self, symbols: Vec<String> ) {
        self.ctx.unsubscribe(symbols, self.sub_flags).await.unwrap(); // 取消订阅行情数据
        while let Some(msg) = self.receiver.recv().await {
            println!("{:?}", msg); // 处理接收到的推送消息
        }
    }
}
