use longport::{Config, quote::{SubFlags}, TradeContext, decimal};
use std::sync::Arc;
use longport::trade::{OrderSide, OrderType, PushEvent, SubmitOrderOptions, TimeInForceType};
use tokio::sync::mpsc;

/// TradeCollector 结构体用于管理交易订阅的上下文和接收器
pub struct TradeCollectors {
    ctx: TradeContext, // 交易上下文，用于处理订阅和取消订阅操作
    receiver: mpsc::UnboundedReceiver<PushEvent>, // 接收推送事件的消息接收器
    symbols: Vec<String>, // 当前关注的股票代码列表
    sub_flags: SubFlags, // 订阅标志，指定订阅的数据类型
}

impl TradeCollectors {
    /// 创建一个新的 TradeCollector 实例
    ///
    /// # 参数
    ///
    /// * `symbols` - 要订阅的股票代码列表
    ///
    /// # 返回值
    ///
    /// 返回一个新创建的 TradeCollector 实例
    pub async fn new(symbols: Vec<String>) -> Self {
        let config = Arc::new(Config::from_env().unwrap()); // 从环境变量中加载配置
        let (ctx, receiver) = TradeContext::try_new(config).await.expect("TradeContext init err"); // 初始化交易上下文和接收器
        TradeCollectors {
            ctx,
            receiver,
            symbols,
            sub_flags:SubFlags::TRADE, // 默认订阅交易数据
        }
    }

    /// 订阅当前保存的股票代码的交易数据
    pub async fn subscribe(&mut self) {
        while let Some(msg) = self.receiver.recv().await {
            println!("{:?}", msg); // 处理接收到的推送消息
        }
    }

    /// 取消订阅指定的股票代码的交易数据
    ///
    /// # 参数
    ///
    /// * `symbols` - 要取消订阅的股票代码列表
    pub async fn unsubscribe(&mut self) {}
}