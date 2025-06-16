use longport::{Config, quote::{QuoteContext, SubFlags}, TradeContext};
use std::sync::Arc;
use longport::quote::PushEvent;
use longport::trade::PushEvent as TradePushEvent;
use tokio::sync::mpsc;

/// `Context` trait 定义了上下文需要实现的方法，支持泛型化设计。
pub trait Context {
    /// 尝试创建一个新的上下文实例。
    ///
    /// # 参数
    /// - `config`: 配置的引用计数实例。
    ///
    /// # 返回值
    /// 返回一个包含上下文和消息接收器的元组。
    async fn try_new(config: Arc<Config>) -> (Self, mpsc::UnboundedReceiver<PushEvent>);

    /// 订阅指定的股票代码。
    ///
    /// # 参数
    /// - `symbols`: 股票代码列表。
    /// - `sub_flags`: 订阅标志。
    /// - `is_first`: 是否为首次订阅。
    ///
    /// # 返回值
    /// 返回 `Result`，表示订阅是否成功。
    async fn subscribe(&self, symbols: &[String], sub_flags: SubFlags, is_first: bool) -> Result<(), Box<dyn std::error::Error>>;

    /// 取消订阅指定的股票代码。
    ///
    /// # 参数
    /// - `symbols`: 股票代码列表。
    /// - `sub_flags`: 订阅标志。
    ///
    /// # 返回值
    /// 返回 `Result`，表示取消订阅是否成功。
    async fn unsubscribe(&self, symbols: Vec<String>, sub_flags: SubFlags) -> Result<(), Box<dyn std::error::Error>>;
}

/// 为 QuoteContext 实现 Context trait
impl Context for QuoteContext {
    async fn try_new(config: Arc<Config>) -> (Self, mpsc::UnboundedReceiver<PushEvent>) {
        QuoteContext::try_new(config).await
    }

    async fn subscribe(&self, symbols: &[String], sub_flags: SubFlags, is_first: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.subscribe(symbols, sub_flags, is_first).await
    }

    async fn unsubscribe(&self, symbols: Vec<String>, sub_flags: SubFlags) -> Result<(), Box<dyn std::error::Error>> {
        self.unsubscribe(symbols, sub_flags).await
    }
}

/// 为 TradeContext 实现 Context trait
impl Context for TradeContext {
    async fn try_new(config: Arc<Config>) -> (Self, mpsc::UnboundedReceiver<TradePushEvent>) {
        TradeContext::try_new(config).await
    }

    async fn subscribe(&self, symbols: &[String], sub_flags: SubFlags, is_first: bool) -> Result<(), Box<dyn std::error::Error>> {
        self.subscribe(symbols, sub_flags, is_first).await
    }

    async fn unsubscribe(&self, symbols: Vec<String>, sub_flags: SubFlags) -> Result<(), Box<dyn std::error::Error>> {
        self.unsubscribe(symbols, sub_flags).await
    }
}

// 新增：为 QuoteContext 和 TradeContext 提供具体的 Collector 实现
pub type QuoteCollector = Collector<QuoteContext>;
pub type TradeCollector = Collector<TradeContext>;

impl QuoteCollector {
    /// 创建一个新的 `QuoteCollector` 实例。
    ///
    /// # 参数
    /// - `symbols`: 需要订阅的股票代码列表。
    /// - `sub_flags`: 订阅标志，指定订阅的数据类型。
    ///
    /// # 返回值
    /// 返回一个初始化完成的 `QuoteCollector` 实例。
    pub async fn new(symbols: Vec<String>, sub_flags: SubFlags) -> Self {
        let config = Arc::new(Config::from_env().unwrap());
        let (ctx, receiver) = QuoteContext::try_new(config).await.expect("QuoteContext init err");
        QuoteCollector {
            ctx,
            receiver,
            symbols,
            sub_flags,
        }
    }
}

impl TradeCollector {
    /// 创建一个新的 `TradeCollector` 实例。
    ///
    /// # 参数
    /// - `symbols`: 需要订阅的股票代码列表。
    /// - `sub_flags`: 订阅标志，指定订阅的数据类型。
    ///
    /// # 返回值
    /// 返回一个初始化完成的 `TradeCollector` 实例。
    pub async fn new(symbols: Vec<String>, sub_flags: SubFlags) -> Self {
        let config = Arc::new(Config::from_env().unwrap());
        let (ctx, receiver) = TradeContext::try_new(config).await.expect("TradeContext init err");
        TradeCollector {
            ctx,
            receiver,
            symbols,
            sub_flags,
        }
    }
}